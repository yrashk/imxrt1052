#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::DBGSELECT {
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
#[doc = "Possible values of the field `INDEX`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INDEXR {
    #[doc = "CONTROL"]
    CONTROL,
    #[doc = "OTPKEY0"]
    OTPKEY0,
    #[doc = "OTPKEY1"]
    OTPKEY1,
    #[doc = "OTPKEY2"]
    OTPKEY2,
    #[doc = "OTPKEY3"]
    OTPKEY3,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl INDEXR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            INDEXR::CONTROL => 1,
            INDEXR::OTPKEY0 => 16,
            INDEXR::OTPKEY1 => 17,
            INDEXR::OTPKEY2 => 18,
            INDEXR::OTPKEY3 => 19,
            INDEXR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> INDEXR {
        match value {
            1 => INDEXR::CONTROL,
            16 => INDEXR::OTPKEY0,
            17 => INDEXR::OTPKEY1,
            18 => INDEXR::OTPKEY2,
            19 => INDEXR::OTPKEY3,
            i => INDEXR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `CONTROL`"]
    #[inline]
    pub fn is_control(&self) -> bool {
        *self == INDEXR::CONTROL
    }
    #[doc = "Checks if the value of the field is `OTPKEY0`"]
    #[inline]
    pub fn is_otpkey0(&self) -> bool {
        *self == INDEXR::OTPKEY0
    }
    #[doc = "Checks if the value of the field is `OTPKEY1`"]
    #[inline]
    pub fn is_otpkey1(&self) -> bool {
        *self == INDEXR::OTPKEY1
    }
    #[doc = "Checks if the value of the field is `OTPKEY2`"]
    #[inline]
    pub fn is_otpkey2(&self) -> bool {
        *self == INDEXR::OTPKEY2
    }
    #[doc = "Checks if the value of the field is `OTPKEY3`"]
    #[inline]
    pub fn is_otpkey3(&self) -> bool {
        *self == INDEXR::OTPKEY3
    }
}
#[doc = "Values that can be written to the field `INDEX`"]
pub enum INDEXW {
    #[doc = "CONTROL"]
    CONTROL,
    #[doc = "OTPKEY0"]
    OTPKEY0,
    #[doc = "OTPKEY1"]
    OTPKEY1,
    #[doc = "OTPKEY2"]
    OTPKEY2,
    #[doc = "OTPKEY3"]
    OTPKEY3,
}
impl INDEXW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            INDEXW::CONTROL => 1,
            INDEXW::OTPKEY0 => 16,
            INDEXW::OTPKEY1 => 17,
            INDEXW::OTPKEY2 => 18,
            INDEXW::OTPKEY3 => 19,
        }
    }
}
#[doc = r" Proxy"]
pub struct _INDEXW<'a> {
    w: &'a mut W,
}
impl<'a> _INDEXW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: INDEXW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "CONTROL"]
    #[inline]
    pub fn control(self) -> &'a mut W {
        self.variant(INDEXW::CONTROL)
    }
    #[doc = "OTPKEY0"]
    #[inline]
    pub fn otpkey0(self) -> &'a mut W {
        self.variant(INDEXW::OTPKEY0)
    }
    #[doc = "OTPKEY1"]
    #[inline]
    pub fn otpkey1(self) -> &'a mut W {
        self.variant(INDEXW::OTPKEY1)
    }
    #[doc = "OTPKEY2"]
    #[inline]
    pub fn otpkey2(self) -> &'a mut W {
        self.variant(INDEXW::OTPKEY2)
    }
    #[doc = "OTPKEY3"]
    #[inline]
    pub fn otpkey3(self) -> &'a mut W {
        self.variant(INDEXW::OTPKEY3)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 255;
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
    #[doc = "Bits 0:7 - Selects a value to read via the debug data register."]
    #[inline]
    pub fn index(&self) -> INDEXR {
        INDEXR::_from({
            const MASK: u8 = 255;
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
    #[doc = "Bits 0:7 - Selects a value to read via the debug data register."]
    #[inline]
    pub fn index(&mut self) -> _INDEXW {
        _INDEXW { w: self }
    }
}
