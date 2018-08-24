#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::CSR {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = "Possible values of the field `REF_EN_B`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REF_EN_BR {
    #[doc = "value of CCM_REF_EN_B is '0'"]
    REF_EN_B_0,
    #[doc = "value of CCM_REF_EN_B is '1'"]
    REF_EN_B_1,
}
impl REF_EN_BR {
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
            REF_EN_BR::REF_EN_B_0 => false,
            REF_EN_BR::REF_EN_B_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> REF_EN_BR {
        match value {
            false => REF_EN_BR::REF_EN_B_0,
            true => REF_EN_BR::REF_EN_B_1,
        }
    }
    #[doc = "Checks if the value of the field is `REF_EN_B_0`"]
    #[inline]
    pub fn is_ref_en_b_0(&self) -> bool {
        *self == REF_EN_BR::REF_EN_B_0
    }
    #[doc = "Checks if the value of the field is `REF_EN_B_1`"]
    #[inline]
    pub fn is_ref_en_b_1(&self) -> bool {
        *self == REF_EN_BR::REF_EN_B_1
    }
}
#[doc = "Possible values of the field `CAMP2_READY`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CAMP2_READYR {
    #[doc = "CAMP2 is not ready."]
    CAMP2_READY_0,
    #[doc = "CAMP2 is ready."]
    CAMP2_READY_1,
}
impl CAMP2_READYR {
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
            CAMP2_READYR::CAMP2_READY_0 => false,
            CAMP2_READYR::CAMP2_READY_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CAMP2_READYR {
        match value {
            false => CAMP2_READYR::CAMP2_READY_0,
            true => CAMP2_READYR::CAMP2_READY_1,
        }
    }
    #[doc = "Checks if the value of the field is `CAMP2_READY_0`"]
    #[inline]
    pub fn is_camp2_ready_0(&self) -> bool {
        *self == CAMP2_READYR::CAMP2_READY_0
    }
    #[doc = "Checks if the value of the field is `CAMP2_READY_1`"]
    #[inline]
    pub fn is_camp2_ready_1(&self) -> bool {
        *self == CAMP2_READYR::CAMP2_READY_1
    }
}
#[doc = "Possible values of the field `COSC_READY`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum COSC_READYR {
    #[doc = "on board oscillator is not ready."]
    COSC_READY_0,
    #[doc = "on board oscillator is ready."]
    COSC_READY_1,
}
impl COSC_READYR {
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
            COSC_READYR::COSC_READY_0 => false,
            COSC_READYR::COSC_READY_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> COSC_READYR {
        match value {
            false => COSC_READYR::COSC_READY_0,
            true => COSC_READYR::COSC_READY_1,
        }
    }
    #[doc = "Checks if the value of the field is `COSC_READY_0`"]
    #[inline]
    pub fn is_cosc_ready_0(&self) -> bool {
        *self == COSC_READYR::COSC_READY_0
    }
    #[doc = "Checks if the value of the field is `COSC_READY_1`"]
    #[inline]
    pub fn is_cosc_ready_1(&self) -> bool {
        *self == COSC_READYR::COSC_READY_1
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - Status of the value of CCM_REF_EN_B output of ccm"]
    #[inline]
    pub fn ref_en_b(&self) -> REF_EN_BR {
        REF_EN_BR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - Status indication of CAMP2."]
    #[inline]
    pub fn camp2_ready(&self) -> CAMP2_READYR {
        CAMP2_READYR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 5 - Status indication of on board oscillator"]
    #[inline]
    pub fn cosc_ready(&self) -> COSC_READYR {
        COSC_READYR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
}
