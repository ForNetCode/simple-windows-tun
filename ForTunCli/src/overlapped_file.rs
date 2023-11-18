use anyhow::bail;
use windows::Win32::Foundation::{CloseHandle, ERROR_IO_PENDING, HANDLE};
use windows::Win32::Storage::FileSystem::{ReadFile, WriteFile};
use windows::Win32::System::IO::{CancelIoEx, DeviceIoControl, GetOverlappedResult, OVERLAPPED};

pub struct WinOverlappedFile {
    pub file: HANDLE,
}

impl WinOverlappedFile {
    pub fn new(file: HANDLE) -> anyhow::Result<Self> {
        Ok(Self { file })
    }

    pub fn device_io_control_sync(
        &mut self,
        dwiocontrolcode: u32,
        lpinbuffer: Option<*const ::core::ffi::c_void>,
        ninbuffersize: u32,
        lpoutbuffer: Option<*mut ::core::ffi::c_void>,
        noutbuffersize: u32,
        lpbytesreturned: Option<*mut u32>,
    ) -> windows::core::Result<()> {
        unsafe {
            DeviceIoControl(
                self.file,
                dwiocontrolcode,
                lpinbuffer,
                ninbuffersize,
                lpoutbuffer,
                noutbuffersize,
                lpbytesreturned,
                None,
            )
        }
    }

    // driver would wait
    pub fn read(&self, buf: &mut [u8], overlapped: &mut OVERLAPPED) -> anyhow::Result<usize> {
        let mut size = 0;
        let ret = unsafe { ReadFile(self.file, Some(buf), Some(&mut size), Some(overlapped)) };
        match ret {
            Ok(()) => Ok(size as usize),
            Err(e) => {
                if e.code() == ERROR_IO_PENDING.into() {
                    let r = unsafe { GetOverlappedResult(self.file, overlapped, &mut size, true) };
                    if r.is_ok() {
                        Ok(size as usize)
                    } else {
                        bail!("GetOverlappedResult error: {:?}", r)
                    }
                } else {
                    bail!("read file error: {:?}", e)
                }
            }
        }
    }
    /*
    pub fn read_result(&self, overlapped: &mut OVERLAPPED) -> io::Result<Option<usize>> {
        let mut size = 0;
        let r = unsafe {
            GetOverlappedResult(
                self.file,
                overlapped,
                &mut size,
                false,
            )
        };
        if r.as_bool() {
            Ok(Some(size as usize))
        } else {
            let last_error = unsafe { GetLastError() };
            if last_error == ERROR_IO_PENDING {
                Ok(None)
            } else {
                Err(io::Error::from_raw_os_error(last_error.0 as i32))
            }
        }
    }*/

    // this is quick , so block
    pub fn write(&self, buf: &[u8], overlapped: &mut OVERLAPPED) -> anyhow::Result<usize> {
        let mut size = 0;
        let r = unsafe { WriteFile(self.file, Some(buf), Some(&mut size), Some(overlapped)) };
        match r {
            Ok(()) => Ok(size as usize),
            Err(e) => {
                if e.code() == ERROR_IO_PENDING.into() {
                    let r = unsafe { GetOverlappedResult(self.file, overlapped, &mut size, true) };
                    if r.is_ok() {
                        Ok(size as usize)
                    } else {
                        bail!("GetOverlappedResult Error: {:?}", r)
                    }
                } else {
                    bail!("write file error: {:?}", e)
                }
            }
        }
    }

    pub fn cancel_io(&self, overlapped: &mut OVERLAPPED) -> windows::core::Result<()> {
        unsafe { CancelIoEx(self.file, Some(overlapped)) }
    }
}

impl Drop for WinOverlappedFile {
    fn drop(&mut self) {
        if !self.file.is_invalid() {
            unsafe { CloseHandle(self.file) };
        }
    }
}
