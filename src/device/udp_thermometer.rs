use crate::device::thermometer::SmartThermometer;
use std::io::ErrorKind;
use std::net::UdpSocket;
use std::sync::atomic::AtomicBool;
use std::sync::{Arc, RwLock};
use std::thread;
use std::thread::JoinHandle;
use std::time::Duration;

#[derive(Debug)]
pub struct UdpThermometer {
    pub temperature: Arc<RwLock<f32>>,
    subscriber_shutdown_flag: Arc<AtomicBool>,
    subscriber_handle: Option<JoinHandle<()>>,
}

impl SmartThermometer for UdpThermometer {
    fn get_temperature(&self) -> f32 {
        match self.temperature.read() {
            Ok(temperature) => *temperature,
            Err(e) => {
                eprintln!("thermometer failed to acquire lock: {}", e);
                f32::NEG_INFINITY
            }
        }
    }
}
impl UdpThermometer {
    pub fn new(address: &str) -> Self {
        let temperature = Arc::new(RwLock::new(0.0));
        let subscriber_shutdown_flag = Arc::new(AtomicBool::new(false));

        let temperature_clone = temperature.clone();
        let flag_clone = subscriber_shutdown_flag.clone();

        let address_clone = address.to_string();
        let handle = thread::spawn(move || {
            Self::subscribe(address_clone.as_ref(), temperature_clone, flag_clone);
        });

        Self {
            temperature,
            subscriber_shutdown_flag,
            subscriber_handle: Some(handle),
        }
    }
    pub fn get_temperature(&self) -> f32 {
        match self.temperature.read() {
            Ok(temperature) => *temperature,
            Err(e) => {
                eprintln!("thermometer failed to acquire lock: {}", e);
                f32::NEG_INFINITY
            }
        }
    }

    fn subscribe(
        address: &str,
        temperature: Arc<RwLock<f32>>,
        subscriber_shutdown_flag: Arc<AtomicBool>,
    ) {
        let socket = match UdpSocket::bind(address) {
            Ok(socket) => socket,
            Err(e) => {
                eprintln!("Failed to bind socket: {}", e);
                return;
            }
        };

        socket
            .set_read_timeout(Some(Duration::from_millis(500)))
            .ok();

        while !subscriber_shutdown_flag.load(std::sync::atomic::Ordering::Relaxed) {
            let mut buf = [0; 12];
            match socket.recv_from(&mut buf) {
                Ok((number_of_bytes, _src_addr)) => {
                    let filled_buf = &mut buf[..number_of_bytes];
                    let msg = TemperatureTelemetry::from(filled_buf);
                    let temperature_guard = temperature.write();
                    match temperature_guard {
                        Ok(mut temp) => {
                            *temp = msg.temperature;
                        }
                        Err(e) => {
                            eprintln!("Temperature update failed: {}", e);
                        }
                    }
                    continue;
                }
                Err(ref e) => match e.kind() {
                    ErrorKind::WouldBlock | ErrorKind::TimedOut => {
                        continue;
                    }
                    e => {
                        eprintln!("Error receiving from socket: {}", e);
                    }
                },
            }
        }
        println!("Thermometer subscriber shutting down");
    }
}

impl Drop for UdpThermometer {
    fn drop(&mut self) {
        self.subscriber_shutdown_flag
            .store(true, std::sync::atomic::Ordering::Relaxed);
        if let Some(handle) = self.subscriber_handle.take() {
            let _ = handle.join();
        }
    }
}

#[derive(Debug)]
pub struct TemperatureTelemetry {
    pub timestamp: u64,
    pub temperature: f32,
}

impl From<TemperatureTelemetry> for Vec<u8> {
    fn from(temp: TemperatureTelemetry) -> Vec<u8> {
        let timestamp: [u8; 8] = temp.timestamp.to_le_bytes();
        let temperature: [u8; 4] = temp.temperature.to_le_bytes();
        let mut buf = vec![0; 12];
        buf[0..8].copy_from_slice(&timestamp);
        buf[8..].copy_from_slice(&temperature);
        buf
    }
}

impl From<&[u8]> for TemperatureTelemetry {
    fn from(data: &[u8]) -> Self {
        let timestamp: [u8; 8] = data[0..8].try_into().unwrap();
        let temperature: [u8; 4] = data[8..].try_into().unwrap();

        Self {
            timestamp: u64::from_le_bytes(timestamp),
            temperature: f32::from_le_bytes(temperature),
        }
    }
}

impl From<&mut [u8]> for TemperatureTelemetry {
    fn from(data: &mut [u8]) -> Self {
        let data = data.iter().as_slice();
        data.into()
    }
}
