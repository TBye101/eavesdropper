use libc::timeval;

#[derive(Queryable)]
pub struct Packets {
    id: u32,
    payload: Vec<u8>,
    captured_timestamp: timeval,
    capture_length: u32,
    packet_length: u32
}