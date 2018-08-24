#[doc = r" Value read from the register"]
pub struct R {
    bits: u8,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u8,
}
impl super::CR1 {
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
#[doc = "Possible values of the field `EN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ENR {
    #[doc = "Analog Comparator is disabled."]
    EN_0,
    #[doc = "Analog Comparator is enabled."]
    EN_1,
}
impl ENR {
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
            ENR::EN_0 => false,
            ENR::EN_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ENR {
        match value {
            false => ENR::EN_0,
            true => ENR::EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `EN_0`"]
    #[inline]
    pub fn is_en_0(&self) -> bool {
        *self == ENR::EN_0
    }
    #[doc = "Checks if the value of the field is `EN_1`"]
    #[inline]
    pub fn is_en_1(&self) -> bool {
        *self == ENR::EN_1
    }
}
#[doc = "Possible values of the field `OPE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OPER {
    #[doc = "CMPO is not available on the associated CMPO output pin. If the comparator does not own the pin, this field has no effect."]
    OPE_0,
    #[doc = "CMPO is available on the associated CMPO output pin. The comparator output (CMPO) is driven out on the associated CMPO output pin if the comparator owns the pin. If the comparator does not own the field, this bit has no effect."]
    OPE_1,
}
impl OPER {
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
            OPER::OPE_0 => false,
            OPER::OPE_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> OPER {
        match value {
            false => OPER::OPE_0,
            true => OPER::OPE_1,
        }
    }
    #[doc = "Checks if the value of the field is `OPE_0`"]
    #[inline]
    pub fn is_ope_0(&self) -> bool {
        *self == OPER::OPE_0
    }
    #[doc = "Checks if the value of the field is `OPE_1`"]
    #[inline]
    pub fn is_ope_1(&self) -> bool {
        *self == OPER::OPE_1
    }
}
#[doc = "Possible values of the field `COS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum COSR {
    #[doc = "Set the filtered comparator output (CMPO) to equal COUT."]
    COS_0,
    #[doc = "Set the unfiltered comparator output (CMPO) to equal COUTA."]
    COS_1,
}
impl COSR {
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
            COSR::COS_0 => false,
            COSR::COS_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> COSR {
        match value {
            false => COSR::COS_0,
            true => COSR::COS_1,
        }
    }
    #[doc = "Checks if the value of the field is `COS_0`"]
    #[inline]
    pub fn is_cos_0(&self) -> bool {
        *self == COSR::COS_0
    }
    #[doc = "Checks if the value of the field is `COS_1`"]
    #[inline]
    pub fn is_cos_1(&self) -> bool {
        *self == COSR::COS_1
    }
}
#[doc = "Possible values of the field `INV`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INVR {
    #[doc = "Does not invert the comparator output."]
    INV_0,
    #[doc = "Inverts the comparator output."]
    INV_1,
}
impl INVR {
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
            INVR::INV_0 => false,
            INVR::INV_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> INVR {
        match value {
            false => INVR::INV_0,
            true => INVR::INV_1,
        }
    }
    #[doc = "Checks if the value of the field is `INV_0`"]
    #[inline]
    pub fn is_inv_0(&self) -> bool {
        *self == INVR::INV_0
    }
    #[doc = "Checks if the value of the field is `INV_1`"]
    #[inline]
    pub fn is_inv_1(&self) -> bool {
        *self == INVR::INV_1
    }
}
#[doc = "Possible values of the field `PMODE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PMODER {
    #[doc = "Low-Speed (LS) Comparison mode selected. In this mode, CMP has slower output propagation delay and lower current consumption."]
    PMODE_0,
    #[doc = "High-Speed (HS) Comparison mode selected. In this mode, CMP has faster output propagation delay and higher current consumption."]
    PMODE_1,
}
impl PMODER {
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
            PMODER::PMODE_0 => false,
            PMODER::PMODE_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PMODER {
        match value {
            false => PMODER::PMODE_0,
            true => PMODER::PMODE_1,
        }
    }
    #[doc = "Checks if the value of the field is `PMODE_0`"]
    #[inline]
    pub fn is_pmode_0(&self) -> bool {
        *self == PMODER::PMODE_0
    }
    #[doc = "Checks if the value of the field is `PMODE_1`"]
    #[inline]
    pub fn is_pmode_1(&self) -> bool {
        *self == PMODER::PMODE_1
    }
}
#[doc = "Possible values of the field `WE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WER {
    #[doc = "Windowing mode is not selected."]
    WE_0,
    #[doc = "Windowing mode is selected."]
    WE_1,
}
impl WER {
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
            WER::WE_0 => false,
            WER::WE_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> WER {
        match value {
            false => WER::WE_0,
            true => WER::WE_1,
        }
    }
    #[doc = "Checks if the value of the field is `WE_0`"]
    #[inline]
    pub fn is_we_0(&self) -> bool {
        *self == WER::WE_0
    }
    #[doc = "Checks if the value of the field is `WE_1`"]
    #[inline]
    pub fn is_we_1(&self) -> bool {
        *self == WER::WE_1
    }
}
#[doc = "Possible values of the field `SE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SER {
    #[doc = "Sampling mode is not selected."]
    SE_0,
    #[doc = "Sampling mode is selected."]
    SE_1,
}
impl SER {
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
            SER::SE_0 => false,
            SER::SE_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SER {
        match value {
            false => SER::SE_0,
            true => SER::SE_1,
        }
    }
    #[doc = "Checks if the value of the field is `SE_0`"]
    #[inline]
    pub fn is_se_0(&self) -> bool {
        *self == SER::SE_0
    }
    #[doc = "Checks if the value of the field is `SE_1`"]
    #[inline]
    pub fn is_se_1(&self) -> bool {
        *self == SER::SE_1
    }
}
#[doc = "Values that can be written to the field `EN`"]
pub enum ENW {
    #[doc = "Analog Comparator is disabled."]
    EN_0,
    #[doc = "Analog Comparator is enabled."]
    EN_1,
}
impl ENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ENW::EN_0 => false,
            ENW::EN_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ENW<'a> {
    w: &'a mut W,
}
impl<'a> _ENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Analog Comparator is disabled."]
    #[inline]
    pub fn en_0(self) -> &'a mut W {
        self.variant(ENW::EN_0)
    }
    #[doc = "Analog Comparator is enabled."]
    #[inline]
    pub fn en_1(self) -> &'a mut W {
        self.variant(ENW::EN_1)
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
        self.w.bits &= !((MASK as u8) << OFFSET);
        self.w.bits |= ((value & MASK) as u8) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `OPE`"]
