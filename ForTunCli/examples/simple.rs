use std::time::Duration;
use tokio::net::UdpSocket;
use windows::core::GUID;
use fortun_cli::{create_async_tun, net_config};


// Run with Administrator Permission
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt().init();

    let guid = GUID::from("15D95261-C48F-428E-853F-FF080ACA23FA");
    //let inf_path = std::env::current_dir().unwrap().join("driver\\fortun\\fortun.inf");
    let inf_path = "C:/DriverTest/Drivers/ForTun.inf";

    let (mut read,mut write, device) = create_async_tun(&guid, "ForTunTest", inf_path).unwrap();
    net_config(device.instance_id.clone(), "10.0.0.2", "255.255.0.0",1428);

    let mut read_buf:Vec<u8> = vec![0;2048];

    let task = tokio::spawn(async move {
        while let Ok(size) = read.read(read_buf.as_mut_slice()) {
            println!("receive: {size}");
            tokio::time::sleep(Duration::from_secs(1)).await;
        }
    });


    /*
    let addr = "10.0.0.2:8080";

    let socket = UdpSocket::bind(&addr).await?;
    println!("Listening on: {}", socket.local_addr()?);

    let mut to_send = None;
    let mut buf = vec![0;1024];
    tokio::spawn(async move {
        loop {
            if let Some((size, peer)) = to_send {
                let amt = socket.send_to(b"pong", &peer).await.unwrap();
                println!("Echoed {}/{} bytes to {}", amt, size, peer);
            }
            to_send = Some(socket.recv_from(&mut buf).await.unwrap());
        }
    });
    tokio::time::sleep(Duration::from_secs(1)).await;


    let socket = UdpSocket::bind("10.0.0.2:8089").await?;
    const MAX_DATAGRAM_SIZE: usize = 1204;
    socket.connect(addr).await?;
    let data = b"ping";
    socket.send(data).await?;
    let mut data = vec![0u8; MAX_DATAGRAM_SIZE];

    let len = socket.recv(&mut data).await?;
    println!(
        "Received {} bytes:\n{}",
        len,
        String::from_utf8_lossy(&data[..len])
    );
    tokio::time::sleep(Duration::from_secs(2)).await;
*/
    let _ = task.await;


    Ok(())
}
