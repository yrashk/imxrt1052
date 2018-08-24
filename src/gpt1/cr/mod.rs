#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CR {
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
    #[doc = "GPT is disabled."]
    EN_0,
    #[doc = "GPT is enabled."]
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
#[doc = "Possible values of the field `ENMOD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ENMODR {
    #[doc = "GPT counter will retain its value when it is disabled."]
    ENMOD_0,
    #[doc = "GPT counter value is reset to 0 when it is disabled."]
    ENMOD_1,
}
impl ENMODR {
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
            ENMODR::ENMOD_0 => false,
            ENMODR::ENMOD_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ENMODR {
        match value {
            false => ENMODR::ENMOD_0,
            true => ENMODR::ENMOD_1,
        }
    }
    #[doc = "Checks if the value of the field is `ENMOD_0`"]
    #[inline]
    pub fn is_enmod_0(&self) -> bool {
        *self == ENMODR::ENMOD_0
    }
    #[doc = "Checks if the value of the field is `ENMOD_1`"]
    #[inline]
    pub fn is_enmod_1(&self) -> bool {
        *self == ENMODR::ENMOD_1
    }
}
#[doc = "Possible values of the field `DBGEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DBGENR {
    #[doc = "GPT is disabled in debug mode."]
    DBGEN_0,
    #[doc = "GPT is enabled in debug mode."]
    DBGEN_1,
}
impl DBGENR {
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
            DBGENR::DBGEN_0 => false,
            DBGENR::DBGEN_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DBGENR {
        match value {
            false => DBGENR::DBGEN_0,
            true => DBGENR::DBGEN_1,
        }
    }
    #[doc = "Checks if the value of the field is `DBGEN_0`"]
    #[inline]
    pub fn is_dbgen_0(&self) -> bool {
        *self == DBGENR::DBGEN_0
    }
    #[doc = "Checks if the value of the field is `DBGEN_1`"]
    #[inline]
    pub fn is_dbgen_1(&self) -> bool {
        *self == DBGENR::DBGEN_1
    }
}
#[doc = "Possible values of the field `WAITEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WAITENR {
    #[doc = "GPT is disabled in wait mode."]
    WAITEN_0,
    #[doc = "GPT is enabled in wait mode."]
    WAITEN_1,
}
impl WAITENR {
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
            WAITENR::WAITEN_0 => false,
            WAITENR::WAITEN_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> WAITENR {
        match value {
            false => WAITENR::WAITEN_0,
            true => WAITENR::WAITEN_1,
        }
    }
    #[doc = "Checks if the value of the field is `WAITEN_0`"]
    #[inline]
    pub fn is_waiten_0(&self) -> bool {
        *self == WAITENR::WAITEN_0
    }
    #[doc = "Checks if the value of the field is `WAITEN_1`"]
    #[inline]
    pub fn is_waiten_1(&self) -> bool {
        *self == WAITENR::WAITEN_1
    }
}
#[doc = "Possible values of the field `DOZEEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DOZEENR {
    #[doc = "GPT is disabled in doze mode."]
    DOZEEN_0,
    #[doc = "GPT is enabled in doze mode."]
    DOZEEN_1,
}
impl DOZEENR {
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
            DOZEENR::DOZEEN_0 => false,
            DOZEENR::DOZEEN_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DOZEENR {
        match value {
            false => DOZEENR::DOZEEN_0,
            true => DOZEENR::DOZEEN_1,
        }
    }
    #[doc = "Checks if the value of the field is `DOZEEN_0`"]
    #[inline]
    pub fn is_dozeen_0(&self) -> bool {
        *self == DOZEENR::DOZEEN_0
    }
    #[doc = "Checks if the value of the field is `DOZEEN_1`"]
    #[inline]
    pub fn is_dozeen_1(&self) -> bool {
        *self == DOZEENR::DOZEEN_1
    }
}
#[doc = "Possible values of the field `STOPEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum STOPENR {
    #[doc = "GPT is disabled in Stop mode."]
    STOPEN_0,
    #[doc = "GPT is enabled in Stop mode."]
    STOPEN_1,
}
impl STOPENR {
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
            STOPENR::STOPEN_0 => false,
            STOPENR::STOPEN_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> STOPENR {
        match value {
            false => STOPENR::STOPEN_0,
            true => STOPENR::STOPEN_1,
        }
    }
    #[doc = "Checks if the value of the field is `STOPEN_0`"]
    #[inline]
    pub fn is_stopen_0(&self) -> bool {
        *self == STOPENR::STOPEN_0
    }
    #[doc = "Checks if the value of the field is `STOPEN_1`"]
    #[inline]
    pub fn is_stopen_1(&self) -> bool {
        *self == STOPENR::STOPEN_1
    }
}
#[doc = "Possible values of the field `CLKSRC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CLKSRCR {
    #[doc = "No clock"]
    CLKSRC_0,
    #[doc = "Peripheral Clock (ipg_clk)"]
    CLKSRC_1,
    #[doc = "High Frequency Reference Clock (ipg_clk_highfreq)"]
    CLKSRC_2,
    #[doc = "External Clock"]
    CLKSRC_3,
    #[doc = "Low Frequency Reference Clock (ipg_clk_32k)"]
    CLKSRC_4,
    #[doc = "Crystal oscillator as Reference Clock (ipg_clk_24M)"]
    CLKSRC_5,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl CLKSRCR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            CLKSRCR::CLKSRC_0 => 0,
            CLKSRCR::CLKSRC_1 => 1,
            CLKSRCR::CLKSRC_2 => 2,
            CLKSRCR::CLKSRC_3 => 3,
            CLKSRCR::CLKSRC_4 => 4,
            CLKSRCR::CLKSRC_5 => 5,
            CLKSRCR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> CLKSRCR {
        match value {
            0 => CLKSRCR::CLKSRC_0,
            1 => CLKSRCR::CLKSRC_1,
            2 => CLKSRCR::CLKSRC_2,
            3 => CLKSRCR::CLKSRC_3,
            4 => CLKSRCR::CLKSRC_4,
            5 => CLKSRCR::CLKSRC_5,
            i => CLKSRCR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `CLKSRC_0`"]
    #[inline]
    pub fn is_clksrc_0(&self) -> bool {
        *self == CLKSRCR::CLKSRC_0
    }
    #[doc = "Checks if the value of the field is `CLKSRC_1`"]
    #[inline]
    pub fn is_clksrc_1(&self) -> bool {
        *self == CLKSRCR::CLKSRC_1
    }
    #[doc = "Checks if the value of the field is `CLKSRC_2`"]
    #[inline]
    pub fn is_clksrc_2(&self) -> bool {
        *self == CLKSRCR::CLKSRC_2
    }
    #[doc = "Checks if the value of the field is `CLKSRC_3`"]
    #[inline]
    pub fn is_clksrc_3(&self) -> bool {
        *self == CLKSRCR::CLKSRC_3
    }
    #[doc = "Checks if the value of the field is `CLKSRC_4`"]
    #[inline]
    pub fn is_clksrc_4(&self) -> bool {
        *self == CLKSRCR::CLKSRC_4
    }
    #[doc = "Checks if the value of the field is `CLKSRC_5`"]
    #[inline]
    pub fn is_clksrc_5(&self) -> bool {
        *self == CLKSRCR::CLKSRC_5
    }
}
#[doc = "Possible values of the field `FRR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FRRR {
    #[doc = "Restart mode"]
    FRR_0,
    #[doc = "Free-Run mode"]
    FRR_1,
}
impl FRRR {
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
            FRRR::FRR_0 => false,
            FRRR::FRR_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> FRRR {
        match value {
            false => FRRR::FRR_0,
            true => FRRR::FRR_1,
        }
    }
    #[doc = "Checks if the value of the field is `FRR_0`"]
    #[inline]
    pub fn is_frr_0(&self) -> bool {
        *self == FRRR::FRR_0
    }
    #[doc = "Checks if the value of the field is `FRR_1`"]
    #[inline]
    pub fn is_frr_1(&self) -> bool {
        *self == FRRR::FRR_1
    }
}
#[doc = "Possible values of the field `EN_24M`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EN_24MR {
    #[doc = "24M clock disabled"]
    EN_24M_0,
    #[doc = "24M clock enabled"]
    EN_24M_1,
}
impl EN_24MR {
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
            EN_24MR::EN_24M_0 => false,
            EN_24MR::EN_24M_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> EN_24MR {
        match value {
            false => EN_24MR::EN_24M_0,
            true => EN_24MR::EN_24M_1,
        }
    }
    #[doc = "Checks if the value of the field is `EN_24M_0`"]
    #[inline]
    pub fn is_en_24m_0(&self) -> bool {
        *self == EN_24MR::EN_24M_0
    }
    #[doc = "Checks if the value of the field is `EN_24M_1`"]
    #[inline]
    pub fn is_en_24m_1(&self) -> bool {
        *self == EN_24MR::EN_24M_1
    }
}
#[doc = "Possible values of the field `SWR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SWRR {
    #[doc = "GPT is not in reset state"]
    SWR_0,
    #[doc = "GPT is in reset state"]
    SWR_1,
}
impl SWRR {
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
            SWRR::SWR_0 => false,
            SWRR::SWR_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SWRR {
        match value {
            false => SWRR::SWR_0,
            true => SWRR::SWR_1,
        }
    }
    #[doc = "Checks if the value of the field is `SWR_0`"]
    #[inline]
    pub fn is_swr_0(&self) -> bool {
        *self == SWRR::SWR_0
    }
    #[doc = "Checks if the value of the field is `SWR_1`"]
    #[inline]
    pub fn is_swr_1(&self) -> bool {
        *self == SWRR::SWR_1
    }
}
#[doc = r" Value of the field"]
pub struct IM1R {
    bits: u8,
}
impl IM1R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Possible values of the field `IM2`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IM2R {
    #[doc = "capture disabled"]
    IM2_0,
    #[doc = "capture on rising edge only"]
    IM2_1,
    #[doc = "capture on falling edge only"]
    IM2_2,
    #[doc = "capture on both edges"]
    IM2_3,
}
impl IM2R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            IM2R::IM2_0 => 0,
            IM2R::IM2_1 => 1,
            IM2R::IM2_2 => 2,
            IM2R::IM2_3 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> IM2R {
        match value {
            0 => IM2R::IM2_0,
            1 => IM2R::IM2_1,
            2 => IM2R::IM2_2,
            3 => IM2R::IM2_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `IM2_0`"]
    #[inline]
    pub fn is_im2_0(&self) -> bool {
        *self == IM2R::IM2_0
    }
    #[doc = "Checks if the value of the field is `IM2_1`"]
    #[inline]
    pub fn is_im2_1(&self) -> bool {
        *self == IM2R::IM2_1
    }
    #[doc = "Checks if the value of the field is `IM2_2`"]
    #[inline]
    pub fn is_im2_2(&self) -> bool {
        *self == IM2R::IM2_2
    }
    #[doc = "Checks if the value of the field is `IM2_3`"]
    #[inline]
    pub fn is_im2_3(&self) -> bool {
        *self == IM2R::IM2_3
    }
}
#[doc = r" Value of the field"]
pub struct OM1R {
    bits: u8,
}
impl OM1R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct OM2R {
    bits: u8,
}
impl OM2R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Possible values of the field `OM3`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OM3R {
    #[doc = "Output disconnected. No response on pin."]
    OM3_0,
    #[doc = "Toggle output pin"]
    OM3_1,
    #[doc = "Clear output pin"]
    OM3_2,
    #[doc = "Set output pin"]
    OM3_3,
    #[doc = "Generate an active low pulse (that is one input clock wide) on the output pin."]
    OM3_4,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl OM3R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            OM3R::OM3_0 => 0,
            OM3R::OM3_1 => 1,
            OM3R::OM3_2 => 2,
            OM3R::OM3_3 => 3,
            OM3R::OM3_4 => 4,
            OM3R::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> OM3R {
        match value {
            0 => OM3R::OM3_0,
            1 => OM3R::OM3_1,
            2 => OM3R::OM3_2,
            3 => OM3R::OM3_3,
            4 => OM3R::OM3_4,
            i => OM3R::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `OM3_0`"]
    #[inline]
    pub fn is_om3_0(&self) -> bool {
        *self == OM3R::OM3_0
    }
    #[doc = "Checks if the value of the field is `OM3_1`"]
    #[inline]
    pub fn is_om3_1(&self) -> bool {
        *self == OM3R::OM3_1
    }
    #[doc = "Checks if the value of the field is `OM3_2`"]
    #[inline]
    pub fn is_om3_2(&self) -> bool {
        *self == OM3R::OM3_2
    }
    #[doc = "Checks if the value of the field is `OM3_3`"]
    #[inline]
    pub fn is_om3_3(&self) -> bool {
        *self == OM3R::OM3_3
    }
    #[doc = "Checks if the value of the field is `OM3_4`"]
    #[inline]
    pub fn is_om3_4(&self) -> bool {
        *self == OM3R::OM3_4
    }
}
#[doc = "Values that can be written to the field `EN`"]
pub enum ENW {
    #[doc = "GPT is disabled."]
    EN_0,
    #[doc = "GPT is enabled."]
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
    #[doc = "GPT is disabled."]
    #[inline]
    pub fn en_0(self) -> &'a mut W {
        self.variant(ENW::EN_0)
    }
    #[doc = "GPT is enabled."]
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
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `ENMOD`"]
pub enum ENMODW {
    #[doc = "GPT counter will retain its value when it is disabled."]
    ENMOD_0,
    #[doc = "GPT counter value is reset to 0 when it is disabled."]
    ENMOD_1,
}
impl ENMODW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ENMODW::ENMOD_0 => false,
            ENMODW::ENMOD_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ENMODW<'a> {
    w: &'a mut W,
}
impl<'a> _ENMODW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ENMODW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "GPT counter will retain its value when it is disabled."]
    #[inline]
    pub fn enmod_0(self) -> &'a mut W {
        self.variant(ENMODW::ENMOD_0)
    }
    #[doc = "GPT counter value is reset to 0 when it is disabled."]
    #[inline]
    pub fn enmod_1(self) -> &'a mut W {
        self.variant(ENMODW::ENMOD_1)
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
#[doc = "Values that can be written to the field `DBGEN`"]
pub enum DBGENW {
    #[doc = "GPT is disabled in debug mode."]
    DBGEN_0,
    #[doc = "GPT is enabled in debug mode."]
    DBGEN_1,
}
impl DBGENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DBGENW::DBGEN_0 => false,
            DBGENW::DBGEN_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DBGENW<'a> {
    w: &'a mut W,
}
impl<'a> _DBGENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DBGENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "GPT is disabled in debug mode."]
    #[inline]
    pub fn dbgen_0(self) -> &'a mut W {
        self.variant(DBGENW::DBGEN_0)
    }
    #[doc = "GPT is enabled in debug mode."]
    #[inline]
    pub fn dbgen_1(self) -> &'a mut W {
        self.variant(DBGENW::DBGEN_1)
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
#[doc = "Values that can be written to the field `WAITEN`"]
pub enum WAITENW {
    #[doc = "GPT is disabled in wait mode."]
    WAITEN_0,
    #[doc = "GPT is enabled in wait mode."]
    WAITEN_1,
}
impl WAITENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            WAITENW::WAITEN_0 => false,
            WAITENW::WAITEN_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _WAITENW<'a> {
    w: &'a mut W,
}
impl<'a> _WAITENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: WAITENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "GPT is disabled in wait mode."]
    #[inline]
    pub fn waiten_0(self) -> &'a mut W {
        self.variant(WAITENW::WAITEN_0)
    }
    #[doc = "GPT is enabled in wait mode."]
    #[inline]
    pub fn waiten_1(self) -> &'a mut W {
        self.variant(WAITENW::WAITEN_1)
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
#[doc = "Values that can be written to the field `DOZEEN`"]
pub enum DOZEENW {
    #[doc = "GPT is disabled in doze mode."]
    DOZEEN_0,
    #[doc = "GPT is enabled in doze mode."]
    DOZEEN_1,
}
impl DOZEENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DOZEENW::DOZEEN_0 => false,
            DOZEENW::DOZEEN_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DOZEENW<'a> {
    w: &'a mut W,
}
impl<'a> _DOZEENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DOZEENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "GPT is disabled in doze mode."]
    #[inline]
    pub fn dozeen_0(self) -> &'a mut W {
        self.variant(DOZEENW::DOZEEN_0)
    }
    #[doc = "GPT is enabled in doze mode."]
    #[inline]
    pub fn dozeen_1(self) -> &'a mut W {
        self.variant(DOZEENW::DOZEEN_1)
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
#[doc = "Values that can be written to the field `STOPEN`"]
pub enum STOPENW {
    #[doc = "GPT is disabled in Stop mode."]
    STOPEN_0,
    #[doc = "GPT is enabled in Stop mode."]
    STOPEN_1,
}
impl STOPENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            STOPENW::STOPEN_0 => false,
            STOPENW::STOPEN_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _STOPENW<'a> {
    w: &'a mut W,
}
impl<'a> _STOPENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: STOPENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "GPT is disabled in Stop mode."]
    #[inline]
    pub fn stopen_0(self) -> &'a mut W {
        self.variant(STOPENW::STOPEN_0)
    }
    #[doc = "GPT is enabled in Stop mode."]
    #[inline]
    pub fn stopen_1(self) -> &'a mut W {
        self.variant(STOPENW::STOPEN_1)
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
#[doc = "Values that can be written to the field `CLKSRC`"]
pub enum CLKSRCW {
    #[doc = "No clock"]
    CLKSRC_0,
    #[doc = "Peripheral Clock (ipg_clk)"]
    CLKSRC_1,
    #[doc = "High Frequency Reference Clock (ipg_clk_highfreq)"]
    CLKSRC_2,
    #[doc = "External Clock"]
    CLKSRC_3,
    #[doc = "Low Frequency Reference Clock (ipg_clk_32k)"]
    CLKSRC_4,
    #[doc = "Crystal oscillator as Reference Clock (ipg_clk_24M)"]
    CLKSRC_5,
}
impl CLKSRCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            CLKSRCW::CLKSRC_0 => 0,
            CLKSRCW::CLKSRC_1 => 1,
            CLKSRCW::CLKSRC_2 => 2,
            CLKSRCW::CLKSRC_3 => 3,
            CLKSRCW::CLKSRC_4 => 4,
            CLKSRCW::CLKSRC_5 => 5,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CLKSRCW<'a> {
    w: &'a mut W,
}
impl<'a> _CLKSRCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CLKSRCW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "No clock"]
    #[inline]
    pub fn clksrc_0(self) -> &'a mut W {
        self.variant(CLKSRCW::CLKSRC_0)
    }
    #[doc = "Peripheral Clock (ipg_clk)"]
    #[inline]
    pub fn clksrc_1(self) -> &'a mut W {
        self.variant(CLKSRCW::CLKSRC_1)
    }
    #[doc = "High Frequency Reference Clock (ipg_clk_highfreq)"]
    #[inline]
    pub fn clksrc_2(self) -> &'a mut W {
        self.variant(CLKSRCW::CLKSRC_2)
    }
    #[doc = "External Clock"]
    #[inline]
    pub fn clksrc_3(self) -> &'a mut W {
        self.variant(CLKSRCW::CLKSRC_3)
    }
    #[doc = "Low Frequency Reference Clock (ipg_clk_32k)"]
    #[inline]
    pub fn clksrc_4(self) -> &'a mut W {
        self.variant(CLKSRCW::CLKSRC_4)
    }
    #[doc = "Crystal oscillator as Reference Clock (ipg_clk_24M)"]
    #[inline]
    pub fn clksrc_5(self) -> &'a mut W {
        self.variant(CLKSRCW::CLKSRC_5)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 6;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `FRR`"]
