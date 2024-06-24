//! Serial I/O driver.

use crate::{
    module::{
        descriptor::PDescriptor,
        stdio::{PInput, POutput},
    },
    shared::UsbSerial,
    types::error::PStaticResult,
};

pub struct Serial {
    inner: UsbSerial,
}
impl PDescriptor for Serial {
    const ID: usize = 10;
}
impl PInput<char> for Serial {
    fn poll(&mut self) -> PStaticResult<Option<char>> {}
}
impl POutput<char> for Serial {
    fn send(&mut self, it: char) -> PStaticResult<()> {
        todo!()
    }
}
