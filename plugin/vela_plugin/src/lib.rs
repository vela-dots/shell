mod cutils;
mod qalculator;

#[cxx::bridge(namespace = "Vela")]
mod ffi {
    extern "Rust" {
        fn register_types();
    }
}

pub fn register_types() {
    cutils::register();
    qalculator::register();
}
