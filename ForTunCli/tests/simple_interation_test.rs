use packet::Builder;
use tracing_test::traced_test;
use windows::core::GUID;
use fortun_cli::{create_async_tun, net_config};

#[traced_test]
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
fn ping(){

    let guid = GUID::from("f579d929-6c40-4e5a-8532-180199a4e322");
    let (mut read,write, device) = create_async_tun(&guid, "ForTunTest", "./driver/fortun/fortun.inf").unwrap();
    net_config(device.instance_id, "10.0.0.2", "255.255.0.0",1208);
    device.start_adapter().unwrap();
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