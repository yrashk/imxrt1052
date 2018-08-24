#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::MIX_CTRL {
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
#[doc = "Possible values of the field `DMAEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DMAENR {
    #[doc = "Disable"]
    DMAEN_0,
    #[doc = "Enable"]
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
#[doc = "Possible values of the field `BCEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BCENR {
    #[doc = "Disable"]
    BCEN_0,
    #[doc = "Enable"]
    BCEN_1,
}
impl BCENR {
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
            BCENR::BCEN_0 => false,
            BCENR::BCEN_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> BCENR {
        match value {
            false => BCENR::BCEN_0,
            true => BCENR::BCEN_1,
        }
    }
    #[doc = "Checks if the value of the field is `BCEN_0`"]
    #[inline]
    pub fn is_bcen_0(&self) -> bool {
        *self == BCENR::BCEN_0
    }
    #[doc = "Checks if the value of the field is `BCEN_1`"]
    #[inline]
    pub fn is_bcen_1(&self) -> bool {
        *self == BCENR::BCEN_1
    }
}
#[doc = "Possible values of the field `AC12EN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AC12ENR {
    #[doc = "Disable"]
    AC12EN_0,
    #[doc = "Enable"]
    AC12EN_1,
}
impl AC12ENR {
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
            AC12ENR::AC12EN_0 => false,
            AC12ENR::AC12EN_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> AC12ENR {
        match value {
            false => AC12ENR::AC12EN_0,
            true => AC12ENR::AC12EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `AC12EN_0`"]
    #[inline]
    pub fn is_ac12en_0(&self) -> bool {
        *self == AC12ENR::AC12EN_0
    }
    #[doc = "Checks if the value of the field is `AC12EN_1`"]
    #[inline]
    pub fn is_ac12en_1(&self) -> bool {
        *self == AC12ENR::AC12EN_1
    }
}
#[doc = r" Value of the field"]
pub struct DDR_ENR {
    bits: bool,
}
impl DDR_ENR {
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
#[doc = "Possible values of the field `DTDSEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DTDSELR {
    #[doc = "Write (Host to Card)"]
    DTDSEL_0,
    #[doc = "Read (Card to Host)"]
    DTDSEL_1,
}
impl DTDSELR {
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
            DTDSELR::DTDSEL_0 => false,
            DTDSELR::DTDSEL_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DTDSELR {
        match value {
            false => DTDSELR::DTDSEL_0,
            true => DTDSELR::DTDSEL_1,
        }
    }
    #[doc = "Checks if the value of the field is `DTDSEL_0`"]
    #[inline]
    pub fn is_dtdsel_0(&self) -> bool {
        *self == DTDSELR::DTDSEL_0
    }
    #[doc = "Checks if the value of the field is `DTDSEL_1`"]
    #[inline]
    pub fn is_dtdsel_1(&self) -> bool {
        *self == DTDSELR::DTDSEL_1
    }
}
#[doc = "Possible values of the field `MSBSEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MSBSELR {
    #[doc = "Single Block"]
    MSBSEL_0,
    #[doc = "Multiple Blocks"]
    MSBSEL_1,
}
impl MSBSELR {
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
            MSBSELR::MSBSEL_0 => false,
            MSBSELR::MSBSEL_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> MSBSELR {
        match value {
            false => MSBSELR::MSBSEL_0,
            true => MSBSELR::MSBSEL_1,
        }
    }
    #[doc = "Checks if the value of the field is `MSBSEL_0`"]
    #[inline]
    pub fn is_msbsel_0(&self) -> bool {
        *self == MSBSELR::MSBSEL_0
    }
    #[doc = "Checks if the value of the field is `MSBSEL_1`"]
    #[inline]
    pub fn is_msbsel_1(&self) -> bool {
        *self == MSBSELR::MSBSEL_1
    }
}
#[doc = r" Value of the field"]
pub struct NIBBLE_POSR {
    bits: bool,
}
impl NIBBLE_POSR {
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
pub struct AC23ENR {
    bits: bool,
}
impl AC23ENR {
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
#[doc = "Possible values of the field `EXE_TUNE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EXE_TUNER {
    #[doc = "Not Tuned or Tuning Completed"]
    EXE_TUNE_0,
    #[doc = "Execute Tuning"]
    EXE_TUNE_1,
}
impl EXE_TUNER {
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
            EXE_TUNER::EXE_TUNE_0 => false,
            EXE_TUNER::EXE_TUNE_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> EXE_TUNER {
        match value {
            false => EXE_TUNER::EXE_TUNE_0,
            true => EXE_TUNER::EXE_TUNE_1,
        }
    }
    #[doc = "Checks if the value of the field is `EXE_TUNE_0`"]
    #[inline]
    pub fn is_exe_tune_0(&self) -> bool {
        *self == EXE_TUNER::EXE_TUNE_0
    }
    #[doc = "Checks if the value of the field is `EXE_TUNE_1`"]
    #[inline]
    pub fn is_exe_tune_1(&self) -> bool {
        *self == EXE_TUNER::EXE_TUNE_1
    }
}
#[doc = "Possible values of the field `SMP_CLK_SEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SMP_CLK_SELR {
    #[doc = "Fixed clock is used to sample data / cmd"]
    SMP_CLK_SEL_0,
    #[doc = "Tuned clock is used to sample data / cmd"]
    SMP_CLK_SEL_1,
}
impl SMP_CLK_SELR {
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
            SMP_CLK_SELR::SMP_CLK_SEL_0 => false,
            SMP_CLK_SELR::SMP_CLK_SEL_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SMP_CLK_SELR {
        match value {
            false => SMP_CLK_SELR::SMP_CLK_SEL_0,
            true => SMP_CLK_SELR::SMP_CLK_SEL_1,
        }
    }
    #[doc = "Checks if the value of the field is `SMP_CLK_SEL_0`"]
    #[inline]
    pub fn is_smp_clk_sel_0(&self) -> bool {
        *self == SMP_CLK_SELR::SMP_CLK_SEL_0
    }
    #[doc = "Checks if the value of the field is `SMP_CLK_SEL_1`"]
    #[inline]
    pub fn is_smp_clk_sel_1(&self) -> bool {
        *self == SMP_CLK_SELR::SMP_CLK_SEL_1
    }
}
#[doc = "Possible values of the field `AUTO_TUNE_EN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AUTO_TUNE_ENR {
    #[doc = "Disable auto tuning"]
    AUTO_TUNE_EN_0,
    #[doc = "Enable auto tuning"]
    AUTO_TUNE_EN_1,
}
impl AUTO_TUNE_ENR {
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
            AUTO_TUNE_ENR::AUTO_TUNE_EN_0 => false,
            AUTO_TUNE_ENR::AUTO_TUNE_EN_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> AUTO_TUNE_ENR {
        match value {
            false => AUTO_TUNE_ENR::AUTO_TUNE_EN_0,
            true => AUTO_TUNE_ENR::AUTO_TUNE_EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `AUTO_TUNE_EN_0`"]
    #[inline]
    pub fn is_auto_tune_en_0(&self) -> bool {
        *self == AUTO_TUNE_ENR::AUTO_TUNE_EN_0
    }
    #[doc = "Checks if the value of the field is `AUTO_TUNE_EN_1`"]
    #[inline]
    pub fn is_auto_tune_en_1(&self) -> bool {
        *self == AUTO_TUNE_ENR::AUTO_TUNE_EN_1
    }
}
#[doc = "Possible values of the field `FBCLK_SEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FBCLK_SELR {
    #[doc = "Feedback clock comes from the loopback CLK"]
    FBCLK_SEL_0,
    #[doc = "Feedback clock comes from the ipp_card_clk_out"]
    FBCLK_SEL_1,
}
impl FBCLK_SELR {
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
            FBCLK_SELR::FBCLK_SEL_0 => false,
            FBCLK_SELR::FBCLK_SEL_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> FBCLK_SELR {
        match value {
            false => FBCLK_SELR::FBCLK_SEL_0,
            true => FBCLK_SELR::FBCLK_SEL_1,
        }
    }
    #[doc = "Checks if the value of the field is `FBCLK_SEL_0`"]
    #[inline]
    pub fn is_fbclk_sel_0(&self) -> bool {
        *self == FBCLK_SELR::FBCLK_SEL_0
    }
    #[doc = "Checks if the value of the field is `FBCLK_SEL_1`"]
    #[inline]
    pub fn is_fbclk_sel_1(&self) -> bool {
        *self == FBCLK_SELR::FBCLK_SEL_1
    }
}
#[doc = "Values that can be written to the field `DMAEN`"]
pub enum DMAENW {
    #[doc = "Disable"]
    DMAEN_0,
    #[doc = "Enable"]
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
    #[doc = "Disable"]
    #[inline]
    pub fn dmaen_0(self) -> &'a mut W {
        self.variant(DMAENW::DMAEN_0)
    }
    #[doc = "Enable"]
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
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `BCEN`"]
pub enum BCENW {
    #[doc = "Disable"]
    BCEN_0,
    #[doc = "Enable"]
    BCEN_1,
}
impl BCENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            BCENW::BCEN_0 => false,
            BCENW::BCEN_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _BCENW<'a> {
    w: &'a mut W,
}
impl<'a> _BCENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: BCENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable"]
    #[inline]
    pub fn bcen_0(self) -> &'a mut W {
        self.variant(BCENW::BCEN_0)
    }
    #[doc = "Enable"]
    #[inline]
    pub fn bcen_1(self) -> &'a mut W {
        self.variant(BCENW::BCEN_1)
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
#[doc = "Values that can be written to the field `AC12EN`"]
pub enum AC12ENW {
    #[doc = "Disable"]
    AC12EN_0,
    #[doc = "Enable"]
    AC12EN_1,
}
impl AC12ENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            AC12ENW::AC12EN_0 => false,
            AC12ENW::AC12EN_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _AC12ENW<'a> {
    w: &'a mut W,
}
impl<'a> _AC12ENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: AC12ENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable"]
    #[inline]
    pub fn ac12en_0(self) -> &'a mut W {
        self.variant(AC12ENW::AC12EN_0)
    }
    #[doc = "Enable"]
    #[inline]
    pub fn ac12en_1(self) -> &'a mut W {
        self.variant(AC12ENW::AC12EN_1)
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
#[doc = r" Proxy"]
pub struct _DDR_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _DDR_ENW<'a> {
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
#[doc = "Values that can be written to the field `DTDSEL`"]
pub enum DTDSELW {
    #[doc = "Write (Host to Card)"]
    DTDSEL_0,
    #[doc = "Read (Card to Host)"]
    DTDSEL_1,
}
impl DTDSELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DTDSELW::DTDSEL_0 => false,
            DTDSELW::DTDSEL_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DTDSELW<'a> {
    w: &'a mut W,
}
impl<'a> _DTDSELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DTDSELW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Write (Host to Card)"]
    #[inline]
    pub fn dtdsel_0(self) -> &'a mut W {
        self.variant(DTDSELW::DTDSEL_0)
    }
    #[doc = "Read (Card to Host)"]
    #[inline]
    pub fn dtdsel_1(self) -> &'a mut W {
        self.variant(DTDSELW::DTDSEL_1)
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
#[doc = "Values that can be written to the field `MSBSEL`"]
pub enum MSBSELW {
    #[doc = "Single Block"]
    MSBSEL_0,
    #[doc = "Multiple Blocks"]
    MSBSEL_1,
}
impl MSBSELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            MSBSELW::MSBSEL_0 => false,
            MSBSELW::MSBSEL_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MSBSELW<'a> {
    w: &'a mut W,
}
impl<'a> _MSBSELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MSBSELW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Single Block"]
    #[inline]
    pub fn msbsel_0(self) -> &'a mut W {
        self.variant(MSBSELW::MSBSEL_0)
    }
    #[doc = "Multiple Blocks"]
    #[inline]
    pub fn msbsel_1(self) -> &'a mut W {
        self.variant(MSBSELW::MSBSEL_1)
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
#[doc = r" Proxy"]
pub struct _NIBBLE_POSW<'a> {
    w: &'a mut W,
}
impl<'a> _NIBBLE_POSW<'a> {
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
pub struct _AC23ENW<'a> {
    w: &'a mut W,
}
impl<'a> _AC23ENW<'a> {
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
#[doc = "Values that can be written to the field `EXE_TUNE`"]
pub enum EXE_TUNEW {
    #[doc = "Not Tuned or Tuning Completed"]
    EXE_TUNE_0,
    #[doc = "Execute Tuning"]
    EXE_TUNE_1,
}
impl EXE_TUNEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            EXE_TUNEW::EXE_TUNE_0 => false,
            EXE_TUNEW::EXE_TUNE_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _EXE_TUNEW<'a> {
    w: &'a mut W,
}
impl<'a> _EXE_TUNEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: EXE_TUNEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Not Tuned or Tuning Completed"]
    #[inline]
    pub fn exe_tune_0(self) -> &'a mut W {
        self.variant(EXE_TUNEW::EXE_TUNE_0)
    }
    #[doc = "Execute Tuning"]
    #[inline]
    pub fn exe_tune_1(self) -> &'a mut W {
        self.variant(EXE_TUNEW::EXE_TUNE_1)
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
        const OFFSET: u8 = 22;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `SMP_CLK_SEL`"]
