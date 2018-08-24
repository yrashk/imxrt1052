#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::IPTXFCR {
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
pub struct CLRIPTXFR {
    bits: bool,
}
impl CLRIPTXFR {
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
#[doc = "Possible values of the field `TXDMAEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXDMAENR {
    #[doc = "IP TX FIFO would be filled by processor."]
    TXDMAEN_0,
    #[doc = "IP TX FIFO would be filled by DMA."]
    TXDMAEN_1,
}
impl TXDMAENR {
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
            TXDMAENR::TXDMAEN_0 => false,
            TXDMAENR::TXDMAEN_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TXDMAENR {
        match value {
            false => TXDMAENR::TXDMAEN_0,
            true => TXDMAENR::TXDMAEN_1,
        }
    }
    #[doc = "Checks if the value of the field is `TXDMAEN_0`"]
    #[inline]
    pub fn is_txdmaen_0(&self) -> bool {
        *self == TXDMAENR::TXDMAEN_0
    }
    #[doc = "Checks if the value of the field is `TXDMAEN_1`"]
    #[inline]
    pub fn is_txdmaen_1(&self) -> bool {
        *self == TXDMAENR::TXDMAEN_1
    }
}
#[doc = r" Value of the field"]
pub struct TXWMRKR {
    bits: u8,
}
impl TXWMRKR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Proxy"]
pub struct _CLRIPTXFW<'a> {
    w: &'a mut W,
}
impl<'a> _CLRIPTXFW<'a> {
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
#[doc = "Values that can be written to the field `TXDMAEN`"]
pub enum TXDMAENW {
    #[doc = "IP TX FIFO would be filled by processor."]
    TXDMAEN_0,
    #[doc = "IP TX FIFO would be filled by DMA."]
    TXDMAEN_1,
}
impl TXDMAENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TXDMAENW::TXDMAEN_0 => false,
            TXDMAENW::TXDMAEN_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TXDMAENW<'a> {
    w: &'a mut W,
}
impl<'a> _TXDMAENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TXDMAENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "IP TX FIFO would be filled by processor."]
    #[inline]
    pub fn txdmaen_0(self) -> &'a mut W {
        self.variant(TXDMAENW::TXDMAEN_0)
    }
    #[doc = "IP TX FIFO would be filled by DMA."]
    #[inline]
    pub fn txdmaen_1(self) -> &'a mut W {
        self.variant(TXDMAENW::TXDMAEN_1)
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
pub struct _TXWMRKW<'a> {
    w: &'a mut W,
}
impl<'a> _TXWMRKW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
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
    #[doc = "Bit 0 - Clear all valid data entries in IP TX FIFO."]
    #[inline]
    pub fn clriptxf(&self) -> CLRIPTXFR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        CLRIPTXFR { bits }
    }
    #[doc = "Bit 1 - IP TX FIFO filling by DMA enabled."]
    #[inline]
    pub fn txdmaen(&self) -> TXDMAENR {
        TXDMAENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 2:5 - Watermark level is (TXWMRK+1)*64 Bits."]
    #[inline]
    pub fn txwmrk(&self) -> TXWMRKR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        TXWMRKR { bits }
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
    #[doc = "Bit 0 - Clear all valid data entries in IP TX FIFO."]
    #[inline]
    pub fn clriptxf(&mut self) -> _CLRIPTXFW {
        _CLRIPTXFW { w: self }
    }
    #[doc = "Bit 1 - IP TX FIFO filling by DMA enabled."]
    #[inline]
    pub fn txdmaen(&mut self) -> _TXDMAENW {
        _TXDMAENW { w: self }
    }
    #[doc = "Bits 2:5 - Watermark level is (TXWMRK+1)*64 Bits."]
    #[inline]
    pub fn txwmrk(&mut self) -> _TXWMRKW {
        _TXWMRKW { w: self }
    }
}
