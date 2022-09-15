mod deno_core_main;

#[macro_use]
extern crate log;

use android_logger::Config;
use jni::objects::JClass;
use jni::sys::jstring;
use jni::JNIEnv;
use log::Level;

#[no_mangle]
pub extern "C" fn Java_com_atstudio_denouser_MainActivity_stringFromRustJNI(
    env: JNIEnv,
    _thiz: JClass,
) -> jstring {
    env.new_string("String from Rust")
        .expect("Can't get string from env.")
        .into_inner()
}

#[no_mangle]
pub extern "C" fn Java_com_atstudio_denouser_MainActivity_runTest(_env: JNIEnv, _thiz: JClass) {
    android_logger::init_once(Config::default().with_min_level(Level::Trace));
    info!("start runTest");

    deno_core_main::run_main();
}
