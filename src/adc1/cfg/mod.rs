#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CFG {
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
#[doc = "Possible values of the field `ADICLK`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADICLKR {
    #[doc = "IPG clock"]
    ADICLK_0,
    #[doc = "IPG clock divided by 2"]
    ADICLK_1,
    #[doc = "Asynchronous clock (ADACK)"]
    ADICLK_3,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl ADICLKR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            ADICLKR::ADICLK_0 => 0,
            ADICLKR::ADICLK_1 => 1,
            ADICLKR::ADICLK_3 => 3,
            ADICLKR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> ADICLKR {
        match value {
            0 => ADICLKR::ADICLK_0,
            1 => ADICLKR::ADICLK_1,
            3 => ADICLKR::ADICLK_3,
            i => ADICLKR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `ADICLK_0`"]
    #[inline]
    pub fn is_adiclk_0(&self) -> bool {
        *self == ADICLKR::ADICLK_0
    }
    #[doc = "Checks if the value of the field is `ADICLK_1`"]
    #[inline]
    pub fn is_adiclk_1(&self) -> bool {
        *self == ADICLKR::ADICLK_1
    }
    #[doc = "Checks if the value of the field is `ADICLK_3`"]
    #[inline]
    pub fn is_adiclk_3(&self) -> bool {
        *self == ADICLKR::ADICLK_3
    }
}
#[doc = "Possible values of the field `MODE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MODER {
    #[doc = "8-bit conversion"]
    MODE_0,
    #[doc = "10-bit conversion"]
    MODE_1,
    #[doc = "12-bit conversion"]
    MODE_2,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl MODER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            MODER::MODE_0 => 0,
            MODER::MODE_1 => 1,
            MODER::MODE_2 => 2,
            MODER::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> MODER {
        match value {
            0 => MODER::MODE_0,
            1 => MODER::MODE_1,
            2 => MODER::MODE_2,
            i => MODER::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `MODE_0`"]
    #[inline]
    pub fn is_mode_0(&self) -> bool {
        *self == MODER::MODE_0
    }
    #[doc = "Checks if the value of the field is `MODE_1`"]
    #[inline]
    pub fn is_mode_1(&self) -> bool {
        *self == MODER::MODE_1
    }
    #[doc = "Checks if the value of the field is `MODE_2`"]
    #[inline]
    pub fn is_mode_2(&self) -> bool {
        *self == MODER::MODE_2
    }
}
#[doc = "Possible values of the field `ADLSMP`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADLSMPR {
    #[doc = "Short sample mode."]
    ADLSMP_0,
    #[doc = "Long sample mode."]
    ADLSMP_1,
}
impl ADLSMPR {
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
            ADLSMPR::ADLSMP_0 => false,
            ADLSMPR::ADLSMP_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ADLSMPR {
        match value {
            false => ADLSMPR::ADLSMP_0,
            true => ADLSMPR::ADLSMP_1,
        }
    }
    #[doc = "Checks if the value of the field is `ADLSMP_0`"]
    #[inline]
    pub fn is_adlsmp_0(&self) -> bool {
        *self == ADLSMPR::ADLSMP_0
    }
    #[doc = "Checks if the value of the field is `ADLSMP_1`"]
    #[inline]
    pub fn is_adlsmp_1(&self) -> bool {
        *self == ADLSMPR::ADLSMP_1
    }
}
#[doc = "Possible values of the field `ADIV`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADIVR {
    #[doc = "Input clock"]
    ADIV_0,
    #[doc = "Input clock / 2"]
    ADIV_1,
    #[doc = "Input clock / 4"]
    ADIV_2,
    #[doc = "Input clock / 8"]
    ADIV_3,
}
impl ADIVR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            ADIVR::ADIV_0 => 0,
            ADIVR::ADIV_1 => 1,
            ADIVR::ADIV_2 => 2,
            ADIVR::ADIV_3 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> ADIVR {
        match value {
            0 => ADIVR::ADIV_0,
            1 => ADIVR::ADIV_1,
            2 => ADIVR::ADIV_2,
            3 => ADIVR::ADIV_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `ADIV_0`"]
    #[inline]
    pub fn is_adiv_0(&self) -> bool {
        *self == ADIVR::ADIV_0
    }
    #[doc = "Checks if the value of the field is `ADIV_1`"]
    #[inline]
    pub fn is_adiv_1(&self) -> bool {
        *self == ADIVR::ADIV_1
    }
    #[doc = "Checks if the value of the field is `ADIV_2`"]
    #[inline]
    pub fn is_adiv_2(&self) -> bool {
        *self == ADIVR::ADIV_2
    }
    #[doc = "Checks if the value of the field is `ADIV_3`"]
    #[inline]
    pub fn is_adiv_3(&self) -> bool {
        *self == ADIVR::ADIV_3
    }
}
#[doc = "Possible values of the field `ADLPC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADLPCR {
    #[doc = "ADC hard block not in low power mode."]
    ADLPC_0,
    #[doc = "ADC hard block in low power mode."]
    ADLPC_1,
}
impl ADLPCR {
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
            ADLPCR::ADLPC_0 => false,
            ADLPCR::ADLPC_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ADLPCR {
        match value {
            false => ADLPCR::ADLPC_0,
            true => ADLPCR::ADLPC_1,
        }
    }
    #[doc = "Checks if the value of the field is `ADLPC_0`"]
    #[inline]
    pub fn is_adlpc_0(&self) -> bool {
        *self == ADLPCR::ADLPC_0
    }
    #[doc = "Checks if the value of the field is `ADLPC_1`"]
    #[inline]
    pub fn is_adlpc_1(&self) -> bool {
        *self == ADLPCR::ADLPC_1
    }
}
#[doc = "Possible values of the field `ADSTS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADSTSR {
    #[doc = "Sample period (ADC clocks) = 2 if ADLSMP=0b Sample period (ADC clocks) = 12 if ADLSMP=1b"]
    ADSTS_0,
    #[doc = "Sample period (ADC clocks) = 4 if ADLSMP=0b Sample period (ADC clocks) = 16 if ADLSMP=1b"]
    ADSTS_1,
    #[doc = "Sample period (ADC clocks) = 6 if ADLSMP=0b Sample period (ADC clocks) = 20 if ADLSMP=1b"]
    ADSTS_2,
    #[doc = "Sample period (ADC clocks) = 8 if ADLSMP=0b Sample period (ADC clocks) = 24 if ADLSMP=1b"]
    ADSTS_3,
}
impl ADSTSR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            ADSTSR::ADSTS_0 => 0,
            ADSTSR::ADSTS_1 => 1,
            ADSTSR::ADSTS_2 => 2,
            ADSTSR::ADSTS_3 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> ADSTSR {
        match value {
            0 => ADSTSR::ADSTS_0,
            1 => ADSTSR::ADSTS_1,
            2 => ADSTSR::ADSTS_2,
            3 => ADSTSR::ADSTS_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `ADSTS_0`"]
    #[inline]
    pub fn is_adsts_0(&self) -> bool {
        *self == ADSTSR::ADSTS_0
    }
    #[doc = "Checks if the value of the field is `ADSTS_1`"]
    #[inline]
    pub fn is_adsts_1(&self) -> bool {
        *self == ADSTSR::ADSTS_1
    }
    #[doc = "Checks if the value of the field is `ADSTS_2`"]
    #[inline]
    pub fn is_adsts_2(&self) -> bool {
        *self == ADSTSR::ADSTS_2
    }
    #[doc = "Checks if the value of the field is `ADSTS_3`"]
    #[inline]
    pub fn is_adsts_3(&self) -> bool {
        *self == ADSTSR::ADSTS_3
    }
}
#[doc = "Possible values of the field `ADHSC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADHSCR {
    #[doc = "Normal conversion selected."]
    ADHSC_0,
    #[doc = "High speed conversion selected."]
    ADHSC_1,
}
impl ADHSCR {
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
            ADHSCR::ADHSC_0 => false,
            ADHSCR::ADHSC_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ADHSCR {
        match value {
            false => ADHSCR::ADHSC_0,
            true => ADHSCR::ADHSC_1,
        }
    }
    #[doc = "Checks if the value of the field is `ADHSC_0`"]
    #[inline]
    pub fn is_adhsc_0(&self) -> bool {
        *self == ADHSCR::ADHSC_0
    }
    #[doc = "Checks if the value of the field is `ADHSC_1`"]
    #[inline]
    pub fn is_adhsc_1(&self) -> bool {
        *self == ADHSCR::ADHSC_1
    }
}
#[doc = "Possible values of the field `REFSEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REFSELR {
    #[doc = "Selects VREFH/VREFL as reference voltage."]
    REFSEL_0,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl REFSELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            REFSELR::REFSEL_0 => 0,
            REFSELR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> REFSELR {
        match value {
            0 => REFSELR::REFSEL_0,
            i => REFSELR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `REFSEL_0`"]
    #[inline]
    pub fn is_refsel_0(&self) -> bool {
        *self == REFSELR::REFSEL_0
    }
}
#[doc = "Possible values of the field `ADTRG`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADTRGR {
    #[doc = "Software trigger selected"]
    ADTRG_0,
    #[doc = "Hardware trigger selected"]
    ADTRG_1,
}
impl ADTRGR {
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
            ADTRGR::ADTRG_0 => false,
            ADTRGR::ADTRG_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ADTRGR {
        match value {
            false => ADTRGR::ADTRG_0,
            true => ADTRGR::ADTRG_1,
        }
    }
    #[doc = "Checks if the value of the field is `ADTRG_0`"]
    #[inline]
    pub fn is_adtrg_0(&self) -> bool {
        *self == ADTRGR::ADTRG_0
    }
    #[doc = "Checks if the value of the field is `ADTRG_1`"]
    #[inline]
    pub fn is_adtrg_1(&self) -> bool {
        *self == ADTRGR::ADTRG_1
    }
}
#[doc = "Possible values of the field `AVGS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AVGSR {
    #[doc = "4 samples averaged"]
    AVGS_0,
    #[doc = "8 samples averaged"]
    AVGS_1,
    #[doc = "16 samples averaged"]
    AVGS_2,
    #[doc = "32 samples averaged"]
    AVGS_3,
}
impl AVGSR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            AVGSR::AVGS_0 => 0,
            AVGSR::AVGS_1 => 1,
            AVGSR::AVGS_2 => 2,
            AVGSR::AVGS_3 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> AVGSR {
        match value {
            0 => AVGSR::AVGS_0,
            1 => AVGSR::AVGS_1,
            2 => AVGSR::AVGS_2,
            3 => AVGSR::AVGS_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `AVGS_0`"]
    #[inline]
    pub fn is_avgs_0(&self) -> bool {
        *self == AVGSR::AVGS_0
    }
    #[doc = "Checks if the value of the field is `AVGS_1`"]
    #[inline]
    pub fn is_avgs_1(&self) -> bool {
        *self == AVGSR::AVGS_1
    }
    #[doc = "Checks if the value of the field is `AVGS_2`"]
    #[inline]
    pub fn is_avgs_2(&self) -> bool {
        *self == AVGSR::AVGS_2
    }
    #[doc = "Checks if the value of the field is `AVGS_3`"]
    #[inline]
    pub fn is_avgs_3(&self) -> bool {
        *self == AVGSR::AVGS_3
    }
}
#[doc = "Possible values of the field `OVWREN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OVWRENR {
    #[doc = "Disable the overwriting. Existing Data in Data result register will not be overwritten by subsequent converted data."]
    OVWREN_0,
    #[doc = "Enable the overwriting."]
    OVWREN_1,
}
impl OVWRENR {
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
            OVWRENR::OVWREN_0 => false,
            OVWRENR::OVWREN_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> OVWRENR {
        match value {
            false => OVWRENR::OVWREN_0,
            true => OVWRENR::OVWREN_1,
        }
    }
    #[doc = "Checks if the value of the field is `OVWREN_0`"]
    #[inline]
    pub fn is_ovwren_0(&self) -> bool {
        *self == OVWRENR::OVWREN_0
    }
    #[doc = "Checks if the value of the field is `OVWREN_1`"]
    #[inline]
    pub fn is_ovwren_1(&self) -> bool {
        *self == OVWRENR::OVWREN_1
    }
}
#[doc = "Values that can be written to the field `ADICLK`"]
pub enum ADICLKW {
    #[doc = "IPG clock"]
    ADICLK_0,
    #[doc = "IPG clock divided by 2"]
    ADICLK_1,
    #[doc = "Asynchronous clock (ADACK)"]
    ADICLK_3,
}
impl ADICLKW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            ADICLKW::ADICLK_0 => 0,
            ADICLKW::ADICLK_1 => 1,
            ADICLKW::ADICLK_3 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ADICLKW<'a> {
    w: &'a mut W,
}
impl<'a> _ADICLKW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ADICLKW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "IPG clock"]
    #[inline]
    pub fn adiclk_0(self) -> &'a mut W {
        self.variant(ADICLKW::ADICLK_0)
    }
    #[doc = "IPG clock divided by 2"]
    #[inline]
    pub fn adiclk_1(self) -> &'a mut W {
        self.variant(ADICLKW::ADICLK_1)
    }
    #[doc = "Asynchronous clock (ADACK)"]
    #[inline]
    pub fn adiclk_3(self) -> &'a mut W {
        self.variant(ADICLKW::ADICLK_3)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `MODE`"]
