#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::TCR {
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
pub struct FRAMESZR {
    bits: u16,
}
impl FRAMESZR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }
}
#[doc = "Possible values of the field `WIDTH`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WIDTHR {
    #[doc = "1 bit transfer"]
    WIDTH_0,
    #[doc = "2 bit transfer"]
    WIDTH_1,
    #[doc = "4 bit transfer"]
    WIDTH_2,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl WIDTHR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            WIDTHR::WIDTH_0 => 0,
            WIDTHR::WIDTH_1 => 1,
            WIDTHR::WIDTH_2 => 2,
            WIDTHR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> WIDTHR {
        match value {
            0 => WIDTHR::WIDTH_0,
            1 => WIDTHR::WIDTH_1,
            2 => WIDTHR::WIDTH_2,
            i => WIDTHR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `WIDTH_0`"]
    #[inline]
    pub fn is_width_0(&self) -> bool {
        *self == WIDTHR::WIDTH_0
    }
    #[doc = "Checks if the value of the field is `WIDTH_1`"]
    #[inline]
    pub fn is_width_1(&self) -> bool {
        *self == WIDTHR::WIDTH_1
    }
    #[doc = "Checks if the value of the field is `WIDTH_2`"]
    #[inline]
    pub fn is_width_2(&self) -> bool {
        *self == WIDTHR::WIDTH_2
    }
}
#[doc = "Possible values of the field `TXMSK`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXMSKR {
    #[doc = "Normal transfer"]
    TXMSK_0,
    #[doc = "Mask transmit data"]
    TXMSK_1,
}
impl TXMSKR {
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
            TXMSKR::TXMSK_0 => false,
            TXMSKR::TXMSK_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TXMSKR {
        match value {
            false => TXMSKR::TXMSK_0,
            true => TXMSKR::TXMSK_1,
        }
    }
    #[doc = "Checks if the value of the field is `TXMSK_0`"]
    #[inline]
    pub fn is_txmsk_0(&self) -> bool {
        *self == TXMSKR::TXMSK_0
    }
    #[doc = "Checks if the value of the field is `TXMSK_1`"]
    #[inline]
    pub fn is_txmsk_1(&self) -> bool {
        *self == TXMSKR::TXMSK_1
    }
}
#[doc = "Possible values of the field `RXMSK`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXMSKR {
    #[doc = "Normal transfer"]
    RXMSK_0,
    #[doc = "Receive data is masked"]
    RXMSK_1,
}
impl RXMSKR {
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
            RXMSKR::RXMSK_0 => false,
            RXMSKR::RXMSK_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RXMSKR {
        match value {
            false => RXMSKR::RXMSK_0,
            true => RXMSKR::RXMSK_1,
        }
    }
    #[doc = "Checks if the value of the field is `RXMSK_0`"]
    #[inline]
    pub fn is_rxmsk_0(&self) -> bool {
        *self == RXMSKR::RXMSK_0
    }
    #[doc = "Checks if the value of the field is `RXMSK_1`"]
    #[inline]
    pub fn is_rxmsk_1(&self) -> bool {
        *self == RXMSKR::RXMSK_1
    }
}
#[doc = "Possible values of the field `CONTC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CONTCR {
    #[doc = "Command word for start of new transfer"]
    CONTC_0,
    #[doc = "Command word for continuing transfer"]
    CONTC_1,
}
impl CONTCR {
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
            CONTCR::CONTC_0 => false,
            CONTCR::CONTC_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CONTCR {
        match value {
            false => CONTCR::CONTC_0,
            true => CONTCR::CONTC_1,
        }
    }
    #[doc = "Checks if the value of the field is `CONTC_0`"]
    #[inline]
    pub fn is_contc_0(&self) -> bool {
        *self == CONTCR::CONTC_0
    }
    #[doc = "Checks if the value of the field is `CONTC_1`"]
    #[inline]
    pub fn is_contc_1(&self) -> bool {
        *self == CONTCR::CONTC_1
    }
}
#[doc = "Possible values of the field `CONT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CONTR {
    #[doc = "Continuous transfer is disabled"]
    CONT_0,
    #[doc = "Continuous transfer is enabled"]
    CONT_1,
}
impl CONTR {
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
            CONTR::CONT_0 => false,
            CONTR::CONT_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CONTR {
        match value {
            false => CONTR::CONT_0,
            true => CONTR::CONT_1,
        }
    }
    #[doc = "Checks if the value of the field is `CONT_0`"]
    #[inline]
    pub fn is_cont_0(&self) -> bool {
        *self == CONTR::CONT_0
    }
    #[doc = "Checks if the value of the field is `CONT_1`"]
    #[inline]
    pub fn is_cont_1(&self) -> bool {
        *self == CONTR::CONT_1
    }
}
#[doc = "Possible values of the field `BYSW`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BYSWR {
    #[doc = "Byte swap is disabled"]
    BYSW_0,
    #[doc = "Byte swap is enabled"]
    BYSW_1,
}
impl BYSWR {
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
            BYSWR::BYSW_0 => false,
            BYSWR::BYSW_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> BYSWR {
        match value {
            false => BYSWR::BYSW_0,
            true => BYSWR::BYSW_1,
        }
    }
    #[doc = "Checks if the value of the field is `BYSW_0`"]
    #[inline]
    pub fn is_bysw_0(&self) -> bool {
        *self == BYSWR::BYSW_0
    }
    #[doc = "Checks if the value of the field is `BYSW_1`"]
    #[inline]
    pub fn is_bysw_1(&self) -> bool {
        *self == BYSWR::BYSW_1
    }
}
#[doc = "Possible values of the field `LSBF`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LSBFR {
    #[doc = "Data is transferred MSB first"]
    LSBF_0,
    #[doc = "Data is transferred LSB first"]
    LSBF_1,
}
impl LSBFR {
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
            LSBFR::LSBF_0 => false,
            LSBFR::LSBF_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> LSBFR {
        match value {
            false => LSBFR::LSBF_0,
            true => LSBFR::LSBF_1,
        }
    }
    #[doc = "Checks if the value of the field is `LSBF_0`"]
    #[inline]
    pub fn is_lsbf_0(&self) -> bool {
        *self == LSBFR::LSBF_0
    }
    #[doc = "Checks if the value of the field is `LSBF_1`"]
    #[inline]
    pub fn is_lsbf_1(&self) -> bool {
        *self == LSBFR::LSBF_1
    }
}
#[doc = "Possible values of the field `PCS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PCSR {
    #[doc = "Transfer using LPSPI_PCS[0]"]
    PCS_0,
    #[doc = "Transfer using LPSPI_PCS[1]"]
    PCS_1,
    #[doc = "Transfer using LPSPI_PCS[2]"]
    PCS_2,
    #[doc = "Transfer using LPSPI_PCS[3]"]
    PCS_3,
}
impl PCSR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            PCSR::PCS_0 => 0,
            PCSR::PCS_1 => 1,
            PCSR::PCS_2 => 2,
            PCSR::PCS_3 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> PCSR {
        match value {
            0 => PCSR::PCS_0,
            1 => PCSR::PCS_1,
            2 => PCSR::PCS_2,
            3 => PCSR::PCS_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `PCS_0`"]
    #[inline]
    pub fn is_pcs_0(&self) -> bool {
        *self == PCSR::PCS_0
    }
    #[doc = "Checks if the value of the field is `PCS_1`"]
    #[inline]
    pub fn is_pcs_1(&self) -> bool {
        *self == PCSR::PCS_1
    }
    #[doc = "Checks if the value of the field is `PCS_2`"]
    #[inline]
    pub fn is_pcs_2(&self) -> bool {
        *self == PCSR::PCS_2
    }
    #[doc = "Checks if the value of the field is `PCS_3`"]
    #[inline]
    pub fn is_pcs_3(&self) -> bool {
        *self == PCSR::PCS_3
    }
}
#[doc = "Possible values of the field `PRESCALE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PRESCALER {
    #[doc = "Divide by 1"]
    PRESCALE_0,
    #[doc = "Divide by 2"]
    PRESCALE_1,
    #[doc = "Divide by 4"]
    PRESCALE_2,
    #[doc = "Divide by 8"]
    PRESCALE_3,
    #[doc = "Divide by 16"]
    PRESCALE_4,
    #[doc = "Divide by 32"]
    PRESCALE_5,
    #[doc = "Divide by 64"]
    PRESCALE_6,
    #[doc = "Divide by 128"]
    PRESCALE_7,
}
impl PRESCALER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            PRESCALER::PRESCALE_0 => 0,
            PRESCALER::PRESCALE_1 => 1,
            PRESCALER::PRESCALE_2 => 2,
            PRESCALER::PRESCALE_3 => 3,
            PRESCALER::PRESCALE_4 => 4,
            PRESCALER::PRESCALE_5 => 5,
            PRESCALER::PRESCALE_6 => 6,
            PRESCALER::PRESCALE_7 => 7,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> PRESCALER {
        match value {
            0 => PRESCALER::PRESCALE_0,
            1 => PRESCALER::PRESCALE_1,
            2 => PRESCALER::PRESCALE_2,
            3 => PRESCALER::PRESCALE_3,
            4 => PRESCALER::PRESCALE_4,
            5 => PRESCALER::PRESCALE_5,
            6 => PRESCALER::PRESCALE_6,
            7 => PRESCALER::PRESCALE_7,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `PRESCALE_0`"]
    #[inline]
    pub fn is_prescale_0(&self) -> bool {
        *self == PRESCALER::PRESCALE_0
    }
    #[doc = "Checks if the value of the field is `PRESCALE_1`"]
    #[inline]
    pub fn is_prescale_1(&self) -> bool {
        *self == PRESCALER::PRESCALE_1
    }
    #[doc = "Checks if the value of the field is `PRESCALE_2`"]
    #[inline]
    pub fn is_prescale_2(&self) -> bool {
        *self == PRESCALER::PRESCALE_2
    }
    #[doc = "Checks if the value of the field is `PRESCALE_3`"]
    #[inline]
    pub fn is_prescale_3(&self) -> bool {
        *self == PRESCALER::PRESCALE_3
    }
    #[doc = "Checks if the value of the field is `PRESCALE_4`"]
    #[inline]
    pub fn is_prescale_4(&self) -> bool {
        *self == PRESCALER::PRESCALE_4
    }
    #[doc = "Checks if the value of the field is `PRESCALE_5`"]
    #[inline]
    pub fn is_prescale_5(&self) -> bool {
        *self == PRESCALER::PRESCALE_5
    }
    #[doc = "Checks if the value of the field is `PRESCALE_6`"]
    #[inline]
    pub fn is_prescale_6(&self) -> bool {
        *self == PRESCALER::PRESCALE_6
    }
    #[doc = "Checks if the value of the field is `PRESCALE_7`"]
    #[inline]
    pub fn is_prescale_7(&self) -> bool {
        *self == PRESCALER::PRESCALE_7
    }
}
#[doc = "Possible values of the field `CPHA`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CPHAR {
    #[doc = "Data is captured on the leading edge of SCK and changed on the following edge of SCK"]
    CPHA_0,
    #[doc = "Data is changed on the leading edge of SCK and captured on the following edge of SCK"]
    CPHA_1,
}
impl CPHAR {
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
            CPHAR::CPHA_0 => false,
            CPHAR::CPHA_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CPHAR {
        match value {
            false => CPHAR::CPHA_0,
            true => CPHAR::CPHA_1,
        }
    }
    #[doc = "Checks if the value of the field is `CPHA_0`"]
    #[inline]
    pub fn is_cpha_0(&self) -> bool {
        *self == CPHAR::CPHA_0
    }
    #[doc = "Checks if the value of the field is `CPHA_1`"]
    #[inline]
    pub fn is_cpha_1(&self) -> bool {
        *self == CPHAR::CPHA_1
    }
}
#[doc = "Possible values of the field `CPOL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CPOLR {
    #[doc = "The inactive state value of SCK is low"]
    CPOL_0,
    #[doc = "The inactive state value of SCK is high"]
    CPOL_1,
}
impl CPOLR {
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
            CPOLR::CPOL_0 => false,
            CPOLR::CPOL_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CPOLR {
        match value {
            false => CPOLR::CPOL_0,
            true => CPOLR::CPOL_1,
        }
    }
    #[doc = "Checks if the value of the field is `CPOL_0`"]
    #[inline]
    pub fn is_cpol_0(&self) -> bool {
        *self == CPOLR::CPOL_0
    }
    #[doc = "Checks if the value of the field is `CPOL_1`"]
    #[inline]
    pub fn is_cpol_1(&self) -> bool {
        *self == CPOLR::CPOL_1
    }
}
#[doc = r" Proxy"]
pub struct _FRAMESZW<'a> {
    w: &'a mut W,
}
impl<'a> _FRAMESZW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        const MASK: u16 = 4095;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `WIDTH`"]
pub enum WIDTHW {
    #[doc = "1 bit transfer"]
    WIDTH_0,
    #[doc = "2 bit transfer"]
    WIDTH_1,
    #[doc = "4 bit transfer"]
    WIDTH_2,
}
impl WIDTHW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            WIDTHW::WIDTH_0 => 0,
            WIDTHW::WIDTH_1 => 1,
            WIDTHW::WIDTH_2 => 2,
        }
    }
}
#[doc = r" Proxy"]
pub struct _WIDTHW<'a> {
    w: &'a mut W,
}
impl<'a> _WIDTHW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: WIDTHW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "1 bit transfer"]
    #[inline]
    pub fn width_0(self) -> &'a mut W {
        self.variant(WIDTHW::WIDTH_0)
    }
    #[doc = "2 bit transfer"]
    #[inline]
    pub fn width_1(self) -> &'a mut W {
        self.variant(WIDTHW::WIDTH_1)
    }
    #[doc = "4 bit transfer"]
    #[inline]
    pub fn width_2(self) -> &'a mut W {
        self.variant(WIDTHW::WIDTH_2)
    }
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
#[doc = "Values that can be written to the field `TXMSK`"]
pub enum TXMSKW {
    #[doc = "Normal transfer"]
    TXMSK_0,
    #[doc = "Mask transmit data"]
    TXMSK_1,
}
impl TXMSKW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TXMSKW::TXMSK_0 => false,
            TXMSKW::TXMSK_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TXMSKW<'a> {
    w: &'a mut W,
}
impl<'a> _TXMSKW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TXMSKW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Normal transfer"]
    #[inline]
    pub fn txmsk_0(self) -> &'a mut W {
        self.variant(TXMSKW::TXMSK_0)
    }
    #[doc = "Mask transmit data"]
    #[inline]
    pub fn txmsk_1(self) -> &'a mut W {
        self.variant(TXMSKW::TXMSK_1)
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
        const OFFSET: u8 = 18;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `RXMSK`"]
