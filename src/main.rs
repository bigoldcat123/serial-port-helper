use std::{
    fs::File,
    io::{Read, Write, stdin, stdout},
    time::Duration,
};

use tokio::{sync::mpsc::channel, time::sleep};
use usb_play::{UsbReader, UsbWriter};

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let device = nusb::list_devices()
        .unwrap()
        .find(|x| x.vendor_id() == 0x1234)
        .expect("no such device");
    println!("{:#?}", device);
    let device = device.open().unwrap();
    println!("{:#?}", device.active_configuration());
    device.detach_and_claim_interface(0x01).unwrap();
    let interface = device.claim_interface(0x01).unwrap();

    let mut out = stdout();
    let in_ = stdin();
    let interface_in = interface.clone();

    let (tx, rx) = channel(8);
    let reader = UsbReader::new(interface_in, tx);
    tokio::spawn(async { reader.run().await });

    let mut writer = UsbWriter::new(interface, rx);
    // loop {
    //     let mut cmd = String::new();
    //     out.write_all(b">").unwrap();
    //     out.flush().unwrap();
    //     in_.read_line(&mut cmd).unwrap();
    //     writer.write(cmd.as_bytes()).await;
    // }
    let mut f = File::open("/Users/dadigua/Desktop/embed-rust/huihui-150*100.bin").unwrap();
    let mut buf = [0; 64];
    while let Ok(n) = f.read(&mut buf) {
        if n == 0 {
            break;
        }
        writer.write(&buf[..n]).await;
        // sleep(Duration::from_millis(2)).await;
    }
}
