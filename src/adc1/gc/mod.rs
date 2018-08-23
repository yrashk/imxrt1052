#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::GC {
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
#[doc = "Possible values of the field `ADACKEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADACKENR {
    #[doc = "Asynchronous clock output disabled; Asynchronous clock only enabled if selected by ADICLK and a conversion is active."]
    ADACKEN_0,
    #[doc = "Asynchronous clock and clock output enabled regardless of the state of the ADC"]
    ADACKEN_1,
}
impl ADACKENR {
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
            ADACKENR::ADACKEN_0 => false,
            ADACKENR::ADACKEN_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ADACKENR {
        match value {
            false => ADACKENR::ADACKEN_0,
            true => ADACKENR::ADACKEN_1,
        }
    }
    #[doc = "Checks if the value of the field is `ADACKEN_0`"]
    #[inline]
    pub fn is_adacken_0(&self) -> bool {
        *self == ADACKENR::ADACKEN_0
    }
    #[doc = "Checks if the value of the field is `ADACKEN_1`"]
    #[inline]
    pub fn is_adacken_1(&self) -> bool {
        *self == ADACKENR::ADACKEN_1
    }
}
#[doc = "Possible values of the field `DMAEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DMAENR {
    #[doc = "DMA disabled (default)"]
    DMAEN_0,
    #[doc = "DMA enabled"]
    DMAEN_1,
}
impl DMAENR {
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
            DMAENR::DMAEN_0 => false,
            DMAENR::DMAEN_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DMAENR {
        match value {
            false => DMAENR::DMAEN_0,
            true => DMAENR::DMAEN_1,
        }
    }
    #[doc = "Checks if the value of the field is `DMAEN_0`"]
    #[inline]
    pub fn is_dmaen_0(&self) -> bool {
        *self == DMAENR::DMAEN_0
    }
    #[doc = "Checks if the value of the field is `DMAEN_1`"]
    #[inline]
    pub fn is_dmaen_1(&self) -> bool {
        *self == DMAENR::DMAEN_1
    }
}
#[doc = "Possible values of the field `ACREN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ACRENR {
    #[doc = "Range function disabled. Only the compare value 1 of ADC_CV register (CV1) is compared."]
    ACREN_0,
    #[doc = "Range function enabled. Both compare values of ADC_CV registers (CV1 and CV2) are compared."]
    ACREN_1,
}
impl ACRENR {
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
            ACRENR::ACREN_0 => false,
            ACRENR::ACREN_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ACRENR {
        match value {
            false => ACRENR::ACREN_0,
            true => ACRENR::ACREN_1,
        }
    }
    #[doc = "Checks if the value of the field is `ACREN_0`"]
    #[inline]
    pub fn is_acren_0(&self) -> bool {
        *self == ACRENR::ACREN_0
    }
    #[doc = "Checks if the value of the field is `ACREN_1`"]
    #[inline]
    pub fn is_acren_1(&self) -> bool {
        *self == ACRENR::ACREN_1
    }
}
#[doc = "Possible values of the field `ACFGT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ACFGTR {
    #[doc = "Configures \"Less Than Threshold, Outside Range Not Inclusive and Inside Range Not Inclusive\" functionality based on the values placed in the ADC_CV register."]
    ACFGT_0,
    #[doc = "Configures \"Greater Than Or Equal To Threshold, Outside Range Inclusive and Inside Range Inclusive\" functionality based on the values placed in the ADC_CV registers."]
    ACFGT_1,
}
impl ACFGTR {
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
            ACFGTR::ACFGT_0 => false,
            ACFGTR::ACFGT_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ACFGTR {
        match value {
            false => ACFGTR::ACFGT_0,
            true => ACFGTR::ACFGT_1,
        }
    }
    #[doc = "Checks if the value of the field is `ACFGT_0`"]
    #[inline]
    pub fn is_acfgt_0(&self) -> bool {
        *self == ACFGTR::ACFGT_0
    }
    #[doc = "Checks if the value of the field is `ACFGT_1`"]
    #[inline]
    pub fn is_acfgt_1(&self) -> bool {
        *self == ACFGTR::ACFGT_1
    }
}
#[doc = "Possible values of the field `ACFE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ACFER {
    #[doc = "Compare function disabled"]
    ACFE_0,
    #[doc = "Compare function enabled"]
    ACFE_1,
}
impl ACFER {
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
            ACFER::ACFE_0 => false,
            ACFER::ACFE_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ACFER {
        match value {
            false => ACFER::ACFE_0,
            true => ACFER::ACFE_1,
        }
    }
    #[doc = "Checks if the value of the field is `ACFE_0`"]
    #[inline]
    pub fn is_acfe_0(&self) -> bool {
        *self == ACFER::ACFE_0
    }
    #[doc = "Checks if the value of the field is `ACFE_1`"]
    #[inline]
    pub fn is_acfe_1(&self) -> bool {
        *self == ACFER::ACFE_1
    }
}
#[doc = "Possible values of the field `AVGE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AVGER {
    #[doc = "Hardware average function disabled"]
    AVGE_0,
    #[doc = "Hardware average function enabled"]
    AVGE_1,
}
impl AVGER {
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
            AVGER::AVGE_0 => false,
            AVGER::AVGE_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> AVGER {
        match value {
            false => AVGER::AVGE_0,
            true => AVGER::AVGE_1,
        }
    }
    #[doc = "Checks if the value of the field is `AVGE_0`"]
    #[inline]
    pub fn is_avge_0(&self) -> bool {
        *self == AVGER::AVGE_0
    }
    #[doc = "Checks if the value of the field is `AVGE_1`"]
    #[inline]
    pub fn is_avge_1(&self) -> bool {
        *self == AVGER::AVGE_1
    }
}
#[doc = "Possible values of the field `ADCO`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADCOR {
    #[doc = "One conversion or one set of conversions if the hardware average function is enabled (AVGE=1) after initiating a conversion."]
    ADCO_0,
    #[doc = "Continuous conversions or sets of conversions if the hardware average function is enabled (AVGE=1) after initiating a conversion."]
    ADCO_1,
}
impl ADCOR {
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
            ADCOR::ADCO_0 => false,
            ADCOR::ADCO_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ADCOR {
        match value {
            false => ADCOR::ADCO_0,
            true => ADCOR::ADCO_1,
        }
    }
    #[doc = "Checks if the value of the field is `ADCO_0`"]
    #[inline]
    pub fn is_adco_0(&self) -> bool {
        *self == ADCOR::ADCO_0
    }
    #[doc = "Checks if the value of the field is `ADCO_1`"]
    #[inline]
    pub fn is_adco_1(&self) -> bool {
        *self == ADCOR::ADCO_1
    }
}
#[doc = r" Value of the field"]
pub struct CALR {
    bits: bool,
}
impl CALR {
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
#[doc = "Values that can be written to the field `ADACKEN`"]
pub enum ADACKENW {
    #[doc = "Asynchronous clock output disabled; Asynchronous clock only enabled if selected by ADICLK and a conversion is active."]
    ADACKEN_0,
    #[doc = "Asynchronous clock and clock output enabled regardless of the state of the ADC"]
    ADACKEN_1,
}
impl ADACKENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ADACKENW::ADACKEN_0 => false,
            ADACKENW::ADACKEN_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ADACKENW<'a> {
    w: &'a mut W,
}
impl<'a> _ADACKENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ADACKENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Asynchronous clock output disabled; Asynchronous clock only enabled if selected by ADICLK and a conversion is active."]
    #[inline]
    pub fn adacken_0(self) -> &'a mut W {
        self.variant(ADACKENW::ADACKEN_0)
    }
    #[doc = "Asynchronous clock and clock output enabled regardless of the state of the ADC"]
    #[inline]
    pub fn adacken_1(self) -> &'a mut W {
        self.variant(ADACKENW::ADACKEN_1)
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
#[doc = "Values that can be written to the field `DMAEN`"]
pub enum DMAENW {
    #[doc = "DMA disabled (default)"]
    DMAEN_0,
    #[doc = "DMA enabled"]
    DMAEN_1,
}
impl DMAENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DMAENW::DMAEN_0 => false,
            DMAENW::DMAEN_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DMAENW<'a> {
    w: &'a mut W,
}
impl<'a> _DMAENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DMAENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "DMA disabled (default)"]
    #[inline]
    pub fn dmaen_0(self) -> &'a mut W {
        self.variant(DMAENW::DMAEN_0)
    }
    #[doc = "DMA enabled"]
    #[inline]
    pub fn dmaen_1(self) -> &'a mut W {
        self.variant(DMAENW::DMAEN_1)
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
#[doc = "Values that can be written to the field `ACREN`"]
pub enum ACRENW {
    #[doc = "Range function disabled. Only the compare value 1 of ADC_CV register (CV1) is compared."]
    ACREN_0,
    #[doc = "Range function enabled. Both compare values of ADC_CV registers (CV1 and CV2) are compared."]
    ACREN_1,
}
impl ACRENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ACRENW::ACREN_0 => false,
            ACRENW::ACREN_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ACRENW<'a> {
    w: &'a mut W,
}
impl<'a> _ACRENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ACRENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Range function disabled. Only the compare value 1 of ADC_CV register (CV1) is compared."]
    #[inline]
    pub fn acren_0(self) -> &'a mut W {
        self.variant(ACRENW::ACREN_0)
    }
    #[doc = "Range function enabled. Both compare values of ADC_CV registers (CV1 and CV2) are compared."]
    #[inline]
    pub fn acren_1(self) -> &'a mut W {
        self.variant(ACRENW::ACREN_1)
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
        const OFFSET: u8 = 2;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `ACFGT`"]
