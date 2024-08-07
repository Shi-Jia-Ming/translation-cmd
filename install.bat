@echo off
cargo build --release

@echo off
copy ".\target\conf\config.toml" "C:\Program Files\Trans\conf\config.toml" >nul
copy ".\target\release\trans.exe" "C:\Program Files\Trans\bin\trans.exe" >nul

echo Install finished!
