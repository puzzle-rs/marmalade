use web_sys::console;

pub fn log(value: &str) {
    console::log_1(&value.into());
}
