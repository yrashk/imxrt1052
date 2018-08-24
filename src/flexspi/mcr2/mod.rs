#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::MCR2 {
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
#[doc = "Possible values of the field `CLRAHBBUFOPT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CLRAHBBUFOPTR {
    #[doc = "AHB RX/TX Buffer will not be cleaned automatically when FlexSPI return Stop mode ACK."]
    CLRAHBBUFOPT_0,
    #[doc = "AHB RX/TX Buffer will be cleaned automatically when FlexSPI return Stop mode ACK."]
    CLRAHBBUFOPT_1,
}
impl CLRAHBBUFOPTR {
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
            CLRAHBBUFOPTR::CLRAHBBUFOPT_0 => false,
            CLRAHBBUFOPTR::CLRAHBBUFOPT_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CLRAHBBUFOPTR {
        match value {
            false => CLRAHBBUFOPTR::CLRAHBBUFOPT_0,
            true => CLRAHBBUFOPTR::CLRAHBBUFOPT_1,
        }
    }
    #[doc = "Checks if the value of the field is `CLRAHBBUFOPT_0`"]
    #[inline]
    pub fn is_clrahbbufopt_0(&self) -> bool {
        *self == CLRAHBBUFOPTR::CLRAHBBUFOPT_0
    }
    #[doc = "Checks if the value of the field is `CLRAHBBUFOPT_1`"]
    #[inline]
    pub fn is_clrahbbufopt_1(&self) -> bool {
        *self == CLRAHBBUFOPTR::CLRAHBBUFOPT_1
    }
}
#[doc = r" Value of the field"]
pub struct CLRLEARNPHASER {
    bits: bool,
}
impl CLRLEARNPHASER {
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
#[doc = "Possible values of the field `SAMEDEVICEEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SAMEDEVICEENR {
    #[doc = "In Individual mode, FLSHA1CRx/FLSHA2CRx/FLSHB1CRx/FLSHB2CRx register setting will be applied to Flash A1/A2/B1/B2 seperately. In Parallel mode, FLSHA1CRx register setting will be applied to Flash A1 and B1, FLSHA2CRx register setting will be applied to Flash A2 and B2. FLSHB1CRx/FLSHB2CRx register settings will be ignored."]
    SAMEDEVICEEN_0,
    #[doc = "FLSHA1CR0/FLSHA1CR1/FLSHA1CR2 register settings will be applied to Flash A1/A2/B1/B2. FLSHA2CRx/FLSHB1CRx/FLSHB2CRx will be ignored."]
    SAMEDEVICEEN_1,
}
impl SAMEDEVICEENR {
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
            SAMEDEVICEENR::SAMEDEVICEEN_0 => false,
            SAMEDEVICEENR::SAMEDEVICEEN_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SAMEDEVICEENR {
        match value {
            false => SAMEDEVICEENR::SAMEDEVICEEN_0,
            true => SAMEDEVICEENR::SAMEDEVICEEN_1,
        }
    }
    #[doc = "Checks if the value of the field is `SAMEDEVICEEN_0`"]
    #[inline]
    pub fn is_samedeviceen_0(&self) -> bool {
        *self == SAMEDEVICEENR::SAMEDEVICEEN_0
    }
    #[doc = "Checks if the value of the field is `SAMEDEVICEEN_1`"]
    #[inline]
    pub fn is_samedeviceen_1(&self) -> bool {
        *self == SAMEDEVICEENR::SAMEDEVICEEN_1
    }
}
#[doc = "Possible values of the field `SCKBDIFFOPT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SCKBDIFFOPTR {
    #[doc = "SCKB pad is used as port B SCK clock output. Port B flash access is available."]
    SCKBDIFFOPT_0,
    #[doc = "SCKB pad is used as port A SCK inverted clock output (Differential clock to SCKA). Port B flash access is not available."]
    SCKBDIFFOPT_1,
}
impl SCKBDIFFOPTR {
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
            SCKBDIFFOPTR::SCKBDIFFOPT_0 => false,
            SCKBDIFFOPTR::SCKBDIFFOPT_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SCKBDIFFOPTR {
        match value {
            false => SCKBDIFFOPTR::SCKBDIFFOPT_0,
            true => SCKBDIFFOPTR::SCKBDIFFOPT_1,
        }
    }
    #[doc = "Checks if the value of the field is `SCKBDIFFOPT_0`"]
    #[inline]
    pub fn is_sckbdiffopt_0(&self) -> bool {
        *self == SCKBDIFFOPTR::SCKBDIFFOPT_0
    }
    #[doc = "Checks if the value of the field is `SCKBDIFFOPT_1`"]
    #[inline]
    pub fn is_sckbdiffopt_1(&self) -> bool {
        *self == SCKBDIFFOPTR::SCKBDIFFOPT_1
    }
}
#[doc = r" Value of the field"]
pub struct RESUMEWAITR {
    bits: u8,
}
impl RESUMEWAITR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Values that can be written to the field `CLRAHBBUFOPT`"]
pub enum CLRAHBBUFOPTW {
    #[doc = "AHB RX/TX Buffer will not be cleaned automatically when FlexSPI return Stop mode ACK."]
    CLRAHBBUFOPT_0,
    #[doc = "AHB RX/TX Buffer will be cleaned automatically when FlexSPI return Stop mode ACK."]
    CLRAHBBUFOPT_1,
}
impl CLRAHBBUFOPTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CLRAHBBUFOPTW::CLRAHBBUFOPT_0 => false,
            CLRAHBBUFOPTW::CLRAHBBUFOPT_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CLRAHBBUFOPTW<'a> {
    w: &'a mut W,
}
impl<'a> _CLRAHBBUFOPTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CLRAHBBUFOPTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "AHB RX/TX Buffer will not be cleaned automatically when FlexSPI return Stop mode ACK."]
    #[inline]
    pub fn clrahbbufopt_0(self) -> &'a mut W {
        self.variant(CLRAHBBUFOPTW::CLRAHBBUFOPT_0)
    }
    #[doc = "AHB RX/TX Buffer will be cleaned automatically when FlexSPI return Stop mode ACK."]
    #[inline]
    pub fn clrahbbufopt_1(self) -> &'a mut W {
        self.variant(CLRAHBBUFOPTW::CLRAHBBUFOPT_1)
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
        const OFFSET: u8 = 11;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _CLRLEARNPHASEW<'a> {
    w: &'a mut W,
}
impl<'a> _CLRLEARNPHASEW<'a> {
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
        const OFFSET: u8 = 14;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `SAMEDEVICEEN`"]
pub enum SAMEDEVICEENW {
    #[doc = "In Individual mode, FLSHA1CRx/FLSHA2CRx/FLSHB1CRx/FLSHB2CRx register setting will be applied to Flash A1/A2/B1/B2 seperately. In Parallel mode, FLSHA1CRx register setting will be applied to Flash A1 and B1, FLSHA2CRx register setting will be applied to Flash A2 and B2. FLSHB1CRx/FLSHB2CRx register settings will be ignored."]
    SAMEDEVICEEN_0,
    #[doc = "FLSHA1CR0/FLSHA1CR1/FLSHA1CR2 register settings will be applied to Flash A1/A2/B1/B2. FLSHA2CRx/FLSHB1CRx/FLSHB2CRx will be ignored."]
    SAMEDEVICEEN_1,
}
impl SAMEDEVICEENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SAMEDEVICEENW::SAMEDEVICEEN_0 => false,
            SAMEDEVICEENW::SAMEDEVICEEN_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SAMEDEVICEENW<'a> {
    w: &'a mut W,
}
impl<'a> _SAMEDEVICEENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SAMEDEVICEENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "In Individual mode, FLSHA1CRx/FLSHA2CRx/FLSHB1CRx/FLSHB2CRx register setting will be applied to Flash A1/A2/B1/B2 seperately. In Parallel mode, FLSHA1CRx register setting will be applied to Flash A1 and B1, FLSHA2CRx register setting will be applied to Flash A2 and B2. FLSHB1CRx/FLSHB2CRx register settings will be ignored."]
    #[inline]
    pub fn samedeviceen_0(self) -> &'a mut W {
        self.variant(SAMEDEVICEENW::SAMEDEVICEEN_0)
    }
    #[doc = "FLSHA1CR0/FLSHA1CR1/FLSHA1CR2 register settings will be applied to Flash A1/A2/B1/B2. FLSHA2CRx/FLSHB1CRx/FLSHB2CRx will be ignored."]
    #[inline]
    pub fn samedeviceen_1(self) -> &'a mut W {
        self.variant(SAMEDEVICEENW::SAMEDEVICEEN_1)
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
        const OFFSET: u8 = 15;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `SCKBDIFFOPT`"]
