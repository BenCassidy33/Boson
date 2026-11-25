pub mod spectra_ipc;

use std::{
    sync::{
        Arc, Mutex, MutexGuard,
        atomic::{AtomicU64, Ordering},
    },
    thread::JoinHandle,
    time::Duration,
};

use boson_ipc::{IpcHub, IpcMessageExt, ProcessRecv};
use sysinfo::{DiskUsage, Process, System};

use crate::spectra::spectra_ipc::{ReplyFrameData, RequestFrameData};

#[derive(Debug, Clone)]
pub struct Frame {
    capture_time: std::time::Instant,

    mem_usage: u64,
    virtual_mem_usage: u64,
    gpu_usage: f32,
    gpu_mem_usage: u64,
    cpu_usage: f32,
    acc_cpu_time: u64,
    disk_usage: DiskUsage,
}

impl Frame {
    pub fn capture(system: &mut sysinfo::System) -> anyhow::Result<Self> {
        system.refresh_memory();
        system.refresh_cpu_usage();

        let Some(proc) = Spectra::get_current_process(system)? else {
            return anyhow::bail!("Failed to get current process!");
        };

        let mem_usage = proc.memory();
        let cpu_usage = proc.cpu_usage();

        let gpu_usage = proc.gpu_usage().unwrap_or_default();
        let gpu_mem_usage = proc.gpu_memory().unwrap_or_default();

        let virtual_mem_usage = proc.virtual_memory();
        let acc_cpu_time = proc.accumulated_cpu_time();
        let disk_usage = proc.disk_usage();

        Ok(Self {
            capture_time: std::time::Instant::now(),

            mem_usage,
            cpu_usage,
            gpu_usage,
            gpu_mem_usage,
            virtual_mem_usage,
            acc_cpu_time,
            disk_usage,
        })
    }
}

#[derive(Debug)]
pub struct PlatformInfo {
    cpu_arch: String,
    system_name: String,
    kernel_version: String,
    os_version: String,
    host_name: String,
}

impl PlatformInfo {
    pub fn new() -> anyhow::Result<Self> {
        Ok(Self {
            cpu_arch: System::cpu_arch(),
            system_name: System::name()?,
            kernel_version: System::kernel_long_version()?,
            os_version: System::long_os_version()?,
            host_name: System::host_name()?,
        })
    }
}

pub struct Spectra {
    platform_info: PlatformInfo,
    pid: sysinfo::Pid,
    capture_interval: Arc<AtomicU64>,

    system: Arc<Mutex<sysinfo::System>>,
    frames: Arc<Mutex<Vec<Frame>>>,
    frame_capture_thread: Option<JoinHandle<()>>,

    ipc_hub: Arc<IpcHub>,
    recv: ProcessRecv,
}

impl Spectra {
    pub fn new(ipc_hub: Arc<IpcHub>, recv: ProcessRecv, interval_ms: u64) -> anyhow::Result<Self> {
        Ok(Self {
            platform_info: PlatformInfo::new()?,
            pid: sysinfo::get_current_pid()?,
            capture_interval: Arc::new(AtomicU64::from(interval_ms)),

            system: Arc::new(Mutex::new(sysinfo::System::new_all()?)),
            frames: Arc::new(Mutex::new(Vec::new())),
            frame_capture_thread: None,

            ipc_hub,
            recv,
        })
    }

    pub fn get_current_process(system: &System) -> anyhow::Result<Option<&Process>> {
        let current_pid = sysinfo::get_current_pid()?;
        Ok(system.process(current_pid))
    }

    pub fn set_capture_interval_ms(&self, ms: u64) {
        self.capture_interval.store(ms, Ordering::Relaxed);
    }

    pub fn start(mut self) -> anyhow::Result<JoinHandle<()>> {
        let runtime = Arc::new(
            tokio::runtime::Builder::new_multi_thread()
                .enable_all()
                .build()?,
        );

        let frames = Arc::clone(&self.frames);
        let system = Arc::clone(&self.system);
        let interval_atomic = Arc::clone(&self.capture_interval);

        let handle = std::thread::Builder::new()
            .name("Spectra Backend".into())
            .spawn(move || {
                let result = runtime.block_on(self.run(frames, system, interval_atomic));

                if let Err(err) = result {
                    log::error!("Spectra exited: {err}");
                };
            })?;

        Ok(handle)
    }

    async fn run(
        &mut self,
        frames: Arc<Mutex<Vec<Frame>>>,
        system: Arc<Mutex<System>>,
        interval_atomic: Arc<AtomicU64>,
    ) -> anyhow::Result<()> {
        let inital_interval = interval_atomic.load(Ordering::Relaxed);
        let mut ticker = tokio::time::interval(Duration::from_millis(inital_interval));
        ticker.set_missed_tick_behavior(tokio::time::MissedTickBehavior::Skip);

        loop {
            let interval = interval_atomic.load(Ordering::Relaxed).max(1);

            tokio::select! {
                msg = self.recv.recv() => {
                    match msg {
                        Some(msg) => {
                            self.handle_message(msg).await?;
                        },

                        _ => {}
                    };
                }

                _ = ticker.tick() => {
                    let frame_result = {
                        let mut system = system.lock().unwrap();
                        Frame::capture(&mut system)
                    };

                    match frame_result {
                        Ok(frame) => {
                            frames.lock().unwrap().push(frame);
                        },

                        Err(err) => {
                            log::error!("Frame capture failed!");
                        }
                    };
                }
            };
        }

        Ok(())
    }

    async fn handle_message(&self, message: boson_ipc::IpcMessageWrapper) -> anyhow::Result<()> {
        if message.payload.is::<RequestFrameData>() {
            let frames: Vec<_> = self.get_frames().iter().cloned().collect();

            self.ipc_hub
                .send(boson_ipc::Process::Photon, ReplyFrameData { inner: frames });

            return Ok(());
        }

        anyhow::bail!("Invalid Message: {:?}", message.payload.type_name())
    }

    pub fn get_frames(&self) -> MutexGuard<Vec<Frame>> {
        self.frames.lock().unwrap()
    }
}
