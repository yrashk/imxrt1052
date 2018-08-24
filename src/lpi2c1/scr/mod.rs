#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::SCR {
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
#[doc = "Possible values of the field `SEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SENR {
    #[doc = "I2C Slave mode is disabled"]
    SEN_0,
    #[doc = "I2C Slave mode is enabled"]
    SEN_1,
}
impl SENR {
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
            SENR::SEN_0 => false,
            SENR::SEN_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SENR {
        match value {
            false => SENR::SEN_0,
            true => SENR::SEN_1,
        }
    }
    #[doc = "Checks if the value of the field is `SEN_0`"]
    #[inline]
    pub fn is_sen_0(&self) -> bool {
        *self == SENR::SEN_0
    }
    #[doc = "Checks if the value of the field is `SEN_1`"]
    #[inline]
    pub fn is_sen_1(&self) -> bool {
        *self == SENR::SEN_1
    }
}
#[doc = "Possible values of the field `RST`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RSTR {
    #[doc = "Slave mode logic is not reset"]
    RST_0,
    #[doc = "Slave mode logic is reset"]
    RST_1,
}
impl RSTR {
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
            RSTR::RST_0 => false,
            RSTR::RST_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RSTR {
        match value {
            false => RSTR::RST_0,
            true => RSTR::RST_1,
        }
    }
    #[doc = "Checks if the value of the field is `RST_0`"]
    #[inline]
    pub fn is_rst_0(&self) -> bool {
        *self == RSTR::RST_0
    }
    #[doc = "Checks if the value of the field is `RST_1`"]
    #[inline]
    pub fn is_rst_1(&self) -> bool {
        *self == RSTR::RST_1
    }
}
#[doc = "Possible values of the field `FILTEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FILTENR {
    #[doc = "Disable digital filter and output delay counter for slave mode"]
    FILTEN_0,
    #[doc = "Enable digital filter and output delay counter for slave mode"]
    FILTEN_1,
}
impl FILTENR {
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
            FILTENR::FILTEN_0 => false,
            FILTENR::FILTEN_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> FILTENR {
        match value {
            false => FILTENR::FILTEN_0,
            true => FILTENR::FILTEN_1,
        }
    }
    #[doc = "Checks if the value of the field is `FILTEN_0`"]
    #[inline]
    pub fn is_filten_0(&self) -> bool {
        *self == FILTENR::FILTEN_0
    }
    #[doc = "Checks if the value of the field is `FILTEN_1`"]
    #[inline]
    pub fn is_filten_1(&self) -> bool {
        *self == FILTENR::FILTEN_1
    }
}
#[doc = "Possible values of the field `FILTDZ`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FILTDZR {
    #[doc = "Filter remains enabled in Doze mode"]
    FILTDZ_0,
    #[doc = "Filter is disabled in Doze mode"]
    FILTDZ_1,
}
impl FILTDZR {
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
            FILTDZR::FILTDZ_0 => false,
            FILTDZR::FILTDZ_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> FILTDZR {
        match value {
            false => FILTDZR::FILTDZ_0,
            true => FILTDZR::FILTDZ_1,
        }
    }
    #[doc = "Checks if the value of the field is `FILTDZ_0`"]
    #[inline]
    pub fn is_filtdz_0(&self) -> bool {
        *self == FILTDZR::FILTDZ_0
    }
    #[doc = "Checks if the value of the field is `FILTDZ_1`"]
    #[inline]
    pub fn is_filtdz_1(&self) -> bool {
        *self == FILTDZR::FILTDZ_1
    }
}
#[doc = "Values that can be written to the field `SEN`"]
pub enum SENW {
    #[doc = "I2C Slave mode is disabled"]
    SEN_0,
    #[doc = "I2C Slave mode is enabled"]
    SEN_1,
}
impl SENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SENW::SEN_0 => false,
            SENW::SEN_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SENW<'a> {
    w: &'a mut W,
}
impl<'a> _SENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "I2C Slave mode is disabled"]
    #[inline]
    pub fn sen_0(self) -> &'a mut W {
        self.variant(SENW::SEN_0)
    }
    #[doc = "I2C Slave mode is enabled"]
    #[inline]
    pub fn sen_1(self) -> &'a mut W {
        self.variant(SENW::SEN_1)
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
#[doc = "Values that can be written to the field `RST`"]
pub enum RSTW {
    #[doc = "Slave mode logic is not reset"]
    RST_0,
    #[doc = "Slave mode logic is reset"]
    RST_1,
}
impl RSTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RSTW::RST_0 => false,
            RSTW::RST_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RSTW<'a> {
    w: &'a mut W,
}
impl<'a> _RSTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RSTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Slave mode logic is not reset"]
    #[inline]
    pub fn rst_0(self) -> &'a mut W {
        self.variant(RSTW::RST_0)
    }
    #[doc = "Slave mode logic is reset"]
    #[inline]
    pub fn rst_1(self) -> &'a mut W {
        self.variant(RSTW::RST_1)
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
#[doc = "Values that can be written to the field `FILTEN`"]
pub enum FILTENW {
    #[doc = "Disable digital filter and output delay counter for slave mode"]
    FILTEN_0,
    #[doc = "Enable digital filter and output delay counter for slave mode"]
    FILTEN_1,
}
impl FILTENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            FILTENW::FILTEN_0 => false,
            FILTENW::FILTEN_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FILTENW<'a> {
    w: &'a mut W,
}
impl<'a> _FILTENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FILTENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable digital filter and output delay counter for slave mode"]
    #[inline]
    pub fn filten_0(self) -> &'a mut W {
        self.variant(FILTENW::FILTEN_0)
    }
    #[doc = "Enable digital filter and output delay counter for slave mode"]
    #[inline]
    pub fn filten_1(self) -> &'a mut W {
        self.variant(FILTENW::FILTEN_1)
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
        const OFFSET: u8 = 4;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `FILTDZ`"]
