use std::{
    collections::HashMap,
    io::{Seek, Write},
};

use super::name::Label;

/// Represents anything that can be part of a dns packet (Question, Resource Record, RData)
pub(crate) trait WireFormat<'a> {
    /// Parse the contents of the data buffer starting at the given `position`
    /// It is necessary to pass the full buffer to this function, to be able to correctly implement name compression
    /// The implementor must `position` to ensure that is at the end of the data just parsed
    fn parse(data: &'a [u8], position: &mut usize) -> crate::Result<Self>
    where
        Self: Sized;

    /// Write this part bytes to the writer
    fn write_to<T: Write>(&self, out: &mut T) -> crate::Result<()>;

    fn write_compressed_to<T: Write + Seek>(
        &'a self,
        out: &mut T,
        _name_refs: &mut HashMap<&'a [Label<'a>], usize>,
    ) -> crate::Result<()> {
        self.write_to(out)
    }

    fn write_compressed_only_name_to<T: Write + Seek>(
        &'a self,
        out: &mut T,
        _name_refs: &mut HashMap<&'a [Label<'a>], usize>,
    ) -> crate::Result<()> {
        self.write_to(out)
    }

    /// Returns the length in bytes of this content
    fn len(&self) -> usize;
}
