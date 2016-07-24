use std::{fmt, cmp, mem};

#[derive(Copy, Clone)]
#[repr(C)]
pub struct Guid { // TODO: fields should not be public (requires const fn constructor)
    pub Data1: u32,
    pub Data2: u16,
    pub Data3: u16,
    pub Data4: [u8; 8]
}

impl AsRef<::w::GUID> for Guid {
    fn as_ref(&self) -> &::w::GUID {
        unsafe { mem::transmute(self) }
    } 
}

impl From<::w::GUID> for Guid {
    fn from(guid: ::w::GUID) -> Self {
        unsafe { mem::transmute(guid) }
    }
}

impl fmt::Debug for Guid {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:08X}-{:04X}-{:04X}-{:02X}{:02X}-{:02X}{:02X}{:02X}{:02X}{:02X}{:02X}",
            self.Data1, self.Data2, self.Data3,
            self.Data4[0], self.Data4[1], self.Data4[2], self.Data4[3],
            self.Data4[4], self.Data4[5], self.Data4[6], self.Data4[7])
    }
}

impl cmp::PartialEq<Guid> for Guid {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.Data1 == other.Data1 && self.Data2 == other.Data2 && self.Data3 == other.Data3 && self.Data4 == other.Data4
    }
}

impl cmp::Eq for Guid {}