pub enum FRRW {
    #[doc = "Restart mode"]
    FRR_0,
    #[doc = "Free-Run mode"]
    FRR_1,
}
impl FRRW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            FRRW::FRR_0 => false,
            FRRW::FRR_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FRRW<'a> {
    w: &'a mut W,
}
impl<'a> _FRRW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FRRW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Restart mode"]
    #[inline]
    pub fn frr_0(self) -> &'a mut W {
        self.variant(FRRW::FRR_0)
    }
    #[doc = "Free-Run mode"]
    #[inline]
    pub fn frr_1(self) -> &'a mut W {
        self.variant(FRRW::FRR_1)
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
#[doc = "Values that can be written to the field `EN_24M`"]
pub enum EN_24MW {
    #[doc = "24M clock disabled"]
    EN_24M_0,
    #[doc = "24M clock enabled"]
    EN_24M_1,
}
impl EN_24MW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            EN_24MW::EN_24M_0 => false,
            EN_24MW::EN_24M_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _EN_24MW<'a> {
    w: &'a mut W,
}
impl<'a> _EN_24MW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: EN_24MW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "24M clock disabled"]
    #[inline]
    pub fn en_24m_0(self) -> &'a mut W {
        self.variant(EN_24MW::EN_24M_0)
    }
    #[doc = "24M clock enabled"]
    #[inline]
    pub fn en_24m_1(self) -> &'a mut W {
        self.variant(EN_24MW::EN_24M_1)
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
        const OFFSET: u8 = 10;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `SWR`"]