pub enum OPEW {
    #[doc = "CMPO is not available on the associated CMPO output pin. If the comparator does not own the pin, this field has no effect."]
    OPE_0,
    #[doc = "CMPO is available on the associated CMPO output pin. The comparator output (CMPO) is driven out on the associated CMPO output pin if the comparator owns the pin. If the comparator does not own the field, this bit has no effect."]
    OPE_1,
}
impl OPEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            OPEW::OPE_0 => false,
            OPEW::OPE_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _OPEW<'a> {
    w: &'a mut W,
}
impl<'a> _OPEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: OPEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "CMPO is not available on the associated CMPO output pin. If the comparator does not own the pin, this field has no effect."]
    #[inline]
    pub fn ope_0(self) -> &'a mut W {
        self.variant(OPEW::OPE_0)
    }
    #[doc = "CMPO is available on the associated CMPO output pin. The comparator output (CMPO) is driven out on the associated CMPO output pin if the comparator owns the pin. If the comparator does not own the field, this bit has no effect."]
    #[inline]
    pub fn ope_1(self) -> &'a mut W {
        self.variant(OPEW::OPE_1)
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
        self.w.bits &= !((MASK as u8) << OFFSET);
        self.w.bits |= ((value & MASK) as u8) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `COS`"]
pub enum COSW {
    #[doc = "Set the filtered comparator output (CMPO) to equal COUT."]
    COS_0,
    #[doc = "Set the unfiltered comparator output (CMPO) to equal COUTA."]
    COS_1,
}
impl COSW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            COSW::COS_0 => false,
            COSW::COS_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _COSW<'a> {
    w: &'a mut W,
}
impl<'a> _COSW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: COSW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Set the filtered comparator output (CMPO) to equal COUT."]
    #[inline]
    pub fn cos_0(self) -> &'a mut W {
        self.variant(COSW::COS_0)
    }
    #[doc = "Set the unfiltered comparator output (CMPO) to equal COUTA."]
    #[inline]
    pub fn cos_1(self) -> &'a mut W {
        self.variant(COSW::COS_1)
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
        self.w.bits &= !((MASK as u8) << OFFSET);
        self.w.bits |= ((value & MASK) as u8) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `INV`"]
pub enum INVW {
    #[doc = "Does not invert the comparator output."]
    INV_0,
    #[doc = "Inverts the comparator output."]
    INV_1,
}
impl INVW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            INVW::INV_0 => false,
            INVW::INV_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _INVW<'a> {
    w: &'a mut W,
}
impl<'a> _INVW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: INVW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Does not invert the comparator output."]
    #[inline]
    pub fn inv_0(self) -> &'a mut W {
        self.variant(INVW::INV_0)
    }
    #[doc = "Inverts the comparator output."]
    #[inline]
    pub fn inv_1(self) -> &'a mut W {
        self.variant(INVW::INV_1)
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
        self.w.bits &= !((MASK as u8) << OFFSET);
        self.w.bits |= ((value & MASK) as u8) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `PMODE`"]
