#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::IPCR1 {
    #[doc = r" Modifies the contents of the register"]
    #[inline]
    pub fn modify<F>(&self, f: F)
    where
        for<'w> F: FnOnce(&R, &'w mut W) -> &'w mut W,
    {
        let bits = self.register.get();
        let r = R { bits: bits };
        let mut w = W { bits: bits };
        f(&r, &mut w);
        self.register.set(w.bits);
    }
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
    #[doc = r" Writes to the register"]
    #[inline]
    pub fn write<F>(&self, f: F)
    where
        F: FnOnce(&mut W) -> &mut W,
    {
        let mut w = W::reset_value();
        f(&mut w);
        self.register.set(w.bits);
    }
    #[doc = r" Writes the reset value to the register"]
    #[inline]
    pub fn reset(&self) {
        self.write(|w| w)
    }
}
#[doc = "Possible values of the field `DATSZ`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DATSZR {
    #[doc = "4"]
    DATSZ_0,
    #[doc = "1"]
    DATSZ_1,
    #[doc = "2"]
    DATSZ_2,
    #[doc = "3"]
    DATSZ_3,
    #[doc = "4"]
    DATSZ_4,
    #[doc = "4"]
    DATSZ_5,
    #[doc = "4"]
    DATSZ_6,
    #[doc = "4"]
    DATSZ_7,
}
impl DATSZR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            DATSZR::DATSZ_0 => 0,
            DATSZR::DATSZ_1 => 1,
            DATSZR::DATSZ_2 => 2,
            DATSZR::DATSZ_3 => 3,
            DATSZR::DATSZ_4 => 4,
            DATSZR::DATSZ_5 => 5,
            DATSZR::DATSZ_6 => 6,
            DATSZR::DATSZ_7 => 7,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> DATSZR {
        match value {
            0 => DATSZR::DATSZ_0,
            1 => DATSZR::DATSZ_1,
            2 => DATSZR::DATSZ_2,
            3 => DATSZR::DATSZ_3,
            4 => DATSZR::DATSZ_4,
            5 => DATSZR::DATSZ_5,
            6 => DATSZR::DATSZ_6,
            7 => DATSZR::DATSZ_7,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DATSZ_0`"]
    #[inline]
    pub fn is_datsz_0(&self) -> bool {
        *self == DATSZR::DATSZ_0
    }
    #[doc = "Checks if the value of the field is `DATSZ_1`"]
    #[inline]
    pub fn is_datsz_1(&self) -> bool {
        *self == DATSZR::DATSZ_1
    }
    #[doc = "Checks if the value of the field is `DATSZ_2`"]
    #[inline]
    pub fn is_datsz_2(&self) -> bool {
        *self == DATSZR::DATSZ_2
    }
    #[doc = "Checks if the value of the field is `DATSZ_3`"]
    #[inline]
    pub fn is_datsz_3(&self) -> bool {
        *self == DATSZR::DATSZ_3
    }
    #[doc = "Checks if the value of the field is `DATSZ_4`"]
    #[inline]
    pub fn is_datsz_4(&self) -> bool {
        *self == DATSZR::DATSZ_4
    }
    #[doc = "Checks if the value of the field is `DATSZ_5`"]
    #[inline]
    pub fn is_datsz_5(&self) -> bool {
        *self == DATSZR::DATSZ_5
    }
    #[doc = "Checks if the value of the field is `DATSZ_6`"]
    #[inline]
    pub fn is_datsz_6(&self) -> bool {
        *self == DATSZR::DATSZ_6
    }
    #[doc = "Checks if the value of the field is `DATSZ_7`"]
    #[inline]
    pub fn is_datsz_7(&self) -> bool {
        *self == DATSZR::DATSZ_7
    }
}
#[doc = "Values that can be written to the field `DATSZ`"]
pub enum DATSZW {
    #[doc = "4"]
    DATSZ_0,
    #[doc = "1"]
    DATSZ_1,
    #[doc = "2"]
    DATSZ_2,
    #[doc = "3"]
    DATSZ_3,
    #[doc = "4"]
    DATSZ_4,
    #[doc = "4"]
    DATSZ_5,
    #[doc = "4"]
    DATSZ_6,
    #[doc = "4"]
    DATSZ_7,
}
impl DATSZW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            DATSZW::DATSZ_0 => 0,
            DATSZW::DATSZ_1 => 1,
            DATSZW::DATSZ_2 => 2,
            DATSZW::DATSZ_3 => 3,
            DATSZW::DATSZ_4 => 4,
            DATSZW::DATSZ_5 => 5,
            DATSZW::DATSZ_6 => 6,
            DATSZW::DATSZ_7 => 7,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DATSZW<'a> {
    w: &'a mut W,
}
impl<'a> _DATSZW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DATSZW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "4"]
    #[inline]
    pub fn datsz_0(self) -> &'a mut W {
        self.variant(DATSZW::DATSZ_0)
    }
    #[doc = "1"]
    #[inline]
    pub fn datsz_1(self) -> &'a mut W {
        self.variant(DATSZW::DATSZ_1)
    }
    #[doc = "2"]
    #[inline]
    pub fn datsz_2(self) -> &'a mut W {
        self.variant(DATSZW::DATSZ_2)
    }
    #[doc = "3"]
    #[inline]
    pub fn datsz_3(self) -> &'a mut W {
        self.variant(DATSZW::DATSZ_3)
    }
    #[doc = "4"]
    #[inline]
    pub fn datsz_4(self) -> &'a mut W {
        self.variant(DATSZW::DATSZ_4)
    }
    #[doc = "4"]
    #[inline]
    pub fn datsz_5(self) -> &'a mut W {
        self.variant(DATSZW::DATSZ_5)
    }
    #[doc = "4"]
    #[inline]
    pub fn datsz_6(self) -> &'a mut W {
        self.variant(DATSZW::DATSZ_6)
    }
    #[doc = "4"]
    #[inline]
    pub fn datsz_7(self) -> &'a mut W {
        self.variant(DATSZW::DATSZ_7)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:2 - Data Size in Byte"]
    #[inline]
    pub fn datsz(&self) -> DATSZR {
        DATSZR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 0 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:2 - Data Size in Byte"]
    #[inline]
    pub fn datsz(&mut self) -> _DATSZW {
        _DATSZW { w: self }
    }
}
