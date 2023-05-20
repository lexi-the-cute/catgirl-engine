cd ../..
mkdir -p target/libs

# Build SDL
emcmake cmake -G Ninja -S android/app/jni -B android/app/jni/build -DBUILD_SHARED_LIBS=off
emcmake ninja -C android/app/jni/build

mkdir -p target/libs
cp -av android/app/jni/build/*/*.a target/libs
ls -liallh target/libs

# Build Catgirl Engine
cargo build --target wasm32-unknown-emscripten --release --lib

python3 start.py