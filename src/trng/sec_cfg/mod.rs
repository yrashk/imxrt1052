#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::SEC_CFG {
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
pub struct UNUSED0R {
    bits: bool,
}
impl UNUSED0R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
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
}
#[doc = "Possible values of the field `NO_PRGM`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NO_PRGMR {
    #[doc = "Programability of registers controlled only by the Miscellaneous Control Register's access mode bit."]
    NO_PRGM_0,
    #[doc = "Overides Miscellaneous Control Register access mode and prevents TRNG register programming."]
    NO_PRGM_1,
}
impl NO_PRGMR {
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
            NO_PRGMR::NO_PRGM_0 => false,
            NO_PRGMR::NO_PRGM_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> NO_PRGMR {
        match value {
            false => NO_PRGMR::NO_PRGM_0,
            true => NO_PRGMR::NO_PRGM_1,
        }
    }
    #[doc = "Checks if the value of the field is `NO_PRGM_0`"]
    #[inline]
    pub fn is_no_prgm_0(&self) -> bool {
        *self == NO_PRGMR::NO_PRGM_0
    }
    #[doc = "Checks if the value of the field is `NO_PRGM_1`"]
    #[inline]
    pub fn is_no_prgm_1(&self) -> bool {
        *self == NO_PRGMR::NO_PRGM_1
    }
}
#[doc = r" Value of the field"]
pub struct UNUSED2R {
    bits: bool,
}
impl UNUSED2R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
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
}
#[doc = r" Proxy"]
pub struct _UNUSED0W<'a> {
    w: &'a mut W,
}
impl<'a> _UNUSED0W<'a> {
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
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `NO_PRGM`"]
pub enum NO_PRGMW {
    #[doc = "Programability of registers controlled only by the Miscellaneous Control Register's access mode bit."]
    NO_PRGM_0,
    #[doc = "Overides Miscellaneous Control Register access mode and prevents TRNG register programming."]
    NO_PRGM_1,
}
impl NO_PRGMW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            NO_PRGMW::NO_PRGM_0 => false,
            NO_PRGMW::NO_PRGM_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _NO_PRGMW<'a> {
    w: &'a mut W,
}
impl<'a> _NO_PRGMW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: NO_PRGMW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Programability of registers controlled only by the Miscellaneous Control Register's access mode bit."]
    #[inline]
    pub fn no_prgm_0(self) -> &'a mut W {
        self.variant(NO_PRGMW::NO_PRGM_0)
    }
    #[doc = "Overides Miscellaneous Control Register access mode and prevents TRNG register programming."]
    #[inline]
    pub fn no_prgm_1(self) -> &'a mut W {
        self.variant(NO_PRGMW::NO_PRGM_1)
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
        const OFFSET: u8 = 1;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _UNUSED2W<'a> {
    w: &'a mut W,
}
impl<'a> _UNUSED2W<'a> {
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
        const OFFSET: u8 = 2;
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
    #[doc = "Bit 0 - This bit is unused. Ignore."]
    #[inline]
    pub fn unused0(&self) -> UNUSED0R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        UNUSED0R { bits }
    }
    #[doc = "Bit 1 - If set, the TRNG registers cannot be programmed"]
    #[inline]
    pub fn no_prgm(&self) -> NO_PRGMR {
        NO_PRGMR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - This bit is unused. Ignore."]
    #[inline]
    pub fn unused2(&self) -> UNUSED2R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        UNUSED2R { bits }
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
    #[doc = "Bit 0 - This bit is unused. Ignore."]
    #[inline]
    pub fn unused0(&mut self) -> _UNUSED0W {
        _UNUSED0W { w: self }
    }
    #[doc = "Bit 1 - If set, the TRNG registers cannot be programmed"]
    #[inline]
    pub fn no_prgm(&mut self) -> _NO_PRGMW {
        _NO_PRGMW { w: self }
    }
    #[doc = "Bit 2 - This bit is unused. Ignore."]
    #[inline]
    pub fn unused2(&mut self) -> _UNUSED2W {
        _UNUSED2W { w: self }
    }
}
