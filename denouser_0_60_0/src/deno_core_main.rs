use deno_core::JsRuntime;
use deno_core::RuntimeOptions;

pub fn run_main() {
    JsRuntime::new(RuntimeOptions::default());
}

#[cfg(test)]
mod tests {
    use super::run_main;

    #[test]
    fn it_works() {
        run_main();
    }
}
