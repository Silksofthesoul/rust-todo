$env:RUSTFLAGS="-C opt-level=s"
cargo build --release
$env:RUSTFLAGS=""
upx --best .\target\release\todo.exe
