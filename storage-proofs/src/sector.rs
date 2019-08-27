use std::collections::BTreeSet;
use std::fmt;

use byteorder::ByteOrder;

/// An ordered set of `SectorId`s.
pub type OrderedSectorSet = BTreeSet<SectorId>;

/// Identifier for a single sector.
#[derive(Debug, Clone, Copy, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub struct SectorId(u64);

impl From<u64> for SectorId {
    fn from(n: u64) -> Self {
        SectorId(n)
    }
}

impl From<SectorId> for u64 {
    fn from(n: SectorId) -> Self {
        n.0
    }
}

impl fmt::Display for SectorId {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "SectorId({})", self.0)
    }
}

impl SectorId {
    pub fn as_fr_safe(self) -> [u8; 31] {
        let mut buf: [u8; 31] = [0; 31];
        byteorder::LittleEndian::write_u64(&mut buf[0..8], self.0);
        buf
    }
}