use cxx_qt::{QMutex, QMutexLocker, QObject};
use std::sync::{Arc, Mutex};

#[derive(QObject)]
pub struct Service {
    inner: Arc<Mutex<Inner>>,
}

struct Inner {
    ref_count: i32,
}

impl Default for Service {
    fn default() -> Self {
        Self {
            inner: Arc::new(Mutex::new(Inner { ref_count: 0 })),
        }
    }
}

impl Service {
    #[qproperty(cpp_name = "refCount")]
    pub fn ref_count(&self) -> i32 {
        let inner = self.inner.lock().unwrap();
        inner.ref_count
    }

    #[qinvokable]
    pub fn ref_service(&self) {
        let mut needs_start = false;
        {
            let mut inner = self.inner.lock().unwrap();
            if inner.ref_count == 0 {
                needs_start = true;
            }
            inner.ref_count += 1;
        }
        self.refCountChanged();
        if needs_start {
            self.start();
        }
    }

    #[qinvokable]
    pub fn unref_service(&self) {
        let mut needs_stop = false;
        {
            let mut inner = self.inner.lock().unwrap();
            if inner.ref_count == 0 {
                return;
            }
            inner.ref_count -= 1;
            if inner.ref_count == 0 {
                needs_stop = true;
            }
        }
        self.refCountChanged();
        if needs_stop {
            self.stop();
        }
    }

    #[cxx_qt::qsignal]
    fn refCountChanged(&self);

    #[cxx_qt::qinvokable]
    pub fn start(&self) {
        // TODO: Override in subclass
    }

    #[cxx_qt::qinvokable]
    pub fn stop(&self) {
        // TODO: Override in subclass
    }
}

pub fn register() {
    qml_register_type::<Service>("Vela", 1, 0, "Service");
}
