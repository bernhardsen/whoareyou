use std::io::Cursor;
use image::{open, imageops, DynamicImage};
use rand::Rng;
use rand_pcg::Pcg64;
use rand_seeder::Seeder;
use crate::suspect::{EyeColor, Gender, Glasses, HairColor, HairLength, Nose, Shirt, Suspect};

static BACKGROUND_IMAGE: &str = "img/background.png";

pub(crate) fn make_face(sus: &Suspect) -> Vec<u8> {

    let base = open(BACKGROUND_IMAGE).unwrap();
    let images_to_apply = choose_images(&sus);
    let face_image = paint_face(base, images_to_apply);
    // Save to output.png as a test
    face_image.save(String::from("output.png")).unwrap();
    let mut buffer = Vec::new();
    let mut writer = Cursor::new(&mut buffer);
    face_image.write_to(&mut writer, image::ImageOutputFormat::Png).unwrap();
    buffer
}

fn paint_face(mut buffer: DynamicImage, images: Vec<FacialFeatureImage>) -> DynamicImage {
    for image in images {
        let feature_img = open(image.path).unwrap();
        imageops::overlay(&mut buffer, &feature_img, 0, 0);
    }
    buffer
}

fn choose_images(sus: &Suspect) -> Vec<FacialFeatureImage> {
    let mut rng: Pcg64 = Seeder::from(&sus.name).make_rng();
    let mut images: Vec<FacialFeatureImage> = Vec::new();
    // Pretend there is logic here
    images.push(get_shirt(&sus.shirt, &mut rng));
    images.push(get_mouth(&sus.gender, &mut rng));
    images.push(get_nose(&sus.nose, &mut rng));
    images.push(get_eyes(&sus.eye_color, &sus.gender, &mut rng));
    match sus.glasses {
        Glasses::RED | Glasses::BLACK | Glasses::BLUE => images.push(get_glasses(&sus.glasses, &mut rng)),
        _ => ()
    }
    images.push(get_hair(&sus.hair_color, &sus.hair_length, &mut rng));
    images
}

fn get_shirt(shirt: &Shirt, rng: &mut Pcg64) -> FacialFeatureImage {
    let color = match shirt { Shirt::WHITE => "white", Shirt::BLACK => "black", Shirt::BLUE => "blue" };
    let variant = if rng.gen() { "1" } else { "2" };
    FacialFeatureImage { path: format!("img/shirt_{}_{}.png", color, variant) }
}

fn get_mouth(gender: &Gender, rng: &mut Pcg64) -> FacialFeatureImage {
    let gender_str = match gender { Gender::Male => "male", Gender::Female => "female" };
    let variant = rng.gen_range(1..5);
    FacialFeatureImage { path: format!("img/mouth_{}_{}.png", gender_str, variant) }
}

fn get_nose(nose: &Nose, rng: &mut Pcg64) -> FacialFeatureImage {
    let size = match nose { Nose::BIG => "big", Nose::SMALL => "small" };
    let variant = if rng.gen() { "1" } else { "2" };
    FacialFeatureImage { path: format!("img/{}_nose_{}.png", size, variant) }
}

fn get_eyes(eyes: &EyeColor, gender: &Gender, rng: &mut Pcg64) -> FacialFeatureImage {
    let color = match eyes { EyeColor::BLUE => "blue", EyeColor::GREEN => "green", EyeColor::BROWN => "brown" };
    let gender_str = match gender { Gender::Male => "male", Gender::Female => "female" };
    let variant = if rng.gen() { "1" } else { "2" };
    FacialFeatureImage { path: format!("img/{}_eyes_{}_{}.png", color, gender_str, variant) }
}

fn get_glasses(glasses: &Glasses, rng: &mut Pcg64) -> FacialFeatureImage {
    let color = match glasses {
        Glasses::BLUE => "blue",
        Glasses::BLACK => "black",
        Glasses::RED => "red",
        Glasses::NONE => "none",
    };
    let variant = if rng.gen() { "1" } else { "2" };
    FacialFeatureImage { path: format!("img/glasses_{}_{}.png", color, variant) }
}

fn get_hair(color: &HairColor, hair_length: &HairLength, rng: &mut Pcg64) -> FacialFeatureImage {
    let color = match color {
        HairColor::RED => "red",
        HairColor::GRAY => "gray",
        HairColor::BROWN => "brown",
        HairColor::BLONDE => "blonde",
    };
    let length = match hair_length { HairLength::LONG => "long", HairLength::SHORT => "short" };
    let variant = if rng.gen() { "1" } else { "2" };
    FacialFeatureImage { path: format!("img/{}_{}_hair_{}.png", color, length, variant) }
}

struct FacialFeatureImage {
    path: String,
}