pub enum FILTDZW {
    #[doc = "Filter remains enabled in Doze mode"]
    FILTDZ_0,
    #[doc = "Filter is disabled in Doze mode"]
    FILTDZ_1,
}
impl FILTDZW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            FILTDZW::FILTDZ_0 => false,
            FILTDZW::FILTDZ_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FILTDZW<'a> {
    w: &'a mut W,
}
impl<'a> _FILTDZW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FILTDZW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Filter remains enabled in Doze mode"]
    #[inline]
    pub fn filtdz_0(self) -> &'a mut W {
        self.variant(FILTDZW::FILTDZ_0)
    }
    #[doc = "Filter is disabled in Doze mode"]
    #[inline]
    pub fn filtdz_1(self) -> &'a mut W {
        self.variant(FILTDZW::FILTDZ_1)
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
        const OFFSET: u8 = 5;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `RTF`"]
pub enum RTFW {
    #[doc = "No effect"]
    RTF_0,
    #[doc = "Transmit Data Register is now empty"]
    RTF_1,
}
impl RTFW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RTFW::RTF_0 => false,
            RTFW::RTF_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RTFW<'a> {
    w: &'a mut W,
}
impl<'a> _RTFW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RTFW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No effect"]
    #[inline]
    pub fn rtf_0(self) -> &'a mut W {
        self.variant(RTFW::RTF_0)
    }
    #[doc = "Transmit Data Register is now empty"]
    #[inline]
    pub fn rtf_1(self) -> &'a mut W {
        self.variant(RTFW::RTF_1)
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
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `RRF`"]
pub enum RRFW {
    #[doc = "No effect"]
    RRF_0,
    #[doc = "Receive Data Register is now empty"]
    RRF_1,
}
impl RRFW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RRFW::RRF_0 => false,
            RRFW::RRF_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RRFW<'a> {
    w: &'a mut W,
}
impl<'a> _RRFW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RRFW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No effect"]
    #[inline]
    pub fn rrf_0(self) -> &'a mut W {
        self.variant(RRFW::RRF_0)
    }
    #[doc = "Receive Data Register is now empty"]
    #[inline]
    pub fn rrf_1(self) -> &'a mut W {
        self.variant(RRFW::RRF_1)
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
        const OFFSET: u8 = 9;
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
    #[doc = "Bit 0 - Slave Enable"]
    #[inline]
    pub fn sen(&self) -> SENR {
        SENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - Software Reset"]
    #[inline]
    pub fn rst(&self) -> RSTR {
        RSTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 4 - Filter Enable"]
    #[inline]
    pub fn filten(&self) -> FILTENR {
        FILTENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 5 - Filter Doze Enable"]
    #[inline]
    pub fn filtdz(&self) -> FILTDZR {
        FILTDZR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
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
    #[doc = "Bit 0 - Slave Enable"]
    #[inline]
    pub fn sen(&mut self) -> _SENW {
        _SENW { w: self }
    }
    #[doc = "Bit 1 - Software Reset"]
    #[inline]
    pub fn rst(&mut self) -> _RSTW {
        _RSTW { w: self }
    }
    #[doc = "Bit 4 - Filter Enable"]
    #[inline]
    pub fn filten(&mut self) -> _FILTENW {
        _FILTENW { w: self }
    }
    #[doc = "Bit 5 - Filter Doze Enable"]
    #[inline]
    pub fn filtdz(&mut self) -> _FILTDZW {
        _FILTDZW { w: self }
    }
    #[doc = "Bit 8 - Reset Transmit FIFO"]
    #[inline]
    pub fn rtf(&mut self) -> _RTFW {
        _RTFW { w: self }
    }
    #[doc = "Bit 9 - Reset Receive FIFO"]
    #[inline]
    pub fn rrf(&mut self) -> _RRFW {
        _RRFW { w: self }
    }
}
