use super::prelude::*;

/// Prints the data sent to console for debugging purposes.
pub struct Debug;

impl Transport for &Debug {
    type Output = ();
    #[inline(always)]
    fn send(self, message: ProtocolData) {
        println!("Message: {message:#?}");
    }
}
