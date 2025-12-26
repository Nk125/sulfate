/// Prints the data sent to console for debugging purposes.
pub struct Debug;

impl<'a> super::Transport for &'a Debug {
    #[inline(always)]
    fn send(self, message: super::ProtocolData) {
        println!("Message: {message:#?}");
    }
}
