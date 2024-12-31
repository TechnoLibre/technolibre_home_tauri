# Installation
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
. "$HOME/.cargo/env"

cargo install tauri-cli
cargo tauri build
cargo install create-tauri-app
cargo-create-tauri-app

rustup target add aarch64-linux-android armv7-linux-androideabi i686-linux-android x86_64-linux-android
export NDK_HOME="$ANDROID_HOME/ndk/$(ls -1 $ANDROID_HOME/ndk)"

cargo install --locked trunk
cargo install --locked wasm-bindgen-cli
rustup target add wasm32-unknown-unknown

cargo tauri android init
cargo tauri android dev