pub enum RXMSKW {
    #[doc = "Normal transfer"]
    RXMSK_0,
    #[doc = "Receive data is masked"]
    RXMSK_1,
}
impl RXMSKW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RXMSKW::RXMSK_0 => false,
            RXMSKW::RXMSK_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RXMSKW<'a> {
    w: &'a mut W,
}
impl<'a> _RXMSKW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RXMSKW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Normal transfer"]
    #[inline]
    pub fn rxmsk_0(self) -> &'a mut W {
        self.variant(RXMSKW::RXMSK_0)
    }
    #[doc = "Receive data is masked"]
    #[inline]
    pub fn rxmsk_1(self) -> &'a mut W {
        self.variant(RXMSKW::RXMSK_1)
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
#[doc = "Values that can be written to the field `CONTC`"]
pub enum CONTCW {
    #[doc = "Command word for start of new transfer"]
    CONTC_0,
    #[doc = "Command word for continuing transfer"]
    CONTC_1,
}
impl CONTCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CONTCW::CONTC_0 => false,
            CONTCW::CONTC_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CONTCW<'a> {
    w: &'a mut W,
}
impl<'a> _CONTCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CONTCW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Command word for start of new transfer"]
    #[inline]
    pub fn contc_0(self) -> &'a mut W {
        self.variant(CONTCW::CONTC_0)
    }
    #[doc = "Command word for continuing transfer"]
    #[inline]
    pub fn contc_1(self) -> &'a mut W {
        self.variant(CONTCW::CONTC_1)
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
        const OFFSET: u8 = 20;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `CONT`"]