pub enum PMODEW {
    #[doc = "Low-Speed (LS) Comparison mode selected. In this mode, CMP has slower output propagation delay and lower current consumption."]
    PMODE_0,
    #[doc = "High-Speed (HS) Comparison mode selected. In this mode, CMP has faster output propagation delay and higher current consumption."]
    PMODE_1,
}
impl PMODEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PMODEW::PMODE_0 => false,
            PMODEW::PMODE_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PMODEW<'a> {
    w: &'a mut W,
}
impl<'a> _PMODEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PMODEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Low-Speed (LS) Comparison mode selected. In this mode, CMP has slower output propagation delay and lower current consumption."]
    #[inline]
    pub fn pmode_0(self) -> &'a mut W {
        self.variant(PMODEW::PMODE_0)
    }
    #[doc = "High-Speed (HS) Comparison mode selected. In this mode, CMP has faster output propagation delay and higher current consumption."]
    #[inline]
    pub fn pmode_1(self) -> &'a mut W {
        self.variant(PMODEW::PMODE_1)
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
        self.w.bits &= !((MASK as u8) << OFFSET);
        self.w.bits |= ((value & MASK) as u8) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `WE`"]
pub enum WEW {
    #[doc = "Windowing mode is not selected."]
    WE_0,
    #[doc = "Windowing mode is selected."]
    WE_1,
}
impl WEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            WEW::WE_0 => false,
            WEW::WE_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _WEW<'a> {
    w: &'a mut W,
}
impl<'a> _WEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: WEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Windowing mode is not selected."]
    #[inline]
    pub fn we_0(self) -> &'a mut W {
        self.variant(WEW::WE_0)
    }
    #[doc = "Windowing mode is selected."]
    #[inline]
    pub fn we_1(self) -> &'a mut W {
        self.variant(WEW::WE_1)
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
        self.w.bits &= !((MASK as u8) << OFFSET);
        self.w.bits |= ((value & MASK) as u8) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `SE`"]
pub enum SEW {
    #[doc = "Sampling mode is not selected."]
    SE_0,
    #[doc = "Sampling mode is selected."]
    SE_1,
}
impl SEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SEW::SE_0 => false,
            SEW::SE_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SEW<'a> {
    w: &'a mut W,
}
impl<'a> _SEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Sampling mode is not selected."]
    #[inline]
    pub fn se_0(self) -> &'a mut W {
        self.variant(SEW::SE_0)
    }
    #[doc = "Sampling mode is selected."]
    #[inline]
    pub fn se_1(self) -> &'a mut W {
        self.variant(SEW::SE_1)
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
        self.w.bits &= !((MASK as u8) << OFFSET);
        self.w.bits |= ((value & MASK) as u8) << OFFSET;
        self.w
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
    #[doc = "Bit 0 - Comparator Module Enable"]
    #[inline]
    pub fn en(&self) -> ENR {
        ENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        })
    }
    #[doc = "Bit 1 - Comparator Output Pin Enable"]
    #[inline]
    pub fn ope(&self) -> OPER {
        OPER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        })
    }
    #[doc = "Bit 2 - Comparator Output Select"]
    #[inline]
    pub fn cos(&self) -> COSR {
        COSR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        })
    }
    #[doc = "Bit 3 - Comparator INVERT"]
    #[inline]
    pub fn inv(&self) -> INVR {
        INVR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        })
    }
    #[doc = "Bit 4 - Power Mode Select"]
    #[inline]
    pub fn pmode(&self) -> PMODER {
        PMODER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        })
    }
    #[doc = "Bit 6 - Windowing Enable"]
    #[inline]
    pub fn we(&self) -> WER {
        WER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        })
    }
    #[doc = "Bit 7 - Sample Enable"]
    #[inline]
    pub fn se(&self) -> SER {
        SER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u8) != 0
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
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - Comparator Module Enable"]
    #[inline]
    pub fn en(&mut self) -> _ENW {
        _ENW { w: self }
    }
    #[doc = "Bit 1 - Comparator Output Pin Enable"]
    #[inline]
    pub fn ope(&mut self) -> _OPEW {
        _OPEW { w: self }
    }
    #[doc = "Bit 2 - Comparator Output Select"]
    #[inline]
    pub fn cos(&mut self) -> _COSW {
        _COSW { w: self }
    }
    #[doc = "Bit 3 - Comparator INVERT"]
    #[inline]
    pub fn inv(&mut self) -> _INVW {
        _INVW { w: self }
    }
    #[doc = "Bit 4 - Power Mode Select"]
    #[inline]
    pub fn pmode(&mut self) -> _PMODEW {
        _PMODEW { w: self }
    }
    #[doc = "Bit 6 - Windowing Enable"]
    #[inline]
    pub fn we(&mut self) -> _WEW {
        _WEW { w: self }
    }
    #[doc = "Bit 7 - Sample Enable"]
    #[inline]
    pub fn se(&mut self) -> _SEW {
        _SEW { w: self }
    }
}
