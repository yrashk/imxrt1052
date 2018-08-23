#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::HWGENERAL {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = "Possible values of the field `PHYW`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PHYWR {
    #[doc = "8 bit wide data bus Software non-programmable"]
    PHYW_0,
    #[doc = "16 bit wide data bus Software non-programmable"]
    PHYW_1,
    #[doc = "Reset to 8 bit wide data bus Software programmable"]
    PHYW_2,
    #[doc = "Reset to 16 bit wide data bus Software programmable"]
    PHYW_3,
}
impl PHYWR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            PHYWR::PHYW_0 => 0,
            PHYWR::PHYW_1 => 1,
            PHYWR::PHYW_2 => 2,
            PHYWR::PHYW_3 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> PHYWR {
        match value {
            0 => PHYWR::PHYW_0,
            1 => PHYWR::PHYW_1,
            2 => PHYWR::PHYW_2,
            3 => PHYWR::PHYW_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `PHYW_0`"]
    #[inline]
    pub fn is_phyw_0(&self) -> bool {
        *self == PHYWR::PHYW_0
    }
    #[doc = "Checks if the value of the field is `PHYW_1`"]
    #[inline]
    pub fn is_phyw_1(&self) -> bool {
        *self == PHYWR::PHYW_1
    }
    #[doc = "Checks if the value of the field is `PHYW_2`"]
    #[inline]
    pub fn is_phyw_2(&self) -> bool {
        *self == PHYWR::PHYW_2
    }
    #[doc = "Checks if the value of the field is `PHYW_3`"]
    #[inline]
    pub fn is_phyw_3(&self) -> bool {
        *self == PHYWR::PHYW_3
    }
}
#[doc = "Possible values of the field `PHYM`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PHYMR {
    #[doc = "UTMI/UMTI+"]
    PHYM_0,
    #[doc = "ULPI DDR"]
    PHYM_1,
    #[doc = "ULPI"]
    PHYM_2,
    #[doc = "Serial Only"]
    PHYM_3,
    #[doc = "Software programmable - reset to UTMI/UTMI+"]
    PHYM_4,
    #[doc = "Software programmable - reset to ULPI DDR"]
    PHYM_5,
    #[doc = "Software programmable - reset to ULPI"]
    PHYM_6,
    #[doc = "Software programmable - reset to Serial"]
    PHYM_7,
}
impl PHYMR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            PHYMR::PHYM_0 => 0,
            PHYMR::PHYM_1 => 1,
            PHYMR::PHYM_2 => 2,
            PHYMR::PHYM_3 => 3,
            PHYMR::PHYM_4 => 4,
            PHYMR::PHYM_5 => 5,
            PHYMR::PHYM_6 => 6,
            PHYMR::PHYM_7 => 7,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> PHYMR {
        match value {
            0 => PHYMR::PHYM_0,
            1 => PHYMR::PHYM_1,
            2 => PHYMR::PHYM_2,
            3 => PHYMR::PHYM_3,
            4 => PHYMR::PHYM_4,
            5 => PHYMR::PHYM_5,
            6 => PHYMR::PHYM_6,
            7 => PHYMR::PHYM_7,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `PHYM_0`"]
    #[inline]
    pub fn is_phym_0(&self) -> bool {
        *self == PHYMR::PHYM_0
    }
    #[doc = "Checks if the value of the field is `PHYM_1`"]
    #[inline]
    pub fn is_phym_1(&self) -> bool {
        *self == PHYMR::PHYM_1
    }
    #[doc = "Checks if the value of the field is `PHYM_2`"]
    #[inline]
    pub fn is_phym_2(&self) -> bool {
        *self == PHYMR::PHYM_2
    }
    #[doc = "Checks if the value of the field is `PHYM_3`"]
    #[inline]
    pub fn is_phym_3(&self) -> bool {
        *self == PHYMR::PHYM_3
    }
    #[doc = "Checks if the value of the field is `PHYM_4`"]
    #[inline]
    pub fn is_phym_4(&self) -> bool {
        *self == PHYMR::PHYM_4
    }
    #[doc = "Checks if the value of the field is `PHYM_5`"]
    #[inline]
    pub fn is_phym_5(&self) -> bool {
        *self == PHYMR::PHYM_5
    }
    #[doc = "Checks if the value of the field is `PHYM_6`"]
    #[inline]
    pub fn is_phym_6(&self) -> bool {
        *self == PHYMR::PHYM_6
    }
    #[doc = "Checks if the value of the field is `PHYM_7`"]
    #[inline]
    pub fn is_phym_7(&self) -> bool {
        *self == PHYMR::PHYM_7
    }
}
#[doc = "Possible values of the field `SM`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SMR {
    #[doc = "No Serial Engine, always use parallel signalling."]
    SM_0,
    #[doc = "Serial Engine present, always use serial signalling for FS/LS."]
    SM_1,
    #[doc = "Software programmable - Reset to use parallel signalling for FS/LS"]
    SM_2,
    #[doc = "Software programmable - Reset to use serial signalling for FS/LS"]
    SM_3,
}
impl SMR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            SMR::SM_0 => 0,
            SMR::SM_1 => 1,
            SMR::SM_2 => 2,
            SMR::SM_3 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> SMR {
        match value {
            0 => SMR::SM_0,
            1 => SMR::SM_1,
            2 => SMR::SM_2,
            3 => SMR::SM_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `SM_0`"]
    #[inline]
    pub fn is_sm_0(&self) -> bool {
        *self == SMR::SM_0
    }
    #[doc = "Checks if the value of the field is `SM_1`"]
    #[inline]
    pub fn is_sm_1(&self) -> bool {
        *self == SMR::SM_1
    }
    #[doc = "Checks if the value of the field is `SM_2`"]
    #[inline]
    pub fn is_sm_2(&self) -> bool {
        *self == SMR::SM_2
    }
    #[doc = "Checks if the value of the field is `SM_3`"]
    #[inline]
    pub fn is_sm_3(&self) -> bool {
        *self == SMR::SM_3
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 4:5 - Data width of the transciever connected to the controller core. PHYW bit reset value is"]
    #[inline]
    pub fn phyw(&self) -> PHYWR {
        PHYWR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 6:8 - Transciever type"]
    #[inline]
    pub fn phym(&self) -> PHYMR {
        PHYMR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 9:10 - Serial interface mode capability"]
    #[inline]
    pub fn sm(&self) -> SMR {
        SMR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
}