pub enum CONTW {
    #[doc = "Continuous transfer is disabled"]
    CONT_0,
    #[doc = "Continuous transfer is enabled"]
    CONT_1,
}
impl CONTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CONTW::CONT_0 => false,
            CONTW::CONT_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CONTW<'a> {
    w: &'a mut W,
}
impl<'a> _CONTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CONTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Continuous transfer is disabled"]
    #[inline]
    pub fn cont_0(self) -> &'a mut W {
        self.variant(CONTW::CONT_0)
    }
    #[doc = "Continuous transfer is enabled"]
    #[inline]
    pub fn cont_1(self) -> &'a mut W {
        self.variant(CONTW::CONT_1)
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
        const OFFSET: u8 = 21;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `BYSW`"]
pub enum BYSWW {
    #[doc = "Byte swap is disabled"]
    BYSW_0,
    #[doc = "Byte swap is enabled"]
    BYSW_1,
}
impl BYSWW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            BYSWW::BYSW_0 => false,
            BYSWW::BYSW_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _BYSWW<'a> {
    w: &'a mut W,
}
impl<'a> _BYSWW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: BYSWW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Byte swap is disabled"]
    #[inline]
    pub fn bysw_0(self) -> &'a mut W {
        self.variant(BYSWW::BYSW_0)
    }
    #[doc = "Byte swap is enabled"]
    #[inline]
    pub fn bysw_1(self) -> &'a mut W {
        self.variant(BYSWW::BYSW_1)
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
#[doc = "Values that can be written to the field `LSBF`"]
pub enum LSBFW {
    #[doc = "Data is transferred MSB first"]
    LSBF_0,
    #[doc = "Data is transferred LSB first"]
    LSBF_1,
}
impl LSBFW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            LSBFW::LSBF_0 => false,
            LSBFW::LSBF_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _LSBFW<'a> {
    w: &'a mut W,
}
impl<'a> _LSBFW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: LSBFW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Data is transferred MSB first"]
    #[inline]
    pub fn lsbf_0(self) -> &'a mut W {
        self.variant(LSBFW::LSBF_0)
    }
    #[doc = "Data is transferred LSB first"]
    #[inline]
    pub fn lsbf_1(self) -> &'a mut W {
        self.variant(LSBFW::LSBF_1)
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
#[doc = "Values that can be written to the field `PCS`"]
pub enum PCSW {
    #[doc = "Transfer using LPSPI_PCS[0]"]
    PCS_0,
    #[doc = "Transfer using LPSPI_PCS[1]"]
    PCS_1,
    #[doc = "Transfer using LPSPI_PCS[2]"]
    PCS_2,
    #[doc = "Transfer using LPSPI_PCS[3]"]
    PCS_3,
}
impl PCSW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            PCSW::PCS_0 => 0,
            PCSW::PCS_1 => 1,
            PCSW::PCS_2 => 2,
            PCSW::PCS_3 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PCSW<'a> {
    w: &'a mut W,
}
impl<'a> _PCSW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PCSW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Transfer using LPSPI_PCS[0]"]
    #[inline]
    pub fn pcs_0(self) -> &'a mut W {
        self.variant(PCSW::PCS_0)
    }
    #[doc = "Transfer using LPSPI_PCS[1]"]
    #[inline]
    pub fn pcs_1(self) -> &'a mut W {
        self.variant(PCSW::PCS_1)
    }
    #[doc = "Transfer using LPSPI_PCS[2]"]
    #[inline]
    pub fn pcs_2(self) -> &'a mut W {
        self.variant(PCSW::PCS_2)
    }
    #[doc = "Transfer using LPSPI_PCS[3]"]
    #[inline]
    pub fn pcs_3(self) -> &'a mut W {
        self.variant(PCSW::PCS_3)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 24;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `PRESCALE`"]
