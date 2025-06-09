use std::io::{Write, stdin, stdout};

use tokio::sync::mpsc::channel;
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
    loop {
        let mut cmd = String::new();
        out.write_all(b">").unwrap();
        out.flush().unwrap();
        in_.read_line(&mut cmd).unwrap();
        writer.write(cmd.as_bytes()).await;
    }
}
