#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::PORTSC1 {
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
pub struct CCSR {
    bits: bool,
}
impl CCSR {
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
pub struct CSCR {
    bits: bool,
}
impl CSCR {
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
pub struct PER {
    bits: bool,
}
impl PER {
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
pub struct PECR {
    bits: bool,
}
impl PECR {
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
#[doc = "Possible values of the field `OCA`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OCAR {
    #[doc = "This port does not have an over-current condition."]
    OCA_0,
    #[doc = "This port currently has an over-current condition"]
    OCA_1,
}
impl OCAR {
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
            OCAR::OCA_0 => false,
            OCAR::OCA_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> OCAR {
        match value {
            false => OCAR::OCA_0,
            true => OCAR::OCA_1,
        }
    }
    #[doc = "Checks if the value of the field is `OCA_0`"]
    #[inline]
    pub fn is_oca_0(&self) -> bool {
        *self == OCAR::OCA_0
    }
    #[doc = "Checks if the value of the field is `OCA_1`"]
    #[inline]
    pub fn is_oca_1(&self) -> bool {
        *self == OCAR::OCA_1
    }
}
#[doc = r" Value of the field"]
pub struct OCCR {
    bits: bool,
}
impl OCCR {
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
pub struct FPRR {
    bits: bool,
}
impl FPRR {
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
pub struct SUSPR {
    bits: bool,
}
impl SUSPR {
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
pub struct PRR {
    bits: bool,
}
impl PRR {
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
pub struct HSPR {
    bits: bool,
}
impl HSPR {
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
#[doc = "Possible values of the field `LS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LSR {
    #[doc = "SE0"]
    LS_0,
    #[doc = "K-state"]
    LS_1,
    #[doc = "J-state"]
    LS_2,
    #[doc = "Undefined"]
    LS_3,
}
impl LSR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            LSR::LS_0 => 0,
            LSR::LS_1 => 1,
            LSR::LS_2 => 2,
            LSR::LS_3 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> LSR {
        match value {
            0 => LSR::LS_0,
            1 => LSR::LS_1,
            2 => LSR::LS_2,
            3 => LSR::LS_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `LS_0`"]
    #[inline]
    pub fn is_ls_0(&self) -> bool {
        *self == LSR::LS_0
    }
    #[doc = "Checks if the value of the field is `LS_1`"]
    #[inline]
    pub fn is_ls_1(&self) -> bool {
        *self == LSR::LS_1
    }
    #[doc = "Checks if the value of the field is `LS_2`"]
    #[inline]
    pub fn is_ls_2(&self) -> bool {
        *self == LSR::LS_2
    }
    #[doc = "Checks if the value of the field is `LS_3`"]
    #[inline]
    pub fn is_ls_3(&self) -> bool {
        *self == LSR::LS_3
    }
}
#[doc = r" Value of the field"]
pub struct PPR {
    bits: bool,
}
impl PPR {
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
pub struct POR {
    bits: bool,
}
impl POR {
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
#[doc = "Possible values of the field `PIC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PICR {
    #[doc = "Port indicators are off"]
    PIC_0,
    #[doc = "Amber"]
    PIC_1,
    #[doc = "Green"]
    PIC_2,
    #[doc = "Undefined"]
    PIC_3,
}
impl PICR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            PICR::PIC_0 => 0,
            PICR::PIC_1 => 1,
            PICR::PIC_2 => 2,
            PICR::PIC_3 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> PICR {
        match value {
            0 => PICR::PIC_0,
            1 => PICR::PIC_1,
            2 => PICR::PIC_2,
            3 => PICR::PIC_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `PIC_0`"]
    #[inline]
    pub fn is_pic_0(&self) -> bool {
        *self == PICR::PIC_0
    }
    #[doc = "Checks if the value of the field is `PIC_1`"]
    #[inline]
    pub fn is_pic_1(&self) -> bool {
        *self == PICR::PIC_1
    }
    #[doc = "Checks if the value of the field is `PIC_2`"]
    #[inline]
    pub fn is_pic_2(&self) -> bool {
        *self == PICR::PIC_2
    }
    #[doc = "Checks if the value of the field is `PIC_3`"]
    #[inline]
    pub fn is_pic_3(&self) -> bool {
        *self == PICR::PIC_3
    }
}
#[doc = "Possible values of the field `PTC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PTCR {
    #[doc = "TEST_MODE_DISABLE"]
    PTC_0,
    #[doc = "J_STATE"]
    PTC_1,
    #[doc = "K_STATE"]
    PTC_2,
    #[doc = "SE0 (host) / NAK (device)"]
    PTC_3,
    #[doc = "Packet"]
    PTC_4,
    #[doc = "FORCE_ENABLE_HS"]
    PTC_5,
    #[doc = "FORCE_ENABLE_FS"]
    PTC_6,
    #[doc = "FORCE_ENABLE_LS"]
    PTC_7,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl PTCR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            PTCR::PTC_0 => 0,
            PTCR::PTC_1 => 1,
            PTCR::PTC_2 => 2,
            PTCR::PTC_3 => 3,
            PTCR::PTC_4 => 4,
            PTCR::PTC_5 => 5,
            PTCR::PTC_6 => 6,
            PTCR::PTC_7 => 7,
            PTCR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> PTCR {
        match value {
            0 => PTCR::PTC_0,
            1 => PTCR::PTC_1,
            2 => PTCR::PTC_2,
            3 => PTCR::PTC_3,
            4 => PTCR::PTC_4,
            5 => PTCR::PTC_5,
            6 => PTCR::PTC_6,
            7 => PTCR::PTC_7,
            i => PTCR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `PTC_0`"]
    #[inline]
    pub fn is_ptc_0(&self) -> bool {
        *self == PTCR::PTC_0
    }
    #[doc = "Checks if the value of the field is `PTC_1`"]
    #[inline]
    pub fn is_ptc_1(&self) -> bool {
        *self == PTCR::PTC_1
    }
    #[doc = "Checks if the value of the field is `PTC_2`"]
    #[inline]
    pub fn is_ptc_2(&self) -> bool {
        *self == PTCR::PTC_2
    }
    #[doc = "Checks if the value of the field is `PTC_3`"]
    #[inline]
    pub fn is_ptc_3(&self) -> bool {
        *self == PTCR::PTC_3
    }
    #[doc = "Checks if the value of the field is `PTC_4`"]
    #[inline]
    pub fn is_ptc_4(&self) -> bool {
        *self == PTCR::PTC_4
    }
    #[doc = "Checks if the value of the field is `PTC_5`"]
    #[inline]
    pub fn is_ptc_5(&self) -> bool {
        *self == PTCR::PTC_5
    }
    #[doc = "Checks if the value of the field is `PTC_6`"]
    #[inline]
    pub fn is_ptc_6(&self) -> bool {
        *self == PTCR::PTC_6
    }
    #[doc = "Checks if the value of the field is `PTC_7`"]
    #[inline]
    pub fn is_ptc_7(&self) -> bool {
        *self == PTCR::PTC_7
    }
}
#[doc = r" Value of the field"]
pub struct WKCNR {
    bits: bool,
}
impl WKCNR {
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
pub struct WKDCR {
    bits: bool,
}
impl WKDCR {
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
pub struct WKOCR {
    bits: bool,
}
impl WKOCR {
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
#[doc = "Possible values of the field `PHCD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PHCDR {
    #[doc = "Enable PHY clock"]
    PHCD_0,
    #[doc = "Disable PHY clock"]
    PHCD_1,
}
impl PHCDR {
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
            PHCDR::PHCD_0 => false,
            PHCDR::PHCD_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PHCDR {
        match value {
            false => PHCDR::PHCD_0,
            true => PHCDR::PHCD_1,
        }
    }
    #[doc = "Checks if the value of the field is `PHCD_0`"]
    #[inline]
    pub fn is_phcd_0(&self) -> bool {
        *self == PHCDR::PHCD_0
    }
    #[doc = "Checks if the value of the field is `PHCD_1`"]
    #[inline]
    pub fn is_phcd_1(&self) -> bool {
        *self == PHCDR::PHCD_1
    }
}
#[doc = "Possible values of the field `PFSC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PFSCR {
    #[doc = "Normal operation"]
    PFSC_0,
    #[doc = "Forced to full speed"]
    PFSC_1,
}
impl PFSCR {
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
            PFSCR::PFSC_0 => false,
            PFSCR::PFSC_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PFSCR {
        match value {
            false => PFSCR::PFSC_0,
            true => PFSCR::PFSC_1,
        }
    }
    #[doc = "Checks if the value of the field is `PFSC_0`"]
    #[inline]
    pub fn is_pfsc_0(&self) -> bool {
        *self == PFSCR::PFSC_0
    }
    #[doc = "Checks if the value of the field is `PFSC_1`"]
    #[inline]
    pub fn is_pfsc_1(&self) -> bool {
        *self == PFSCR::PFSC_1
    }
}
#[doc = r" Value of the field"]
pub struct PTS_2R {
    bits: bool,
}
impl PTS_2R {
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
#[doc = "Possible values of the field `PSPD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PSPDR {
    #[doc = "Full Speed"]
    PSPD_0,
    #[doc = "Low Speed"]
    PSPD_1,
    #[doc = "High Speed"]
    PSPD_2,
    #[doc = "Undefined"]
    PSPD_3,
}
impl PSPDR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            PSPDR::PSPD_0 => 0,
            PSPDR::PSPD_1 => 1,
            PSPDR::PSPD_2 => 2,
            PSPDR::PSPD_3 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> PSPDR {
        match value {
            0 => PSPDR::PSPD_0,
            1 => PSPDR::PSPD_1,
            2 => PSPDR::PSPD_2,
            3 => PSPDR::PSPD_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `PSPD_0`"]
    #[inline]
    pub fn is_pspd_0(&self) -> bool {
        *self == PSPDR::PSPD_0
    }
    #[doc = "Checks if the value of the field is `PSPD_1`"]
    #[inline]
    pub fn is_pspd_1(&self) -> bool {
        *self == PSPDR::PSPD_1
    }
    #[doc = "Checks if the value of the field is `PSPD_2`"]
    #[inline]
    pub fn is_pspd_2(&self) -> bool {
        *self == PSPDR::PSPD_2
    }
    #[doc = "Checks if the value of the field is `PSPD_3`"]
    #[inline]
    pub fn is_pspd_3(&self) -> bool {
        *self == PSPDR::PSPD_3
    }
}
#[doc = "Possible values of the field `PTW`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PTWR {
    #[doc = "Select the 8-bit UTMI interface [60MHz]"]
    PTW_0,
    #[doc = "Select the 16-bit UTMI interface [30MHz]"]
    PTW_1,
}
impl PTWR {
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
            PTWR::PTW_0 => false,
            PTWR::PTW_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PTWR {
        match value {
            false => PTWR::PTW_0,
            true => PTWR::PTW_1,
        }
    }
    #[doc = "Checks if the value of the field is `PTW_0`"]
    #[inline]
    pub fn is_ptw_0(&self) -> bool {
        *self == PTWR::PTW_0
    }
    #[doc = "Checks if the value of the field is `PTW_1`"]
    #[inline]
    pub fn is_ptw_1(&self) -> bool {
        *self == PTWR::PTW_1
    }
}
#[doc = r" Value of the field"]
pub struct STSR {
    bits: bool,
}
impl STSR {
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
pub struct PTS_1R {
    bits: u8,
}
impl PTS_1R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Proxy"]
pub struct _CSCW<'a> {
    w: &'a mut W,
}
impl<'a> _CSCW<'a> {
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
pub struct _PEW<'a> {
    w: &'a mut W,
}
impl<'a> _PEW<'a> {
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
pub struct _PECW<'a> {
    w: &'a mut W,
}
impl<'a> _PECW<'a> {
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
pub struct _OCCW<'a> {
    w: &'a mut W,
}
impl<'a> _OCCW<'a> {
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
pub struct _FPRW<'a> {
    w: &'a mut W,
}
impl<'a> _FPRW<'a> {
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
pub struct _SUSPW<'a> {
    w: &'a mut W,
}
impl<'a> _SUSPW<'a> {
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
#[doc = r" Proxy"]
pub struct _PRW<'a> {
    w: &'a mut W,
}
impl<'a> _PRW<'a> {
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
#[doc = "Values that can be written to the field `LS`"]
pub enum LSW {
    #[doc = "SE0"]
    LS_0,
    #[doc = "K-state"]
    LS_1,
    #[doc = "J-state"]
    LS_2,
    #[doc = "Undefined"]
    LS_3,
}
impl LSW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            LSW::LS_0 => 0,
            LSW::LS_1 => 1,
            LSW::LS_2 => 2,
            LSW::LS_3 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _LSW<'a> {
    w: &'a mut W,
}
impl<'a> _LSW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: LSW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "SE0"]
    #[inline]
    pub fn ls_0(self) -> &'a mut W {
        self.variant(LSW::LS_0)
    }
    #[doc = "K-state"]
    #[inline]
    pub fn ls_1(self) -> &'a mut W {
        self.variant(LSW::LS_1)
    }
    #[doc = "J-state"]
    #[inline]
    pub fn ls_2(self) -> &'a mut W {
        self.variant(LSW::LS_2)
    }
    #[doc = "Undefined"]
    #[inline]
    pub fn ls_3(self) -> &'a mut W {
        self.variant(LSW::LS_3)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 10;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _PPW<'a> {
    w: &'a mut W,
}
impl<'a> _PPW<'a> {
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
        const OFFSET: u8 = 12;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _POW<'a> {
    w: &'a mut W,
}
impl<'a> _POW<'a> {
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
#[doc = "Values that can be written to the field `PIC`"]
pub enum PICW {
    #[doc = "Port indicators are off"]
    PIC_0,
    #[doc = "Amber"]
    PIC_1,
    #[doc = "Green"]
    PIC_2,
    #[doc = "Undefined"]
    PIC_3,
}
impl PICW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            PICW::PIC_0 => 0,
            PICW::PIC_1 => 1,
            PICW::PIC_2 => 2,
            PICW::PIC_3 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PICW<'a> {
    w: &'a mut W,
}
impl<'a> _PICW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PICW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Port indicators are off"]
    #[inline]
    pub fn pic_0(self) -> &'a mut W {
        self.variant(PICW::PIC_0)
    }
    #[doc = "Amber"]
    #[inline]
    pub fn pic_1(self) -> &'a mut W {
        self.variant(PICW::PIC_1)
    }
    #[doc = "Green"]
    #[inline]
    pub fn pic_2(self) -> &'a mut W {
        self.variant(PICW::PIC_2)
    }
    #[doc = "Undefined"]
    #[inline]
    pub fn pic_3(self) -> &'a mut W {
        self.variant(PICW::PIC_3)
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
#[doc = "Values that can be written to the field `PTC`"]
pub enum PTCW {
    #[doc = "TEST_MODE_DISABLE"]
    PTC_0,
    #[doc = "J_STATE"]
    PTC_1,
    #[doc = "K_STATE"]
    PTC_2,
    #[doc = "SE0 (host) / NAK (device)"]
    PTC_3,
    #[doc = "Packet"]
    PTC_4,
    #[doc = "FORCE_ENABLE_HS"]
    PTC_5,
    #[doc = "FORCE_ENABLE_FS"]
    PTC_6,
    #[doc = "FORCE_ENABLE_LS"]
    PTC_7,
}
impl PTCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            PTCW::PTC_0 => 0,
            PTCW::PTC_1 => 1,
            PTCW::PTC_2 => 2,
            PTCW::PTC_3 => 3,
            PTCW::PTC_4 => 4,
            PTCW::PTC_5 => 5,
            PTCW::PTC_6 => 6,
            PTCW::PTC_7 => 7,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PTCW<'a> {
    w: &'a mut W,
}
impl<'a> _PTCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PTCW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "TEST_MODE_DISABLE"]
    #[inline]
    pub fn ptc_0(self) -> &'a mut W {
        self.variant(PTCW::PTC_0)
    }
    #[doc = "J_STATE"]
    #[inline]
    pub fn ptc_1(self) -> &'a mut W {
        self.variant(PTCW::PTC_1)
    }
    #[doc = "K_STATE"]
    #[inline]
    pub fn ptc_2(self) -> &'a mut W {
        self.variant(PTCW::PTC_2)
    }
    #[doc = "SE0 (host) / NAK (device)"]
    #[inline]
    pub fn ptc_3(self) -> &'a mut W {
        self.variant(PTCW::PTC_3)
    }
    #[doc = "Packet"]
    #[inline]
    pub fn ptc_4(self) -> &'a mut W {
        self.variant(PTCW::PTC_4)
    }
    #[doc = "FORCE_ENABLE_HS"]
    #[inline]
    pub fn ptc_5(self) -> &'a mut W {
        self.variant(PTCW::PTC_5)
    }
    #[doc = "FORCE_ENABLE_FS"]
    #[inline]
    pub fn ptc_6(self) -> &'a mut W {
        self.variant(PTCW::PTC_6)
    }
    #[doc = "FORCE_ENABLE_LS"]
    #[inline]
    pub fn ptc_7(self) -> &'a mut W {
        self.variant(PTCW::PTC_7)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 16;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _WKCNW<'a> {
    w: &'a mut W,
}
impl<'a> _WKCNW<'a> {
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
pub struct _WKDCW<'a> {
    w: &'a mut W,
}
impl<'a> _WKDCW<'a> {
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
pub struct _WKOCW<'a> {
    w: &'a mut W,
}
impl<'a> _WKOCW<'a> {
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
#[doc = "Values that can be written to the field `PHCD`"]
pub enum PHCDW {
    #[doc = "Enable PHY clock"]
    PHCD_0,
    #[doc = "Disable PHY clock"]
    PHCD_1,
}
impl PHCDW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PHCDW::PHCD_0 => false,
            PHCDW::PHCD_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PHCDW<'a> {
    w: &'a mut W,
}
impl<'a> _PHCDW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PHCDW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Enable PHY clock"]
    #[inline]
    pub fn phcd_0(self) -> &'a mut W {
        self.variant(PHCDW::PHCD_0)
    }
    #[doc = "Disable PHY clock"]
    #[inline]
    pub fn phcd_1(self) -> &'a mut W {
        self.variant(PHCDW::PHCD_1)
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
#[doc = "Values that can be written to the field `PFSC`"]
pub enum PFSCW {
    #[doc = "Normal operation"]
    PFSC_0,
    #[doc = "Forced to full speed"]
    PFSC_1,
}
impl PFSCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PFSCW::PFSC_0 => false,
            PFSCW::PFSC_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PFSCW<'a> {
    w: &'a mut W,
}
impl<'a> _PFSCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PFSCW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Normal operation"]
    #[inline]
    pub fn pfsc_0(self) -> &'a mut W {
        self.variant(PFSCW::PFSC_0)
    }
    #[doc = "Forced to full speed"]
    #[inline]
    pub fn pfsc_1(self) -> &'a mut W {
        self.variant(PFSCW::PFSC_1)
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
#[doc = r" Proxy"]
pub struct _PTS_2W<'a> {
    w: &'a mut W,
}
impl<'a> _PTS_2W<'a> {
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
#[doc = "Values that can be written to the field `PSPD`"]
pub enum PSPDW {
    #[doc = "Full Speed"]
    PSPD_0,
    #[doc = "Low Speed"]
    PSPD_1,
    #[doc = "High Speed"]
    PSPD_2,
    #[doc = "Undefined"]
    PSPD_3,
}
impl PSPDW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            PSPDW::PSPD_0 => 0,
            PSPDW::PSPD_1 => 1,
            PSPDW::PSPD_2 => 2,
            PSPDW::PSPD_3 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PSPDW<'a> {
    w: &'a mut W,
}
impl<'a> _PSPDW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PSPDW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Full Speed"]
    #[inline]
    pub fn pspd_0(self) -> &'a mut W {
        self.variant(PSPDW::PSPD_0)
    }
    #[doc = "Low Speed"]
    #[inline]
    pub fn pspd_1(self) -> &'a mut W {
        self.variant(PSPDW::PSPD_1)
    }
    #[doc = "High Speed"]
    #[inline]
    pub fn pspd_2(self) -> &'a mut W {
        self.variant(PSPDW::PSPD_2)
    }
    #[doc = "Undefined"]
    #[inline]
    pub fn pspd_3(self) -> &'a mut W {
        self.variant(PSPDW::PSPD_3)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 26;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `PTW`"]