pub enum PRESCALEW {
    #[doc = "Divide by 1"]
    PRESCALE_0,
    #[doc = "Divide by 2"]
    PRESCALE_1,
    #[doc = "Divide by 4"]
    PRESCALE_2,
    #[doc = "Divide by 8"]
    PRESCALE_3,
    #[doc = "Divide by 16"]
    PRESCALE_4,
    #[doc = "Divide by 32"]
    PRESCALE_5,
    #[doc = "Divide by 64"]
    PRESCALE_6,
    #[doc = "Divide by 128"]
    PRESCALE_7,
}
impl PRESCALEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            PRESCALEW::PRESCALE_0 => 0,
            PRESCALEW::PRESCALE_1 => 1,
            PRESCALEW::PRESCALE_2 => 2,
            PRESCALEW::PRESCALE_3 => 3,
            PRESCALEW::PRESCALE_4 => 4,
            PRESCALEW::PRESCALE_5 => 5,
            PRESCALEW::PRESCALE_6 => 6,
            PRESCALEW::PRESCALE_7 => 7,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PRESCALEW<'a> {
    w: &'a mut W,
}
impl<'a> _PRESCALEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PRESCALEW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Divide by 1"]
    #[inline]
    pub fn prescale_0(self) -> &'a mut W {
        self.variant(PRESCALEW::PRESCALE_0)
    }
    #[doc = "Divide by 2"]
    #[inline]
    pub fn prescale_1(self) -> &'a mut W {
        self.variant(PRESCALEW::PRESCALE_1)
    }
    #[doc = "Divide by 4"]
    #[inline]
    pub fn prescale_2(self) -> &'a mut W {
        self.variant(PRESCALEW::PRESCALE_2)
    }
    #[doc = "Divide by 8"]
    #[inline]
    pub fn prescale_3(self) -> &'a mut W {
        self.variant(PRESCALEW::PRESCALE_3)
    }
    #[doc = "Divide by 16"]
    #[inline]
    pub fn prescale_4(self) -> &'a mut W {
        self.variant(PRESCALEW::PRESCALE_4)
    }
    #[doc = "Divide by 32"]
    #[inline]
    pub fn prescale_5(self) -> &'a mut W {
        self.variant(PRESCALEW::PRESCALE_5)
    }
    #[doc = "Divide by 64"]
    #[inline]
    pub fn prescale_6(self) -> &'a mut W {
        self.variant(PRESCALEW::PRESCALE_6)
    }
    #[doc = "Divide by 128"]
    #[inline]
    pub fn prescale_7(self) -> &'a mut W {
        self.variant(PRESCALEW::PRESCALE_7)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 27;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `CPHA`"]
