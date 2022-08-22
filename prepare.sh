cargo build --release

mkdir release
cp target/release/main ./release/backup-script
cp settings.json ./release/settings.json

sudo chmod +x ./release/backup-script

zip -r release.zip ./release

rm -rf release
cargo clean