use cxx_qt::QObject;
use std::pin::Pin;

use crate::service::ffi::Service as CxxService;

#[derive(QObject, Default)]
pub struct ServiceRef {
    service: *mut CxxService,
}

impl ServiceRef {
    #[qproperty(cpp_name = "service")]
    pub fn service(&self) -> *mut CxxService {
        self.service
    }

    #[qproperty(cpp_name = "service")]
    pub fn set_service(self: Pin<&mut Self>, new_service: *mut FxxService) {
        if self.service == new_service {
            return;
        }

        if !self.service.is_null() {
            unsafe {
                if let Some(old_srv) = self.service.as_mut() {
                    old_srv.unref_service();
                }
            }
        }

        self.service = new_service;

        self.as_ref().service_changed();

        if !new_service.is_null() {
            unsafe {
                if let Some(new_srv) = new_service.as_mut() {
                    new_srv.ref_service();
                }
            }
        }
    }

    #[cxx_qt::qsignal]
    fn service_changed(&self);
}

pub fn register() {
    qml_register_type::<ServiceRef>("Vela", 1, 0, "ServiceRef");
}
