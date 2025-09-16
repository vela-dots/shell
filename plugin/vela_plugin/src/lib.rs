mod app_entry;
mod appdb;
mod audio_collector;
mod cutils;
mod qalculator;
mod service;
mod service_ref;

#[cxx::bridge(namespace = "Vela")]
mod ffi {
    extern "Rust" {
        fn register_types();
    }
}

pub fn register_types() {
    cutils::register();
    qalculator::register();
    app_entry::register();
    appdb::register();
    service::register();
    service_ref::register();
    audio_collector::register();
}
