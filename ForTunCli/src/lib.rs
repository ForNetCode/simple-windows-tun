mod device_ops;
pub mod overlapped_file;

//use std::os::windows::io::FromRawHandle;
use crate::overlapped_file::WinOverlappedFile;
use anyhow::bail;
pub use device_ops::get_net_index;
pub use device_ops::init_device;
pub use device_ops::install_driver;
pub use device_ops::net_config;
pub use device_ops::AdapterDevice;
use std::path::Path;
use std::sync::Arc;
use windows::core::GUID;
use windows::Win32::Foundation::CloseHandle;
use windows::Win32::System::IO::OVERLAPPED;

pub struct OverlappedWrap {
    pub overlapped: OVERLAPPED,
}

//unsafe impl Send for OverlappedWrap {}

pub struct ReadFile {
    file: Arc<WinOverlappedFile>,
    overlapped: OVERLAPPED,
}

unsafe impl Send for ReadFile {}

impl ReadFile {
    // this would block...
    pub fn read(&mut self, buf: &mut [u8]) -> anyhow::Result<usize> {
        self.file.read(buf, &mut self.overlapped)
    }
}

impl Drop for ReadFile {
    fn drop(&mut self) {
        let _ = self.file.cancel_io(&mut self.overlapped);
    }
}

pub struct WriteFile {
    file: Arc<WinOverlappedFile>,
    overlapped: OVERLAPPED,
}

unsafe impl Send for WriteFile {}

impl WriteFile {
    //this would Finish quick
    pub fn write(&mut self, buf: &[u8]) -> anyhow::Result<usize> {
        self.file.write(buf, &mut self.overlapped)
    }
}

impl Drop for WriteFile {
    fn drop(&mut self) {
        let _ = self.file.cancel_io(&mut self.overlapped);
    }
}

pub type TunSocket = (ReadFile, WriteFile, AdapterDevice);

pub fn create_async_tun<T: AsRef<Path>>(
    device_id: &GUID,
    name: &str,
    inf_path: T,
) -> anyhow::Result<TunSocket> {
    let device = init_device(
        device_id, name, inf_path, /*"C:/DriverTest/Drivers/ForTun.inf"*/
    )?;
    let file = device.start_adapter()?;
    let file = match WinOverlappedFile::new(file) {
        Ok(file) => Arc::new(file),
        Err(err) => {
            unsafe { CloseHandle(file) };
            bail!("create winOverlappedFile error:{}", err)
        }
    };

    Ok((
        ReadFile {
            file: file.clone(),
            overlapped: OVERLAPPED::default(),
        },
        WriteFile {
            file: file.clone(),
            overlapped: OVERLAPPED::default(),
        },
        device,
    ))
}
