use client::info;
use client::transport::*;

fn main() {
    let debug_transport = &Debug {};

    debug_transport.send(ProtocolData::InitialConnection);

    debug_transport.send(ProtocolData::OSInfo(info::system::get_os_info()));
}
