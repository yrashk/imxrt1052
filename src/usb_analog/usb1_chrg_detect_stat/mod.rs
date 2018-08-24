#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::USB1_CHRG_DETECT_STAT {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = "Possible values of the field `PLUG_CONTACT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PLUG_CONTACTR {
    #[doc = "The USB plug has not made contact."]
    NO_CONTACT,
    #[doc = "The USB plug has made good contact."]
    GOOD_CONTACT,
}
impl PLUG_CONTACTR {
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
            PLUG_CONTACTR::NO_CONTACT => false,
            PLUG_CONTACTR::GOOD_CONTACT => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PLUG_CONTACTR {
        match value {
            false => PLUG_CONTACTR::NO_CONTACT,
            true => PLUG_CONTACTR::GOOD_CONTACT,
        }
    }
    #[doc = "Checks if the value of the field is `NO_CONTACT`"]
    #[inline]
    pub fn is_no_contact(&self) -> bool {
        *self == PLUG_CONTACTR::NO_CONTACT
    }
    #[doc = "Checks if the value of the field is `GOOD_CONTACT`"]
    #[inline]
    pub fn is_good_contact(&self) -> bool {
        *self == PLUG_CONTACTR::GOOD_CONTACT
    }
}
#[doc = "Possible values of the field `CHRG_DETECTED`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CHRG_DETECTEDR {
    #[doc = "The USB port is not connected to a charger."]
    CHARGER_NOT_PRESENT,
    #[doc = "A charger (either a dedicated charger or a host charger) is connected to the USB port."]
    CHARGER_PRESENT,
}
impl CHRG_DETECTEDR {
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
            CHRG_DETECTEDR::CHARGER_NOT_PRESENT => false,
            CHRG_DETECTEDR::CHARGER_PRESENT => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CHRG_DETECTEDR {
        match value {
            false => CHRG_DETECTEDR::CHARGER_NOT_PRESENT,
            true => CHRG_DETECTEDR::CHARGER_PRESENT,
        }
    }
    #[doc = "Checks if the value of the field is `CHARGER_NOT_PRESENT`"]
    #[inline]
    pub fn is_charger_not_present(&self) -> bool {
        *self == CHRG_DETECTEDR::CHARGER_NOT_PRESENT
    }
    #[doc = "Checks if the value of the field is `CHARGER_PRESENT`"]
    #[inline]
    pub fn is_charger_present(&self) -> bool {
        *self == CHRG_DETECTEDR::CHARGER_PRESENT
    }
}
#[doc = r" Value of the field"]
pub struct DM_STATER {
    bits: bool,
}
impl DM_STATER {
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
pub struct DP_STATER {
    bits: bool,
}
impl DP_STATER {
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
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - State of the USB plug contact detector."]
    #[inline]
    pub fn plug_contact(&self) -> PLUG_CONTACTR {
        PLUG_CONTACTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - State of charger detection. This bit is a read only version of the state of the analog signal."]
    #[inline]
    pub fn chrg_detected(&self) -> CHRG_DETECTEDR {
        CHRG_DETECTEDR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - DM line state output of the charger detector."]
    #[inline]
    pub fn dm_state(&self) -> DM_STATER {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        DM_STATER { bits }
    }
    #[doc = "Bit 3 - DP line state output of the charger detector."]
    #[inline]
    pub fn dp_state(&self) -> DP_STATER {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        DP_STATER { bits }
    }
}