pub enum CPHAW {
    #[doc = "Data is captured on the leading edge of SCK and changed on the following edge of SCK"]
    CPHA_0,
    #[doc = "Data is changed on the leading edge of SCK and captured on the following edge of SCK"]
    CPHA_1,
}
impl CPHAW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CPHAW::CPHA_0 => false,
            CPHAW::CPHA_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CPHAW<'a> {
    w: &'a mut W,
}
impl<'a> _CPHAW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CPHAW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Data is captured on the leading edge of SCK and changed on the following edge of SCK"]
    #[inline]
    pub fn cpha_0(self) -> &'a mut W {
        self.variant(CPHAW::CPHA_0)
    }
    #[doc = "Data is changed on the leading edge of SCK and captured on the following edge of SCK"]
    #[inline]
    pub fn cpha_1(self) -> &'a mut W {
        self.variant(CPHAW::CPHA_1)
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
        const OFFSET: u8 = 30;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `CPOL`"]
pub enum CPOLW {
    #[doc = "The inactive state value of SCK is low"]
    CPOL_0,
    #[doc = "The inactive state value of SCK is high"]
    CPOL_1,
}
impl CPOLW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CPOLW::CPOL_0 => false,
            CPOLW::CPOL_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CPOLW<'a> {
    w: &'a mut W,
}
impl<'a> _CPOLW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CPOLW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The inactive state value of SCK is low"]
    #[inline]
    pub fn cpol_0(self) -> &'a mut W {
        self.variant(CPOLW::CPOL_0)
    }
    #[doc = "The inactive state value of SCK is high"]
    #[inline]
    pub fn cpol_1(self) -> &'a mut W {
        self.variant(CPOLW::CPOL_1)
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
    #[doc = "Bits 0:11 - Frame Size"]
    #[inline]
    pub fn framesz(&self) -> FRAMESZR {
        let bits = {
            const MASK: u16 = 4095;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        };
        FRAMESZR { bits }
    }
    #[doc = "Bits 16:17 - Transfer Width"]
    #[inline]
    pub fn width(&self) -> WIDTHR {
        WIDTHR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 18 - Transmit Data Mask"]
    #[inline]
    pub fn txmsk(&self) -> TXMSKR {
        TXMSKR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 19 - Receive Data Mask"]
    #[inline]
    pub fn rxmsk(&self) -> RXMSKR {
        RXMSKR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 19;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 20 - Continuing Command"]
    #[inline]
    pub fn contc(&self) -> CONTCR {
        CONTCR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 21 - Continuous Transfer"]
    #[inline]
    pub fn cont(&self) -> CONTR {
        CONTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 21;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 22 - Byte Swap"]
    #[inline]
    pub fn bysw(&self) -> BYSWR {
        BYSWR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 22;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 23 - LSB First"]
    #[inline]
    pub fn lsbf(&self) -> LSBFR {
        LSBFR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 23;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 24:25 - Peripheral Chip Select"]
    #[inline]
    pub fn pcs(&self) -> PCSR {
        PCSR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 27:29 - Prescaler Value"]
    #[inline]
    pub fn prescale(&self) -> PRESCALER {
        PRESCALER::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 27;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 30 - Clock Phase"]
    #[inline]
    pub fn cpha(&self) -> CPHAR {
        CPHAR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 30;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 31 - Clock Polarity"]
    #[inline]
    pub fn cpol(&self) -> CPOLR {
        CPOLR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 31;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 31 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:11 - Frame Size"]
    #[inline]
    pub fn framesz(&mut self) -> _FRAMESZW {
        _FRAMESZW { w: self }
    }
    #[doc = "Bits 16:17 - Transfer Width"]
    #[inline]
    pub fn width(&mut self) -> _WIDTHW {
        _WIDTHW { w: self }
    }
    #[doc = "Bit 18 - Transmit Data Mask"]
    #[inline]
    pub fn txmsk(&mut self) -> _TXMSKW {
        _TXMSKW { w: self }
    }
    #[doc = "Bit 19 - Receive Data Mask"]
    #[inline]
    pub fn rxmsk(&mut self) -> _RXMSKW {
        _RXMSKW { w: self }
    }
    #[doc = "Bit 20 - Continuing Command"]
    #[inline]
    pub fn contc(&mut self) -> _CONTCW {
        _CONTCW { w: self }
    }
    #[doc = "Bit 21 - Continuous Transfer"]
    #[inline]
    pub fn cont(&mut self) -> _CONTW {
        _CONTW { w: self }
    }
    #[doc = "Bit 22 - Byte Swap"]
    #[inline]
    pub fn bysw(&mut self) -> _BYSWW {
        _BYSWW { w: self }
    }
    #[doc = "Bit 23 - LSB First"]
    #[inline]
    pub fn lsbf(&mut self) -> _LSBFW {
        _LSBFW { w: self }
    }
    #[doc = "Bits 24:25 - Peripheral Chip Select"]
    #[inline]
    pub fn pcs(&mut self) -> _PCSW {
        _PCSW { w: self }
    }
    #[doc = "Bits 27:29 - Prescaler Value"]
    #[inline]
    pub fn prescale(&mut self) -> _PRESCALEW {
        _PRESCALEW { w: self }
    }
    #[doc = "Bit 30 - Clock Phase"]
    #[inline]
    pub fn cpha(&mut self) -> _CPHAW {
        _CPHAW { w: self }
    }
    #[doc = "Bit 31 - Clock Polarity"]
    #[inline]
    pub fn cpol(&mut self) -> _CPOLW {
        _CPOLW { w: self }
    }
}
