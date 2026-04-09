use client::info;
use sulfate_core::transport::{Debug, prelude::*};

fn main() {
    let debug_transport = &Debug {};

    debug_transport.send(ProtocolData::InitialConnection);

    debug_transport.send(ProtocolData::OSInfo(info::system::get_os_info()));
}
