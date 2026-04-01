use crate::device::smart_socket::SocketBackend;
use std::fmt::{Display, Formatter};
use std::io;
use std::io::{Error, ErrorKind, Read, Write};
use std::net::TcpStream;
#[derive(Debug)]
pub struct TcpElectricalSocket {
    address: String,
}

impl TcpElectricalSocket {
    pub fn new(address: &str) -> Self {
        Self {
            address: address.into(),
        }
    }

    pub fn send_command<C: CommandEncoder>(
        cmd: C,
        stream: &mut impl Write,
    ) -> Result<(), io::Error> {
        stream.write_all(&cmd.encode_request())
    }
}

impl SocketBackend for TcpElectricalSocket {
    fn toggle(&mut self) {
        let mut stream = TcpStream::connect(&self.address).unwrap();
        let _ = TcpElectricalSocket::send_command(ToggleCmd {}, &mut stream);
        stream.flush().unwrap();
        let mut response = [0u8; 5];
        stream.read_exact(response.as_mut_slice()).unwrap();
        let decoded_response = ToggleCmd::decode_response(response);
        debug_assert!(decoded_response.is_some());
        debug_assert_eq!(decoded_response.unwrap(), ());
    }

    fn get_power(&self) -> f32 {
        let mut stream = TcpStream::connect(&self.address).unwrap();
        let _ = TcpElectricalSocket::send_command(GetPowerCmd {}, &mut stream);
        stream.flush().unwrap();
        let mut response = [0u8; 5];
        stream.read_exact(response.as_mut_slice()).unwrap();
        let decoded_response = GetPowerCmd::decode_response(response);
        decoded_response.unwrap_or(f32::NEG_INFINITY)
    }
}

#[derive(Debug)]
pub enum Command {
    Toggle = 0,
    GetPower = 1,
}

#[derive(Debug)]
struct UnknownCommand;

impl Display for UnknownCommand {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Unknown command")
    }
}

impl Command {
    pub fn encode(self) -> u8 {
        self as u8
    }
    pub fn try_decode(bits: u8) -> std::io::Result<Self> {
        match bits {
            0 => Ok(Self::Toggle),
            1 => Ok(Self::GetPower),
            _ => Err(Error::new(
                ErrorKind::InvalidData,
                UnknownCommand.to_string(),
            )),
        }
    }
}

pub trait CommandEncoder {
    type Response;
    type ResponseType;
    const CMD: Command;
    fn encode_request(&self) -> [u8; 1] {
        [Self::CMD.encode()]
    }
    fn decode_response(response: Self::ResponseType) -> Option<Self::Response>;
    fn encode_response(&self, response: Self::Response) -> Self::ResponseType;
}

#[derive(Debug)]
pub struct ToggleCmd;

impl CommandEncoder for ToggleCmd {
    type Response = ();
    type ResponseType = [u8; 5];
    const CMD: Command = Command::Toggle;

    fn decode_response(_: Self::ResponseType) -> Option<Self::Response> {
        Some(())
    }

    fn encode_response(&self, _: Self::Response) -> Self::ResponseType {
        Response::ack().encode()
    }
}

#[derive(Debug)]
pub struct GetPowerCmd;

impl CommandEncoder for GetPowerCmd {
    type Response = f32;
    type ResponseType = [u8; 5];
    const CMD: Command = Command::GetPower;
    fn decode_response(response: Self::ResponseType) -> Option<Self::Response> {
        if let Some(decoded) = Response::decode(response) {
            return Some(f32::from_be_bytes(decoded.data));
        }
        None
    }

    fn encode_response(&self, power: f32) -> Self::ResponseType {
        Response {
            status: Status::Ok,
            data: power.to_be_bytes(),
        }
        .encode()
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(u8)]
pub enum Status {
    Ok = 0,
    ErrUnknownCommand = 1,
}

impl Status {
    pub fn encode(self) -> u8 {
        self as u8
    }
    pub fn decode(byte: u8) -> Option<Self> {
        match byte {
            0 => Some(Status::Ok),
            1 => Some(Status::ErrUnknownCommand),
            _ => None,
        }
    }
}

#[derive(Debug)]
pub struct Response {
    pub status: Status,
    pub data: [u8; 4], // для GetPower: 3 байта из f32 + 1 байт статуса
}

impl Response {
    pub fn ack() -> Self {
        Self {
            status: Status::Ok,
            data: [0; 4],
        }
    }

    pub fn encode(self) -> [u8; 5] {
        let mut buf = [0u8; 5];
        buf[0] = self.status.encode();
        buf[1..].copy_from_slice(&self.data);
        buf
    }

    pub fn decode(buf: [u8; 5]) -> Option<Self> {
        Some(Self {
            status: Status::decode(buf[0])?,
            data: buf[1..].try_into().ok()?,
        })
    }
}
