#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::RX_TOG {
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
pub struct ENVADJR {
    bits: u8,
}
impl ENVADJR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct RSVD0R {
    bits: bool,
}
impl RSVD0R {
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
#[doc = r" Value of the field"]
pub struct DISCONADJR {
    bits: u8,
}
impl DISCONADJR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct RSVD1R {
    bits: u16,
}
impl RSVD1R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct RXDBYPASSR {
    bits: bool,
}
impl RXDBYPASSR {
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
#[doc = r" Value of the field"]
pub struct RSVD2R {
    bits: u16,
}
impl RSVD2R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }
}
#[doc = r" Proxy"]
pub struct _ENVADJW<'a> {
    w: &'a mut W,
}
impl<'a> _ENVADJW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _DISCONADJW<'a> {
    w: &'a mut W,
}
impl<'a> _DISCONADJW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 4;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _RXDBYPASSW<'a> {
    w: &'a mut W,
}
impl<'a> _RXDBYPASSW<'a> {
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
        const OFFSET: u8 = 22;
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
    #[doc = "Bits 0:2 - The ENVADJ field adjusts the trip point for the envelope detector"]
    #[inline]
    pub fn envadj(&self) -> ENVADJR {
        let bits = {
            const MASK: u8 = 7;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        ENVADJR { bits }
    }
    #[doc = "Bit 3 - Reserved."]
    #[inline]
    pub fn rsvd0(&self) -> RSVD0R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        RSVD0R { bits }
    }
    #[doc = "Bits 4:6 - The DISCONADJ field adjusts the trip point for the disconnect detector: 000 = Trip-Level Voltage is 0"]
    #[inline]
    pub fn disconadj(&self) -> DISCONADJR {
        let bits = {
            const MASK: u8 = 7;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        DISCONADJR { bits }
    }
    #[doc = "Bits 7:21 - Reserved."]
    #[inline]
    pub fn rsvd1(&self) -> RSVD1R {
        let bits = {
            const MASK: u16 = 32767;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        };
        RSVD1R { bits }
    }
    #[doc = "Bit 22 - 0 = Normal operation"]
    #[inline]
    pub fn rxdbypass(&self) -> RXDBYPASSR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 22;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        RXDBYPASSR { bits }
    }
    #[doc = "Bits 23:31 - Reserved."]
    #[inline]
    pub fn rsvd2(&self) -> RSVD2R {
        let bits = {
            const MASK: u16 = 511;
            const OFFSET: u8 = 23;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        };
        RSVD2R { bits }
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
    #[doc = "Bits 0:2 - The ENVADJ field adjusts the trip point for the envelope detector"]
    #[inline]
    pub fn envadj(&mut self) -> _ENVADJW {
        _ENVADJW { w: self }
    }
    #[doc = "Bits 4:6 - The DISCONADJ field adjusts the trip point for the disconnect detector: 000 = Trip-Level Voltage is 0"]
    #[inline]
    pub fn disconadj(&mut self) -> _DISCONADJW {
        _DISCONADJW { w: self }
    }
    #[doc = "Bit 22 - 0 = Normal operation"]
    #[inline]
    pub fn rxdbypass(&mut self) -> _RXDBYPASSW {
        _RXDBYPASSW { w: self }
    }
}
