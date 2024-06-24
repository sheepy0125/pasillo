//! Standard input / output devices.

use crate::{module::descriptor::PDescriptor, types::error::PStaticResult};

/// An input file of any kind.
pub trait PInput<PollOut = u8>: PDescriptor {
    fn poll(&mut self) -> PStaticResult<Option<PollOut>>;
}
/// An output file of any kind.
pub trait POutput<PollIn = u8>: PDescriptor {
    fn send(&mut self, it: PollIn) -> PStaticResult<()>;
}

pub trait Stdin: PInput<char> {
    const ID: usize = 1;
}
pub trait Stdout: POutput<char> {
    const ID: usize = 2;
}
pub trait Stderr: POutput<char> {
    const ID: usize = 3;
}

pub trait NullInput<T>: PInput<T> {
    const ID: usize = 0;
}
pub trait NullOutput<T>: POutput<T> {
    const ID: usize = 0;
}
