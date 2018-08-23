#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::ROMPATCHA {
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
#[doc = "Possible values of the field `THUMBX`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum THUMBXR {
    #[doc = "ARM patch"]
    THUMBX_0,
    #[doc = "THUMB patch (ignore if data fix)"]
    THUMBX_1,
}
impl THUMBXR {
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
            THUMBXR::THUMBX_0 => false,
            THUMBXR::THUMBX_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> THUMBXR {
        match value {
            false => THUMBXR::THUMBX_0,
            true => THUMBXR::THUMBX_1,
        }
    }
    #[doc = "Checks if the value of the field is `THUMBX_0`"]
    #[inline]
    pub fn is_thumbx_0(&self) -> bool {
        *self == THUMBXR::THUMBX_0
    }
    #[doc = "Checks if the value of the field is `THUMBX_1`"]
    #[inline]
    pub fn is_thumbx_1(&self) -> bool {
        *self == THUMBXR::THUMBX_1
    }
}
#[doc = r" Value of the field"]
pub struct ADDRXR {
    bits: u32,
}
impl ADDRXR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
}
#[doc = "Values that can be written to the field `THUMBX`"]
pub enum THUMBXW {
    #[doc = "ARM patch"]
    THUMBX_0,
    #[doc = "THUMB patch (ignore if data fix)"]
    THUMBX_1,
}
impl THUMBXW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            THUMBXW::THUMBX_0 => false,
            THUMBXW::THUMBX_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _THUMBXW<'a> {
    w: &'a mut W,
}
impl<'a> _THUMBXW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: THUMBXW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "ARM patch"]
    #[inline]
    pub fn thumbx_0(self) -> &'a mut W {
        self.variant(THUMBXW::THUMBX_0)
    }
    #[doc = "THUMB patch (ignore if data fix)"]
    #[inline]
    pub fn thumbx_1(self) -> &'a mut W {
        self.variant(THUMBXW::THUMBX_1)
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
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _ADDRXW<'a> {
    w: &'a mut W,
}
impl<'a> _ADDRXW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        const MASK: u32 = 4194303;
        const OFFSET: u8 = 1;
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
    #[doc = "Bit 0 - THUMB Comparator Select - Indicates that this address will trigger a THUMB opcode patch or an ARM opcode patch"]
    #[inline]
    pub fn thumbx(&self) -> THUMBXR {
        THUMBXR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 1:22 - Address Comparator Registers - Indicates the memory address to be watched"]
    #[inline]
    pub fn addrx(&self) -> ADDRXR {
        let bits = {
            const MASK: u32 = 4194303;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) as u32
        };
        ADDRXR { bits }
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
    #[doc = "Bit 0 - THUMB Comparator Select - Indicates that this address will trigger a THUMB opcode patch or an ARM opcode patch"]
    #[inline]
    pub fn thumbx(&mut self) -> _THUMBXW {
        _THUMBXW { w: self }
    }
    #[doc = "Bits 1:22 - Address Comparator Registers - Indicates the memory address to be watched"]
    #[inline]
    pub fn addrx(&mut self) -> _ADDRXW {
        _ADDRXW { w: self }
    }
}
