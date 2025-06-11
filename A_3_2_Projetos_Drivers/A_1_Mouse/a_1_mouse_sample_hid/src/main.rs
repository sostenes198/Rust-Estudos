use std::ptr::read;
use hidapi::HidApi;
use std::time::Duration;
use std::thread;

fn find_devices(){
    let api = HidApi::new().unwrap();

    println!("Connected HDI devices:");
    for device in api.device_list() {
        println!(
            "VID: {:04x}, PID: {:04x}, Manufacture: {:?}, Product: {:?}",
            device.vendor_id(),
            device.product_id(),
            device.manufacturer_string(),
            device.product_string(),
        );
    }
}

fn read_device(vid:u16, pid:u16){
    let api = HidApi::new().expect("Falha ao iniciar HIDAPI");

    let device = api.open(vid, pid).expect("Failed to open HDI device");

    println!("Opened HDI device and starting read device. Use Ctrl+C to exit");

    loop{
        let mut buf =  [0u8; 64];
        match device.read(&mut buf) {
            Ok(len) => {
                println!("Read {} bytes from HDI", len);
            }
            Err(e) => {
                println!("Error reading from HDI: {:?}", e);
            }
        }
        thread::sleep(Duration::from_millis(50));
    }
}

fn main() {
    // find_devices();
    read_device(0x046d, 0xc077)
}
