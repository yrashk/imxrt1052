#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::RCR2 {
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
pub struct DIVR {
    bits: u8,
}
impl DIVR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Possible values of the field `BCD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BCDR {
    #[doc = "Bit clock is generated externally in Slave mode."]
    BCD_0,
    #[doc = "Bit clock is generated internally in Master mode."]
    BCD_1,
}
impl BCDR {
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
            BCDR::BCD_0 => false,
            BCDR::BCD_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> BCDR {
        match value {
            false => BCDR::BCD_0,
            true => BCDR::BCD_1,
        }
    }
    #[doc = "Checks if the value of the field is `BCD_0`"]
    #[inline]
    pub fn is_bcd_0(&self) -> bool {
        *self == BCDR::BCD_0
    }
    #[doc = "Checks if the value of the field is `BCD_1`"]
    #[inline]
    pub fn is_bcd_1(&self) -> bool {
        *self == BCDR::BCD_1
    }
}
#[doc = "Possible values of the field `BCP`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BCPR {
    #[doc = "Bit Clock is active high with drive outputs on rising edge and sample inputs on falling edge."]
    BCP_0,
    #[doc = "Bit Clock is active low with drive outputs on falling edge and sample inputs on rising edge."]
    BCP_1,
}
impl BCPR {
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
            BCPR::BCP_0 => false,
            BCPR::BCP_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> BCPR {
        match value {
            false => BCPR::BCP_0,
            true => BCPR::BCP_1,
        }
    }
    #[doc = "Checks if the value of the field is `BCP_0`"]
    #[inline]
    pub fn is_bcp_0(&self) -> bool {
        *self == BCPR::BCP_0
    }
    #[doc = "Checks if the value of the field is `BCP_1`"]
    #[inline]
    pub fn is_bcp_1(&self) -> bool {
        *self == BCPR::BCP_1
    }
}
#[doc = "Possible values of the field `MSEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MSELR {
    #[doc = "Bus Clock selected."]
    MSEL_0,
    #[doc = "Master Clock (MCLK) 1 option selected."]
    MSEL_1,
    #[doc = "Master Clock (MCLK) 2 option selected."]
    MSEL_2,
    #[doc = "Master Clock (MCLK) 3 option selected."]
    MSEL_3,
}
impl MSELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            MSELR::MSEL_0 => 0,
            MSELR::MSEL_1 => 1,
            MSELR::MSEL_2 => 2,
            MSELR::MSEL_3 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> MSELR {
        match value {
            0 => MSELR::MSEL_0,
            1 => MSELR::MSEL_1,
            2 => MSELR::MSEL_2,
            3 => MSELR::MSEL_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `MSEL_0`"]
    #[inline]
    pub fn is_msel_0(&self) -> bool {
        *self == MSELR::MSEL_0
    }
    #[doc = "Checks if the value of the field is `MSEL_1`"]
    #[inline]
    pub fn is_msel_1(&self) -> bool {
        *self == MSELR::MSEL_1
    }
    #[doc = "Checks if the value of the field is `MSEL_2`"]
    #[inline]
    pub fn is_msel_2(&self) -> bool {
        *self == MSELR::MSEL_2
    }
    #[doc = "Checks if the value of the field is `MSEL_3`"]
    #[inline]
    pub fn is_msel_3(&self) -> bool {
        *self == MSELR::MSEL_3
    }
}
#[doc = "Possible values of the field `BCI`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BCIR {
    #[doc = "No effect."]
    BCI_0,
    #[doc = "Internal logic is clocked as if bit clock was externally generated."]
    BCI_1,
}
impl BCIR {
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
            BCIR::BCI_0 => false,
            BCIR::BCI_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> BCIR {
        match value {
            false => BCIR::BCI_0,
            true => BCIR::BCI_1,
        }
    }
    #[doc = "Checks if the value of the field is `BCI_0`"]
    #[inline]
    pub fn is_bci_0(&self) -> bool {
        *self == BCIR::BCI_0
    }
    #[doc = "Checks if the value of the field is `BCI_1`"]
    #[inline]
    pub fn is_bci_1(&self) -> bool {
        *self == BCIR::BCI_1
    }
}
#[doc = "Possible values of the field `BCS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BCSR {
    #[doc = "Use the normal bit clock source."]
    BCS_0,
    #[doc = "Swap the bit clock source."]
    BCS_1,
}
impl BCSR {
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
            BCSR::BCS_0 => false,
            BCSR::BCS_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> BCSR {
        match value {
            false => BCSR::BCS_0,
            true => BCSR::BCS_1,
        }
    }
    #[doc = "Checks if the value of the field is `BCS_0`"]
    #[inline]
    pub fn is_bcs_0(&self) -> bool {
        *self == BCSR::BCS_0
    }
    #[doc = "Checks if the value of the field is `BCS_1`"]
    #[inline]
    pub fn is_bcs_1(&self) -> bool {
        *self == BCSR::BCS_1
    }
}
#[doc = "Possible values of the field `SYNC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SYNCR {
    #[doc = "Asynchronous mode."]
    SYNC_0,
    #[doc = "Synchronous with transmitter."]
    SYNC_1,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl SYNCR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            SYNCR::SYNC_0 => 0,
            SYNCR::SYNC_1 => 1,
            SYNCR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> SYNCR {
        match value {
            0 => SYNCR::SYNC_0,
            1 => SYNCR::SYNC_1,
            i => SYNCR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `SYNC_0`"]
    #[inline]
    pub fn is_sync_0(&self) -> bool {
        *self == SYNCR::SYNC_0
    }
    #[doc = "Checks if the value of the field is `SYNC_1`"]
    #[inline]
    pub fn is_sync_1(&self) -> bool {
        *self == SYNCR::SYNC_1
    }
}
#[doc = r" Proxy"]
pub struct _DIVW<'a> {
    w: &'a mut W,
}
impl<'a> _DIVW<'a> {
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
#[doc = "Values that can be written to the field `BCD`"]
pub enum BCDW {
    #[doc = "Bit clock is generated externally in Slave mode."]
    BCD_0,
    #[doc = "Bit clock is generated internally in Master mode."]
    BCD_1,
}
impl BCDW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            BCDW::BCD_0 => false,
            BCDW::BCD_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _BCDW<'a> {
    w: &'a mut W,
}
impl<'a> _BCDW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: BCDW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Bit clock is generated externally in Slave mode."]
    #[inline]
    pub fn bcd_0(self) -> &'a mut W {
        self.variant(BCDW::BCD_0)
    }
    #[doc = "Bit clock is generated internally in Master mode."]
    #[inline]
    pub fn bcd_1(self) -> &'a mut W {
        self.variant(BCDW::BCD_1)
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
        const OFFSET: u8 = 24;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `BCP`"]
