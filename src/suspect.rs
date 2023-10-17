
use rand::prelude::*;
use rand_seeder::{Seeder};
use rand_pcg::Pcg64;
use std::string::String;
use serde::Serialize;
use serde::Deserialize;

#[derive(Serialize)]
pub(crate) struct Suspect {
    pub(crate) name: String, // The name is the ID
    pub(crate) gender: Gender,
    pub(crate) hair_color: HairColor,
    pub(crate) hair_length: HairLength,
    pub(crate) nose: Nose,
    pub(crate) eye_color: EyeColor,
    pub(crate) glasses: Glasses,
    pub(crate) shirt: Shirt,
}

impl Suspect {
    pub(crate) fn from_name(name: &String, gender: Gender) -> Suspect {
        let mut rng: Pcg64 = Seeder::from(&name).make_rng();
        println!("Generating character: {}", name);

        Suspect {
            name: name.clone(),
            gender: gender,
            hair_color: random_hair_color(&mut rng),
            hair_length: random_hair_length(&mut rng, gender),
            nose: random_nose(&mut rng),
            eye_color: random_eye_color(&mut rng),
            glasses: random_glasses(&mut rng),
            shirt: random_shirt(&mut rng),
        }
    }
}

fn random_hair_color(rng: &mut Pcg64) -> HairColor {
    match rng.gen_range(0..4) {
        0 => HairColor::RED,
        1 => HairColor::BROWN,
        2 => HairColor::BLONDE,
        _ => HairColor::GRAY,
    }
}

fn random_hair_length(rng: &mut Pcg64, gender: Gender) -> HairLength {
    // Males get short hair 1 out of 2 times, while females get 1 out of 3
    let range = match gender { Gender::Male => 2, Gender::Female => 3 };
    if rng.gen_range(0..range) == 0 { HairLength::SHORT } else { HairLength::LONG }
}

fn random_nose(rng: &mut Pcg64) -> Nose {
    if rng.gen() { Nose::BIG } else { Nose::SMALL }
}

fn random_eye_color(rng: &mut Pcg64) -> EyeColor {
    match rng.gen_range(0..3) {
        0 => EyeColor::BLUE,
        1 => EyeColor::BROWN,
        _ => EyeColor::GREEN
    }
}

fn random_glasses(rng: &mut Pcg64) -> Glasses {
    // The RNG range is set intentionally higher to give ~43% chance of having glasses
    match rng.gen_range(0..7) {
        0 => Glasses::BLACK,
        1 => Glasses::BLUE,
        2 => Glasses::RED,
        _ => Glasses::NONE,
    }
}

fn random_shirt(rng: &mut Pcg64) -> Shirt {
    match rng.gen_range(0..3) {
        0 => Shirt::BLUE,
        1 => Shirt::WHITE,
        _ => Shirt::BLACK,
    }
}

#[derive(Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub(crate) enum Gender {
    Male,
    Female,
}

#[derive(Serialize)]
pub(crate) enum HairColor {
    RED,
    BROWN,
    BLONDE,
    GRAY,
}

#[derive(Serialize)]
pub(crate) enum HairLength {
    SHORT,
    LONG,
}

#[derive(Serialize)]
pub(crate) enum Nose {
    SMALL,
    BIG,
}

#[derive(Serialize)]
pub(crate) enum EyeColor {
    BLUE,
    BROWN,
    GREEN,
}

#[derive(Serialize)]
pub(crate) enum Glasses {
    NONE,
    BLUE,
    RED,
    BLACK,
}

#[derive(Serialize)]
pub(crate) enum Shirt {
    BLUE,
    WHITE,
    BLACK,
}

