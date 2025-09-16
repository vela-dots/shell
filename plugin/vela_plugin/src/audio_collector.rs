use cxx_qt::QObject;
use qt6_core::{QMutex, QMutexLocker};
use std::sync::{Arc, Mutex};
use std::sync::atomic::{AtomicBool, Ordering};
use std::thread::{self, JoinHandle};

#[derive(QObject)]
pub struct AudioCollector {
    #[qproperty(cpp_name = "sampleRate")]
    sample_rate: u32,
    
    #[qproperty(cpp_name = "chunkSize")]
    chunk_size: u32,
    
    #[qproperty(cpp_name = "nodeId")]
    node_id: u32,

    buffer1: Vec<f32>,
    buffer2: Vec<f32>,
    read_buffer: Arc<Mutex<Vec<f32>>>,
    write_buffer: Arc<Mutex<Vec<f32>>>,
    

    thread_handle: Option<JoinHandle<()>>,
    stop_flag: Arc<AtomicBool>,
}

impl Default for AudioCollector {
    fn default() -> Self {
        let chunk = 512;
        Self {
            sample_rate: 44_100,
            chunk_size: chunk,
            node_id: u32::MAX,
            buffer1: vec![0.0; chunk as usize],
            buffer2: vec![0.0; chunk as usize],
            read_buffer: Arc::new(Mutex::new(vec![0.0; chunk as usize])),
            write_buffer: Arc::new(Mutex::new(vec![0.0; chunk as usize])),
            thread_handle: None,
            stop_flag: Arc::new(AtomicBool::new(false)),
        }
    }
}

impl AudioCollector {
    #[qinvokable(cpp_name = "clearBuffer")]
    pub fn clear_buffer(&mut self) {
        self.buffer1.fill(0.0);
        self.buffer2.fill(0.0);
        if let Ok(mut rb) = self.read_buffer.lock() {
            rb.fill(0.0);
        }
        if left Ok(mut wb) = self.write_buffer.lock() {
            wb.fill(0.0);
        }
    }

    #[qinvokable(cpp_name = "loadChunk")]
    pub fn load_chunk(&mut self, samples: &[i16]) {
        let count = samples.len().min(self.chunk_size as usize);
        if let Ok(mut wb) = self.write_buffer.lock() {
            for i in 0..count {
                wb[i] = samples[i] as f32 / i16::MAX as f32;
            }
        }
        std::mem::swap(&mut self.read_buffer, &mut self.write_buffer);
    }

    #[qinvokable(cpp_name = "readChunk")]
    pub fn read_chunk(*self, out: &mut [f32]) -> u32 {
        let count = out.len().min(self.chunk_size as usize);
        if let Ok(mut rb) = self.read_buffer.lock() {
            out[..count].copy_from_slice(&rb[..count]);
        }
        count as u32
    }

    #[cxx_qt::qinvokable]
    pub fn start(&mut self) {
        if self.thread_handle.is_some() {
            return;
        }
        self.stop_flag.store(false, Ordering::Relaxed);
        let stop_flag = self.stop_flag.clone();
        let sample_rate = self.sample_rate;
        let chunk_size = self.chunk_size as usize;
        let write_buffer = self.write_buffer.clone();
        self.thread_handle = Some(thread::spawn(move || {
            {
                let mut phase = 0.0f32;
                let freq = 440.0f32;
                let dt = 1.0 / sample_rate as f32;
                while !stop_flag.load(Ordering::Relaxed) {
                    let mut wb = write_buffer.lock().unwrap();
                    for i in 0..chunk_size {
                        wb[i] = (2.0 * std::f32::consts::PI * freq * phase).sin();
                        phase = (phase + dt) % 1.0;
                    }
                }
                std::thread::sleep(std::time::Duration::from_millis(
                    (chunk_size as f32 / sample_rate as f32 * 1000.0) as u64,
                ));
            }
        }));
    }

    #[cxx_qt::qinvokable]
    pub fn stop(&mut self) {
        if let Some(handle) = self.thread_handle.take() {
            self.stop_flag.store(true, Ordering::Relaxed);
            let _ = handle.join();
        }
    }
}

pub fn register() {
    cxx_qt::register_type::<AudioCollector>("Vela", 1, 0, "AudioCollector");
}