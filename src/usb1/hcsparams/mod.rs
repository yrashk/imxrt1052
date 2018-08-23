#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::HCSPARAMS {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = r" Value of the field"]
pub struct N_PORTSR {
    bits: u8,
}
impl N_PORTSR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct PPCR {
    bits: bool,
}
impl PPCR {
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
pub struct N_PCCR {
    bits: u8,
}
impl N_PCCR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Possible values of the field `N_CC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum N_CCR {
    #[doc = "There is no internal Companion Controller and port-ownership hand-off is not supported."]
    N_CC_0,
    #[doc = "There are internal companion controller(s) and port-ownership hand-offs is supported."]
    N_CC_1,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl N_CCR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            N_CCR::N_CC_0 => 0,
            N_CCR::N_CC_1 => 1,
            N_CCR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> N_CCR {
        match value {
            0 => N_CCR::N_CC_0,
            1 => N_CCR::N_CC_1,
            i => N_CCR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `N_CC_0`"]
    #[inline]
    pub fn is_n_cc_0(&self) -> bool {
        *self == N_CCR::N_CC_0
    }
    #[doc = "Checks if the value of the field is `N_CC_1`"]
    #[inline]
    pub fn is_n_cc_1(&self) -> bool {
        *self == N_CCR::N_CC_1
    }
}
#[doc = r" Value of the field"]
pub struct PIR {
    bits: bool,
}
impl PIR {
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
pub struct N_PTTR {
    bits: u8,
}
impl N_PTTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct N_TTR {
    bits: u8,
}
impl N_TTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:3 - Number of downstream ports"]
    #[inline]
    pub fn n_ports(&self) -> N_PORTSR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        N_PORTSR { bits }
    }
    #[doc = "Bit 4 - Port Power Control This field indicates whether the host controller implementation includes port power control"]
    #[inline]
    pub fn ppc(&self) -> PPCR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        PPCR { bits }
    }
    #[doc = "Bits 8:11 - Number of Ports per Companion Controller This field indicates the number of ports supported per internal Companion Controller"]
    #[inline]
    pub fn n_pcc(&self) -> N_PCCR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        N_PCCR { bits }
    }
    #[doc = "Bits 12:15 - Number of Companion Controller (N_CC)"]
    #[inline]
    pub fn n_cc(&self) -> N_CCR {
        N_CCR::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 16 - Port Indicators (P INDICATOR) This bit indicates whether the ports support port indicator control"]
    #[inline]
    pub fn pi(&self) -> PIR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        PIR { bits }
    }
    #[doc = "Bits 20:23 - Number of Ports per Transaction Translator (N_PTT)"]
    #[inline]
    pub fn n_ptt(&self) -> N_PTTR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        N_PTTR { bits }
    }
    #[doc = "Bits 24:27 - Number of Transaction Translators (N_TT)"]
    #[inline]
    pub fn n_tt(&self) -> N_TTR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        N_TTR { bits }
    }
}
