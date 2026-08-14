use rkyv::{Archive, Deserialize, Serialize};

#[derive(Archive, Deserialize, Serialize, Debug, PartialEq)]
#[rkyv(compare(PartialEq), derive(Debug))]
pub struct RawVideoFrame {
    pub width: u32,
    pub height: u32,
    pub bytes: Vec<u8>,
}
