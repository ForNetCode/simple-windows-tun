use std::time::Duration;
use tokio::net::UdpSocket;
use windows::core::GUID;
use windows::Win32::Foundation::GetLastError;
use windows::Win32::Storage::FileSystem::ReadFile;
use fortun_cli::{create_async_tun, create_block_tun, net_config};
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt().init();

    let guid = GUID::from("15D95261-C48F-428E-853F-FF080ACA23FA");
    //let inf_path = std::env::current_dir().unwrap().join("driver\\fortun\\fortun.inf");
    let inf_path = "C:/DriverTest/Drivers/ForTun.inf";

    let (device, file_handler) = create_block_tun(&guid, "ForTunTest", inf_path).unwrap();
    net_config(device.instance_id.clone(), "10.0.0.2", "255.255.0.0",1428);



    let mut read_buf:Vec<u8> = Vec::with_capacity(2048);
    let mut i = 10;
    loop {
        let mut size = 0;
        let ret = unsafe {
            ReadFile(file_handler,
                     Some(&mut read_buf),
                     Some(&mut size),
                     None
            )
        };
        println!("read result : {:?}", ret);
        if ret.is_ok() {
            println!("read length: {size}");
        }

        if i> 10 {
            break;
        }
        i+=1;
        tokio::time::sleep(Duration::from_secs(1)).await;
    }



    tokio::time::sleep(Duration::from_secs(1000)).await;
    Ok(())
}