pub enum ACFGTW {
    #[doc = "Configures \"Less Than Threshold, Outside Range Not Inclusive and Inside Range Not Inclusive\" functionality based on the values placed in the ADC_CV register."]
    ACFGT_0,
    #[doc = "Configures \"Greater Than Or Equal To Threshold, Outside Range Inclusive and Inside Range Inclusive\" functionality based on the values placed in the ADC_CV registers."]
    ACFGT_1,
}
impl ACFGTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ACFGTW::ACFGT_0 => false,
            ACFGTW::ACFGT_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ACFGTW<'a> {
    w: &'a mut W,
}
impl<'a> _ACFGTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ACFGTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Configures \"Less Than Threshold, Outside Range Not Inclusive and Inside Range Not Inclusive\" functionality based on the values placed in the ADC_CV register."]
    #[inline]
    pub fn acfgt_0(self) -> &'a mut W {
        self.variant(ACFGTW::ACFGT_0)
    }
    #[doc = "Configures \"Greater Than Or Equal To Threshold, Outside Range Inclusive and Inside Range Inclusive\" functionality based on the values placed in the ADC_CV registers."]
    #[inline]
    pub fn acfgt_1(self) -> &'a mut W {
        self.variant(ACFGTW::ACFGT_1)
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
        const OFFSET: u8 = 3;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `ACFE`"]
