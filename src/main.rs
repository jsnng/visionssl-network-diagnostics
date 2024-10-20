use pcap::{Device, Capture};

fn main() {
    for device in pcap::Device::list().expect("device lookup failed") {
        println!("Found device! {:?}", device);
    }

    let mut cap = pcap::Capture::from_device("en0")
    .unwrap()
    .immediate_mode(true)
    .open()
    .unwrap();

    cap.filter("src host 192.168.8.1", true).unwrap();

    while let Ok(packet) = cap.next_packet() {
        
        println!("got packet! {:?}", packet)
    }
}