pub enum MODEW {
    #[doc = "8-bit conversion"]
    MODE_0,
    #[doc = "10-bit conversion"]
    MODE_1,
    #[doc = "12-bit conversion"]
    MODE_2,
}
impl MODEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            MODEW::MODE_0 => 0,
            MODEW::MODE_1 => 1,
            MODEW::MODE_2 => 2,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MODEW<'a> {
    w: &'a mut W,
}
impl<'a> _MODEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MODEW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "8-bit conversion"]
    #[inline]
    pub fn mode_0(self) -> &'a mut W {
        self.variant(MODEW::MODE_0)
    }
    #[doc = "10-bit conversion"]
    #[inline]
    pub fn mode_1(self) -> &'a mut W {
        self.variant(MODEW::MODE_1)
    }
    #[doc = "12-bit conversion"]
    #[inline]
    pub fn mode_2(self) -> &'a mut W {
        self.variant(MODEW::MODE_2)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 2;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `ADLSMP`"]
pub enum ADLSMPW {
    #[doc = "Short sample mode."]
    ADLSMP_0,
    #[doc = "Long sample mode."]
    ADLSMP_1,
}
impl ADLSMPW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ADLSMPW::ADLSMP_0 => false,
            ADLSMPW::ADLSMP_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ADLSMPW<'a> {
    w: &'a mut W,
}
impl<'a> _ADLSMPW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ADLSMPW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Short sample mode."]
    #[inline]
    pub fn adlsmp_0(self) -> &'a mut W {
        self.variant(ADLSMPW::ADLSMP_0)
    }
    #[doc = "Long sample mode."]
    #[inline]
    pub fn adlsmp_1(self) -> &'a mut W {
        self.variant(ADLSMPW::ADLSMP_1)
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
#[doc = "Values that can be written to the field `ADIV`"]
pub enum ADIVW {
    #[doc = "Input clock"]
    ADIV_0,
    #[doc = "Input clock / 2"]
    ADIV_1,
    #[doc = "Input clock / 4"]
    ADIV_2,
    #[doc = "Input clock / 8"]
    ADIV_3,
}
impl ADIVW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            ADIVW::ADIV_0 => 0,
            ADIVW::ADIV_1 => 1,
            ADIVW::ADIV_2 => 2,
            ADIVW::ADIV_3 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ADIVW<'a> {
    w: &'a mut W,
}
impl<'a> _ADIVW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ADIVW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Input clock"]
    #[inline]
    pub fn adiv_0(self) -> &'a mut W {
        self.variant(ADIVW::ADIV_0)
    }
    #[doc = "Input clock / 2"]
    #[inline]
    pub fn adiv_1(self) -> &'a mut W {
        self.variant(ADIVW::ADIV_1)
    }
    #[doc = "Input clock / 4"]
    #[inline]
    pub fn adiv_2(self) -> &'a mut W {
        self.variant(ADIVW::ADIV_2)
    }
    #[doc = "Input clock / 8"]
    #[inline]
    pub fn adiv_3(self) -> &'a mut W {
        self.variant(ADIVW::ADIV_3)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 5;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `ADLPC`"]
