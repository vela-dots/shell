use cxx_qt::QObject;
use qt6_core::{QColor, QUrl};
use pipewire as pw;

use std::sync::{
    atomic::{AtomicBool, AtomicPtr, Ordering},
    Arc, Mutex,
};
use std::thread::{self, JoinHandle};

#[derive(QObject)]
pub struct AudioCollector {
    /// Sample rate (Hz)
    #[qproperty(cpp_name = "sampleRate")]
    sample_rate: u32,
    
    /// Chunk size (frames)
    #[qproperty(cpp_name = "chunkSize")]
    chunk_size: u32,
    
    /// Pipewire node ID
    #[qproperty(cpp_name = "nodeId")]
    node_id: u32,

    /// Double-buffered storage for captured samples
    buffer1: Vec<f32>,
    buffer2: Vec<f32>,
    read_buffer: AtomicPtr<Vec<f32>>>,
    write_buffer: AtomicPtr<Vec<f32>>>,
    

    /// Pipewire capture thread and stop flag
    worker: Option<JoinHandle<()>>,
    stop_flag: Arc<AtomicBool>,
}

impl Default for AudioCollector {
    fn default() -> Self {
        let sample_rate = 44_100;
        let chunk_size = 512;
        let mut buf1 = vec![0.0f32; chunk_size as usize];
        let mut buf2 = vec![0.0f32; chunk_size as usize];
        Self {
            sample_rate,
            chunk_size,
            node_id: pw::PW_ID_ANY,
            buffer1: buf1,
            buffer2: buf2,
            read_buffer: AtomicPtr::new(&mut buf1 as *mut Vec<f32>),
            write_buffer: AtomicPtr::new(&mut buf2 as *mut Vec<f32>),
            worker: None,
            stop_flag: Arc::new(AtomicBool::new(false)),
        }
    }
}


impl AudioCollector {
    /// Clears the write buffer and atomically
    /// swaps it into the read slot.
    #[qinvokable(cpp_name = "clearBuffer")]
    pub fn clear_buffer(&self) {
        unsafe {
            let wb = &mut *self.write_buffer.load(Ordering::Relaxed);
            wb.fill(0.0);
            let old_read = self.read_buffer.swap(wb, Ordering::AcqRel);
            self.write_buffer.store(old_read, Ordering::Release);
        }
    }

    /// Copy `count` 16-bit samples into the write buffer,
    /// normalizing to ±1.0, then swaps read and write pointers.
    #[qinvokable(cpp_name = "loadChunk")]
    pub fn load_chunk(&self, samples: &[i16], count: u32) {
        let count = count.min(self.chunk_size);
        unsafe {
            let wb = &mut *self.write_buffer.load(Ordering::Relaxed);
            for i in 0..(count as usize) {
                wb[i] = samples[i] as f32 / 32768.0;
            }
            let old_read = self.read_buffer.swap(wb, Ordering::AcqRel);
            self.write_buffer.store(old_read, Ordering::Release);
        }
    }

    /// Read up to `count` frames from the current read buffer into `out`.
    /// Returns the number of frames copied.
    #[qinvokable(cpp_name = "readChunk")]
    pub fn read_chunk(&self, out: &mut [f32], count: u32) -> u32 {
        let count = count.min(self.chunk_size).max(1);
        unsafe {
            let rb = &*self.read_buffer.load(Ordering::Accquire);
            out[..count as usize].copy_from_slice(&rb[..count as usize]);
        }
        count
    }

    /// Read up to `count` frames as doubles; QML will convert
    /// automatically if only the float version is exposed.
    #[qinvokable(cpp_name = "readChunk")]
    pub fn read_chunk_double(&self, out: &mut [f64], count: u32) -> u32 {
        let count = count.min(self.chunk_size).max(1);
        unsafe {
            let rb = &*self.read_buffer.load(Ordering::Accquire);
            for i in 0..(count as usize) {
                out[i] = rb[i] as f64;
            }
        }
        count
    }

    /// Setter for nodeId; restarts worker if running.
    #[qproperty(cpp_name = "nodeId")]
    pub fn set_node_id(&mut self, id: u32) {
        if self.node_id == id {
            return;
        }
        self.node_id = id;
        self.nodeIdChanged();

        if self.worker.is_some() {
            self.stop();
            self.start();
        }
    }

    /// Setter for sampleRate; reallocates buffers and restarts if running.
    #[qproperty(cpp_name = "sampleRate")]
    pub fn set_sample_rate(&mut self, rate: u32) {
        if self.sample_rate == rate {
            return;
        }
        self.sample_rate = rate.max(8000);
        self.realloc_buffers();
        self.sampleRateChanged();
        if self.worker.is_some() {
            self.stop();
            self.start();
        }
    }

    /// Setter for chunkSize; reallocates buffers and restarts if running.
    #[qproperty(cpp_name = "chunkSize")]
    pub fn set_chunk_size(&mut self, size: u32) {
        if self.chunk_size == size {
            return;
        }
        self.chunk_size = size.max(16);
        self.realloc_buffers();
        self.chunkSizeChanged();
        if self.worker.is_some() {
            self.stop();
            self.start();
        }
    }

    fn realloc_buffers(&mut self) {
        // Resize the buffers and reset pointers.
        self.buffer1.resize(self.chunk_size as usize, 0.0);
        self.buffer2.resize(self.chunk_size as usize, 0.0);
        self.read_buffer
            .store(&mut self.buffer1 as *mut Vec<f32>, Ordering::Release);
        self.write_buffer
            .store(&mut self.buffer2 as *mut Vec<f32>, Ordering::Release);
    }

