default: watch

watch:
  @cargo watch -x "run --release"

build:
  @cargo build --release
  @cp ./target/release/portfolio-old-web-backend .

update:
  @just build
  @mv ./portfolio-old-web-backend ../portfolio-old-web-frontend/portfolio-old-bin
