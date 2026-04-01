use smart_home_lib::{
    device::{Device, smart_socket::SmartSocket, smart_thermometer::SmartThermometer},
    room::{Room, Subscriber},
};

fn main() {
    let mut room = Room::default();
    room.add_sub(MySubscriber::default());
    room.add_sub(|device: &Device| println!("Device added: {:?}", device));

    let _ = room.add_device("Thermometer", SmartThermometer::default().into());
    let _ = room.add_device("Socket", SmartSocket::default().into());
}

#[derive(Default)]
struct MySubscriber {}
impl Subscriber for MySubscriber {
    fn on_event(&mut self, device: &Device) {
        println!("MySubscriber got device: {:?}", device);
    }
}
