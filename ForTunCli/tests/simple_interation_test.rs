use std::time::Duration;
use packet::Builder;
use tracing_test::traced_test;
use windows::core::GUID;
use fortun_cli::{create_async_tun, net_config};

#[traced_test]
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn ping(){

    let guid = GUID::from("15D95261-C48F-428E-853F-FF080ACA23FA");
    let inf_path = std::env::current_dir().unwrap().join("driver\\fortun\\fortun.inf");


    let (mut read,write, device) = create_async_tun(&guid, "ForTunTest", inf_path).unwrap();
    net_config(device.instance_id.clone(), "10.0.0.2", "255.255.0.0",1208);

    let mut read_buf:Vec<u8> = Vec::with_capacity(2048);

    let task = tokio::spawn(async move {
        while let Ok(size) = read.read(read_buf.as_mut_slice()) {
            println!("receive: {size}");
        }
    });

    //write.write();
    let icmp = packet::icmp::Builder::default().echo().unwrap().identifier(42).unwrap().sequence(2).unwrap()
        .payload(b"test").unwrap().build().unwrap();
    //let icmp = icmp::Packet::new(icmp);
    tokio::time::sleep(Duration::from_secs(60)).await;

}