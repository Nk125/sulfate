mod debug;

pub mod prelude {
    pub use super::{ProtocolData, Transport};
}

pub use crate::protocol::ProtocolData;
pub use debug::Debug;

/// Common trait to enable multiple conveyors, like TCP or console.
pub trait Transport {
    type Output;
    fn send(self, message: ProtocolData) -> Self::Output;
}
