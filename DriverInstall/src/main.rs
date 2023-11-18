use fortun_cli::install_driver;
use std::env;

mod asset;

fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();
    //let driver_files = vec!["fortun.cat", "fortun.inf", "fortun.sys"];
    let driver_path = "./driver/fortun.inf";

    let driver_path = env::current_exe()
        .unwrap()
        .parent()
        .unwrap()
        .join(driver_path);

    let driver_path = driver_path.to_str().unwrap();
    install_driver(driver_path)
}