    // QML signals autogenerated by `cxx_qt` for properties:
    #[cxx_qt::qsignal]
    fn nodeIdChanged(&self);
    #[cxx_qt::qsignal]
    fn sampleRateChanged(&self);
    #[cxx_qt::qsignal]
    fn chunkSizeChanged(&self);

    /// Start the Pipewire capture thread. Spawns a worker that inits
    /// Pipewire, connects a Stream, and fills the write buffer with samples.
    #[cxx_qt::qinvokable]
    pub fn start(&mut self) {
        if self.worker.is_some() {
            return;
        }
        self.clear_buffer();
        let stop = self.stop_flag.clone();
        stop.store(false, Ordering::Relaxed);
        let sample_rate = self.sample_rate;
        let chunk_size = self.chunk_size;
        let node_id = self.node_id;
        // raw pointers to swap buffers in closure
        let write_ptr = self.write_bugger.load(Ordering::Accquire) as *mut Vec<f32>;
        let read_ptr = self.read_bugger.load(Ordering::Accquire) as *mut Vec<f32>;
        self.worker = Some(thread::spawn(move || {
            // Init Pipewire
            if pw::init().is_err() {
                // fallback to silence if Pipewire fails
                return;
            }

            // Create mainloop/context/core
            let ml = pw::main_loop::MainLoop::new(None).unwrap();
            let ctx = pw::context::Context::new(&ml).unwrap();
            let core = ctx.connect(None).unwrap();

            // Build stream properties
            let props = pw::properties! {
                *pw::keys::MEDIA_TYPE => "Audio",
                *pw::keys::MEDIA_CATEGORY => "Capture",
                *pw::keys::MEDIA_ROLE => "Music",
                *pw::keys::STREAM_CAPTURE_SINK => "true",
                *pw::keys::NODE_PASSIVE => "true",
                *pw::keys::NODE_VIRTUAL => "true",
                *pw::keys::STREAM_DONT_REMIX => "false",
                "channelmix.upmix" => "true",
                // latency: next power of two
                *pw::keys::NODE_LATENCY => format!("{}/{}", next_power_of2(chunk_size * sample_rate / 48000), sample_rate),
            };
            let stream = pw::stream::Stream::new(&core, "vela-audio", props).unwrap();
            // Shared state: format and buffers
            let format = Arc::new(Mutex::new(pw::spa::param::audio::AudioInfoRaw::new()));
            // Listener to capture format and process data
            {
                let format = format.clone();
                unsafe {
                    stream.add_local_listener(
                        move |info| {
                            if info.id == pw::spa::param::ParamType::Format.as_raw() {
                                if let Ok((media_type, media_subtype)) = pw::spa::param::format_utils::parse_format(info.param) {
                                    if media_type == pw::spa::param::media_type::AUDIO
                                        && media_subtype == pw::spa::param::media_subtype::RAW
                                    {
                                        let mut f = format.lock().unwrap();
                                        f.parse(info.param).unwrap();
                                    }
                                }
                            }
                        },
                        move || {
                            if stop.load(Ordering::Relaxed) {
                                ml.quit();
                                return;
                            }
                            if let Some(mut buf) = stream.dequeue_buffer().ok() {
                                let datas = buf.datas_mut();
                                if let Some(data) = datas.first() {
                                    if let Some(ptr) = data.data() {
                                        // Cast pointer to slice of i16 samples
                                        let sample_count = (data.size() / 2).min(chunk_size as usize);
                                        let slice = unsafe {
                                            std::slice::from_raw_parts(ptr as *const i16, sample_count)
                                        };
                                        // Write into the write buffer
                                        unsafe {
                                            let wb = &mut *write_ptr;
                                            for i in 0..sample_count {
                                                wb[i] = slice[i] as f32 / 32768.0;
                                            }
                                            // Swap read/write
                                            std::ptr::swap(write_ptrm read_ptr);
                                        }
                                    }
                                }
                                stream.queue_buffer(buf).unwrap();
                            }
                        },
                    );
                }
            }

            // Request F32LE format
            {
                let mut audio_info = pw::spa::param::audio::AudioInfoRaw::new();
                audio_info.set_format(pw::spa::param::audio::AudioFormat::F32LE);
                audio_info.set_rate(sample_rate);
                audio_info.set_channels(1);
                let pod = pw::spa::pod::serialize::PodSerializer::serialize_audio_info(&audio_info).unwrap();
                let mut params = [pod.as_ref()];
                stream
                    .connect(
                        pw::spa::utils::Direction::Input,
                        Some(node_id),
                        pw::stream::StreamFlags::AUTOCONNECT
                            | pw::stream::StreamFlags::MAP_BUFFERS
                            | pw::stream::StreamFlags::RT_PROCESS,
                        &mut params,
                    )
                    .unwrap();
            }
            ml.run().ok();
        }));
    }

    /// Stop the Pipewire capture thread by setting the stop flag
    /// and joining the thread.
    #[cxx_qt::qinvokable]
    pub fn stop(&mut self) {
        if let Some(handle) = self.worker.take() {
            self.stop_flag.store(true, Ordering::Relaxed);
            let _ = handle.join();
        }
    }
}

// Helper: next power of two calculation
fn next_power_of2(mut n: u32) -> u32 {
    if n == 0 {
        return 1;
    }
    n -= 1;
    n |= n >> 1;
    n |= n >> 2;
    n |= n >> 4;
    n |= n >> 8;
    n |= n >> 16;
    n + 1
}

pub fn register() {
    cxx_qt::register_type::<AudioCollector>("Vela", 1, 0, "AudioCollector");
}