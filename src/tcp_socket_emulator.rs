use smart_home_lib::device::smart_socket::backends::tcp_electrical_socket::{
    Command, CommandEncoder, GetPowerCmd, ToggleCmd,
};
use smart_home_lib::device::smart_socket::socket_state::ElectricalSocketState;
use std::io::{ErrorKind, Read};
use std::net::SocketAddr;
use std::sync::{Arc, RwLock};
use std::{
    io,
    io::{BufReader, Write},
    net::{TcpListener, TcpStream},
    thread,
};

#[derive(Debug)]
struct SocketState {
    pub state: ElectricalSocketState,
    pub power: f32,
}

#[derive(Debug)]
struct ElectricalSocketEmulator {
    listener: TcpListener,
    socket_state: Arc<RwLock<SocketState>>,
}

fn handle_command(stream: TcpStream, state: Arc<RwLock<SocketState>>) -> std::io::Result<()> {
    let mut reader = BufReader::new(stream.try_clone()?);
    let mut writer = stream;
    let mut command_buf = [0u8; 1];

    loop {
        if let Err(e) = reader.read_exact(&mut command_buf) {
            match e.kind() {
                ErrorKind::UnexpectedEof => break,
                _ => return Err(e),
            }
        }
        let command = Command::try_decode(command_buf[0])?;
        match command {
            Command::Toggle => {
                let mut state = state.write().unwrap();
                match state.state {
                    ElectricalSocketState::On => state.state = ElectricalSocketState::Off,
                    ElectricalSocketState::Off => state.state = ElectricalSocketState::On,
                }
                writer.write_all(ToggleCmd {}.encode_response(()).as_slice())?;
            }
            Command::GetPower => {
                let state = state.read().unwrap();
                let power = match state.state {
                    ElectricalSocketState::On => state.power,
                    ElectricalSocketState::Off => 0.0,
                };
                writer.write_all(GetPowerCmd {}.encode_response(power).as_slice())?;
            }
        }
    }

    Ok(())
}

impl ElectricalSocketEmulator {
    pub fn try_new(address: &str) -> io::Result<Self> {
        let listener = TcpListener::bind(address)?;
        Ok(Self {
            listener,
            socket_state: Arc::new(RwLock::new(SocketState {
                state: ElectricalSocketState::Off,
                power: 220.,
            })),
        })
    }

    #[allow(dead_code)] // Used in tests
    fn get_address(&self) -> io::Result<SocketAddr> {
        self.listener.local_addr()
    }

    pub fn emulate(&self) {
        for may_be_connection in self.listener.incoming() {
            match may_be_connection {
                Ok(stream) => {
                    let state = self.socket_state.clone();
                    thread::spawn(move || handle_command(stream, state));
                }
                Err(e) => {
                    eprintln!("Failed to accept TCP connection: {}", e);
                }
            }
        }
    }
}

fn main() {
    match std::env::args().nth(1) {
        Some(address) => match ElectricalSocketEmulator::try_new(address.as_str()) {
            Ok(emulator) => emulator.emulate(),
            Err(e) => {
                eprintln!("Cannot create emulator: {}", e);
            }
        },
        None => {
            eprintln!("Address is not specified");
            std::process::exit(1);
        }
    }
}

#[cfg(test)]
mod integration_tests {
    use smart_home_lib::device::smart_socket::backends::tcp_electrical_socket::{
        CommandEncoder, TcpElectricalSocket,
    };

    use super::*;
    use std::net::TcpStream;
    use std::thread;
    use std::time::Duration;

    fn start_emulator_in_thread() -> String {
        let addr = "127.0.0.1:0";
        let emulator = ElectricalSocketEmulator::try_new(addr).unwrap();
        let local_addr = emulator.get_address().unwrap().to_string();

        thread::spawn(move || emulator.emulate());
        // Startup
        thread::sleep(Duration::from_millis(50));
        local_addr
    }

    #[test]
    fn test_state_consistency_under_load() {
        let addr = start_emulator_in_thread();
        let iterations = 100;
        let mut handles = vec![];

        // Toggle Client
        let addr_toggle = addr.clone();
        handles.push(thread::spawn(move || {
            for _ in 0..iterations {
                let mut stream = TcpStream::connect(&addr_toggle).unwrap();
                TcpElectricalSocket::send_command(ToggleCmd {}, &mut stream).unwrap();
                stream.flush().unwrap();
                let mut buf = [0u8; 5];
                let _ = stream.read_exact(&mut buf);
            }
        }));

        // GetPower clients
        for _ in 0..5 {
            let addr_power = addr.clone();
            handles.push(thread::spawn(move || {
                for _ in 0..iterations {
                    let mut stream = TcpStream::connect(&addr_power).unwrap();
                    TcpElectricalSocket::send_command(GetPowerCmd {}, &mut stream).unwrap();
                    stream.flush().unwrap();
                    let mut buf = [0u8; 5];
                    stream.read_exact(&mut buf).unwrap();
                    let power = GetPowerCmd::decode_response(buf);

                    assert!(power.is_some());

                    let power = power.unwrap();
                    assert!(power == 0.0 || power == 220.0);
                }
            }));
        }

        for h in handles {
            h.join().unwrap();
        }
    }
}
