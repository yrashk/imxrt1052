#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::VID1 {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = "Possible values of the field `MIN_REV`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MIN_REVR {
    #[doc = "Minor revision number for TRNG."]
    MIN_REV_0,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl MIN_REVR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            MIN_REVR::MIN_REV_0 => 0,
            MIN_REVR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> MIN_REVR {
        match value {
            0 => MIN_REVR::MIN_REV_0,
            i => MIN_REVR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `MIN_REV_0`"]
    #[inline]
    pub fn is_min_rev_0(&self) -> bool {
        *self == MIN_REVR::MIN_REV_0
    }
}
#[doc = "Possible values of the field `MAJ_REV`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MAJ_REVR {
    #[doc = "Major revision number for TRNG."]
    MAJ_REV_1,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl MAJ_REVR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            MAJ_REVR::MAJ_REV_1 => 1,
            MAJ_REVR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> MAJ_REVR {
        match value {
            1 => MAJ_REVR::MAJ_REV_1,
            i => MAJ_REVR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `MAJ_REV_1`"]
    #[inline]
    pub fn is_maj_rev_1(&self) -> bool {
        *self == MAJ_REVR::MAJ_REV_1
    }
}
#[doc = "Possible values of the field `IP_ID`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IP_IDR {
    #[doc = "ID for TRNG."]
    IP_ID_48,
    #[doc = r" Reserved"]
    _Reserved(u16),
}
impl IP_IDR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        match *self {
            IP_IDR::IP_ID_48 => 48,
            IP_IDR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u16) -> IP_IDR {
        match value {
            48 => IP_IDR::IP_ID_48,
            i => IP_IDR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `IP_ID_48`"]
    #[inline]
    pub fn is_ip_id_48(&self) -> bool {
        *self == IP_IDR::IP_ID_48
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:7 - Shows the IP's Minor revision of the TRNG."]
    #[inline]
    pub fn min_rev(&self) -> MIN_REVR {
        MIN_REVR::_from({
            const MASK: u8 = 255;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 8:15 - Shows the IP's Major revision of the TRNG."]
    #[inline]
    pub fn maj_rev(&self) -> MAJ_REVR {
        MAJ_REVR::_from({
            const MASK: u8 = 255;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 16:31 - Shows the IP ID."]
    #[inline]
    pub fn ip_id(&self) -> IP_IDR {
        IP_IDR::_from({
            const MASK: u16 = 65535;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        })
    }
}