pub enum ADLPCW {
    #[doc = "ADC hard block not in low power mode."]
    ADLPC_0,
    #[doc = "ADC hard block in low power mode."]
    ADLPC_1,
}
impl ADLPCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ADLPCW::ADLPC_0 => false,
            ADLPCW::ADLPC_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ADLPCW<'a> {
    w: &'a mut W,
}
impl<'a> _ADLPCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ADLPCW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "ADC hard block not in low power mode."]
    #[inline]
    pub fn adlpc_0(self) -> &'a mut W {
        self.variant(ADLPCW::ADLPC_0)
    }
    #[doc = "ADC hard block in low power mode."]
    #[inline]
    pub fn adlpc_1(self) -> &'a mut W {
        self.variant(ADLPCW::ADLPC_1)
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
#[doc = "Values that can be written to the field `ADSTS`"]
pub enum ADSTSW {
    #[doc = "Sample period (ADC clocks) = 2 if ADLSMP=0b Sample period (ADC clocks) = 12 if ADLSMP=1b"]
    ADSTS_0,
    #[doc = "Sample period (ADC clocks) = 4 if ADLSMP=0b Sample period (ADC clocks) = 16 if ADLSMP=1b"]
    ADSTS_1,
    #[doc = "Sample period (ADC clocks) = 6 if ADLSMP=0b Sample period (ADC clocks) = 20 if ADLSMP=1b"]
    ADSTS_2,
    #[doc = "Sample period (ADC clocks) = 8 if ADLSMP=0b Sample period (ADC clocks) = 24 if ADLSMP=1b"]
    ADSTS_3,
}
impl ADSTSW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            ADSTSW::ADSTS_0 => 0,
            ADSTSW::ADSTS_1 => 1,
            ADSTSW::ADSTS_2 => 2,
            ADSTSW::ADSTS_3 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ADSTSW<'a> {
    w: &'a mut W,
}
impl<'a> _ADSTSW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ADSTSW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Sample period (ADC clocks) = 2 if ADLSMP=0b Sample period (ADC clocks) = 12 if ADLSMP=1b"]
    #[inline]
    pub fn adsts_0(self) -> &'a mut W {
        self.variant(ADSTSW::ADSTS_0)
    }
    #[doc = "Sample period (ADC clocks) = 4 if ADLSMP=0b Sample period (ADC clocks) = 16 if ADLSMP=1b"]
    #[inline]
    pub fn adsts_1(self) -> &'a mut W {
        self.variant(ADSTSW::ADSTS_1)
    }
    #[doc = "Sample period (ADC clocks) = 6 if ADLSMP=0b Sample period (ADC clocks) = 20 if ADLSMP=1b"]
    #[inline]
    pub fn adsts_2(self) -> &'a mut W {
        self.variant(ADSTSW::ADSTS_2)
    }
    #[doc = "Sample period (ADC clocks) = 8 if ADLSMP=0b Sample period (ADC clocks) = 24 if ADLSMP=1b"]
    #[inline]
    pub fn adsts_3(self) -> &'a mut W {
        self.variant(ADSTSW::ADSTS_3)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `ADHSC`"]
