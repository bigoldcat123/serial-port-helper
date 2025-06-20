use nusb::{Interface, transfer::RequestBuffer};
use tokio::sync::mpsc::{Receiver, Sender};

pub struct UsbWriter {
    interface: Interface,
    complete_reciver: Receiver<u8>,
}
impl UsbWriter {
    pub fn new(interface: Interface, complete_reciver: Receiver<u8>) -> Self {
        Self {
            interface,
            complete_reciver,
        }
    }
    pub async fn write(&mut self, data: &[u8]) {
        self.interface
            .bulk_out(0x02, data.to_vec())
            .await
            .into_result()
            .unwrap();
        self.complete_reciver.recv().await.unwrap();
    }
}

pub struct UsbReader {
    interface: Interface,
    complete_handle: Sender<u8>,
}
impl UsbReader {
    pub fn new(interface: Interface, complete_handle: Sender<u8>) -> Self {
        Self {
            interface,
            complete_handle,
        }
    }
    pub async fn run(self) {
        loop {
            let data = self
                .interface
                .bulk_in(0x83, RequestBuffer::new(128))
                .await
                .into_result()
                .unwrap();
            println!("{}", String::from_utf8_lossy(data.as_ref()).trim());
            self.complete_handle.send(1).await.unwrap();
        }
    }
}
