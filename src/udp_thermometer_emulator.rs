use smart_home_lib::device::udp_thermometer::TemperatureTelemetry;
use std::io;
use std::net::{SocketAddr, UdpSocket};
use std::path::Path;
use std::thread::sleep;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

struct ThermometerState {
    temperature: f32,
}

struct ThermometerEmulator {
    socket: UdpSocket,
    interval_ms: Duration,
    state: ThermometerState,
}

#[derive(Debug)]
struct Config {
    address: String,
    interval_ms: Duration,
    temperature_c: Option<f32>,
}

impl TryFrom<&Path> for Config {
    type Error = std::io::Error;

    fn try_from(path: &Path) -> Result<Self, Self::Error> {
        let mut may_be_address: Option<String> = None;
        let mut may_be_interval: Option<u64> = None;
        let mut may_be_temperature: Option<f32> = None;

        let contents = std::fs::read_to_string(path)?;
        for line in contents.lines() {
            let line = line.trim().split('=').collect::<Vec<&str>>();
            match line[0] {
                "address" => may_be_address = Some(line[1].trim().to_string()),
                "interval_ms" => may_be_interval = Some(line[1].trim().parse::<u64>().unwrap()),
                "temperature_c" => {
                    may_be_temperature = Some(line[1].trim().parse::<f32>().unwrap())
                }
                _ => {}
            }
        }

        if may_be_address.is_none() || may_be_interval.is_none() {
            return Err(std::io::Error::other("Invalid configuration"));
        }

        Ok(Self {
            address: may_be_address.unwrap(),
            interval_ms: Duration::from_millis(may_be_interval.unwrap()),
            temperature_c: may_be_temperature,
        })
    }
}

impl ThermometerEmulator {
    pub fn new(config_path: &Path) -> std::io::Result<Self> {
        let config: Config = Config::try_from(config_path)?;
        let socket = UdpSocket::bind("0.0.0.0:0")?;
        socket.connect(config.address)?;
        Ok(Self {
            socket,
            interval_ms: config.interval_ms,
            state: ThermometerState {
                temperature: config.temperature_c.unwrap_or(0.0),
            },
        })
    }

    #[allow(dead_code)] // Used in tests
    fn get_address(&self) -> io::Result<SocketAddr> {
        self.socket.local_addr()
    }

    fn emulate(&self) {
        loop {
            let start = SystemTime::now();
            let since_the_epoch = start
                .duration_since(UNIX_EPOCH)
                .expect("time should go forward");

            let pocket = TemperatureTelemetry {
                timestamp: since_the_epoch.as_secs(),
                temperature: self.state.temperature,
            };

            let buf: Vec<u8> = pocket.into();
            let _ = self.socket.send(buf.as_slice());
            sleep(self.interval_ms);
        }
    }
}

fn main() {
    match std::env::args().nth(1) {
        Some(config_path) => match ThermometerEmulator::new(config_path.as_ref()) {
            Ok(emulator) => emulator.emulate(),
            Err(e) => {
                eprintln!("Cannot create emulator: {}", e);
            }
        },
        None => {
            eprintln!("config path is not specified");
            std::process::exit(1);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread;

    fn start_emulator_in_thread() -> SocketAddr {
        let address = SocketAddr::from(([0, 0, 0, 0], 8004));
        let config_content = format!(
            r#"
address={}
interval_ms=500
temperature_c=998

"#,
            address
        );

        let config_file = tempfile::NamedTempFile::new().unwrap();
        std::fs::write(config_file.path(), config_content).unwrap();
        let config_path = config_file.path();
        let emulator = ThermometerEmulator::new(config_path).unwrap();
        thread::spawn(move || emulator.emulate());
        address
    }

    #[test]
    fn emulate() {
        let address = start_emulator_in_thread();
        sleep(Duration::from_millis(500));
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

        let mut buf = [0u8; 12];
        // socket.connect(address).expect("Should connect");
        match socket.recv_from(&mut buf) {
            Ok((number_of_bytes, _addr)) => {
                let filled_buf = &mut buf[..number_of_bytes];
                let msg = TemperatureTelemetry::from(filled_buf);
                assert_eq!(msg.temperature, 998.);
            }
            Err(e) => {
                eprintln!("Error reading from socket: {}", e);
                unreachable!();
            }
        }
    }
}