pub enum ADHSCW {
    #[doc = "Normal conversion selected."]
    ADHSC_0,
    #[doc = "High speed conversion selected."]
    ADHSC_1,
}
impl ADHSCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ADHSCW::ADHSC_0 => false,
            ADHSCW::ADHSC_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ADHSCW<'a> {
    w: &'a mut W,
}
impl<'a> _ADHSCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ADHSCW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Normal conversion selected."]
    #[inline]
    pub fn adhsc_0(self) -> &'a mut W {
        self.variant(ADHSCW::ADHSC_0)
    }
    #[doc = "High speed conversion selected."]
    #[inline]
    pub fn adhsc_1(self) -> &'a mut W {
        self.variant(ADHSCW::ADHSC_1)
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
#[doc = "Values that can be written to the field `REFSEL`"]
pub enum REFSELW {
    #[doc = "Selects VREFH/VREFL as reference voltage."]
    REFSEL_0,
}
impl REFSELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            REFSELW::REFSEL_0 => 0,
        }
    }
}
#[doc = r" Proxy"]
pub struct _REFSELW<'a> {
    w: &'a mut W,
}
impl<'a> _REFSELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: REFSELW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Selects VREFH/VREFL as reference voltage."]
    #[inline]
    pub fn refsel_0(self) -> &'a mut W {
        self.variant(REFSELW::REFSEL_0)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 11;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `ADTRG`"]
