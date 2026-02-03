use core::ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign};

/// Flags for memory mapping.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub(crate) struct MapFlags(usize);

#[stable(feature = "kernel_syscall", since = "1.94.0")]
impl From<MapFlags> for usize {
    fn from(value: MapFlags) -> Self {
        value.0
    }
}

impl From<usize> for MapFlags {
    fn from(value: usize) -> Self {
        Self(value)
    }
}

impl BitAndAssign for MapFlags {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0 &= rhs.0
    }
}

impl BitAnd for MapFlags {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        Self(self.0 & rhs.0)
    }
}

impl BitXorAssign for MapFlags {
    fn bitxor_assign(&mut self, rhs: Self) {
        self.0 ^= rhs.0
    }
}

impl BitOrAssign for MapFlags {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0
    }
}

impl BitXor for MapFlags {
    type Output = Self;

    fn bitxor(self, rhs: Self) -> Self::Output {
        Self(self.0 ^ rhs.0)
    }
}

impl BitOr for MapFlags {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        Self(self.0 | rhs.0)
    }
}

impl MapFlags {
    pub const EMPTY: MapFlags = MapFlags(0);
    pub const WRITE: MapFlags = MapFlags(1 << 0);
    pub const EXECUTE: MapFlags = MapFlags(1 << 1);
}

impl MapFlags {
    pub fn contains(self, flags: Self) -> bool {
        (self.0 & flags.0) != 0
    }
}