pub enum PTWW {
    #[doc = "Select the 8-bit UTMI interface [60MHz]"]
    PTW_0,
    #[doc = "Select the 16-bit UTMI interface [30MHz]"]
    PTW_1,
}
impl PTWW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PTWW::PTW_0 => false,
            PTWW::PTW_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PTWW<'a> {
    w: &'a mut W,
}
impl<'a> _PTWW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PTWW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Select the 8-bit UTMI interface [60MHz]"]
    #[inline]
    pub fn ptw_0(self) -> &'a mut W {
        self.variant(PTWW::PTW_0)
    }
    #[doc = "Select the 16-bit UTMI interface [30MHz]"]
    #[inline]
    pub fn ptw_1(self) -> &'a mut W {
        self.variant(PTWW::PTW_1)
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
        const OFFSET: u8 = 28;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _STSW<'a> {
    w: &'a mut W,
}
impl<'a> _STSW<'a> {
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
pub struct _PTS_1W<'a> {
    w: &'a mut W,
}
impl<'a> _PTS_1W<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 30;
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
    #[doc = "Bit 0 - Current Connect Status-Read Only"]
    #[inline]
    pub fn ccs(&self) -> CCSR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        CCSR { bits }
    }
    #[doc = "Bit 1 - Connect Status Change-R/WC"]
    #[inline]
    pub fn csc(&self) -> CSCR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        CSCR { bits }
    }
    #[doc = "Bit 2 - Port Enabled/Disabled-Read/Write"]
    #[inline]
    pub fn pe(&self) -> PER {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        PER { bits }
    }
    #[doc = "Bit 3 - Port Enable/Disable Change-R/WC"]
    #[inline]
    pub fn pec(&self) -> PECR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        PECR { bits }
    }
    #[doc = "Bit 4 - Over-current Active-Read Only"]
    #[inline]
    pub fn oca(&self) -> OCAR {
        OCAR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 5 - Over-current Change-R/WC"]
    #[inline]
    pub fn occ(&self) -> OCCR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        OCCR { bits }
    }
    #[doc = "Bit 6 - Force Port Resume -Read/Write"]
    #[inline]
    pub fn fpr(&self) -> FPRR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        FPRR { bits }
    }
    #[doc = "Bit 7 - Suspend - Read/Write or Read Only"]
    #[inline]
    pub fn susp(&self) -> SUSPR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        SUSPR { bits }
    }
    #[doc = "Bit 8 - Port Reset - Read/Write or Read Only"]
    #[inline]
    pub fn pr(&self) -> PRR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        PRR { bits }
    }
    #[doc = "Bit 9 - High-Speed Port - Read Only"]
    #[inline]
    pub fn hsp(&self) -> HSPR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        HSPR { bits }
    }
    #[doc = "Bits 10:11 - Line Status-Read Only"]
    #[inline]
    pub fn ls(&self) -> LSR {
        LSR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 12 - Port Power (PP)-Read/Write or Read Only"]
    #[inline]
    pub fn pp(&self) -> PPR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        PPR { bits }
    }
    #[doc = "Bit 13 - Port Owner-Read/Write"]
    #[inline]
    pub fn po(&self) -> POR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        POR { bits }
    }
    #[doc = "Bits 14:15 - Port Indicator Control - Read/Write"]
    #[inline]
    pub fn pic(&self) -> PICR {
        PICR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 16:19 - Port Test Control - Read/Write"]
    #[inline]
    pub fn ptc(&self) -> PTCR {
        PTCR::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 20 - Wake on Connect Enable (WKCNNT_E) - Read/Write"]
    #[inline]
    pub fn wkcn(&self) -> WKCNR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        WKCNR { bits }
    }
    #[doc = "Bit 21 - Wake on Disconnect Enable (WKDSCNNT_E) - Read/Write"]
    #[inline]
    pub fn wkdc(&self) -> WKDCR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 21;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        WKDCR { bits }
    }
    #[doc = "Bit 22 - Wake on Over-current Enable (WKOC_E) - Read/Write"]
    #[inline]
    pub fn wkoc(&self) -> WKOCR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 22;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        WKOCR { bits }
    }
    #[doc = "Bit 23 - PHY Low Power Suspend - Clock Disable (PLPSCD) - Read/Write"]
    #[inline]
    pub fn phcd(&self) -> PHCDR {
        PHCDR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 23;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 24 - Port Force Full Speed Connect - Read/Write"]
    #[inline]
    pub fn pfsc(&self) -> PFSCR {
        PFSCR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 25 - See description at bits 31-30"]
    #[inline]
    pub fn pts_2(&self) -> PTS_2R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 25;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        PTS_2R { bits }
    }
    #[doc = "Bits 26:27 - Port Speed - Read Only. This register field indicates the speed at which the port is operating."]
    #[inline]
    pub fn pspd(&self) -> PSPDR {
        PSPDR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 26;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 28 - Parallel Transceiver Width This bit has no effect if serial interface engine is used"]
    #[inline]
    pub fn ptw(&self) -> PTWR {
        PTWR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 28;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 29 - Serial Transceiver Select 1 Serial Interface Engine is selected 0 Parallel Interface signals is selected Serial Interface Engine can be used in combination with UTMI+/ULPI physical interface to provide FS/LS signaling instead of the parallel interface signals"]
    #[inline]
    pub fn sts(&self) -> STSR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 29;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        STSR { bits }
    }
    #[doc = "Bits 30:31 - All USB port interface modes are listed in this field description, but not all are supported"]
    #[inline]
    pub fn pts_1(&self) -> PTS_1R {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 30;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        PTS_1R { bits }
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 268435456 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 1 - Connect Status Change-R/WC"]
    #[inline]
    pub fn csc(&mut self) -> _CSCW {
        _CSCW { w: self }
    }
    #[doc = "Bit 2 - Port Enabled/Disabled-Read/Write"]
    #[inline]
    pub fn pe(&mut self) -> _PEW {
        _PEW { w: self }
    }
    #[doc = "Bit 3 - Port Enable/Disable Change-R/WC"]
    #[inline]
    pub fn pec(&mut self) -> _PECW {
        _PECW { w: self }
    }
    #[doc = "Bit 5 - Over-current Change-R/WC"]
    #[inline]
    pub fn occ(&mut self) -> _OCCW {
        _OCCW { w: self }
    }
    #[doc = "Bit 6 - Force Port Resume -Read/Write"]
    #[inline]
    pub fn fpr(&mut self) -> _FPRW {
        _FPRW { w: self }
    }
    #[doc = "Bit 7 - Suspend - Read/Write or Read Only"]
    #[inline]
    pub fn susp(&mut self) -> _SUSPW {
        _SUSPW { w: self }
    }
    #[doc = "Bit 8 - Port Reset - Read/Write or Read Only"]
    #[inline]
    pub fn pr(&mut self) -> _PRW {
        _PRW { w: self }
    }
    #[doc = "Bits 10:11 - Line Status-Read Only"]
    #[inline]
    pub fn ls(&mut self) -> _LSW {
        _LSW { w: self }
    }
    #[doc = "Bit 12 - Port Power (PP)-Read/Write or Read Only"]
    #[inline]
    pub fn pp(&mut self) -> _PPW {
        _PPW { w: self }
    }
    #[doc = "Bit 13 - Port Owner-Read/Write"]
    #[inline]
    pub fn po(&mut self) -> _POW {
        _POW { w: self }
    }
    #[doc = "Bits 14:15 - Port Indicator Control - Read/Write"]
    #[inline]
    pub fn pic(&mut self) -> _PICW {
        _PICW { w: self }
    }
    #[doc = "Bits 16:19 - Port Test Control - Read/Write"]
    #[inline]
    pub fn ptc(&mut self) -> _PTCW {
        _PTCW { w: self }
    }
    #[doc = "Bit 20 - Wake on Connect Enable (WKCNNT_E) - Read/Write"]
    #[inline]
    pub fn wkcn(&mut self) -> _WKCNW {
        _WKCNW { w: self }
    }
    #[doc = "Bit 21 - Wake on Disconnect Enable (WKDSCNNT_E) - Read/Write"]
    #[inline]
    pub fn wkdc(&mut self) -> _WKDCW {
        _WKDCW { w: self }
    }
    #[doc = "Bit 22 - Wake on Over-current Enable (WKOC_E) - Read/Write"]
    #[inline]
    pub fn wkoc(&mut self) -> _WKOCW {
        _WKOCW { w: self }
    }
    #[doc = "Bit 23 - PHY Low Power Suspend - Clock Disable (PLPSCD) - Read/Write"]
    #[inline]
    pub fn phcd(&mut self) -> _PHCDW {
        _PHCDW { w: self }
    }
    #[doc = "Bit 24 - Port Force Full Speed Connect - Read/Write"]
    #[inline]
    pub fn pfsc(&mut self) -> _PFSCW {
        _PFSCW { w: self }
    }
    #[doc = "Bit 25 - See description at bits 31-30"]
    #[inline]
    pub fn pts_2(&mut self) -> _PTS_2W {
        _PTS_2W { w: self }
    }
    #[doc = "Bits 26:27 - Port Speed - Read Only. This register field indicates the speed at which the port is operating."]
    #[inline]
    pub fn pspd(&mut self) -> _PSPDW {
        _PSPDW { w: self }
    }
    #[doc = "Bit 28 - Parallel Transceiver Width This bit has no effect if serial interface engine is used"]
    #[inline]
    pub fn ptw(&mut self) -> _PTWW {
        _PTWW { w: self }
    }
    #[doc = "Bit 29 - Serial Transceiver Select 1 Serial Interface Engine is selected 0 Parallel Interface signals is selected Serial Interface Engine can be used in combination with UTMI+/ULPI physical interface to provide FS/LS signaling instead of the parallel interface signals"]
    #[inline]
    pub fn sts(&mut self) -> _STSW {
        _STSW { w: self }
    }
    #[doc = "Bits 30:31 - All USB port interface modes are listed in this field description, but not all are supported"]
    #[inline]
    pub fn pts_1(&mut self) -> _PTS_1W {
        _PTS_1W { w: self }
    }
}