pub enum BCPW {
    #[doc = "Bit Clock is active high with drive outputs on rising edge and sample inputs on falling edge."]
    BCP_0,
    #[doc = "Bit Clock is active low with drive outputs on falling edge and sample inputs on rising edge."]
    BCP_1,
}
impl BCPW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            BCPW::BCP_0 => false,
            BCPW::BCP_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _BCPW<'a> {
    w: &'a mut W,
}
impl<'a> _BCPW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: BCPW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Bit Clock is active high with drive outputs on rising edge and sample inputs on falling edge."]
    #[inline]
    pub fn bcp_0(self) -> &'a mut W {
        self.variant(BCPW::BCP_0)
    }
    #[doc = "Bit Clock is active low with drive outputs on falling edge and sample inputs on rising edge."]
    #[inline]
    pub fn bcp_1(self) -> &'a mut W {
        self.variant(BCPW::BCP_1)
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
        const OFFSET: u8 = 25;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `MSEL`"]
pub enum MSELW {
    #[doc = "Bus Clock selected."]
    MSEL_0,
    #[doc = "Master Clock (MCLK) 1 option selected."]
    MSEL_1,
    #[doc = "Master Clock (MCLK) 2 option selected."]
    MSEL_2,
    #[doc = "Master Clock (MCLK) 3 option selected."]
    MSEL_3,
}
impl MSELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            MSELW::MSEL_0 => 0,
            MSELW::MSEL_1 => 1,
            MSELW::MSEL_2 => 2,
            MSELW::MSEL_3 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MSELW<'a> {
    w: &'a mut W,
}
impl<'a> _MSELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MSELW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Bus Clock selected."]
    #[inline]
    pub fn msel_0(self) -> &'a mut W {
        self.variant(MSELW::MSEL_0)
    }
    #[doc = "Master Clock (MCLK) 1 option selected."]
    #[inline]
    pub fn msel_1(self) -> &'a mut W {
        self.variant(MSELW::MSEL_1)
    }
    #[doc = "Master Clock (MCLK) 2 option selected."]
    #[inline]
    pub fn msel_2(self) -> &'a mut W {
        self.variant(MSELW::MSEL_2)
    }
    #[doc = "Master Clock (MCLK) 3 option selected."]
    #[inline]
    pub fn msel_3(self) -> &'a mut W {
        self.variant(MSELW::MSEL_3)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 26;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `BCI`"]
