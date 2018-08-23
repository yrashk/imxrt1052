#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::REG0 {
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
pub struct PWD_ZCDR {
    bits: bool,
}
impl PWD_ZCDR {
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
pub struct DISABLE_AUTO_CLK_SWITCHR {
    bits: bool,
}
impl DISABLE_AUTO_CLK_SWITCHR {
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
pub struct SEL_CLKR {
    bits: bool,
}
impl SEL_CLKR {
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
pub struct PWD_OSC_INTR {
    bits: bool,
}
impl PWD_OSC_INTR {
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
pub struct PWD_CUR_SNS_CMPR {
    bits: bool,
}
impl PWD_CUR_SNS_CMPR {
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
pub struct CUR_SNS_THRSHR {
    bits: u8,
}
impl CUR_SNS_THRSHR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct PWD_OVERCUR_DETR {
    bits: bool,
}
impl PWD_OVERCUR_DETR {
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
pub struct OVERCUR_TRIG_ADJR {
    bits: u8,
}
impl OVERCUR_TRIG_ADJR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct PWD_CMP_BATT_DETR {
    bits: bool,
}
impl PWD_CMP_BATT_DETR {
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
pub struct ADJ_POSLIMIT_BUCKR {
    bits: u8,
}
impl ADJ_POSLIMIT_BUCKR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct EN_LP_OVERLOAD_SNSR {
    bits: bool,
}
impl EN_LP_OVERLOAD_SNSR {
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
pub struct PWD_HIGH_VOLT_DETR {
    bits: bool,
}
impl PWD_HIGH_VOLT_DETR {
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
pub struct LP_OVERLOAD_THRSHR {
    bits: u8,
}
impl LP_OVERLOAD_THRSHR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct LP_OVERLOAD_FREQ_SELR {
    bits: bool,
}
impl LP_OVERLOAD_FREQ_SELR {
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
pub struct LP_HIGH_HYSR {
    bits: bool,
}
impl LP_HIGH_HYSR {
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
pub struct PWD_CMP_OFFSETR {
    bits: bool,
}
impl PWD_CMP_OFFSETR {
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
pub struct XTALOK_DISABLER {
    bits: bool,
}
impl XTALOK_DISABLER {
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
pub struct CURRENT_ALERT_RESETR {
    bits: bool,
}
impl CURRENT_ALERT_RESETR {
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
pub struct XTAL_24M_OKR {
    bits: bool,
}
impl XTAL_24M_OKR {
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
pub struct STS_DC_OKR {
    bits: bool,
}
impl STS_DC_OKR {
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
pub struct _PWD_ZCDW<'a> {
    w: &'a mut W,
}
impl<'a> _PWD_ZCDW<'a> {
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
pub struct _DISABLE_AUTO_CLK_SWITCHW<'a> {
    w: &'a mut W,
}
impl<'a> _DISABLE_AUTO_CLK_SWITCHW<'a> {
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
pub struct _SEL_CLKW<'a> {
    w: &'a mut W,
}
impl<'a> _SEL_CLKW<'a> {
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
pub struct _PWD_OSC_INTW<'a> {
    w: &'a mut W,
}
impl<'a> _PWD_OSC_INTW<'a> {
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
pub struct _PWD_CUR_SNS_CMPW<'a> {
    w: &'a mut W,
}
impl<'a> _PWD_CUR_SNS_CMPW<'a> {
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
#[doc = r" Proxy"]
pub struct _CUR_SNS_THRSHW<'a> {
    w: &'a mut W,
}
impl<'a> _CUR_SNS_THRSHW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 5;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _PWD_OVERCUR_DETW<'a> {
    w: &'a mut W,
}
impl<'a> _PWD_OVERCUR_DETW<'a> {
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
#[doc = r" Proxy"]
pub struct _OVERCUR_TRIG_ADJW<'a> {
    w: &'a mut W,
}
impl<'a> _OVERCUR_TRIG_ADJW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 9;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _PWD_CMP_BATT_DETW<'a> {
    w: &'a mut W,
}
impl<'a> _PWD_CMP_BATT_DETW<'a> {
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
pub struct _ADJ_POSLIMIT_BUCKW<'a> {
    w: &'a mut W,
}
impl<'a> _ADJ_POSLIMIT_BUCKW<'a> {
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
pub struct _EN_LP_OVERLOAD_SNSW<'a> {
    w: &'a mut W,
}
impl<'a> _EN_LP_OVERLOAD_SNSW<'a> {
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
#[doc = r" Proxy"]
pub struct _PWD_HIGH_VOLT_DETW<'a> {
    w: &'a mut W,
}
impl<'a> _PWD_HIGH_VOLT_DETW<'a> {
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
        const OFFSET: u8 = 17;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _LP_OVERLOAD_THRSHW<'a> {
    w: &'a mut W,
}
impl<'a> _LP_OVERLOAD_THRSHW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 18;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _LP_OVERLOAD_FREQ_SELW<'a> {
    w: &'a mut W,
}
impl<'a> _LP_OVERLOAD_FREQ_SELW<'a> {
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
        const OFFSET: u8 = 20;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _LP_HIGH_HYSW<'a> {
    w: &'a mut W,
}
impl<'a> _LP_HIGH_HYSW<'a> {
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
        const OFFSET: u8 = 21;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _PWD_CMP_OFFSETW<'a> {
    w: &'a mut W,
}
impl<'a> _PWD_CMP_OFFSETW<'a> {
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
        const OFFSET: u8 = 26;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _XTALOK_DISABLEW<'a> {
    w: &'a mut W,
}
impl<'a> _XTALOK_DISABLEW<'a> {
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
        const OFFSET: u8 = 27;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _CURRENT_ALERT_RESETW<'a> {
    w: &'a mut W,
}
impl<'a> _CURRENT_ALERT_RESETW<'a> {
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
#[doc = r" Proxy"]
pub struct _XTAL_24M_OKW<'a> {
    w: &'a mut W,
}
impl<'a> _XTAL_24M_OKW<'a> {
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
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - power down the zero cross detection function for discontinuous conductor mode"]
    #[inline]
    pub fn pwd_zcd(&self) -> PWD_ZCDR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        PWD_ZCDR { bits }
    }
    #[doc = "Bit 1 - Disable automatic clock switch from internal osc to xtal clock."]
    #[inline]
    pub fn disable_auto_clk_switch(&self) -> DISABLE_AUTO_CLK_SWITCHR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        DISABLE_AUTO_CLK_SWITCHR { bits }
    }
    #[doc = "Bit 2 - select 24 MHz Crystal clock for DCDC, when dcdc_disable_auto_clk_switch is set."]
    #[inline]
    pub fn sel_clk(&self) -> SEL_CLKR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        SEL_CLKR { bits }
    }
    #[doc = "Bit 3 - Power down internal osc. Only set this bit, when 24 MHz crystal osc is available"]
    #[inline]
    pub fn pwd_osc_int(&self) -> PWD_OSC_INTR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        PWD_OSC_INTR { bits }
    }
    #[doc = "Bit 4 - The power down signal of the current detector."]
    #[inline]
    pub fn pwd_cur_sns_cmp(&self) -> PWD_CUR_SNS_CMPR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        PWD_CUR_SNS_CMPR { bits }
    }
    #[doc = "Bits 5:7 - Set the threshold of current detector, if the peak current of the inductor exceeds the threshold, the current detector will assert"]
    #[inline]
    pub fn cur_sns_thrsh(&self) -> CUR_SNS_THRSHR {
        let bits = {
            const MASK: u8 = 7;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        CUR_SNS_THRSHR { bits }
    }
    #[doc = "Bit 8 - power down overcurrent detection comparator"]
    #[inline]
    pub fn pwd_overcur_det(&self) -> PWD_OVERCUR_DETR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        PWD_OVERCUR_DETR { bits }
    }
    #[doc = "Bits 9:10 - The threshold of over current detection in run mode and power save mode: run mode power save mode 0x0 1 A 0"]
    #[inline]
    pub fn overcur_trig_adj(&self) -> OVERCUR_TRIG_ADJR {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        OVERCUR_TRIG_ADJR { bits }
    }
    #[doc = "Bit 11 - set to \"1\" to power down the low voltage detection comparator"]
    #[inline]
    pub fn pwd_cmp_batt_det(&self) -> PWD_CMP_BATT_DETR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        PWD_CMP_BATT_DETR { bits }
    }
    #[doc = "Bits 12:15 - adjust value to poslimit_buck register"]
    #[inline]
    pub fn adj_poslimit_buck(&self) -> ADJ_POSLIMIT_BUCKR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        ADJ_POSLIMIT_BUCKR { bits }
    }
    #[doc = "Bit 16 - enable the overload detection in power save mode, if current is larger than the overloading threshold (typical value is 50 mA), DCDC will switch to the run mode automatically"]
    #[inline]
    pub fn en_lp_overload_sns(&self) -> EN_LP_OVERLOAD_SNSR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        EN_LP_OVERLOAD_SNSR { bits }
    }
    #[doc = "Bit 17 - power down overvoltage detection comparator"]
    #[inline]
    pub fn pwd_high_volt_det(&self) -> PWD_HIGH_VOLT_DETR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 17;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        PWD_HIGH_VOLT_DETR { bits }
    }
    #[doc = "Bits 18:19 - the threshold of the counting number of charging times during the period that lp_overload_freq_sel sets in power save mode"]
    #[inline]
    pub fn lp_overload_thrsh(&self) -> LP_OVERLOAD_THRSHR {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        LP_OVERLOAD_THRSHR { bits }
    }
    #[doc = "Bit 20 - the period of counting the charging times in power save mode 0: eight 32k cycle 1: sixteen 32k cycle"]
    #[inline]
    pub fn lp_overload_freq_sel(&self) -> LP_OVERLOAD_FREQ_SELR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        LP_OVERLOAD_FREQ_SELR { bits }
    }
    #[doc = "Bit 21 - Adjust hysteretic value in low power from 12.5mV to 25mV"]
    #[inline]
    pub fn lp_high_hys(&self) -> LP_HIGH_HYSR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 21;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        LP_HIGH_HYSR { bits }
    }
    #[doc = "Bit 26 - power down output range comparator"]
    #[inline]
    pub fn pwd_cmp_offset(&self) -> PWD_CMP_OFFSETR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 26;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        PWD_CMP_OFFSETR { bits }
    }
    #[doc = "Bit 27 - 1'b1: Disable xtalok detection circuit 1'b0: Enable xtalok detection circuit"]
    #[inline]
    pub fn xtalok_disable(&self) -> XTALOK_DISABLER {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 27;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        XTALOK_DISABLER { bits }
    }
    #[doc = "Bit 28 - reset current alert signal"]
    #[inline]
    pub fn current_alert_reset(&self) -> CURRENT_ALERT_RESETR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 28;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        CURRENT_ALERT_RESETR { bits }
    }
    #[doc = "Bit 29 - set to 1 to switch internal ring osc to xtal 24M"]
    #[inline]
    pub fn xtal_24m_ok(&self) -> XTAL_24M_OKR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 29;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        XTAL_24M_OKR { bits }
    }
    #[doc = "Bit 31 - Status register to indicate DCDC status. 1'b1: DCDC already settled 1'b0: DCDC is settling"]
    #[inline]
    pub fn sts_dc_ok(&self) -> STS_DC_OKR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 31;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        STS_DC_OKR { bits }
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 335741201 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - power down the zero cross detection function for discontinuous conductor mode"]
    #[inline]
    pub fn pwd_zcd(&mut self) -> _PWD_ZCDW {
        _PWD_ZCDW { w: self }
    }
    #[doc = "Bit 1 - Disable automatic clock switch from internal osc to xtal clock."]
    #[inline]
    pub fn disable_auto_clk_switch(&mut self) -> _DISABLE_AUTO_CLK_SWITCHW {
        _DISABLE_AUTO_CLK_SWITCHW { w: self }
    }
    #[doc = "Bit 2 - select 24 MHz Crystal clock for DCDC, when dcdc_disable_auto_clk_switch is set."]
    #[inline]
    pub fn sel_clk(&mut self) -> _SEL_CLKW {
        _SEL_CLKW { w: self }
    }
    #[doc = "Bit 3 - Power down internal osc. Only set this bit, when 24 MHz crystal osc is available"]
    #[inline]
    pub fn pwd_osc_int(&mut self) -> _PWD_OSC_INTW {
        _PWD_OSC_INTW { w: self }
    }
    #[doc = "Bit 4 - The power down signal of the current detector."]
    #[inline]
    pub fn pwd_cur_sns_cmp(&mut self) -> _PWD_CUR_SNS_CMPW {
        _PWD_CUR_SNS_CMPW { w: self }
    }
    #[doc = "Bits 5:7 - Set the threshold of current detector, if the peak current of the inductor exceeds the threshold, the current detector will assert"]
    #[inline]
    pub fn cur_sns_thrsh(&mut self) -> _CUR_SNS_THRSHW {
        _CUR_SNS_THRSHW { w: self }
    }
    #[doc = "Bit 8 - power down overcurrent detection comparator"]
    #[inline]
    pub fn pwd_overcur_det(&mut self) -> _PWD_OVERCUR_DETW {
        _PWD_OVERCUR_DETW { w: self }
    }
    #[doc = "Bits 9:10 - The threshold of over current detection in run mode and power save mode: run mode power save mode 0x0 1 A 0"]
    #[inline]
    pub fn overcur_trig_adj(&mut self) -> _OVERCUR_TRIG_ADJW {
        _OVERCUR_TRIG_ADJW { w: self }
    }
    #[doc = "Bit 11 - set to \"1\" to power down the low voltage detection comparator"]
    #[inline]
    pub fn pwd_cmp_batt_det(&mut self) -> _PWD_CMP_BATT_DETW {
        _PWD_CMP_BATT_DETW { w: self }
    }
    #[doc = "Bits 12:15 - adjust value to poslimit_buck register"]
    #[inline]
    pub fn adj_poslimit_buck(&mut self) -> _ADJ_POSLIMIT_BUCKW {
        _ADJ_POSLIMIT_BUCKW { w: self }
    }
    #[doc = "Bit 16 - enable the overload detection in power save mode, if current is larger than the overloading threshold (typical value is 50 mA), DCDC will switch to the run mode automatically"]
    #[inline]
    pub fn en_lp_overload_sns(&mut self) -> _EN_LP_OVERLOAD_SNSW {
        _EN_LP_OVERLOAD_SNSW { w: self }
    }
    #[doc = "Bit 17 - power down overvoltage detection comparator"]
    #[inline]
    pub fn pwd_high_volt_det(&mut self) -> _PWD_HIGH_VOLT_DETW {
        _PWD_HIGH_VOLT_DETW { w: self }
    }
    #[doc = "Bits 18:19 - the threshold of the counting number of charging times during the period that lp_overload_freq_sel sets in power save mode"]
    #[inline]
    pub fn lp_overload_thrsh(&mut self) -> _LP_OVERLOAD_THRSHW {
        _LP_OVERLOAD_THRSHW { w: self }
    }
    #[doc = "Bit 20 - the period of counting the charging times in power save mode 0: eight 32k cycle 1: sixteen 32k cycle"]
    #[inline]
    pub fn lp_overload_freq_sel(&mut self) -> _LP_OVERLOAD_FREQ_SELW {
        _LP_OVERLOAD_FREQ_SELW { w: self }
    }
    #[doc = "Bit 21 - Adjust hysteretic value in low power from 12.5mV to 25mV"]
    #[inline]
    pub fn lp_high_hys(&mut self) -> _LP_HIGH_HYSW {
        _LP_HIGH_HYSW { w: self }
    }
    #[doc = "Bit 26 - power down output range comparator"]
    #[inline]
    pub fn pwd_cmp_offset(&mut self) -> _PWD_CMP_OFFSETW {
        _PWD_CMP_OFFSETW { w: self }
    }
    #[doc = "Bit 27 - 1'b1: Disable xtalok detection circuit 1'b0: Enable xtalok detection circuit"]
    #[inline]
    pub fn xtalok_disable(&mut self) -> _XTALOK_DISABLEW {
        _XTALOK_DISABLEW { w: self }
    }
    #[doc = "Bit 28 - reset current alert signal"]
    #[inline]
    pub fn current_alert_reset(&mut self) -> _CURRENT_ALERT_RESETW {
        _CURRENT_ALERT_RESETW { w: self }
    }
    #[doc = "Bit 29 - set to 1 to switch internal ring osc to xtal 24M"]
    #[inline]
    pub fn xtal_24m_ok(&mut self) -> _XTAL_24M_OKW {
        _XTAL_24M_OKW { w: self }
    }
}
