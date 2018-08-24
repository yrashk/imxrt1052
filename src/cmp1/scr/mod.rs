#[doc = r" Value read from the register"]
pub struct R {
    bits: u8,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u8,
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
#[doc = r" Value of the field"]
pub struct COUTR {
    bits: bool,
}
impl COUTR {
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
#[doc = "Possible values of the field `CFF`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CFFR {
    #[doc = "Falling-edge on COUT has not been detected."]
    CFF_0,
    #[doc = "Falling-edge on COUT has occurred."]
    CFF_1,
}
impl CFFR {
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
            CFFR::CFF_0 => false,
            CFFR::CFF_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CFFR {
        match value {
            false => CFFR::CFF_0,
            true => CFFR::CFF_1,
        }
    }
    #[doc = "Checks if the value of the field is `CFF_0`"]
    #[inline]
    pub fn is_cff_0(&self) -> bool {
        *self == CFFR::CFF_0
    }
    #[doc = "Checks if the value of the field is `CFF_1`"]
    #[inline]
    pub fn is_cff_1(&self) -> bool {
        *self == CFFR::CFF_1
    }
}
#[doc = "Possible values of the field `CFR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CFRR {
    #[doc = "Rising-edge on COUT has not been detected."]
    CFR_0,
    #[doc = "Rising-edge on COUT has occurred."]
    CFR_1,
}
impl CFRR {
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
            CFRR::CFR_0 => false,
            CFRR::CFR_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CFRR {
        match value {
            false => CFRR::CFR_0,
            true => CFRR::CFR_1,
        }
    }
    #[doc = "Checks if the value of the field is `CFR_0`"]
    #[inline]
    pub fn is_cfr_0(&self) -> bool {
        *self == CFRR::CFR_0
    }
    #[doc = "Checks if the value of the field is `CFR_1`"]
    #[inline]
    pub fn is_cfr_1(&self) -> bool {
        *self == CFRR::CFR_1
    }
}
#[doc = "Possible values of the field `IEF`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IEFR {
    #[doc = "Interrupt is disabled."]
    IEF_0,
    #[doc = "Interrupt is enabled."]
    IEF_1,
}
impl IEFR {
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
            IEFR::IEF_0 => false,
            IEFR::IEF_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> IEFR {
        match value {
            false => IEFR::IEF_0,
            true => IEFR::IEF_1,
        }
    }
    #[doc = "Checks if the value of the field is `IEF_0`"]
    #[inline]
    pub fn is_ief_0(&self) -> bool {
        *self == IEFR::IEF_0
    }
    #[doc = "Checks if the value of the field is `IEF_1`"]
    #[inline]
    pub fn is_ief_1(&self) -> bool {
        *self == IEFR::IEF_1
    }
}
#[doc = "Possible values of the field `IER`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IERR {
    #[doc = "Interrupt is disabled."]
    IER_0,
    #[doc = "Interrupt is enabled."]
    IER_1,
}
impl IERR {
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
            IERR::IER_0 => false,
            IERR::IER_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> IERR {
        match value {
            false => IERR::IER_0,
            true => IERR::IER_1,
        }
    }
    #[doc = "Checks if the value of the field is `IER_0`"]
    #[inline]
    pub fn is_ier_0(&self) -> bool {
        *self == IERR::IER_0
    }
    #[doc = "Checks if the value of the field is `IER_1`"]
    #[inline]
    pub fn is_ier_1(&self) -> bool {
        *self == IERR::IER_1
    }
}
#[doc = "Possible values of the field `DMAEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DMAENR {
    #[doc = "DMA is disabled."]
    DMAEN_0,
    #[doc = "DMA is enabled."]
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
#[doc = "Values that can be written to the field `CFF`"]
pub enum CFFW {
    #[doc = "Falling-edge on COUT has not been detected."]
    CFF_0,
    #[doc = "Falling-edge on COUT has occurred."]
    CFF_1,
}
impl CFFW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CFFW::CFF_0 => false,
            CFFW::CFF_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CFFW<'a> {
    w: &'a mut W,
}
impl<'a> _CFFW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CFFW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Falling-edge on COUT has not been detected."]
    #[inline]
    pub fn cff_0(self) -> &'a mut W {
        self.variant(CFFW::CFF_0)
    }
    #[doc = "Falling-edge on COUT has occurred."]
    #[inline]
    pub fn cff_1(self) -> &'a mut W {
        self.variant(CFFW::CFF_1)
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
#[doc = "Values that can be written to the field `CFR`"]
pub enum CFRW {
    #[doc = "Rising-edge on COUT has not been detected."]
    CFR_0,
    #[doc = "Rising-edge on COUT has occurred."]
    CFR_1,
}
impl CFRW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CFRW::CFR_0 => false,
            CFRW::CFR_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CFRW<'a> {
    w: &'a mut W,
}
impl<'a> _CFRW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CFRW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Rising-edge on COUT has not been detected."]
    #[inline]
    pub fn cfr_0(self) -> &'a mut W {
        self.variant(CFRW::CFR_0)
    }
    #[doc = "Rising-edge on COUT has occurred."]
    #[inline]
    pub fn cfr_1(self) -> &'a mut W {
        self.variant(CFRW::CFR_1)
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
#[doc = "Values that can be written to the field `IEF`"]
pub enum IEFW {
    #[doc = "Interrupt is disabled."]
    IEF_0,
    #[doc = "Interrupt is enabled."]
    IEF_1,
}
impl IEFW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            IEFW::IEF_0 => false,
            IEFW::IEF_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _IEFW<'a> {
    w: &'a mut W,
}
impl<'a> _IEFW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: IEFW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Interrupt is disabled."]
    #[inline]
    pub fn ief_0(self) -> &'a mut W {
        self.variant(IEFW::IEF_0)
    }
    #[doc = "Interrupt is enabled."]
    #[inline]
    pub fn ief_1(self) -> &'a mut W {
        self.variant(IEFW::IEF_1)
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
#[doc = "Values that can be written to the field `IER`"]
pub enum IERW {
    #[doc = "Interrupt is disabled."]
    IER_0,
    #[doc = "Interrupt is enabled."]
    IER_1,
}
impl IERW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            IERW::IER_0 => false,
            IERW::IER_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _IERW<'a> {
    w: &'a mut W,
}
impl<'a> _IERW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: IERW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Interrupt is disabled."]
    #[inline]
    pub fn ier_0(self) -> &'a mut W {
        self.variant(IERW::IER_0)
    }
    #[doc = "Interrupt is enabled."]
    #[inline]
    pub fn ier_1(self) -> &'a mut W {
        self.variant(IERW::IER_1)
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
#[doc = "Values that can be written to the field `DMAEN`"]
pub enum DMAENW {
    #[doc = "DMA is disabled."]
    DMAEN_0,
    #[doc = "DMA is enabled."]
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
    #[doc = "DMA is disabled."]
    #[inline]
    pub fn dmaen_0(self) -> &'a mut W {
        self.variant(DMAENW::DMAEN_0)
    }
    #[doc = "DMA is enabled."]
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
        const OFFSET: u8 = 6;
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
    #[doc = "Bit 0 - Analog Comparator Output"]
    #[inline]
    pub fn cout(&self) -> COUTR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        };
        COUTR { bits }
    }
    #[doc = "Bit 1 - Analog Comparator Flag Falling"]
    #[inline]
    pub fn cff(&self) -> CFFR {
        CFFR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        })
    }
    #[doc = "Bit 2 - Analog Comparator Flag Rising"]
    #[inline]
    pub fn cfr(&self) -> CFRR {
        CFRR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        })
    }
    #[doc = "Bit 3 - Comparator Interrupt Enable Falling"]
    #[inline]
    pub fn ief(&self) -> IEFR {
        IEFR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        })
    }
    #[doc = "Bit 4 - Comparator Interrupt Enable Rising"]
    #[inline]
    pub fn ier(&self) -> IERR {
        IERR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        })
    }
    #[doc = "Bit 6 - DMA Enable Control"]
    #[inline]
    pub fn dmaen(&self) -> DMAENR {
        DMAENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
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
    #[doc = "Bit 1 - Analog Comparator Flag Falling"]
    #[inline]
    pub fn cff(&mut self) -> _CFFW {
        _CFFW { w: self }
    }
    #[doc = "Bit 2 - Analog Comparator Flag Rising"]
    #[inline]
    pub fn cfr(&mut self) -> _CFRW {
        _CFRW { w: self }
    }
    #[doc = "Bit 3 - Comparator Interrupt Enable Falling"]
    #[inline]
    pub fn ief(&mut self) -> _IEFW {
        _IEFW { w: self }
    }
    #[doc = "Bit 4 - Comparator Interrupt Enable Rising"]
    #[inline]
    pub fn ier(&mut self) -> _IERW {
        _IERW { w: self }
    }
    #[doc = "Bit 6 - DMA Enable Control"]
    #[inline]
    pub fn dmaen(&mut self) -> _DMAENW {
        _DMAENW { w: self }
    }
}