pub enum ACFEW {
    #[doc = "Compare function disabled"]
    ACFE_0,
    #[doc = "Compare function enabled"]
    ACFE_1,
}
impl ACFEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ACFEW::ACFE_0 => false,
            ACFEW::ACFE_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ACFEW<'a> {
    w: &'a mut W,
}
impl<'a> _ACFEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ACFEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Compare function disabled"]
    #[inline]
    pub fn acfe_0(self) -> &'a mut W {
        self.variant(ACFEW::ACFE_0)
    }
    #[doc = "Compare function enabled"]
    #[inline]
    pub fn acfe_1(self) -> &'a mut W {
        self.variant(ACFEW::ACFE_1)
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
#[doc = "Values that can be written to the field `AVGE`"]
pub enum AVGEW {
    #[doc = "Hardware average function disabled"]
    AVGE_0,
    #[doc = "Hardware average function enabled"]
    AVGE_1,
}
impl AVGEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            AVGEW::AVGE_0 => false,
            AVGEW::AVGE_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _AVGEW<'a> {
    w: &'a mut W,
}
impl<'a> _AVGEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: AVGEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Hardware average function disabled"]
    #[inline]
    pub fn avge_0(self) -> &'a mut W {
        self.variant(AVGEW::AVGE_0)
    }
    #[doc = "Hardware average function enabled"]
    #[inline]
    pub fn avge_1(self) -> &'a mut W {
        self.variant(AVGEW::AVGE_1)
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
#[doc = "Values that can be written to the field `ADCO`"]
pub enum ADCOW {
    #[doc = "One conversion or one set of conversions if the hardware average function is enabled (AVGE=1) after initiating a conversion."]
    ADCO_0,
    #[doc = "Continuous conversions or sets of conversions if the hardware average function is enabled (AVGE=1) after initiating a conversion."]
    ADCO_1,
}
impl ADCOW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ADCOW::ADCO_0 => false,
            ADCOW::ADCO_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ADCOW<'a> {
    w: &'a mut W,
}
impl<'a> _ADCOW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ADCOW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "One conversion or one set of conversions if the hardware average function is enabled (AVGE=1) after initiating a conversion."]
    #[inline]
    pub fn adco_0(self) -> &'a mut W {
        self.variant(ADCOW::ADCO_0)
    }
    #[doc = "Continuous conversions or sets of conversions if the hardware average function is enabled (AVGE=1) after initiating a conversion."]
    #[inline]
    pub fn adco_1(self) -> &'a mut W {
        self.variant(ADCOW::ADCO_1)
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
        const OFFSET: u8 = 6;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _CALW<'a> {
    w: &'a mut W,
}
impl<'a> _CALW<'a> {
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
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - Asynchronous clock output enable"]
    #[inline]
    pub fn adacken(&self) -> ADACKENR {
        ADACKENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - DMA Enable"]
    #[inline]
    pub fn dmaen(&self) -> DMAENR {
        DMAENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - Compare Function Range Enable"]
    #[inline]
    pub fn acren(&self) -> ACRENR {
        ACRENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - Compare Function Greater Than Enable"]
    #[inline]
    pub fn acfgt(&self) -> ACFGTR {
        ACFGTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 4 - Compare Function Enable"]
    #[inline]
    pub fn acfe(&self) -> ACFER {
        ACFER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 5 - Hardware average enable"]
    #[inline]
    pub fn avge(&self) -> AVGER {
        AVGER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 6 - Continuous Conversion Enable"]
    #[inline]
    pub fn adco(&self) -> ADCOR {
        ADCOR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 7 - Calibration"]
    #[inline]
    pub fn cal(&self) -> CALR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        CALR { bits }
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
    #[doc = "Bit 0 - Asynchronous clock output enable"]
    #[inline]
    pub fn adacken(&mut self) -> _ADACKENW {
        _ADACKENW { w: self }
    }
    #[doc = "Bit 1 - DMA Enable"]
    #[inline]
    pub fn dmaen(&mut self) -> _DMAENW {
        _DMAENW { w: self }
    }
    #[doc = "Bit 2 - Compare Function Range Enable"]
    #[inline]
    pub fn acren(&mut self) -> _ACRENW {
        _ACRENW { w: self }
    }
    #[doc = "Bit 3 - Compare Function Greater Than Enable"]
    #[inline]
    pub fn acfgt(&mut self) -> _ACFGTW {
        _ACFGTW { w: self }
    }
    #[doc = "Bit 4 - Compare Function Enable"]
    #[inline]
    pub fn acfe(&mut self) -> _ACFEW {
        _ACFEW { w: self }
    }
    #[doc = "Bit 5 - Hardware average enable"]
    #[inline]
    pub fn avge(&mut self) -> _AVGEW {
        _AVGEW { w: self }
    }
    #[doc = "Bit 6 - Continuous Conversion Enable"]
    #[inline]
    pub fn adco(&mut self) -> _ADCOW {
        _ADCOW { w: self }
    }
    #[doc = "Bit 7 - Calibration"]
    #[inline]
    pub fn cal(&mut self) -> _CALW {
        _CALW { w: self }
    }
}
