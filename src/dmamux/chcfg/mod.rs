#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CHCFG {
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
pub struct SOURCER {
    bits: u8,
}
impl SOURCER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Possible values of the field `A_ON`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum A_ONR {
    #[doc = "DMA Channel Always ON function is disabled"]
    A_ON_0,
    #[doc = "DMA Channel Always ON function is enabled"]
    A_ON_1,
}
impl A_ONR {
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
            A_ONR::A_ON_0 => false,
            A_ONR::A_ON_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> A_ONR {
        match value {
            false => A_ONR::A_ON_0,
            true => A_ONR::A_ON_1,
        }
    }
    #[doc = "Checks if the value of the field is `A_ON_0`"]
    #[inline]
    pub fn is_a_on_0(&self) -> bool {
        *self == A_ONR::A_ON_0
    }
    #[doc = "Checks if the value of the field is `A_ON_1`"]
    #[inline]
    pub fn is_a_on_1(&self) -> bool {
        *self == A_ONR::A_ON_1
    }
}
#[doc = "Possible values of the field `TRIG`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TRIGR {
    #[doc = "Triggering is disabled. If triggering is disabled and ENBL is set, the DMA Channel will simply route the specified source to the DMA channel. (Normal mode)"]
    TRIG_0,
    #[doc = "Triggering is enabled. If triggering is enabled and ENBL is set, the DMA_CH_MUX is in Periodic Trigger mode."]
    TRIG_1,
}
impl TRIGR {
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
            TRIGR::TRIG_0 => false,
            TRIGR::TRIG_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TRIGR {
        match value {
            false => TRIGR::TRIG_0,
            true => TRIGR::TRIG_1,
        }
    }
    #[doc = "Checks if the value of the field is `TRIG_0`"]
    #[inline]
    pub fn is_trig_0(&self) -> bool {
        *self == TRIGR::TRIG_0
    }
    #[doc = "Checks if the value of the field is `TRIG_1`"]
    #[inline]
    pub fn is_trig_1(&self) -> bool {
        *self == TRIGR::TRIG_1
    }
}
#[doc = "Possible values of the field `ENBL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ENBLR {
    #[doc = "DMA Mux channel is disabled"]
    ENBL_0,
    #[doc = "DMA Mux channel is enabled"]
    ENBL_1,
}
impl ENBLR {
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
            ENBLR::ENBL_0 => false,
            ENBLR::ENBL_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ENBLR {
        match value {
            false => ENBLR::ENBL_0,
            true => ENBLR::ENBL_1,
        }
    }
    #[doc = "Checks if the value of the field is `ENBL_0`"]
    #[inline]
    pub fn is_enbl_0(&self) -> bool {
        *self == ENBLR::ENBL_0
    }
    #[doc = "Checks if the value of the field is `ENBL_1`"]
    #[inline]
    pub fn is_enbl_1(&self) -> bool {
        *self == ENBLR::ENBL_1
    }
}
#[doc = r" Proxy"]
pub struct _SOURCEW<'a> {
    w: &'a mut W,
}
impl<'a> _SOURCEW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 127;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `A_ON`"]
pub enum A_ONW {
    #[doc = "DMA Channel Always ON function is disabled"]
    A_ON_0,
    #[doc = "DMA Channel Always ON function is enabled"]
    A_ON_1,
}
impl A_ONW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            A_ONW::A_ON_0 => false,
            A_ONW::A_ON_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _A_ONW<'a> {
    w: &'a mut W,
}
impl<'a> _A_ONW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: A_ONW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "DMA Channel Always ON function is disabled"]
    #[inline]
    pub fn a_on_0(self) -> &'a mut W {
        self.variant(A_ONW::A_ON_0)
    }
    #[doc = "DMA Channel Always ON function is enabled"]
    #[inline]
    pub fn a_on_1(self) -> &'a mut W {
        self.variant(A_ONW::A_ON_1)
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
        const OFFSET: u8 = 29;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `TRIG`"]
pub enum TRIGW {
    #[doc = "Triggering is disabled. If triggering is disabled and ENBL is set, the DMA Channel will simply route the specified source to the DMA channel. (Normal mode)"]
    TRIG_0,
    #[doc = "Triggering is enabled. If triggering is enabled and ENBL is set, the DMA_CH_MUX is in Periodic Trigger mode."]
    TRIG_1,
}
impl TRIGW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TRIGW::TRIG_0 => false,
            TRIGW::TRIG_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TRIGW<'a> {
    w: &'a mut W,
}
impl<'a> _TRIGW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TRIGW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Triggering is disabled. If triggering is disabled and ENBL is set, the DMA Channel will simply route the specified source to the DMA channel. (Normal mode)"]
    #[inline]
    pub fn trig_0(self) -> &'a mut W {
        self.variant(TRIGW::TRIG_0)
    }
    #[doc = "Triggering is enabled. If triggering is enabled and ENBL is set, the DMA_CH_MUX is in Periodic Trigger mode."]
    #[inline]
    pub fn trig_1(self) -> &'a mut W {
        self.variant(TRIGW::TRIG_1)
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
        const OFFSET: u8 = 30;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `ENBL`"]
pub enum ENBLW {
    #[doc = "DMA Mux channel is disabled"]
    ENBL_0,
    #[doc = "DMA Mux channel is enabled"]
    ENBL_1,
}
impl ENBLW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ENBLW::ENBL_0 => false,
            ENBLW::ENBL_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ENBLW<'a> {
    w: &'a mut W,
}
impl<'a> _ENBLW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ENBLW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "DMA Mux channel is disabled"]
    #[inline]
    pub fn enbl_0(self) -> &'a mut W {
        self.variant(ENBLW::ENBL_0)
    }
    #[doc = "DMA Mux channel is enabled"]
    #[inline]
    pub fn enbl_1(self) -> &'a mut W {
        self.variant(ENBLW::ENBL_1)
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
        const OFFSET: u8 = 31;
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
    #[doc = "Bits 0:6 - DMA Channel Source (Slot Number)"]
    #[inline]
    pub fn source(&self) -> SOURCER {
        let bits = {
            const MASK: u8 = 127;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        SOURCER { bits }
    }
    #[doc = "Bit 29 - DMA Channel Always Enable"]
    #[inline]
    pub fn a_on(&self) -> A_ONR {
        A_ONR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 29;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 30 - DMA Channel Trigger Enable"]
    #[inline]
    pub fn trig(&self) -> TRIGR {
        TRIGR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 30;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 31 - DMA Mux Channel Enable"]
    #[inline]
    pub fn enbl(&self) -> ENBLR {
        ENBLR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 31;
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
    #[doc = "Bits 0:6 - DMA Channel Source (Slot Number)"]
    #[inline]
    pub fn source(&mut self) -> _SOURCEW {
        _SOURCEW { w: self }
    }
    #[doc = "Bit 29 - DMA Channel Always Enable"]
    #[inline]
    pub fn a_on(&mut self) -> _A_ONW {
        _A_ONW { w: self }
    }
    #[doc = "Bit 30 - DMA Channel Trigger Enable"]
    #[inline]
    pub fn trig(&mut self) -> _TRIGW {
        _TRIGW { w: self }
    }
    #[doc = "Bit 31 - DMA Mux Channel Enable"]
    #[inline]
    pub fn enbl(&mut self) -> _ENBLW {
        _ENBLW { w: self }
    }
}
