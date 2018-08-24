#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::PIGEON_8_1 {
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
#[doc = "Possible values of the field `SET_CNT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SET_CNTR {
    #[doc = "Start as active"]
    START_ACTIVE,
    #[doc = r" Reserved"]
    _Reserved(u16),
}
impl SET_CNTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        match *self {
            SET_CNTR::START_ACTIVE => 0,
            SET_CNTR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u16) -> SET_CNTR {
        match value {
            0 => SET_CNTR::START_ACTIVE,
            i => SET_CNTR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `START_ACTIVE`"]
    #[inline]
    pub fn is_start_active(&self) -> bool {
        *self == SET_CNTR::START_ACTIVE
    }
}
#[doc = "Possible values of the field `CLR_CNT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CLR_CNTR {
    #[doc = "Keep active until mask off"]
    CLEAR_USING_MASK,
    #[doc = r" Reserved"]
    _Reserved(u16),
}
impl CLR_CNTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        match *self {
            CLR_CNTR::CLEAR_USING_MASK => 0,
            CLR_CNTR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u16) -> CLR_CNTR {
        match value {
            0 => CLR_CNTR::CLEAR_USING_MASK,
            i => CLR_CNTR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `CLEAR_USING_MASK`"]
    #[inline]
    pub fn is_clear_using_mask(&self) -> bool {
        *self == CLR_CNTR::CLEAR_USING_MASK
    }
}
#[doc = "Values that can be written to the field `SET_CNT`"]
pub enum SET_CNTW {
    #[doc = "Start as active"]
    START_ACTIVE,
}
impl SET_CNTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u16 {
        match *self {
            SET_CNTW::START_ACTIVE => 0,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SET_CNTW<'a> {
    w: &'a mut W,
}
impl<'a> _SET_CNTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SET_CNTW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Start as active"]
    #[inline]
    pub fn start_active(self) -> &'a mut W {
        self.variant(SET_CNTW::START_ACTIVE)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        const MASK: u16 = 65535;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `CLR_CNT`"]
pub enum CLR_CNTW {
    #[doc = "Keep active until mask off"]
    CLEAR_USING_MASK,
}
impl CLR_CNTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u16 {
        match *self {
            CLR_CNTW::CLEAR_USING_MASK => 0,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CLR_CNTW<'a> {
    w: &'a mut W,
}
impl<'a> _CLR_CNTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CLR_CNTW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Keep active until mask off"]
    #[inline]
    pub fn clear_using_mask(self) -> &'a mut W {
        self.variant(CLR_CNTW::CLEAR_USING_MASK)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        const MASK: u16 = 65535;
        const OFFSET: u8 = 16;
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
    #[doc = "Bits 0:15 - Assert signal output when counter match this value"]
    #[inline]
    pub fn set_cnt(&self) -> SET_CNTR {
        SET_CNTR::_from({
            const MASK: u16 = 65535;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        })
    }
    #[doc = "Bits 16:31 - Deassert signal output when counter match this value"]
    #[inline]
    pub fn clr_cnt(&self) -> CLR_CNTR {
        CLR_CNTR::_from({
            const MASK: u16 = 65535;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u16
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
    #[doc = "Bits 0:15 - Assert signal output when counter match this value"]
    #[inline]
    pub fn set_cnt(&mut self) -> _SET_CNTW {
        _SET_CNTW { w: self }
    }
    #[doc = "Bits 16:31 - Deassert signal output when counter match this value"]
    #[inline]
    pub fn clr_cnt(&mut self) -> _CLR_CNTW {
        _CLR_CNTW { w: self }
    }
}
