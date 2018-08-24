#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::REG_3P0_SET {
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
#[doc = "Possible values of the field `VBUS_SEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VBUS_SELR {
    #[doc = "Utilize VBUS OTG2 power"]
    USB_OTG2_VBUS,
    #[doc = "Utilize VBUS OTG1 power"]
    USB_OTG1_VBUS,
}
impl VBUS_SELR {
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
            VBUS_SELR::USB_OTG2_VBUS => false,
            VBUS_SELR::USB_OTG1_VBUS => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> VBUS_SELR {
        match value {
            false => VBUS_SELR::USB_OTG2_VBUS,
            true => VBUS_SELR::USB_OTG1_VBUS,
        }
    }
    #[doc = "Checks if the value of the field is `USB_OTG2_VBUS`"]
    #[inline]
    pub fn is_usb_otg2_vbus(&self) -> bool {
        *self == VBUS_SELR::USB_OTG2_VBUS
    }
    #[doc = "Checks if the value of the field is `USB_OTG1_VBUS`"]
    #[inline]
    pub fn is_usb_otg1_vbus(&self) -> bool {
        *self == VBUS_SELR::USB_OTG1_VBUS
    }
}
#[doc = "Possible values of the field `OUTPUT_TRG`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OUTPUT_TRGR {
    #[doc = "2.625V"]
    OUTPUT_TRG_0,
    #[doc = "3.000V"]
    OUTPUT_TRG_15,
    #[doc = "3.400V"]
    OUTPUT_TRG_31,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl OUTPUT_TRGR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            OUTPUT_TRGR::OUTPUT_TRG_0 => 0,
            OUTPUT_TRGR::OUTPUT_TRG_15 => 15,
            OUTPUT_TRGR::OUTPUT_TRG_31 => 31,
            OUTPUT_TRGR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> OUTPUT_TRGR {
        match value {
            0 => OUTPUT_TRGR::OUTPUT_TRG_0,
            15 => OUTPUT_TRGR::OUTPUT_TRG_15,
            31 => OUTPUT_TRGR::OUTPUT_TRG_31,
            i => OUTPUT_TRGR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `OUTPUT_TRG_0`"]
    #[inline]
    pub fn is_output_trg_0(&self) -> bool {
        *self == OUTPUT_TRGR::OUTPUT_TRG_0
    }
    #[doc = "Checks if the value of the field is `OUTPUT_TRG_15`"]
    #[inline]
    pub fn is_output_trg_15(&self) -> bool {
        *self == OUTPUT_TRGR::OUTPUT_TRG_15
    }
    #[doc = "Checks if the value of the field is `OUTPUT_TRG_31`"]
    #[inline]
    pub fn is_output_trg_31(&self) -> bool {
        *self == OUTPUT_TRGR::OUTPUT_TRG_31
    }
}
#[doc = r" Value of the field"]
pub struct BO_VDD3P0R {
    bits: bool,
}
impl BO_VDD3P0R {
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
pub struct OK_VDD3P0R {
    bits: bool,
}
impl OK_VDD3P0R {
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
#[doc = "Values that can be written to the field `VBUS_SEL`"]
pub enum VBUS_SELW {
    #[doc = "Utilize VBUS OTG2 power"]
    USB_OTG2_VBUS,
    #[doc = "Utilize VBUS OTG1 power"]
    USB_OTG1_VBUS,
}
impl VBUS_SELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            VBUS_SELW::USB_OTG2_VBUS => false,
            VBUS_SELW::USB_OTG1_VBUS => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _VBUS_SELW<'a> {
    w: &'a mut W,
}
impl<'a> _VBUS_SELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: VBUS_SELW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Utilize VBUS OTG2 power"]
    #[inline]
    pub fn usb_otg2_vbus(self) -> &'a mut W {
        self.variant(VBUS_SELW::USB_OTG2_VBUS)
    }
    #[doc = "Utilize VBUS OTG1 power"]
    #[inline]
    pub fn usb_otg1_vbus(self) -> &'a mut W {
        self.variant(VBUS_SELW::USB_OTG1_VBUS)
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
        const OFFSET: u8 = 7;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `OUTPUT_TRG`"]
pub enum OUTPUT_TRGW {
    #[doc = "2.625V"]
    OUTPUT_TRG_0,
    #[doc = "3.000V"]
    OUTPUT_TRG_15,
    #[doc = "3.400V"]
    OUTPUT_TRG_31,
}
impl OUTPUT_TRGW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            OUTPUT_TRGW::OUTPUT_TRG_0 => 0,
            OUTPUT_TRGW::OUTPUT_TRG_15 => 15,
            OUTPUT_TRGW::OUTPUT_TRG_31 => 31,
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
    #[doc = "2.625V"]
    #[inline]
    pub fn output_trg_0(self) -> &'a mut W {
        self.variant(OUTPUT_TRGW::OUTPUT_TRG_0)
    }
    #[doc = "3.000V"]
    #[inline]
    pub fn output_trg_15(self) -> &'a mut W {
        self.variant(OUTPUT_TRGW::OUTPUT_TRG_15)
    }
    #[doc = "3.400V"]
    #[inline]
    pub fn output_trg_31(self) -> &'a mut W {
        self.variant(OUTPUT_TRGW::OUTPUT_TRG_31)
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
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - Control bit to enable the regulator output to be set by the programmed target voltage setting and internal bandgap reference"]
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
    #[doc = "Bit 7 - Select input voltage source for LDO_3P0 from either USB_OTG1_VBUS or USB_OTG2_VBUS"]
    #[inline]
    pub fn vbus_sel(&self) -> VBUS_SELR {
        VBUS_SELR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
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
    pub fn bo_vdd3p0(&self) -> BO_VDD3P0R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        BO_VDD3P0R { bits }
    }
    #[doc = "Bit 17 - Status bit that signals when the regulator output is ok. 1 = regulator output > brownout target"]
    #[inline]
    pub fn ok_vdd3p0(&self) -> OK_VDD3P0R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 17;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        OK_VDD3P0R { bits }
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 3956 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - Control bit to enable the regulator output to be set by the programmed target voltage setting and internal bandgap reference"]
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
    #[doc = "Bits 4:6 - Control bits to adjust the regulator brownout offset voltage in 25mV steps"]
    #[inline]
    pub fn bo_offset(&mut self) -> _BO_OFFSETW {
        _BO_OFFSETW { w: self }
    }
    #[doc = "Bit 7 - Select input voltage source for LDO_3P0 from either USB_OTG1_VBUS or USB_OTG2_VBUS"]
    #[inline]
    pub fn vbus_sel(&mut self) -> _VBUS_SELW {
        _VBUS_SELW { w: self }
    }
    #[doc = "Bits 8:12 - Control bits to adjust the regulator output voltage"]
    #[inline]
    pub fn output_trg(&mut self) -> _OUTPUT_TRGW {
        _OUTPUT_TRGW { w: self }
    }
}