pub enum ADTRGW {
    #[doc = "Software trigger selected"]
    ADTRG_0,
    #[doc = "Hardware trigger selected"]
    ADTRG_1,
}
impl ADTRGW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ADTRGW::ADTRG_0 => false,
            ADTRGW::ADTRG_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ADTRGW<'a> {
    w: &'a mut W,
}
impl<'a> _ADTRGW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ADTRGW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Software trigger selected"]
    #[inline]
    pub fn adtrg_0(self) -> &'a mut W {
        self.variant(ADTRGW::ADTRG_0)
    }
    #[doc = "Hardware trigger selected"]
    #[inline]
    pub fn adtrg_1(self) -> &'a mut W {
        self.variant(ADTRGW::ADTRG_1)
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
        const OFFSET: u8 = 13;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `AVGS`"]
pub enum AVGSW {
    #[doc = "4 samples averaged"]
    AVGS_0,
    #[doc = "8 samples averaged"]
    AVGS_1,
    #[doc = "16 samples averaged"]
    AVGS_2,
    #[doc = "32 samples averaged"]
    AVGS_3,
}
impl AVGSW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            AVGSW::AVGS_0 => 0,
            AVGSW::AVGS_1 => 1,
            AVGSW::AVGS_2 => 2,
            AVGSW::AVGS_3 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _AVGSW<'a> {
    w: &'a mut W,
}
impl<'a> _AVGSW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: AVGSW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "4 samples averaged"]
    #[inline]
    pub fn avgs_0(self) -> &'a mut W {
        self.variant(AVGSW::AVGS_0)
    }
    #[doc = "8 samples averaged"]
    #[inline]
    pub fn avgs_1(self) -> &'a mut W {
        self.variant(AVGSW::AVGS_1)
    }
    #[doc = "16 samples averaged"]
    #[inline]
    pub fn avgs_2(self) -> &'a mut W {
        self.variant(AVGSW::AVGS_2)
    }
    #[doc = "32 samples averaged"]
    #[inline]
    pub fn avgs_3(self) -> &'a mut W {
        self.variant(AVGSW::AVGS_3)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 14;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `OVWREN`"]
