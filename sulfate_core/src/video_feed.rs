#[derive(Debug)]
pub struct RawVideoFrame {
    pub width: u32,
    pub height: u32,
    pub bytes: Vec<u8>,
}