pub enum SMP_CLK_SELW {
    #[doc = "Fixed clock is used to sample data / cmd"]
    SMP_CLK_SEL_0,
    #[doc = "Tuned clock is used to sample data / cmd"]
    SMP_CLK_SEL_1,
}
impl SMP_CLK_SELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SMP_CLK_SELW::SMP_CLK_SEL_0 => false,
            SMP_CLK_SELW::SMP_CLK_SEL_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SMP_CLK_SELW<'a> {
    w: &'a mut W,
}
impl<'a> _SMP_CLK_SELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SMP_CLK_SELW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Fixed clock is used to sample data / cmd"]
    #[inline]
    pub fn smp_clk_sel_0(self) -> &'a mut W {
        self.variant(SMP_CLK_SELW::SMP_CLK_SEL_0)
    }
    #[doc = "Tuned clock is used to sample data / cmd"]
    #[inline]
    pub fn smp_clk_sel_1(self) -> &'a mut W {
        self.variant(SMP_CLK_SELW::SMP_CLK_SEL_1)
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
        const OFFSET: u8 = 23;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `AUTO_TUNE_EN`"]
pub enum AUTO_TUNE_ENW {
    #[doc = "Disable auto tuning"]
    AUTO_TUNE_EN_0,
    #[doc = "Enable auto tuning"]
    AUTO_TUNE_EN_1,
}
impl AUTO_TUNE_ENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            AUTO_TUNE_ENW::AUTO_TUNE_EN_0 => false,
            AUTO_TUNE_ENW::AUTO_TUNE_EN_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _AUTO_TUNE_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _AUTO_TUNE_ENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: AUTO_TUNE_ENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable auto tuning"]
    #[inline]
    pub fn auto_tune_en_0(self) -> &'a mut W {
        self.variant(AUTO_TUNE_ENW::AUTO_TUNE_EN_0)
    }
    #[doc = "Enable auto tuning"]
    #[inline]
    pub fn auto_tune_en_1(self) -> &'a mut W {
        self.variant(AUTO_TUNE_ENW::AUTO_TUNE_EN_1)
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
        const OFFSET: u8 = 24;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `FBCLK_SEL`"]
