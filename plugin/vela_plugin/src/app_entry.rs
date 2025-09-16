use cxx_qt::QObject;
use qt6_core::{QString, QUrl};

#[derive(QObject, Default)]
pub struct AppEntry {
    #[qproperty]
    pub id: QString,

    #[qproperty]
    pub name: QString,

    #[qproperty]
    pub desc: QString,

    #[qproperty(cpp_name = "execString")]
    pub exec_string: QString,

    #[qproperty(cpp_name = "wmClass")]
    pub wm_class: QString,

    #[qproperty(cpp_name = "genericName")]
    pub generic_name: QString,

    #[qproperty]
    pub categories: QString,

    #[qproperty]
    pub keywords: QString,

    #[qproperty]
    pub frequency: u32,
}

pub fn register() {
    qml_register_type::<AppEntry>("Vela", 1, 0, "AppEntry");
}
