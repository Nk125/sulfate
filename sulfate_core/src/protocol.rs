use super::system_info::OperatingSystemInfo;
use super::video_feed::RawVideoFrame;

#[derive(Debug)]
/// The message protocol between server and client.
pub enum ProtocolData {
    InitialConnection,
    OSInfo(OperatingSystemInfo),
    VideoFeed(RawVideoFrame),
}