pub enum OVWRENW {
    #[doc = "Disable the overwriting. Existing Data in Data result register will not be overwritten by subsequent converted data."]
    OVWREN_0,
    #[doc = "Enable the overwriting."]
    OVWREN_1,
}
impl OVWRENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            OVWRENW::OVWREN_0 => false,
            OVWRENW::OVWREN_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _OVWRENW<'a> {
    w: &'a mut W,
}
impl<'a> _OVWRENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: OVWRENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable the overwriting. Existing Data in Data result register will not be overwritten by subsequent converted data."]
    #[inline]
    pub fn ovwren_0(self) -> &'a mut W {
        self.variant(OVWRENW::OVWREN_0)
    }
    #[doc = "Enable the overwriting."]
    #[inline]
    pub fn ovwren_1(self) -> &'a mut W {
        self.variant(OVWRENW::OVWREN_1)
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
    #[doc = "Bits 0:1 - Input Clock Select"]
    #[inline]
    pub fn adiclk(&self) -> ADICLKR {
        ADICLKR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 2:3 - Conversion Mode Selection"]
    #[inline]
    pub fn mode(&self) -> MODER {
        MODER::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 4 - Long Sample Time Configuration"]
    #[inline]
    pub fn adlsmp(&self) -> ADLSMPR {
        ADLSMPR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 5:6 - Clock Divide Select"]
    #[inline]
    pub fn adiv(&self) -> ADIVR {
        ADIVR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 7 - Low-Power Configuration"]
    #[inline]
    pub fn adlpc(&self) -> ADLPCR {
        ADLPCR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 8:9 - Defines the sample time duration"]
    #[inline]
    pub fn adsts(&self) -> ADSTSR {
        ADSTSR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 10 - High Speed Configuration"]
    #[inline]
    pub fn adhsc(&self) -> ADHSCR {
        ADHSCR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 11:12 - Voltage Reference Selection"]
    #[inline]
    pub fn refsel(&self) -> REFSELR {
        REFSELR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 13 - Conversion Trigger Select"]
    #[inline]
    pub fn adtrg(&self) -> ADTRGR {
        ADTRGR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 14:15 - Hardware Average select"]
    #[inline]
    pub fn avgs(&self) -> AVGSR {
        AVGSR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 16 - Data Overwrite Enable"]
    #[inline]
    pub fn ovwren(&self) -> OVWRENR {
        OVWRENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 512 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:1 - Input Clock Select"]
    #[inline]
    pub fn adiclk(&mut self) -> _ADICLKW {
        _ADICLKW { w: self }
    }
    #[doc = "Bits 2:3 - Conversion Mode Selection"]
    #[inline]
    pub fn mode(&mut self) -> _MODEW {
        _MODEW { w: self }
    }
    #[doc = "Bit 4 - Long Sample Time Configuration"]
    #[inline]
    pub fn adlsmp(&mut self) -> _ADLSMPW {
        _ADLSMPW { w: self }
    }
    #[doc = "Bits 5:6 - Clock Divide Select"]
    #[inline]
    pub fn adiv(&mut self) -> _ADIVW {
        _ADIVW { w: self }
    }
    #[doc = "Bit 7 - Low-Power Configuration"]
    #[inline]
    pub fn adlpc(&mut self) -> _ADLPCW {
        _ADLPCW { w: self }
    }
    #[doc = "Bits 8:9 - Defines the sample time duration"]
    #[inline]
    pub fn adsts(&mut self) -> _ADSTSW {
        _ADSTSW { w: self }
    }
    #[doc = "Bit 10 - High Speed Configuration"]
    #[inline]
    pub fn adhsc(&mut self) -> _ADHSCW {
        _ADHSCW { w: self }
    }
    #[doc = "Bits 11:12 - Voltage Reference Selection"]
    #[inline]
    pub fn refsel(&mut self) -> _REFSELW {
        _REFSELW { w: self }
    }
    #[doc = "Bit 13 - Conversion Trigger Select"]
    #[inline]
    pub fn adtrg(&mut self) -> _ADTRGW {
        _ADTRGW { w: self }
    }
    #[doc = "Bits 14:15 - Hardware Average select"]
    #[inline]
    pub fn avgs(&mut self) -> _AVGSW {
        _AVGSW { w: self }
    }
    #[doc = "Bit 16 - Data Overwrite Enable"]
    #[inline]
    pub fn ovwren(&mut self) -> _OVWRENW {
        _OVWRENW { w: self }
    }
}
