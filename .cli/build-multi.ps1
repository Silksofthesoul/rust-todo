cls
echo "\n\n>>>>> Build for all platforms\n\n\n"

echo "> Start of build for platform: x86_64-pc-windows-msvc"
cargo build --release --target=x86_64-pc-windows-msvc
echo "/ End of build for platform: x86_64-pc-windows-msvc\n\n"

echo "> Start of build for platform: i686-pc-windows-msvc"
cargo build --release --target=i686-pc-windows-msvc
echo "/ End of build for platform: i686-pc-windows-msvc\n\n"

echo "> Start of build for platform: aarch64-pc-windows-msvc"
cargo build --release --target=aarch64-pc-windows-msvc
echo "/End of build for platform: aarch64-pc-windows-msvc\n\n"

echo "> Start of build for platform: x86_64-unknown-linux-gnu"
cargo build --release --target=x86_64-unknown-linux-gnu
echo "/ End of build for platform: x86_64-unknown-linux-gnu\n\n"

echo "> Start of build for platform: i686-unknown-linux-gnu"
cargo build --release --target=i686-unknown-linux-gnu
echo "/ End of build for platform: i686-unknown-linux-gnu\n\n"

echo "> Start of build for platform: aarch64-unknown-linux-gnu"
cargo build --release --target=aarch64-unknown-linux-gnu
echo "/ End of build for platform: aarch64-unknown-linux-gnu\n\n"

