echo "> Start of build for platform: i686-pc-windows-msvc"
$env:RUSTFLAGS="-C opt-level=s"
cargo build --release --target=i686-pc-windows-msvc
$env:RUSTFLAGS=""
upx --best .\target\i686-pc-windows-msvc\release\todo.exe
echo "/ End of build for platform: i686-pc-windows-msvc\n\n"

echo "> Start of build for platform: x86_64-pc-windows-msvc"
$env:RUSTFLAGS="-C opt-level=s"
cargo build --release --target=x86_64-pc-windows-msvc
$env:RUSTFLAGS=""
upx --best .\target\x86_64-pc-windows-msvc\release\todo.exe
echo "/ End of build for platform: x86_64-pc-windows-msvc\n\n"
