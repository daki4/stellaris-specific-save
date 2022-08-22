cargo build --release

mkdir release
Copy-Item -Path target\release\main.exe -Destination .\release\backup-script.exe
Copy-Item settings.json ./release/settings.json

Compress-Archive ./release release.zip

Remove-Item -Recurse release
# cargo clean
