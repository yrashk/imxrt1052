#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::TX_TOG {
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
pub struct D_CALR {
    bits: u8,
}
impl D_CALR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct RSVD0R {
    bits: u8,
}
impl RSVD0R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct TXCAL45DNR {
    bits: u8,
}
impl TXCAL45DNR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct RSVD1R {
    bits: u8,
}
impl RSVD1R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct TXCAL45DPR {
    bits: u8,
}
impl TXCAL45DPR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct RSVD2R {
    bits: u8,
}
impl RSVD2R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct USBPHY_TX_EDGECTRLR {
    bits: u8,
}
impl USBPHY_TX_EDGECTRLR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct RSVD5R {
    bits: u8,
}
impl RSVD5R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Proxy"]
pub struct _D_CALW<'a> {
    w: &'a mut W,
}
impl<'a> _D_CALW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _RSVD0W<'a> {
    w: &'a mut W,
}
impl<'a> _RSVD0W<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 4;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _TXCAL45DNW<'a> {
    w: &'a mut W,
}
impl<'a> _TXCAL45DNW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _RSVD1W<'a> {
    w: &'a mut W,
}
impl<'a> _RSVD1W<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 12;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _TXCAL45DPW<'a> {
    w: &'a mut W,
}
impl<'a> _TXCAL45DPW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 16;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _USBPHY_TX_EDGECTRLW<'a> {
    w: &'a mut W,
}
impl<'a> _USBPHY_TX_EDGECTRLW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 26;
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
    #[doc = "Bits 0:3 - Resistor Trimming Code: 0000 = 0.16% 0111 = Nominal 1111 = +25%"]
    #[inline]
    pub fn d_cal(&self) -> D_CALR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        D_CALR { bits }
    }
    #[doc = "Bits 4:7 - Reserved. Note: This bit should remain clear."]
    #[inline]
    pub fn rsvd0(&self) -> RSVD0R {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        RSVD0R { bits }
    }
    #[doc = "Bits 8:11 - Decode to select a 45-Ohm resistance to the USB_DN output pin"]
    #[inline]
    pub fn txcal45dn(&self) -> TXCAL45DNR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        TXCAL45DNR { bits }
    }
    #[doc = "Bits 12:15 - Reserved. Note: This bit should remain clear."]
    #[inline]
    pub fn rsvd1(&self) -> RSVD1R {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        RSVD1R { bits }
    }
    #[doc = "Bits 16:19 - Decode to select a 45-Ohm resistance to the USB_DP output pin"]
    #[inline]
    pub fn txcal45dp(&self) -> TXCAL45DPR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        TXCAL45DPR { bits }
    }
    #[doc = "Bits 20:25 - Reserved."]
    #[inline]
    pub fn rsvd2(&self) -> RSVD2R {
        let bits = {
            const MASK: u8 = 63;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        RSVD2R { bits }
    }
    #[doc = "Bits 26:28 - Controls the edge-rate of the current sensing transistors used in HS transmit"]
    #[inline]
    pub fn usbphy_tx_edgectrl(&self) -> USBPHY_TX_EDGECTRLR {
        let bits = {
            const MASK: u8 = 7;
            const OFFSET: u8 = 26;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        USBPHY_TX_EDGECTRLR { bits }
    }
    #[doc = "Bits 29:31 - Reserved."]
    #[inline]
    pub fn rsvd5(&self) -> RSVD5R {
        let bits = {
            const MASK: u8 = 7;
            const OFFSET: u8 = 29;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        RSVD5R { bits }
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 268830215 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:3 - Resistor Trimming Code: 0000 = 0.16% 0111 = Nominal 1111 = +25%"]
    #[inline]
    pub fn d_cal(&mut self) -> _D_CALW {
        _D_CALW { w: self }
    }
    #[doc = "Bits 4:7 - Reserved. Note: This bit should remain clear."]
    #[inline]
    pub fn rsvd0(&mut self) -> _RSVD0W {
        _RSVD0W { w: self }
    }
    #[doc = "Bits 8:11 - Decode to select a 45-Ohm resistance to the USB_DN output pin"]
    #[inline]
    pub fn txcal45dn(&mut self) -> _TXCAL45DNW {
        _TXCAL45DNW { w: self }
    }
    #[doc = "Bits 12:15 - Reserved. Note: This bit should remain clear."]
    #[inline]
    pub fn rsvd1(&mut self) -> _RSVD1W {
        _RSVD1W { w: self }
    }
    #[doc = "Bits 16:19 - Decode to select a 45-Ohm resistance to the USB_DP output pin"]
    #[inline]
    pub fn txcal45dp(&mut self) -> _TXCAL45DPW {
        _TXCAL45DPW { w: self }
    }
    #[doc = "Bits 26:28 - Controls the edge-rate of the current sensing transistors used in HS transmit"]
    #[inline]
    pub fn usbphy_tx_edgectrl(&mut self) -> _USBPHY_TX_EDGECTRLW {
        _USBPHY_TX_EDGECTRLW { w: self }
    }
}
