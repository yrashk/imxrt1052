#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::MCTL {
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
#[doc = "Possible values of the field `SAMP_MODE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SAMP_MODER {
    #[doc = "use Von Neumann data into both Entropy shifter and Statistical Checker"]
    SAMP_MODE_0,
    #[doc = "use raw data into both Entropy shifter and Statistical Checker"]
    SAMP_MODE_1,
    #[doc = "use Von Neumann data into Entropy shifter. Use raw data into Statistical Checker"]
    SAMP_MODE_2,
    #[doc = "undefined/reserved."]
    SAMP_MODE_3,
}
impl SAMP_MODER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            SAMP_MODER::SAMP_MODE_0 => 0,
            SAMP_MODER::SAMP_MODE_1 => 1,
            SAMP_MODER::SAMP_MODE_2 => 2,
            SAMP_MODER::SAMP_MODE_3 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> SAMP_MODER {
        match value {
            0 => SAMP_MODER::SAMP_MODE_0,
            1 => SAMP_MODER::SAMP_MODE_1,
            2 => SAMP_MODER::SAMP_MODE_2,
            3 => SAMP_MODER::SAMP_MODE_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `SAMP_MODE_0`"]
    #[inline]
    pub fn is_samp_mode_0(&self) -> bool {
        *self == SAMP_MODER::SAMP_MODE_0
    }
    #[doc = "Checks if the value of the field is `SAMP_MODE_1`"]
    #[inline]
    pub fn is_samp_mode_1(&self) -> bool {
        *self == SAMP_MODER::SAMP_MODE_1
    }
    #[doc = "Checks if the value of the field is `SAMP_MODE_2`"]
    #[inline]
    pub fn is_samp_mode_2(&self) -> bool {
        *self == SAMP_MODER::SAMP_MODE_2
    }
    #[doc = "Checks if the value of the field is `SAMP_MODE_3`"]
    #[inline]
    pub fn is_samp_mode_3(&self) -> bool {
        *self == SAMP_MODER::SAMP_MODE_3
    }
}
#[doc = "Possible values of the field `OSC_DIV`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OSC_DIVR {
    #[doc = "use ring oscillator with no divide"]
    OSC_DIV_0,
    #[doc = "use ring oscillator divided-by-2"]
    OSC_DIV_1,
    #[doc = "use ring oscillator divided-by-4"]
    OSC_DIV_2,
    #[doc = "use ring oscillator divided-by-8"]
    OSC_DIV_3,
}
impl OSC_DIVR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            OSC_DIVR::OSC_DIV_0 => 0,
            OSC_DIVR::OSC_DIV_1 => 1,
            OSC_DIVR::OSC_DIV_2 => 2,
            OSC_DIVR::OSC_DIV_3 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> OSC_DIVR {
        match value {
            0 => OSC_DIVR::OSC_DIV_0,
            1 => OSC_DIVR::OSC_DIV_1,
            2 => OSC_DIVR::OSC_DIV_2,
            3 => OSC_DIVR::OSC_DIV_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `OSC_DIV_0`"]
    #[inline]
    pub fn is_osc_div_0(&self) -> bool {
        *self == OSC_DIVR::OSC_DIV_0
    }
    #[doc = "Checks if the value of the field is `OSC_DIV_1`"]
    #[inline]
    pub fn is_osc_div_1(&self) -> bool {
        *self == OSC_DIVR::OSC_DIV_1
    }
    #[doc = "Checks if the value of the field is `OSC_DIV_2`"]
    #[inline]
    pub fn is_osc_div_2(&self) -> bool {
        *self == OSC_DIVR::OSC_DIV_2
    }
    #[doc = "Checks if the value of the field is `OSC_DIV_3`"]
    #[inline]
    pub fn is_osc_div_3(&self) -> bool {
        *self == OSC_DIVR::OSC_DIV_3
    }
}
#[doc = r" Value of the field"]
pub struct UNUSED4R {
    bits: bool,
}
impl UNUSED4R {
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
pub struct UNUSED5R {
    bits: bool,
}
impl UNUSED5R {
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
pub struct FOR_SCLKR {
    bits: bool,
}
impl FOR_SCLKR {
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
pub struct FCT_FAILR {
    bits: bool,
}
impl FCT_FAILR {
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
pub struct FCT_VALR {
    bits: bool,
}
impl FCT_VALR {
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
pub struct ENT_VALR {
    bits: bool,
}
impl ENT_VALR {
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
pub struct TST_OUTR {
    bits: bool,
}
impl TST_OUTR {
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
pub struct ERRR {
    bits: bool,
}
impl ERRR {
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
pub struct TSTOP_OKR {
    bits: bool,
}
impl TSTOP_OKR {
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
pub struct LRUN_CONTR {
    bits: bool,
}
impl LRUN_CONTR {
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
pub struct PRGMR {
    bits: bool,
}
impl PRGMR {
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
#[doc = "Values that can be written to the field `SAMP_MODE`"]
pub enum SAMP_MODEW {
    #[doc = "use Von Neumann data into both Entropy shifter and Statistical Checker"]
    SAMP_MODE_0,
    #[doc = "use raw data into both Entropy shifter and Statistical Checker"]
    SAMP_MODE_1,
    #[doc = "use Von Neumann data into Entropy shifter. Use raw data into Statistical Checker"]
    SAMP_MODE_2,
    #[doc = "undefined/reserved."]
    SAMP_MODE_3,
}
impl SAMP_MODEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            SAMP_MODEW::SAMP_MODE_0 => 0,
            SAMP_MODEW::SAMP_MODE_1 => 1,
            SAMP_MODEW::SAMP_MODE_2 => 2,
            SAMP_MODEW::SAMP_MODE_3 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SAMP_MODEW<'a> {
    w: &'a mut W,
}
impl<'a> _SAMP_MODEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SAMP_MODEW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "use Von Neumann data into both Entropy shifter and Statistical Checker"]
    #[inline]
    pub fn samp_mode_0(self) -> &'a mut W {
        self.variant(SAMP_MODEW::SAMP_MODE_0)
    }
    #[doc = "use raw data into both Entropy shifter and Statistical Checker"]
    #[inline]
    pub fn samp_mode_1(self) -> &'a mut W {
        self.variant(SAMP_MODEW::SAMP_MODE_1)
    }
    #[doc = "use Von Neumann data into Entropy shifter. Use raw data into Statistical Checker"]
    #[inline]
    pub fn samp_mode_2(self) -> &'a mut W {
        self.variant(SAMP_MODEW::SAMP_MODE_2)
    }
    #[doc = "undefined/reserved."]
    #[inline]
    pub fn samp_mode_3(self) -> &'a mut W {
        self.variant(SAMP_MODEW::SAMP_MODE_3)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `OSC_DIV`"]
pub enum OSC_DIVW {
    #[doc = "use ring oscillator with no divide"]
    OSC_DIV_0,
    #[doc = "use ring oscillator divided-by-2"]
    OSC_DIV_1,
    #[doc = "use ring oscillator divided-by-4"]
    OSC_DIV_2,
    #[doc = "use ring oscillator divided-by-8"]
    OSC_DIV_3,
}
impl OSC_DIVW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            OSC_DIVW::OSC_DIV_0 => 0,
            OSC_DIVW::OSC_DIV_1 => 1,
            OSC_DIVW::OSC_DIV_2 => 2,
            OSC_DIVW::OSC_DIV_3 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _OSC_DIVW<'a> {
    w: &'a mut W,
}
impl<'a> _OSC_DIVW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: OSC_DIVW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "use ring oscillator with no divide"]
    #[inline]
    pub fn osc_div_0(self) -> &'a mut W {
        self.variant(OSC_DIVW::OSC_DIV_0)
    }
    #[doc = "use ring oscillator divided-by-2"]
    #[inline]
    pub fn osc_div_1(self) -> &'a mut W {
        self.variant(OSC_DIVW::OSC_DIV_1)
    }
    #[doc = "use ring oscillator divided-by-4"]
    #[inline]
    pub fn osc_div_2(self) -> &'a mut W {
        self.variant(OSC_DIVW::OSC_DIV_2)
    }
    #[doc = "use ring oscillator divided-by-8"]
    #[inline]
    pub fn osc_div_3(self) -> &'a mut W {
        self.variant(OSC_DIVW::OSC_DIV_3)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 2;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _RST_DEFW<'a> {
    w: &'a mut W,
}
impl<'a> _RST_DEFW<'a> {
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
        const OFFSET: u8 = 6;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _FOR_SCLKW<'a> {
    w: &'a mut W,
}
impl<'a> _FOR_SCLKW<'a> {
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
        const OFFSET: u8 = 7;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _ERRW<'a> {
    w: &'a mut W,
}
impl<'a> _ERRW<'a> {
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
        const OFFSET: u8 = 12;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _LRUN_CONTW<'a> {
    w: &'a mut W,
}
impl<'a> _LRUN_CONTW<'a> {
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
#[doc = r" Proxy"]
pub struct _PRGMW<'a> {
    w: &'a mut W,
}
impl<'a> _PRGMW<'a> {
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
    #[doc = "Bits 0:1 - Sample Mode"]
    #[inline]
    pub fn samp_mode(&self) -> SAMP_MODER {
        SAMP_MODER::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 2:3 - Oscillator Divide"]
    #[inline]
    pub fn osc_div(&self) -> OSC_DIVR {
        OSC_DIVR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 4 - This bit is unused. Always reads zero."]
    #[inline]
    pub fn unused4(&self) -> UNUSED4R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        UNUSED4R { bits }
    }
    #[doc = "Bit 5 - This bit is unused. Always reads zero."]
    #[inline]
    pub fn unused5(&self) -> UNUSED5R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        UNUSED5R { bits }
    }
    #[doc = "Bit 7 - Force System Clock"]
    #[inline]
    pub fn for_sclk(&self) -> FOR_SCLKR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        FOR_SCLKR { bits }
    }
    #[doc = "Bit 8 - Read only: Frequency Count Fail"]
    #[inline]
    pub fn fct_fail(&self) -> FCT_FAILR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        FCT_FAILR { bits }
    }
    #[doc = "Bit 9 - Read only: Frequency Count Valid. Indicates that a valid frequency count may be read from FRQCNT."]
    #[inline]
    pub fn fct_val(&self) -> FCT_VALR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        FCT_VALR { bits }
    }
    #[doc = "Bit 10 - Read only: Entropy Valid"]
    #[inline]
    pub fn ent_val(&self) -> ENT_VALR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        ENT_VALR { bits }
    }
    #[doc = "Bit 11 - Read only: Test point inside ring oscillator."]
    #[inline]
    pub fn tst_out(&self) -> TST_OUTR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        TST_OUTR { bits }
    }
    #[doc = "Bit 12 - Read: Error status"]
    #[inline]
    pub fn err(&self) -> ERRR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        ERRR { bits }
    }
    #[doc = "Bit 13 - TRNG_OK_TO_STOP"]
    #[inline]
    pub fn tstop_ok(&self) -> TSTOP_OKR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        TSTOP_OKR { bits }
    }
    #[doc = "Bit 14 - Long run count continues between entropy generations"]
    #[inline]
    pub fn lrun_cont(&self) -> LRUN_CONTR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        LRUN_CONTR { bits }
    }
    #[doc = "Bit 16 - Programming Mode Select"]
    #[inline]
    pub fn prgm(&self) -> PRGMR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        PRGMR { bits }
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 73729 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:1 - Sample Mode"]
    #[inline]
    pub fn samp_mode(&mut self) -> _SAMP_MODEW {
        _SAMP_MODEW { w: self }
    }
    #[doc = "Bits 2:3 - Oscillator Divide"]
    #[inline]
    pub fn osc_div(&mut self) -> _OSC_DIVW {
        _OSC_DIVW { w: self }
    }
    #[doc = "Bit 6 - Reset Defaults"]
    #[inline]
    pub fn rst_def(&mut self) -> _RST_DEFW {
        _RST_DEFW { w: self }
    }
    #[doc = "Bit 7 - Force System Clock"]
    #[inline]
    pub fn for_sclk(&mut self) -> _FOR_SCLKW {
        _FOR_SCLKW { w: self }
    }
    #[doc = "Bit 12 - Read: Error status"]
    #[inline]
    pub fn err(&mut self) -> _ERRW {
        _ERRW { w: self }
    }
    #[doc = "Bit 14 - Long run count continues between entropy generations"]
    #[inline]
    pub fn lrun_cont(&mut self) -> _LRUN_CONTW {
        _LRUN_CONTW { w: self }
    }
    #[doc = "Bit 16 - Programming Mode Select"]
    #[inline]
    pub fn prgm(&mut self) -> _PRGMW {
        _PRGMW { w: self }
    }
}
