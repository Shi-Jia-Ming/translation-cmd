cargo build --release

copy ".\target\conf\config.toml" "C:\Program Files\Trans\conf\config.toml"
copy ".\target\release\trans.exe" "C:\Program Files\Trans\bin\trans.exe"
