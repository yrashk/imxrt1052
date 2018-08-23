#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::DIGPROG {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = "Possible values of the field `MINOR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MINORR {
    #[doc = "silicon revision x.0"]
    MINOR_0,
    #[doc = "silicon revision x.1"]
    MINOR_1,
    #[doc = "silicon revision x.2"]
    MINOR_2,
    #[doc = "silicon revision x.3"]
    MINOR_3,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl MINORR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            MINORR::MINOR_0 => 0,
            MINORR::MINOR_1 => 1,
            MINORR::MINOR_2 => 2,
            MINORR::MINOR_3 => 3,
            MINORR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> MINORR {
        match value {
            0 => MINORR::MINOR_0,
            1 => MINORR::MINOR_1,
            2 => MINORR::MINOR_2,
            3 => MINORR::MINOR_3,
            i => MINORR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `MINOR_0`"]
    #[inline]
    pub fn is_minor_0(&self) -> bool {
        *self == MINORR::MINOR_0
    }
    #[doc = "Checks if the value of the field is `MINOR_1`"]
    #[inline]
    pub fn is_minor_1(&self) -> bool {
        *self == MINORR::MINOR_1
    }
    #[doc = "Checks if the value of the field is `MINOR_2`"]
    #[inline]
    pub fn is_minor_2(&self) -> bool {
        *self == MINORR::MINOR_2
    }
    #[doc = "Checks if the value of the field is `MINOR_3`"]
    #[inline]
    pub fn is_minor_3(&self) -> bool {
        *self == MINORR::MINOR_3
    }
}
#[doc = "Possible values of the field `MAJOR_LOWER`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MAJOR_LOWERR {
    #[doc = "silicon revision 1.x"]
    MAJOR_LOWER_0,
    #[doc = "silicon revision 2.x"]
    MAJOR_LOWER_1,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl MAJOR_LOWERR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            MAJOR_LOWERR::MAJOR_LOWER_0 => 0,
            MAJOR_LOWERR::MAJOR_LOWER_1 => 1,
            MAJOR_LOWERR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> MAJOR_LOWERR {
        match value {
            0 => MAJOR_LOWERR::MAJOR_LOWER_0,
            1 => MAJOR_LOWERR::MAJOR_LOWER_1,
            i => MAJOR_LOWERR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `MAJOR_LOWER_0`"]
    #[inline]
    pub fn is_major_lower_0(&self) -> bool {
        *self == MAJOR_LOWERR::MAJOR_LOWER_0
    }
    #[doc = "Checks if the value of the field is `MAJOR_LOWER_1`"]
    #[inline]
    pub fn is_major_lower_1(&self) -> bool {
        *self == MAJOR_LOWERR::MAJOR_LOWER_1
    }
}
#[doc = r" Value of the field"]
pub struct MAJOR_UPPERR {
    bits: u8,
}
impl MAJOR_UPPERR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:7 - MINOR lower byte - Read-only value representing a minor silicon revision."]
    #[inline]
    pub fn minor(&self) -> MINORR {
        MINORR::_from({
            const MASK: u8 = 255;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 8:15 - MAJOR lower byte - Read-only value representing a major silicon revision."]
    #[inline]
    pub fn major_lower(&self) -> MAJOR_LOWERR {
        MAJOR_LOWERR::_from({
            const MASK: u8 = 255;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 16:23 - MAJOR upper byte-Read-only value representing the chip type."]
    #[inline]
    pub fn major_upper(&self) -> MAJOR_UPPERR {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        MAJOR_UPPERR { bits }
    }
}
