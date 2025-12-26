use super::system_info::OperatingSystemInfo;

#[derive(Debug)]
/// The message protocol between server and client.
pub enum ProtocolData {
    InitialConnection,
    OSInfo(OperatingSystemInfo),
}
