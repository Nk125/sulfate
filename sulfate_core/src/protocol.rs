use super::system_info::OperatingSystemInfo;
use super::video_feed::RawVideoFrame;
use rkyv::{Archive, Deserialize, Serialize};

#[derive(Archive, Deserialize, Serialize, Debug, PartialEq)]
#[rkyv(compare(PartialEq), derive(Debug))]
/// The message protocol between server and client.
pub enum ProtocolData {
    InitialConnection,
    OSInfo(OperatingSystemInfo),
    VideoFeed(RawVideoFrame),
}
