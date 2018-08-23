#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::REG_1P1_TOG {
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
pub struct ENABLE_LINREGR {
    bits: bool,
}
impl ENABLE_LINREGR {
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
pub struct ENABLE_BOR {
    bits: bool,
}
impl ENABLE_BOR {
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
pub struct ENABLE_ILIMITR {
    bits: bool,
}
impl ENABLE_ILIMITR {
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
pub struct ENABLE_PULLDOWNR {
    bits: bool,
}
impl ENABLE_PULLDOWNR {
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
pub struct BO_OFFSETR {
    bits: u8,
}
impl BO_OFFSETR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Possible values of the field `OUTPUT_TRG`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OUTPUT_TRGR {
    #[doc = "0.8V"]
    OUTPUT_TRG_4,
    #[doc = "1.1V"]
    OUTPUT_TRG_16,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl OUTPUT_TRGR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            OUTPUT_TRGR::OUTPUT_TRG_4 => 4,
            OUTPUT_TRGR::OUTPUT_TRG_16 => 16,
            OUTPUT_TRGR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> OUTPUT_TRGR {
        match value {
            4 => OUTPUT_TRGR::OUTPUT_TRG_4,
            16 => OUTPUT_TRGR::OUTPUT_TRG_16,
            i => OUTPUT_TRGR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `OUTPUT_TRG_4`"]
    #[inline]
    pub fn is_output_trg_4(&self) -> bool {
        *self == OUTPUT_TRGR::OUTPUT_TRG_4
    }
    #[doc = "Checks if the value of the field is `OUTPUT_TRG_16`"]
    #[inline]
    pub fn is_output_trg_16(&self) -> bool {
        *self == OUTPUT_TRGR::OUTPUT_TRG_16
    }
}
#[doc = r" Value of the field"]
pub struct BO_VDD1P1R {
    bits: bool,
}
impl BO_VDD1P1R {
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
pub struct OK_VDD1P1R {
    bits: bool,
}
impl OK_VDD1P1R {
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
pub struct ENABLE_WEAK_LINREGR {
    bits: bool,
}
impl ENABLE_WEAK_LINREGR {
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
#[doc = "Possible values of the field `SELREF_WEAK_LINREG`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SELREF_WEAK_LINREGR {
    #[doc = "Weak-linreg output tracks low-power-bandgap voltage"]
    SELREF_WEAK_LINREG_0,
    #[doc = "Weak-linreg output tracks VDD_SOC_CAP voltage"]
    SELREF_WEAK_LINREG_1,
}
impl SELREF_WEAK_LINREGR {
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
            SELREF_WEAK_LINREGR::SELREF_WEAK_LINREG_0 => false,
            SELREF_WEAK_LINREGR::SELREF_WEAK_LINREG_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SELREF_WEAK_LINREGR {
        match value {
            false => SELREF_WEAK_LINREGR::SELREF_WEAK_LINREG_0,
            true => SELREF_WEAK_LINREGR::SELREF_WEAK_LINREG_1,
        }
    }
    #[doc = "Checks if the value of the field is `SELREF_WEAK_LINREG_0`"]
    #[inline]
    pub fn is_selref_weak_linreg_0(&self) -> bool {
        *self == SELREF_WEAK_LINREGR::SELREF_WEAK_LINREG_0
    }
    #[doc = "Checks if the value of the field is `SELREF_WEAK_LINREG_1`"]
    #[inline]
    pub fn is_selref_weak_linreg_1(&self) -> bool {
        *self == SELREF_WEAK_LINREGR::SELREF_WEAK_LINREG_1
    }
}
#[doc = r" Proxy"]
pub struct _ENABLE_LINREGW<'a> {
    w: &'a mut W,
}
impl<'a> _ENABLE_LINREGW<'a> {
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
pub struct _ENABLE_BOW<'a> {
    w: &'a mut W,
}
impl<'a> _ENABLE_BOW<'a> {
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
pub struct _ENABLE_ILIMITW<'a> {
    w: &'a mut W,
}
impl<'a> _ENABLE_ILIMITW<'a> {
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
        const OFFSET: u8 = 2;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _ENABLE_PULLDOWNW<'a> {
    w: &'a mut W,
}
impl<'a> _ENABLE_PULLDOWNW<'a> {
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
        const OFFSET: u8 = 3;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _BO_OFFSETW<'a> {
    w: &'a mut W,
}
impl<'a> _BO_OFFSETW<'a> {
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
#[doc = "Values that can be written to the field `OUTPUT_TRG`"]
pub enum OUTPUT_TRGW {
    #[doc = "0.8V"]
    OUTPUT_TRG_4,
    #[doc = "1.1V"]
    OUTPUT_TRG_16,
}
impl OUTPUT_TRGW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            OUTPUT_TRGW::OUTPUT_TRG_4 => 4,
            OUTPUT_TRGW::OUTPUT_TRG_16 => 16,
        }
    }
}
#[doc = r" Proxy"]
pub struct _OUTPUT_TRGW<'a> {
    w: &'a mut W,
}
impl<'a> _OUTPUT_TRGW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: OUTPUT_TRGW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "0.8V"]
    #[inline]
    pub fn output_trg_4(self) -> &'a mut W {
        self.variant(OUTPUT_TRGW::OUTPUT_TRG_4)
    }
    #[doc = "1.1V"]
    #[inline]
    pub fn output_trg_16(self) -> &'a mut W {
        self.variant(OUTPUT_TRGW::OUTPUT_TRG_16)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 31;
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _ENABLE_WEAK_LINREGW<'a> {
    w: &'a mut W,
}
impl<'a> _ENABLE_WEAK_LINREGW<'a> {
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
        const OFFSET: u8 = 18;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `SELREF_WEAK_LINREG`"]
pub enum SELREF_WEAK_LINREGW {
    #[doc = "Weak-linreg output tracks low-power-bandgap voltage"]
    SELREF_WEAK_LINREG_0,
    #[doc = "Weak-linreg output tracks VDD_SOC_CAP voltage"]
    SELREF_WEAK_LINREG_1,
}
impl SELREF_WEAK_LINREGW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SELREF_WEAK_LINREGW::SELREF_WEAK_LINREG_0 => false,
            SELREF_WEAK_LINREGW::SELREF_WEAK_LINREG_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SELREF_WEAK_LINREGW<'a> {
    w: &'a mut W,
}
impl<'a> _SELREF_WEAK_LINREGW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SELREF_WEAK_LINREGW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Weak-linreg output tracks low-power-bandgap voltage"]
    #[inline]
    pub fn selref_weak_linreg_0(self) -> &'a mut W {
        self.variant(SELREF_WEAK_LINREGW::SELREF_WEAK_LINREG_0)
    }
    #[doc = "Weak-linreg output tracks VDD_SOC_CAP voltage"]
    #[inline]
    pub fn selref_weak_linreg_1(self) -> &'a mut W {
        self.variant(SELREF_WEAK_LINREGW::SELREF_WEAK_LINREG_1)
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
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - Control bit to enable the regulator output."]
    #[inline]
    pub fn enable_linreg(&self) -> ENABLE_LINREGR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        ENABLE_LINREGR { bits }
    }
    #[doc = "Bit 1 - Control bit to enable the brownout circuitry in the regulator."]
    #[inline]
    pub fn enable_bo(&self) -> ENABLE_BOR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        ENABLE_BOR { bits }
    }
    #[doc = "Bit 2 - Control bit to enable the current-limit circuitry in the regulator."]
    #[inline]
    pub fn enable_ilimit(&self) -> ENABLE_ILIMITR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        ENABLE_ILIMITR { bits }
    }
    #[doc = "Bit 3 - Control bit to enable the pull-down circuitry in the regulator"]
    #[inline]
    pub fn enable_pulldown(&self) -> ENABLE_PULLDOWNR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        ENABLE_PULLDOWNR { bits }
    }
    #[doc = "Bits 4:6 - Control bits to adjust the regulator brownout offset voltage in 25mV steps"]
    #[inline]
    pub fn bo_offset(&self) -> BO_OFFSETR {
        let bits = {
            const MASK: u8 = 7;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        BO_OFFSETR { bits }
    }
    #[doc = "Bits 8:12 - Control bits to adjust the regulator output voltage"]
    #[inline]
    pub fn output_trg(&self) -> OUTPUT_TRGR {
        OUTPUT_TRGR::_from({
            const MASK: u8 = 31;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 16 - Status bit that signals when a brownout is detected on the regulator output."]
    #[inline]
    pub fn bo_vdd1p1(&self) -> BO_VDD1P1R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        BO_VDD1P1R { bits }
    }
    #[doc = "Bit 17 - Status bit that signals when the regulator output is ok. 1 = regulator output > brownout target"]
    #[inline]
    pub fn ok_vdd1p1(&self) -> OK_VDD1P1R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 17;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        OK_VDD1P1R { bits }
    }
    #[doc = "Bit 18 - Enables the weak 1p1 regulator"]
    #[inline]
    pub fn enable_weak_linreg(&self) -> ENABLE_WEAK_LINREGR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        ENABLE_WEAK_LINREGR { bits }
    }
    #[doc = "Bit 19 - Selects the source for the reference voltage of the weak 1p1 regulator."]
    #[inline]
    pub fn selref_weak_linreg(&self) -> SELREF_WEAK_LINREGR {
        SELREF_WEAK_LINREGR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 19;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 4211 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - Control bit to enable the regulator output."]
    #[inline]
    pub fn enable_linreg(&mut self) -> _ENABLE_LINREGW {
        _ENABLE_LINREGW { w: self }
    }
    #[doc = "Bit 1 - Control bit to enable the brownout circuitry in the regulator."]
    #[inline]
    pub fn enable_bo(&mut self) -> _ENABLE_BOW {
        _ENABLE_BOW { w: self }
    }
    #[doc = "Bit 2 - Control bit to enable the current-limit circuitry in the regulator."]
    #[inline]
    pub fn enable_ilimit(&mut self) -> _ENABLE_ILIMITW {
        _ENABLE_ILIMITW { w: self }
    }
    #[doc = "Bit 3 - Control bit to enable the pull-down circuitry in the regulator"]
    #[inline]
    pub fn enable_pulldown(&mut self) -> _ENABLE_PULLDOWNW {
        _ENABLE_PULLDOWNW { w: self }
    }
    #[doc = "Bits 4:6 - Control bits to adjust the regulator brownout offset voltage in 25mV steps"]
    #[inline]
    pub fn bo_offset(&mut self) -> _BO_OFFSETW {
        _BO_OFFSETW { w: self }
    }
    #[doc = "Bits 8:12 - Control bits to adjust the regulator output voltage"]
    #[inline]
    pub fn output_trg(&mut self) -> _OUTPUT_TRGW {
        _OUTPUT_TRGW { w: self }
    }
    #[doc = "Bit 18 - Enables the weak 1p1 regulator"]
    #[inline]
    pub fn enable_weak_linreg(&mut self) -> _ENABLE_WEAK_LINREGW {
        _ENABLE_WEAK_LINREGW { w: self }
    }
    #[doc = "Bit 19 - Selects the source for the reference voltage of the weak 1p1 regulator."]
    #[inline]
    pub fn selref_weak_linreg(&mut self) -> _SELREF_WEAK_LINREGW {
        _SELREF_WEAK_LINREGW { w: self }
    }
}
