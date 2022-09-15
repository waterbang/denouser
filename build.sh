#!/bin/bash

export CC=/home/autsing/Apps/androidSdk/ndk/android-ndk-r21e/toolchains/llvm/prebuilt/linux-x86_64/bin/aarch64-linux-android21-clang
export AR=/home/autsing/Apps/androidSdk/ndk/android-ndk-r21e/toolchains/llvm/prebuilt/linux-x86_64/bin/aarch64-linux-android-ar
export CARGO_TARGET_AARCH64_LINUX_ANDROID_LINKER=/home/autsing/Apps/androidSdk/ndk/android-ndk-r21e/toolchains/llvm/prebuilt/linux-x86_64/bin/aarch64-linux-android21-clang

cargo build --target aarch64-linux-android

cp ./target/aarch64-linux-android/debug/libdenouser.so ~/OneDrive/Gits/Java/DenoUser/app/libs/arm64-v8a/