pub enum SCKBDIFFOPTW {
    #[doc = "SCKB pad is used as port B SCK clock output. Port B flash access is available."]
    SCKBDIFFOPT_0,
    #[doc = "SCKB pad is used as port A SCK inverted clock output (Differential clock to SCKA). Port B flash access is not available."]
    SCKBDIFFOPT_1,
}
impl SCKBDIFFOPTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SCKBDIFFOPTW::SCKBDIFFOPT_0 => false,
            SCKBDIFFOPTW::SCKBDIFFOPT_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SCKBDIFFOPTW<'a> {
    w: &'a mut W,
}
impl<'a> _SCKBDIFFOPTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SCKBDIFFOPTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "SCKB pad is used as port B SCK clock output. Port B flash access is available."]
    #[inline]
    pub fn sckbdiffopt_0(self) -> &'a mut W {
        self.variant(SCKBDIFFOPTW::SCKBDIFFOPT_0)
    }
    #[doc = "SCKB pad is used as port A SCK inverted clock output (Differential clock to SCKA). Port B flash access is not available."]
    #[inline]
    pub fn sckbdiffopt_1(self) -> &'a mut W {
        self.variant(SCKBDIFFOPTW::SCKBDIFFOPT_1)
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
        const OFFSET: u8 = 19;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _RESUMEWAITW<'a> {
    w: &'a mut W,
}
impl<'a> _RESUMEWAITW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 255;
        const OFFSET: u8 = 24;
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
    #[doc = "Bit 11 - This bit determines whether AHB RX Buffer and AHB TX Buffer will be cleaned automaticaly when FlexSPI returns STOP mode ACK. Software should set this bit if AHB RX Buffer or AHB TX Buffer will be powered off in STOP mode. Otherwise AHB read access after exiting STOP mode may hit AHB RX Buffer or AHB TX Buffer but their data entries are invalid."]
    #[inline]
    pub fn clrahbbufopt(&self) -> CLRAHBBUFOPTR {
        CLRAHBBUFOPTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 14 - The sampling clock phase selection will be reset to phase 0 when this bit is written with 0x1. This bit will be auto-cleared immediately."]
    #[inline]
    pub fn clrlearnphase(&self) -> CLRLEARNPHASER {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        CLRLEARNPHASER { bits }
    }
    #[doc = "Bit 15 - All external devices are same devices (both in types and size) for A1/A2/B1/B2."]
    #[inline]
    pub fn samedeviceen(&self) -> SAMEDEVICEENR {
        SAMEDEVICEENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 15;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 19 - SCKB pad can be used as SCKA differential clock output (inverted clock to SCKA). In this case, port B flash access is not available. After change the value of this feild, MCR0[SWRESET] should be set."]
    #[inline]
    pub fn sckbdiffopt(&self) -> SCKBDIFFOPTR {
        SCKBDIFFOPTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 19;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 24:31 - Wait cycle (in AHB clock cycle) for idle state before suspended command sequence resumed."]
    #[inline]
    pub fn resumewait(&self) -> RESUMEWAITR {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        RESUMEWAITR { bits }
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 536904183 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 11 - This bit determines whether AHB RX Buffer and AHB TX Buffer will be cleaned automaticaly when FlexSPI returns STOP mode ACK. Software should set this bit if AHB RX Buffer or AHB TX Buffer will be powered off in STOP mode. Otherwise AHB read access after exiting STOP mode may hit AHB RX Buffer or AHB TX Buffer but their data entries are invalid."]
    #[inline]
    pub fn clrahbbufopt(&mut self) -> _CLRAHBBUFOPTW {
        _CLRAHBBUFOPTW { w: self }
    }
    #[doc = "Bit 14 - The sampling clock phase selection will be reset to phase 0 when this bit is written with 0x1. This bit will be auto-cleared immediately."]
    #[inline]
    pub fn clrlearnphase(&mut self) -> _CLRLEARNPHASEW {
        _CLRLEARNPHASEW { w: self }
    }
    #[doc = "Bit 15 - All external devices are same devices (both in types and size) for A1/A2/B1/B2."]
    #[inline]
    pub fn samedeviceen(&mut self) -> _SAMEDEVICEENW {
        _SAMEDEVICEENW { w: self }
    }
    #[doc = "Bit 19 - SCKB pad can be used as SCKA differential clock output (inverted clock to SCKA). In this case, port B flash access is not available. After change the value of this feild, MCR0[SWRESET] should be set."]
    #[inline]
    pub fn sckbdiffopt(&mut self) -> _SCKBDIFFOPTW {
        _SCKBDIFFOPTW { w: self }
    }
    #[doc = "Bits 24:31 - Wait cycle (in AHB clock cycle) for idle state before suspended command sequence resumed."]
    #[inline]
    pub fn resumewait(&mut self) -> _RESUMEWAITW {
        _RESUMEWAITW { w: self }
    }
}
