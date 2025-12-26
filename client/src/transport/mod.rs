mod debug;

pub use debug::Debug;
pub use sulfate_core::protocol::ProtocolData;

/// Common trait to enable multiple conveyors, like TCP or console.
pub trait Transport {
    fn send(self, message: ProtocolData);
}
