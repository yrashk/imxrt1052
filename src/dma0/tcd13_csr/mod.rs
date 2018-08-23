#[doc = r" Value read from the register"]
pub struct R {
    bits: u16,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u16,
}
impl super::TCD13_CSR {
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
#[doc = "Possible values of the field `START`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum STARTR {
    #[doc = "The channel is not explicitly started."]
    START_0,
    #[doc = "The channel is explicitly started via a software initiated service request."]
    START_1,
}
impl STARTR {
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
            STARTR::START_0 => false,
            STARTR::START_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> STARTR {
        match value {
            false => STARTR::START_0,
            true => STARTR::START_1,
        }
    }
    #[doc = "Checks if the value of the field is `START_0`"]
    #[inline]
    pub fn is_start_0(&self) -> bool {
        *self == STARTR::START_0
    }
    #[doc = "Checks if the value of the field is `START_1`"]
    #[inline]
    pub fn is_start_1(&self) -> bool {
        *self == STARTR::START_1
    }
}
#[doc = "Possible values of the field `INTMAJOR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INTMAJORR {
    #[doc = "The end-of-major loop interrupt is disabled."]
    INTMAJOR_0,
    #[doc = "The end-of-major loop interrupt is enabled."]
    INTMAJOR_1,
}
impl INTMAJORR {
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
            INTMAJORR::INTMAJOR_0 => false,
            INTMAJORR::INTMAJOR_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> INTMAJORR {
        match value {
            false => INTMAJORR::INTMAJOR_0,
            true => INTMAJORR::INTMAJOR_1,
        }
    }
    #[doc = "Checks if the value of the field is `INTMAJOR_0`"]
    #[inline]
    pub fn is_intmajor_0(&self) -> bool {
        *self == INTMAJORR::INTMAJOR_0
    }
    #[doc = "Checks if the value of the field is `INTMAJOR_1`"]
    #[inline]
    pub fn is_intmajor_1(&self) -> bool {
        *self == INTMAJORR::INTMAJOR_1
    }
}
#[doc = "Possible values of the field `INTHALF`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INTHALFR {
    #[doc = "The half-point interrupt is disabled."]
    INTHALF_0,
    #[doc = "The half-point interrupt is enabled."]
    INTHALF_1,
}
impl INTHALFR {
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
            INTHALFR::INTHALF_0 => false,
            INTHALFR::INTHALF_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> INTHALFR {
        match value {
            false => INTHALFR::INTHALF_0,
            true => INTHALFR::INTHALF_1,
        }
    }
    #[doc = "Checks if the value of the field is `INTHALF_0`"]
    #[inline]
    pub fn is_inthalf_0(&self) -> bool {
        *self == INTHALFR::INTHALF_0
    }
    #[doc = "Checks if the value of the field is `INTHALF_1`"]
    #[inline]
    pub fn is_inthalf_1(&self) -> bool {
        *self == INTHALFR::INTHALF_1
    }
}
#[doc = "Possible values of the field `DREQ`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DREQR {
    #[doc = "no description available"]
    DREQ_0,
    #[doc = "no description available"]
    DREQ_1,
}
impl DREQR {
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
            DREQR::DREQ_0 => false,
            DREQR::DREQ_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DREQR {
        match value {
            false => DREQR::DREQ_0,
            true => DREQR::DREQ_1,
        }
    }
    #[doc = "Checks if the value of the field is `DREQ_0`"]
    #[inline]
    pub fn is_dreq_0(&self) -> bool {
        *self == DREQR::DREQ_0
    }
    #[doc = "Checks if the value of the field is `DREQ_1`"]
    #[inline]
    pub fn is_dreq_1(&self) -> bool {
        *self == DREQR::DREQ_1
    }
}
#[doc = "Possible values of the field `ESG`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ESGR {
    #[doc = "The current channel's TCD is normal format."]
    ESG_0,
    #[doc = "The current channel's TCD specifies a scatter gather format. The DLASTSGA field provides a memory pointer to the next TCD to be loaded into this channel after the major loop completes its execution."]
    ESG_1,
}
impl ESGR {
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
            ESGR::ESG_0 => false,
            ESGR::ESG_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ESGR {
        match value {
            false => ESGR::ESG_0,
            true => ESGR::ESG_1,
        }
    }
    #[doc = "Checks if the value of the field is `ESG_0`"]
    #[inline]
    pub fn is_esg_0(&self) -> bool {
        *self == ESGR::ESG_0
    }
    #[doc = "Checks if the value of the field is `ESG_1`"]
    #[inline]
    pub fn is_esg_1(&self) -> bool {
        *self == ESGR::ESG_1
    }
}
#[doc = "Possible values of the field `MAJORELINK`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MAJORELINKR {
    #[doc = "The channel-to-channel linking is disabled."]
    MAJORELINK_0,
    #[doc = "The channel-to-channel linking is enabled."]
    MAJORELINK_1,
}
impl MAJORELINKR {
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
            MAJORELINKR::MAJORELINK_0 => false,
            MAJORELINKR::MAJORELINK_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> MAJORELINKR {
        match value {
            false => MAJORELINKR::MAJORELINK_0,
            true => MAJORELINKR::MAJORELINK_1,
        }
    }
    #[doc = "Checks if the value of the field is `MAJORELINK_0`"]
    #[inline]
    pub fn is_majorelink_0(&self) -> bool {
        *self == MAJORELINKR::MAJORELINK_0
    }
    #[doc = "Checks if the value of the field is `MAJORELINK_1`"]
    #[inline]
    pub fn is_majorelink_1(&self) -> bool {
        *self == MAJORELINKR::MAJORELINK_1
    }
}
#[doc = r" Value of the field"]
pub struct ACTIVER {
    bits: bool,
}
impl ACTIVER {
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
pub struct DONER {
    bits: bool,
}
impl DONER {
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
pub struct MAJORLINKCHR {
    bits: u8,
}
impl MAJORLINKCHR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Possible values of the field `BWC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BWCR {
    #[doc = "No eDMA engine stalls."]
    BWC_0,
    #[doc = "eDMA engine stalls for 4 cycles after each R/W."]
    BWC_2,
    #[doc = "eDMA engine stalls for 8 cycles after each R/W."]
    BWC_3,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl BWCR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            BWCR::BWC_0 => 0,
            BWCR::BWC_2 => 2,
            BWCR::BWC_3 => 3,
            BWCR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> BWCR {
        match value {
            0 => BWCR::BWC_0,
            2 => BWCR::BWC_2,
            3 => BWCR::BWC_3,
            i => BWCR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `BWC_0`"]
    #[inline]
    pub fn is_bwc_0(&self) -> bool {
        *self == BWCR::BWC_0
    }
    #[doc = "Checks if the value of the field is `BWC_2`"]
    #[inline]
    pub fn is_bwc_2(&self) -> bool {
        *self == BWCR::BWC_2
    }
    #[doc = "Checks if the value of the field is `BWC_3`"]
    #[inline]
    pub fn is_bwc_3(&self) -> bool {
        *self == BWCR::BWC_3
    }
}
#[doc = "Values that can be written to the field `START`"]
pub enum STARTW {
    #[doc = "The channel is not explicitly started."]
    START_0,
    #[doc = "The channel is explicitly started via a software initiated service request."]
    START_1,
}
impl STARTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            STARTW::START_0 => false,
            STARTW::START_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _STARTW<'a> {
    w: &'a mut W,
}
impl<'a> _STARTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: STARTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The channel is not explicitly started."]
    #[inline]
    pub fn start_0(self) -> &'a mut W {
        self.variant(STARTW::START_0)
    }
    #[doc = "The channel is explicitly started via a software initiated service request."]
    #[inline]
    pub fn start_1(self) -> &'a mut W {
        self.variant(STARTW::START_1)
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
        self.w.bits &= !((MASK as u16) << OFFSET);
        self.w.bits |= ((value & MASK) as u16) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `INTMAJOR`"]
pub enum INTMAJORW {
    #[doc = "The end-of-major loop interrupt is disabled."]
    INTMAJOR_0,
    #[doc = "The end-of-major loop interrupt is enabled."]
    INTMAJOR_1,
}
impl INTMAJORW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            INTMAJORW::INTMAJOR_0 => false,
            INTMAJORW::INTMAJOR_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _INTMAJORW<'a> {
    w: &'a mut W,
}
impl<'a> _INTMAJORW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: INTMAJORW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The end-of-major loop interrupt is disabled."]
    #[inline]
    pub fn intmajor_0(self) -> &'a mut W {
        self.variant(INTMAJORW::INTMAJOR_0)
    }
    #[doc = "The end-of-major loop interrupt is enabled."]
    #[inline]
    pub fn intmajor_1(self) -> &'a mut W {
        self.variant(INTMAJORW::INTMAJOR_1)
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
        self.w.bits &= !((MASK as u16) << OFFSET);
        self.w.bits |= ((value & MASK) as u16) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `INTHALF`"]
pub enum INTHALFW {
    #[doc = "The half-point interrupt is disabled."]
    INTHALF_0,
    #[doc = "The half-point interrupt is enabled."]
    INTHALF_1,
}
impl INTHALFW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            INTHALFW::INTHALF_0 => false,
            INTHALFW::INTHALF_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _INTHALFW<'a> {
    w: &'a mut W,
}
impl<'a> _INTHALFW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: INTHALFW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The half-point interrupt is disabled."]
    #[inline]
    pub fn inthalf_0(self) -> &'a mut W {
        self.variant(INTHALFW::INTHALF_0)
    }
    #[doc = "The half-point interrupt is enabled."]
    #[inline]
    pub fn inthalf_1(self) -> &'a mut W {
        self.variant(INTHALFW::INTHALF_1)
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
        self.w.bits &= !((MASK as u16) << OFFSET);
        self.w.bits |= ((value & MASK) as u16) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `DREQ`"]
pub enum DREQW {
    #[doc = "no description available"]
    DREQ_0,
    #[doc = "no description available"]
    DREQ_1,
}
impl DREQW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DREQW::DREQ_0 => false,
            DREQW::DREQ_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DREQW<'a> {
    w: &'a mut W,
}
impl<'a> _DREQW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DREQW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "no description available"]
    #[inline]
    pub fn dreq_0(self) -> &'a mut W {
        self.variant(DREQW::DREQ_0)
    }
    #[doc = "no description available"]
    #[inline]
    pub fn dreq_1(self) -> &'a mut W {
        self.variant(DREQW::DREQ_1)
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
        self.w.bits &= !((MASK as u16) << OFFSET);
        self.w.bits |= ((value & MASK) as u16) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `ESG`"]
pub enum ESGW {
    #[doc = "The current channel's TCD is normal format."]
    ESG_0,
    #[doc = "The current channel's TCD specifies a scatter gather format. The DLASTSGA field provides a memory pointer to the next TCD to be loaded into this channel after the major loop completes its execution."]
    ESG_1,
}
impl ESGW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ESGW::ESG_0 => false,
            ESGW::ESG_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ESGW<'a> {
    w: &'a mut W,
}
impl<'a> _ESGW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ESGW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The current channel's TCD is normal format."]
    #[inline]
    pub fn esg_0(self) -> &'a mut W {
        self.variant(ESGW::ESG_0)
    }
    #[doc = "The current channel's TCD specifies a scatter gather format. The DLASTSGA field provides a memory pointer to the next TCD to be loaded into this channel after the major loop completes its execution."]
    #[inline]
    pub fn esg_1(self) -> &'a mut W {
        self.variant(ESGW::ESG_1)
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
        self.w.bits &= !((MASK as u16) << OFFSET);
        self.w.bits |= ((value & MASK) as u16) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `MAJORELINK`"]
pub enum MAJORELINKW {
    #[doc = "The channel-to-channel linking is disabled."]
    MAJORELINK_0,
    #[doc = "The channel-to-channel linking is enabled."]
    MAJORELINK_1,
}
impl MAJORELINKW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            MAJORELINKW::MAJORELINK_0 => false,
            MAJORELINKW::MAJORELINK_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MAJORELINKW<'a> {
    w: &'a mut W,
}
impl<'a> _MAJORELINKW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MAJORELINKW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The channel-to-channel linking is disabled."]
    #[inline]
    pub fn majorelink_0(self) -> &'a mut W {
        self.variant(MAJORELINKW::MAJORELINK_0)
    }
    #[doc = "The channel-to-channel linking is enabled."]
    #[inline]
    pub fn majorelink_1(self) -> &'a mut W {
        self.variant(MAJORELINKW::MAJORELINK_1)
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
        self.w.bits &= !((MASK as u16) << OFFSET);
        self.w.bits |= ((value & MASK) as u16) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _DONEW<'a> {
    w: &'a mut W,
}
impl<'a> _DONEW<'a> {
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
        self.w.bits &= !((MASK as u16) << OFFSET);
        self.w.bits |= ((value & MASK) as u16) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _MAJORLINKCHW<'a> {
    w: &'a mut W,
}
impl<'a> _MAJORLINKCHW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 31;
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u16) << OFFSET);
        self.w.bits |= ((value & MASK) as u16) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `BWC`"]
pub enum BWCW {
    #[doc = "No eDMA engine stalls."]
    BWC_0,
    #[doc = "eDMA engine stalls for 4 cycles after each R/W."]
    BWC_2,
    #[doc = "eDMA engine stalls for 8 cycles after each R/W."]
    BWC_3,
}
impl BWCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            BWCW::BWC_0 => 0,
            BWCW::BWC_2 => 2,
            BWCW::BWC_3 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _BWCW<'a> {
    w: &'a mut W,
}
impl<'a> _BWCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: BWCW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "No eDMA engine stalls."]
    #[inline]
    pub fn bwc_0(self) -> &'a mut W {
        self.variant(BWCW::BWC_0)
    }
    #[doc = "eDMA engine stalls for 4 cycles after each R/W."]
    #[inline]
    pub fn bwc_2(self) -> &'a mut W {
        self.variant(BWCW::BWC_2)
    }
    #[doc = "eDMA engine stalls for 8 cycles after each R/W."]
    #[inline]
    pub fn bwc_3(self) -> &'a mut W {
        self.variant(BWCW::BWC_3)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 14;
        self.w.bits &= !((MASK as u16) << OFFSET);
        self.w.bits |= ((value & MASK) as u16) << OFFSET;
        self.w
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }
    #[doc = "Bit 0 - Channel Start"]
    #[inline]
    pub fn start(&self) -> STARTR {
        STARTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        })
    }
    #[doc = "Bit 1 - Enable an interrupt when major iteration count completes."]
    #[inline]
    pub fn intmajor(&self) -> INTMAJORR {
        INTMAJORR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        })
    }
    #[doc = "Bit 2 - Enable an interrupt when major counter is half complete."]
    #[inline]
    pub fn inthalf(&self) -> INTHALFR {
        INTHALFR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        })
    }
    #[doc = "Bit 3 - Disable Request"]
    #[inline]
    pub fn dreq(&self) -> DREQR {
        DREQR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        })
    }
    #[doc = "Bit 4 - Enable Scatter/Gather Processing"]
    #[inline]
    pub fn esg(&self) -> ESGR {
        ESGR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        })
    }
    #[doc = "Bit 5 - Enable channel-to-channel linking on major loop complete"]
    #[inline]
    pub fn majorelink(&self) -> MAJORELINKR {
        MAJORELINKR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        })
    }
    #[doc = "Bit 6 - Channel Active"]
    #[inline]
    pub fn active(&self) -> ACTIVER {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        };
        ACTIVER { bits }
    }
    #[doc = "Bit 7 - Channel Done"]
    #[inline]
    pub fn done(&self) -> DONER {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        };
        DONER { bits }
    }
    #[doc = "Bits 8:12 - Major Loop Link Channel Number"]
    #[inline]
    pub fn majorlinkch(&self) -> MAJORLINKCHR {
        let bits = {
            const MASK: u8 = 31;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u16) as u8
        };
        MAJORLINKCHR { bits }
    }
    #[doc = "Bits 14:15 - Bandwidth Control"]
    #[inline]
    pub fn bwc(&self) -> BWCR {
        BWCR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u16) as u8
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
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - Channel Start"]
    #[inline]
    pub fn start(&mut self) -> _STARTW {
        _STARTW { w: self }
    }
    #[doc = "Bit 1 - Enable an interrupt when major iteration count completes."]
    #[inline]
    pub fn intmajor(&mut self) -> _INTMAJORW {
        _INTMAJORW { w: self }
    }
    #[doc = "Bit 2 - Enable an interrupt when major counter is half complete."]
    #[inline]
    pub fn inthalf(&mut self) -> _INTHALFW {
        _INTHALFW { w: self }
    }
    #[doc = "Bit 3 - Disable Request"]
    #[inline]
    pub fn dreq(&mut self) -> _DREQW {
        _DREQW { w: self }
    }
    #[doc = "Bit 4 - Enable Scatter/Gather Processing"]
    #[inline]
    pub fn esg(&mut self) -> _ESGW {
        _ESGW { w: self }
    }
    #[doc = "Bit 5 - Enable channel-to-channel linking on major loop complete"]
    #[inline]
    pub fn majorelink(&mut self) -> _MAJORELINKW {
        _MAJORELINKW { w: self }
    }
    #[doc = "Bit 7 - Channel Done"]
    #[inline]
    pub fn done(&mut self) -> _DONEW {
        _DONEW { w: self }
    }
    #[doc = "Bits 8:12 - Major Loop Link Channel Number"]
    #[inline]
    pub fn majorlinkch(&mut self) -> _MAJORLINKCHW {
        _MAJORLINKCHW { w: self }
    }
    #[doc = "Bits 14:15 - Bandwidth Control"]
    #[inline]
    pub fn bwc(&mut self) -> _BWCW {
        _BWCW { w: self }
    }
}