pub enum BCIW {
    #[doc = "No effect."]
    BCI_0,
    #[doc = "Internal logic is clocked as if bit clock was externally generated."]
    BCI_1,
}
impl BCIW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            BCIW::BCI_0 => false,
            BCIW::BCI_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _BCIW<'a> {
    w: &'a mut W,
}
impl<'a> _BCIW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: BCIW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No effect."]
    #[inline]
    pub fn bci_0(self) -> &'a mut W {
        self.variant(BCIW::BCI_0)
    }
    #[doc = "Internal logic is clocked as if bit clock was externally generated."]
    #[inline]
    pub fn bci_1(self) -> &'a mut W {
        self.variant(BCIW::BCI_1)
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
        const OFFSET: u8 = 28;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `BCS`"]
pub enum BCSW {
    #[doc = "Use the normal bit clock source."]
    BCS_0,
    #[doc = "Swap the bit clock source."]
    BCS_1,
}
impl BCSW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            BCSW::BCS_0 => false,
            BCSW::BCS_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _BCSW<'a> {
    w: &'a mut W,
}
impl<'a> _BCSW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: BCSW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Use the normal bit clock source."]
    #[inline]
    pub fn bcs_0(self) -> &'a mut W {
        self.variant(BCSW::BCS_0)
    }
    #[doc = "Swap the bit clock source."]
    #[inline]
    pub fn bcs_1(self) -> &'a mut W {
        self.variant(BCSW::BCS_1)
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
#[doc = "Values that can be written to the field `SYNC`"]
pub enum SYNCW {
    #[doc = "Asynchronous mode."]
    SYNC_0,
    #[doc = "Synchronous with transmitter."]
    SYNC_1,
}
impl SYNCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            SYNCW::SYNC_0 => 0,
            SYNCW::SYNC_1 => 1,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SYNCW<'a> {
    w: &'a mut W,
}
impl<'a> _SYNCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SYNCW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Asynchronous mode."]
    #[inline]
    pub fn sync_0(self) -> &'a mut W {
        self.variant(SYNCW::SYNC_0)
    }
    #[doc = "Synchronous with transmitter."]
    #[inline]
    pub fn sync_1(self) -> &'a mut W {
        self.variant(SYNCW::SYNC_1)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 30;
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
    #[doc = "Bits 0:7 - Bit Clock Divide"]
    #[inline]
    pub fn div(&self) -> DIVR {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        DIVR { bits }
    }
    #[doc = "Bit 24 - Bit Clock Direction"]
    #[inline]
    pub fn bcd(&self) -> BCDR {
        BCDR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 25 - Bit Clock Polarity"]
    #[inline]
    pub fn bcp(&self) -> BCPR {
        BCPR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 25;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 26:27 - MCLK Select"]
    #[inline]
    pub fn msel(&self) -> MSELR {
        MSELR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 26;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 28 - Bit Clock Input"]
    #[inline]
    pub fn bci(&self) -> BCIR {
        BCIR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 28;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 29 - Bit Clock Swap"]
    #[inline]
    pub fn bcs(&self) -> BCSR {
        BCSR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 29;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 30:31 - Synchronous Mode"]
    #[inline]
    pub fn sync(&self) -> SYNCR {
        SYNCR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 30;
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
    #[doc = "Bits 0:7 - Bit Clock Divide"]
    #[inline]
    pub fn div(&mut self) -> _DIVW {
        _DIVW { w: self }
    }
    #[doc = "Bit 24 - Bit Clock Direction"]
    #[inline]
    pub fn bcd(&mut self) -> _BCDW {
        _BCDW { w: self }
    }
    #[doc = "Bit 25 - Bit Clock Polarity"]
    #[inline]
    pub fn bcp(&mut self) -> _BCPW {
        _BCPW { w: self }
    }
    #[doc = "Bits 26:27 - MCLK Select"]
    #[inline]
    pub fn msel(&mut self) -> _MSELW {
        _MSELW { w: self }
    }
    #[doc = "Bit 28 - Bit Clock Input"]
    #[inline]
    pub fn bci(&mut self) -> _BCIW {
        _BCIW { w: self }
    }
    #[doc = "Bit 29 - Bit Clock Swap"]
    #[inline]
    pub fn bcs(&mut self) -> _BCSW {
        _BCSW { w: self }
    }
    #[doc = "Bits 30:31 - Synchronous Mode"]
    #[inline]
    pub fn sync(&mut self) -> _SYNCW {
        _SYNCW { w: self }
    }
}
