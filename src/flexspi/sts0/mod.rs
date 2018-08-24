#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::STS0 {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = r" Value of the field"]
pub struct SEQIDLER {
    bits: bool,
}
impl SEQIDLER {
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
pub struct ARBIDLER {
    bits: bool,
}
impl ARBIDLER {
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
#[doc = "Possible values of the field `ARBCMDSRC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ARBCMDSRCR {
    #[doc = "Triggered by AHB read command (triggered by AHB read)."]
    ARBCMDSRC_0,
    #[doc = "Triggered by AHB write command (triggered by AHB Write)."]
    ARBCMDSRC_1,
    #[doc = "Triggered by IP command (triggered by setting register bit IPCMD.TRG)."]
    ARBCMDSRC_2,
    #[doc = "Triggered by suspended command (resumed)."]
    ARBCMDSRC_3,
}
impl ARBCMDSRCR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            ARBCMDSRCR::ARBCMDSRC_0 => 0,
            ARBCMDSRCR::ARBCMDSRC_1 => 1,
            ARBCMDSRCR::ARBCMDSRC_2 => 2,
            ARBCMDSRCR::ARBCMDSRC_3 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> ARBCMDSRCR {
        match value {
            0 => ARBCMDSRCR::ARBCMDSRC_0,
            1 => ARBCMDSRCR::ARBCMDSRC_1,
            2 => ARBCMDSRCR::ARBCMDSRC_2,
            3 => ARBCMDSRCR::ARBCMDSRC_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `ARBCMDSRC_0`"]
    #[inline]
    pub fn is_arbcmdsrc_0(&self) -> bool {
        *self == ARBCMDSRCR::ARBCMDSRC_0
    }
    #[doc = "Checks if the value of the field is `ARBCMDSRC_1`"]
    #[inline]
    pub fn is_arbcmdsrc_1(&self) -> bool {
        *self == ARBCMDSRCR::ARBCMDSRC_1
    }
    #[doc = "Checks if the value of the field is `ARBCMDSRC_2`"]
    #[inline]
    pub fn is_arbcmdsrc_2(&self) -> bool {
        *self == ARBCMDSRCR::ARBCMDSRC_2
    }
    #[doc = "Checks if the value of the field is `ARBCMDSRC_3`"]
    #[inline]
    pub fn is_arbcmdsrc_3(&self) -> bool {
        *self == ARBCMDSRCR::ARBCMDSRC_3
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - This status bit indicates the state machine in SEQ_CTL is idle and there is command sequence executing on FlexSPI interface."]
    #[inline]
    pub fn seqidle(&self) -> SEQIDLER {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        SEQIDLER { bits }
    }
    #[doc = "Bit 1 - This status bit indicates the state machine in ARB_CTL is busy and there is command sequence granted by arbitrator and not finished yet on FlexSPI interface. When ARB_CTL state (ARBIDLE=0x1) is idle, there will be no transaction on FlexSPI interface also (SEQIDLE=0x1). So this bit should be polled to wait for FlexSPI controller become idle instead of SEQIDLE."]
    #[inline]
    pub fn arbidle(&self) -> ARBIDLER {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        ARBIDLER { bits }
    }
    #[doc = "Bits 2:3 - This status field indicates the trigger source of current command sequence granted by arbitrator. This field value is meaningless when ARB_CTL is not busy (STS0[ARBIDLE]=0x1)."]
    #[inline]
    pub fn arbcmdsrc(&self) -> ARBCMDSRCR {
        ARBCMDSRCR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
}
