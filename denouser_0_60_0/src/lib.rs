mod deno_core_main;

#[macro_use]
extern crate log;

use android_logger::Config;
use jni::objects::{JClass, JObject};
use jni::sys::jstring;
use jni::JNIEnv;
use log::Level;

#[no_mangle]
pub extern "system" fn Java_com_atstudio_denouser_DenoService_stringFromRustJNI(
    env: JNIEnv,
    context: JObject,
)  {
      android_logger::init_once(
        Config::default()
            .with_min_level(Level::Trace)
            .with_tag("Rust"),
    );

    log::info!("Logging initialised from Rust");

    std::env::set_var("NO_COLOR", "true");
}

#[no_mangle]
pub extern "system"  fn Java_com_atstudio_denouser_DenoService_runTest(
    env: JNIEnv,
    context: JObject) {
    android_logger::init_once(Config::default().with_min_level(Level::Trace));
    info!("start runTest");

    // deno_core_main::run_main();
}