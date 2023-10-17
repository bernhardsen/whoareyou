FROM rust:1.73-slim-buster
WORKDIR /whoareyou
COPY ./ ./
RUN cargo install --path .
CMD ["whoareyou"]