pub enum FBCLK_SELW {
    #[doc = "Feedback clock comes from the loopback CLK"]
    FBCLK_SEL_0,
    #[doc = "Feedback clock comes from the ipp_card_clk_out"]
    FBCLK_SEL_1,
}
impl FBCLK_SELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            FBCLK_SELW::FBCLK_SEL_0 => false,
            FBCLK_SELW::FBCLK_SEL_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FBCLK_SELW<'a> {
    w: &'a mut W,
}
impl<'a> _FBCLK_SELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FBCLK_SELW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Feedback clock comes from the loopback CLK"]
    #[inline]
    pub fn fbclk_sel_0(self) -> &'a mut W {
        self.variant(FBCLK_SELW::FBCLK_SEL_0)
    }
    #[doc = "Feedback clock comes from the ipp_card_clk_out"]
    #[inline]
    pub fn fbclk_sel_1(self) -> &'a mut W {
        self.variant(FBCLK_SELW::FBCLK_SEL_1)
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
        const OFFSET: u8 = 25;
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
    #[doc = "Bit 0 - DMA Enable"]
    #[inline]
    pub fn dmaen(&self) -> DMAENR {
        DMAENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - Block Count Enable"]
    #[inline]
    pub fn bcen(&self) -> BCENR {
        BCENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - Auto CMD12 Enable"]
    #[inline]
    pub fn ac12en(&self) -> AC12ENR {
        AC12ENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - Dual Data Rate mode selection"]
    #[inline]
    pub fn ddr_en(&self) -> DDR_ENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        DDR_ENR { bits }
    }
    #[doc = "Bit 4 - Data Transfer Direction Select"]
    #[inline]
    pub fn dtdsel(&self) -> DTDSELR {
        DTDSELR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 5 - Multi / Single Block Select"]
    #[inline]
    pub fn msbsel(&self) -> MSBSELR {
        MSBSELR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 6 - NIBBLE_POS"]
    #[inline]
    pub fn nibble_pos(&self) -> NIBBLE_POSR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        NIBBLE_POSR { bits }
    }
    #[doc = "Bit 7 - Auto CMD23 Enable"]
    #[inline]
    pub fn ac23en(&self) -> AC23ENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        AC23ENR { bits }
    }
    #[doc = "Bit 22 - Execute Tuning: (Only used for SD3.0, SDR104 mode and EMMC HS200 mode)"]
    #[inline]
    pub fn exe_tune(&self) -> EXE_TUNER {
        EXE_TUNER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 22;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 23 - SMP_CLK_SEL"]
    #[inline]
    pub fn smp_clk_sel(&self) -> SMP_CLK_SELR {
        SMP_CLK_SELR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 23;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 24 - Auto Tuning Enable (Only used for SD3.0, SDR104 mode and and EMMC HS200 mode)"]
    #[inline]
    pub fn auto_tune_en(&self) -> AUTO_TUNE_ENR {
        AUTO_TUNE_ENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 25 - Feedback Clock Source Selection (Only used for SD3.0, SDR104 mode and EMMC HS200 mode)"]
    #[inline]
    pub fn fbclk_sel(&self) -> FBCLK_SELR {
        FBCLK_SELR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 25;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 2147483648 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - DMA Enable"]
    #[inline]
    pub fn dmaen(&mut self) -> _DMAENW {
        _DMAENW { w: self }
    }
    #[doc = "Bit 1 - Block Count Enable"]
    #[inline]
    pub fn bcen(&mut self) -> _BCENW {
        _BCENW { w: self }
    }
    #[doc = "Bit 2 - Auto CMD12 Enable"]
    #[inline]
    pub fn ac12en(&mut self) -> _AC12ENW {
        _AC12ENW { w: self }
    }
    #[doc = "Bit 3 - Dual Data Rate mode selection"]
    #[inline]
    pub fn ddr_en(&mut self) -> _DDR_ENW {
        _DDR_ENW { w: self }
    }
    #[doc = "Bit 4 - Data Transfer Direction Select"]
    #[inline]
    pub fn dtdsel(&mut self) -> _DTDSELW {
        _DTDSELW { w: self }
    }
    #[doc = "Bit 5 - Multi / Single Block Select"]
    #[inline]
    pub fn msbsel(&mut self) -> _MSBSELW {
        _MSBSELW { w: self }
    }
    #[doc = "Bit 6 - NIBBLE_POS"]
    #[inline]
    pub fn nibble_pos(&mut self) -> _NIBBLE_POSW {
        _NIBBLE_POSW { w: self }
    }
    #[doc = "Bit 7 - Auto CMD23 Enable"]
    #[inline]
    pub fn ac23en(&mut self) -> _AC23ENW {
        _AC23ENW { w: self }
    }
    #[doc = "Bit 22 - Execute Tuning: (Only used for SD3.0, SDR104 mode and EMMC HS200 mode)"]
    #[inline]
    pub fn exe_tune(&mut self) -> _EXE_TUNEW {
        _EXE_TUNEW { w: self }
    }
    #[doc = "Bit 23 - SMP_CLK_SEL"]
    #[inline]
    pub fn smp_clk_sel(&mut self) -> _SMP_CLK_SELW {
        _SMP_CLK_SELW { w: self }
    }
    #[doc = "Bit 24 - Auto Tuning Enable (Only used for SD3.0, SDR104 mode and and EMMC HS200 mode)"]
    #[inline]
    pub fn auto_tune_en(&mut self) -> _AUTO_TUNE_ENW {
        _AUTO_TUNE_ENW { w: self }
    }
    #[doc = "Bit 25 - Feedback Clock Source Selection (Only used for SD3.0, SDR104 mode and EMMC HS200 mode)"]
    #[inline]
    pub fn fbclk_sel(&mut self) -> _FBCLK_SELW {
        _FBCLK_SELW { w: self }
    }
}
