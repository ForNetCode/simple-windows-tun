mod asset;

fn main() {
    tracing_subscriber::fmt::init();
    let temp_dir = std::env::temp_dir();
    //
    //let data = asset::Asset::get("abc.txt").unwrap();
    //let data =  std::str::from_utf8(data.data.as_ref());
    //println!("Hello, world! {data:?}");
}
