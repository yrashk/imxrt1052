#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::OFS {
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
#[doc = r" Value of the field"]
pub struct OFSR {
    bits: u16,
}
impl OFSR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }
}
#[doc = "Possible values of the field `SIGN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SIGNR {
    #[doc = "The offset value is added with the raw result"]
    SIGN_0,
    #[doc = "The offset value is subtracted from the raw converted value"]
    SIGN_1,
}
impl SIGNR {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            SIGNR::SIGN_0 => false,
            SIGNR::SIGN_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SIGNR {
        match value {
            false => SIGNR::SIGN_0,
            true => SIGNR::SIGN_1,
        }
    }
    #[doc = "Checks if the value of the field is `SIGN_0`"]
    #[inline]
    pub fn is_sign_0(&self) -> bool {
        *self == SIGNR::SIGN_0
    }
    #[doc = "Checks if the value of the field is `SIGN_1`"]
    #[inline]
    pub fn is_sign_1(&self) -> bool {
        *self == SIGNR::SIGN_1
    }
}
#[doc = r" Proxy"]
pub struct _OFSW<'a> {
    w: &'a mut W,
}
impl<'a> _OFSW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        const MASK: u16 = 4095;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `SIGN`"]
pub enum SIGNW {
    #[doc = "The offset value is added with the raw result"]
    SIGN_0,
    #[doc = "The offset value is subtracted from the raw converted value"]
    SIGN_1,
}
impl SIGNW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SIGNW::SIGN_0 => false,
            SIGNW::SIGN_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SIGNW<'a> {
    w: &'a mut W,
}
impl<'a> _SIGNW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SIGNW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The offset value is added with the raw result"]
    #[inline]
    pub fn sign_0(self) -> &'a mut W {
        self.variant(SIGNW::SIGN_0)
    }
    #[doc = "The offset value is subtracted from the raw converted value"]
    #[inline]
    pub fn sign_1(self) -> &'a mut W {
        self.variant(SIGNW::SIGN_1)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 12;
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
    #[doc = "Bits 0:11 - Offset value"]
    #[inline]
    pub fn ofs(&self) -> OFSR {
        let bits = {
            const MASK: u16 = 4095;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        };
        OFSR { bits }
    }
    #[doc = "Bit 12 - Sign bit"]
    #[inline]
    pub fn sign(&self) -> SIGNR {
        SIGNR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) != 0
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
    #[doc = "Bits 0:11 - Offset value"]
    #[inline]
    pub fn ofs(&mut self) -> _OFSW {
        _OFSW { w: self }
    }
    #[doc = "Bit 12 - Sign bit"]
    #[inline]
    pub fn sign(&mut self) -> _SIGNW {
        _SIGNW { w: self }
    }
}
