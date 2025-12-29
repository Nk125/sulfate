use super::prelude::*;

/// Prints the data sent to console for debugging purposes.
pub struct Debug;

impl<'a> Transport for &'a Debug {
    #[inline(always)]
    fn send(self, message: ProtocolData) {
        println!("Message: {message:#?}");
    }
}