pub enum SWRW {
    #[doc = "GPT is not in reset state"]
    SWR_0,
    #[doc = "GPT is in reset state"]
    SWR_1,
}
impl SWRW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SWRW::SWR_0 => false,
            SWRW::SWR_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SWRW<'a> {
    w: &'a mut W,
}
impl<'a> _SWRW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SWRW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "GPT is not in reset state"]
    #[inline]
    pub fn swr_0(self) -> &'a mut W {
        self.variant(SWRW::SWR_0)
    }
    #[doc = "GPT is in reset state"]
    #[inline]
    pub fn swr_1(self) -> &'a mut W {
        self.variant(SWRW::SWR_1)
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
        const OFFSET: u8 = 15;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _IM1W<'a> {
    w: &'a mut W,
}
impl<'a> _IM1W<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 16;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `IM2`"]
pub enum IM2W {
    #[doc = "capture disabled"]
    IM2_0,
    #[doc = "capture on rising edge only"]
    IM2_1,
    #[doc = "capture on falling edge only"]
    IM2_2,
    #[doc = "capture on both edges"]
    IM2_3,
}
impl IM2W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            IM2W::IM2_0 => 0,
            IM2W::IM2_1 => 1,
            IM2W::IM2_2 => 2,
            IM2W::IM2_3 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _IM2W<'a> {
    w: &'a mut W,
}
impl<'a> _IM2W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: IM2W) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "capture disabled"]
    #[inline]
    pub fn im2_0(self) -> &'a mut W {
        self.variant(IM2W::IM2_0)
    }
    #[doc = "capture on rising edge only"]
    #[inline]
    pub fn im2_1(self) -> &'a mut W {
        self.variant(IM2W::IM2_1)
    }
    #[doc = "capture on falling edge only"]
    #[inline]
    pub fn im2_2(self) -> &'a mut W {
        self.variant(IM2W::IM2_2)
    }
    #[doc = "capture on both edges"]
    #[inline]
    pub fn im2_3(self) -> &'a mut W {
        self.variant(IM2W::IM2_3)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 18;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _OM1W<'a> {
    w: &'a mut W,
}
impl<'a> _OM1W<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 20;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _OM2W<'a> {
    w: &'a mut W,
}
impl<'a> _OM2W<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 23;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `OM3`"]
pub enum OM3W {
    #[doc = "Output disconnected. No response on pin."]
    OM3_0,
    #[doc = "Toggle output pin"]
    OM3_1,
    #[doc = "Clear output pin"]
    OM3_2,
    #[doc = "Set output pin"]
    OM3_3,
    #[doc = "Generate an active low pulse (that is one input clock wide) on the output pin."]
    OM3_4,
}
impl OM3W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            OM3W::OM3_0 => 0,
            OM3W::OM3_1 => 1,
            OM3W::OM3_2 => 2,
            OM3W::OM3_3 => 3,
            OM3W::OM3_4 => 4,
        }
    }
}
#[doc = r" Proxy"]
pub struct _OM3W<'a> {
    w: &'a mut W,
}
impl<'a> _OM3W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: OM3W) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Output disconnected. No response on pin."]
    #[inline]
    pub fn om3_0(self) -> &'a mut W {
        self.variant(OM3W::OM3_0)
    }
    #[doc = "Toggle output pin"]
    #[inline]
    pub fn om3_1(self) -> &'a mut W {
        self.variant(OM3W::OM3_1)
    }
    #[doc = "Clear output pin"]
    #[inline]
    pub fn om3_2(self) -> &'a mut W {
        self.variant(OM3W::OM3_2)
    }
    #[doc = "Set output pin"]
    #[inline]
    pub fn om3_3(self) -> &'a mut W {
        self.variant(OM3W::OM3_3)
    }
    #[doc = "Generate an active low pulse (that is one input clock wide) on the output pin."]
    #[inline]
    pub fn om3_4(self) -> &'a mut W {
        self.variant(OM3W::OM3_4)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 26;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _FO1W<'a> {
    w: &'a mut W,
}
impl<'a> _FO1W<'a> {
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
#[doc = r" Proxy"]
pub struct _FO2W<'a> {
    w: &'a mut W,
}
impl<'a> _FO2W<'a> {
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
        const OFFSET: u8 = 30;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `FO3`"]
pub enum FO3W {
    #[doc = "Writing a 0 has no effect."]
    FO3_0,
    #[doc = "Causes the programmed pin action on the timer Output Compare n pin; the OFn flag is not set."]
    FO3_1,
}
impl FO3W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            FO3W::FO3_0 => false,
            FO3W::FO3_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FO3W<'a> {
    w: &'a mut W,
}
impl<'a> _FO3W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FO3W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Writing a 0 has no effect."]
    #[inline]
    pub fn fo3_0(self) -> &'a mut W {
        self.variant(FO3W::FO3_0)
    }
    #[doc = "Causes the programmed pin action on the timer Output Compare n pin; the OFn flag is not set."]
    #[inline]
    pub fn fo3_1(self) -> &'a mut W {
        self.variant(FO3W::FO3_1)
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
        const OFFSET: u8 = 31;
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
    #[doc = "Bit 0 - GPT Enable"]
    #[inline]
    pub fn en(&self) -> ENR {
        ENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - GPT Enable mode"]
    #[inline]
    pub fn enmod(&self) -> ENMODR {
        ENMODR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - GPT debug mode enable"]
    #[inline]
    pub fn dbgen(&self) -> DBGENR {
        DBGENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - GPT Wait Mode enable"]
    #[inline]
    pub fn waiten(&self) -> WAITENR {
        WAITENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 4 - GPT Doze Mode Enable"]
    #[inline]
    pub fn dozeen(&self) -> DOZEENR {
        DOZEENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 5 - GPT Stop Mode enable"]
    #[inline]
    pub fn stopen(&self) -> STOPENR {
        STOPENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 6:8 - Clock Source select"]
    #[inline]
    pub fn clksrc(&self) -> CLKSRCR {
        CLKSRCR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 9 - Free-Run or Restart mode"]
    #[inline]
    pub fn frr(&self) -> FRRR {
        FRRR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 10 - Enable 24 MHz clock input from crystal"]
    #[inline]
    pub fn en_24m(&self) -> EN_24MR {
        EN_24MR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 15 - Software reset"]
    #[inline]
    pub fn swr(&self) -> SWRR {
        SWRR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 15;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 16:17 - See IM2"]
    #[inline]
    pub fn im1(&self) -> IM1R {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        IM1R { bits }
    }
    #[doc = "Bits 18:19 - IM2 (bits 19-18, Input Capture Channel 2 operating mode) IM1 (bits 17-16, Input Capture Channel 1 operating mode) The IMn bit field determines the transition on the input pin (for Input capture channel n), which will trigger a capture event"]
    #[inline]
    pub fn im2(&self) -> IM2R {
        IM2R::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 20:22 - See OM3"]
    #[inline]
    pub fn om1(&self) -> OM1R {
        let bits = {
            const MASK: u8 = 7;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        OM1R { bits }
    }
    #[doc = "Bits 23:25 - See OM3"]
    #[inline]
    pub fn om2(&self) -> OM2R {
        let bits = {
            const MASK: u8 = 7;
            const OFFSET: u8 = 23;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        OM2R { bits }
    }
    #[doc = "Bits 26:28 - OM3 (bits 28-26) controls the Output Compare Channel 3 operating mode"]
    #[inline]
    pub fn om3(&self) -> OM3R {
        OM3R::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 26;
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
    #[doc = "Bit 0 - GPT Enable"]
    #[inline]
    pub fn en(&mut self) -> _ENW {
        _ENW { w: self }
    }
    #[doc = "Bit 1 - GPT Enable mode"]
    #[inline]
    pub fn enmod(&mut self) -> _ENMODW {
        _ENMODW { w: self }
    }
    #[doc = "Bit 2 - GPT debug mode enable"]
    #[inline]
    pub fn dbgen(&mut self) -> _DBGENW {
        _DBGENW { w: self }
    }
    #[doc = "Bit 3 - GPT Wait Mode enable"]
    #[inline]
    pub fn waiten(&mut self) -> _WAITENW {
        _WAITENW { w: self }
    }
    #[doc = "Bit 4 - GPT Doze Mode Enable"]
    #[inline]
    pub fn dozeen(&mut self) -> _DOZEENW {
        _DOZEENW { w: self }
    }
    #[doc = "Bit 5 - GPT Stop Mode enable"]
    #[inline]
    pub fn stopen(&mut self) -> _STOPENW {
        _STOPENW { w: self }
    }
    #[doc = "Bits 6:8 - Clock Source select"]
    #[inline]
    pub fn clksrc(&mut self) -> _CLKSRCW {
        _CLKSRCW { w: self }
    }
    #[doc = "Bit 9 - Free-Run or Restart mode"]
    #[inline]
    pub fn frr(&mut self) -> _FRRW {
        _FRRW { w: self }
    }
    #[doc = "Bit 10 - Enable 24 MHz clock input from crystal"]
    #[inline]
    pub fn en_24m(&mut self) -> _EN_24MW {
        _EN_24MW { w: self }
    }
    #[doc = "Bit 15 - Software reset"]
    #[inline]
    pub fn swr(&mut self) -> _SWRW {
        _SWRW { w: self }
    }
    #[doc = "Bits 16:17 - See IM2"]
    #[inline]
    pub fn im1(&mut self) -> _IM1W {
        _IM1W { w: self }
    }
    #[doc = "Bits 18:19 - IM2 (bits 19-18, Input Capture Channel 2 operating mode) IM1 (bits 17-16, Input Capture Channel 1 operating mode) The IMn bit field determines the transition on the input pin (for Input capture channel n), which will trigger a capture event"]
    #[inline]
    pub fn im2(&mut self) -> _IM2W {
        _IM2W { w: self }
    }
    #[doc = "Bits 20:22 - See OM3"]
    #[inline]
    pub fn om1(&mut self) -> _OM1W {
        _OM1W { w: self }
    }
    #[doc = "Bits 23:25 - See OM3"]
    #[inline]
    pub fn om2(&mut self) -> _OM2W {
        _OM2W { w: self }
    }
    #[doc = "Bits 26:28 - OM3 (bits 28-26) controls the Output Compare Channel 3 operating mode"]
    #[inline]
    pub fn om3(&mut self) -> _OM3W {
        _OM3W { w: self }
    }
    #[doc = "Bit 29 - See F03"]
    #[inline]
    pub fn fo1(&mut self) -> _FO1W {
        _FO1W { w: self }
    }
    #[doc = "Bit 30 - See F03"]
    #[inline]
    pub fn fo2(&mut self) -> _FO2W {
        _FO2W { w: self }
    }
    #[doc = "Bit 31 - FO3 Force Output Compare Channel 3 FO2 Force Output Compare Channel 2 FO1 Force Output Compare Channel 1 The FOn bit causes the pin action programmed for the timer Output Compare n pin (according to the OMn bits in this register)"]
    #[inline]
    pub fn fo3(&mut self) -> _FO3W {
        _FO3W { w: self }
    }
}
