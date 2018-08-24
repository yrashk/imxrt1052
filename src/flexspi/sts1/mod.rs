#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::STS1 {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = r" Value of the field"]
pub struct AHBCMDERRIDR {
    bits: u8,
}
impl AHBCMDERRIDR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Possible values of the field `AHBCMDERRCODE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AHBCMDERRCODER {
    #[doc = "No error."]
    AHBCMDERRCODE_0,
    #[doc = "AHB Write command with JMP_ON_CS instruction used in the sequence."]
    AHBCMDERRCODE_2,
    #[doc = "There is unknown instruction opcode in the sequence."]
    AHBCMDERRCODE_3,
    #[doc = "Instruction DUMMY_SDR/DUMMY_RWDS_SDR used in DDR sequence."]
    AHBCMDERRCODE_4,
    #[doc = "Instruction DUMMY_DDR/DUMMY_RWDS_DDR used in SDR sequence."]
    AHBCMDERRCODE_5,
    #[doc = "Sequence execution timeout."]
    AHBCMDERRCODE_14,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl AHBCMDERRCODER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            AHBCMDERRCODER::AHBCMDERRCODE_0 => 0,
            AHBCMDERRCODER::AHBCMDERRCODE_2 => 2,
            AHBCMDERRCODER::AHBCMDERRCODE_3 => 3,
            AHBCMDERRCODER::AHBCMDERRCODE_4 => 4,
            AHBCMDERRCODER::AHBCMDERRCODE_5 => 5,
            AHBCMDERRCODER::AHBCMDERRCODE_14 => 14,
            AHBCMDERRCODER::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> AHBCMDERRCODER {
        match value {
            0 => AHBCMDERRCODER::AHBCMDERRCODE_0,
            2 => AHBCMDERRCODER::AHBCMDERRCODE_2,
            3 => AHBCMDERRCODER::AHBCMDERRCODE_3,
            4 => AHBCMDERRCODER::AHBCMDERRCODE_4,
            5 => AHBCMDERRCODER::AHBCMDERRCODE_5,
            14 => AHBCMDERRCODER::AHBCMDERRCODE_14,
            i => AHBCMDERRCODER::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `AHBCMDERRCODE_0`"]
    #[inline]
    pub fn is_ahbcmderrcode_0(&self) -> bool {
        *self == AHBCMDERRCODER::AHBCMDERRCODE_0
    }
    #[doc = "Checks if the value of the field is `AHBCMDERRCODE_2`"]
    #[inline]
    pub fn is_ahbcmderrcode_2(&self) -> bool {
        *self == AHBCMDERRCODER::AHBCMDERRCODE_2
    }
    #[doc = "Checks if the value of the field is `AHBCMDERRCODE_3`"]
    #[inline]
    pub fn is_ahbcmderrcode_3(&self) -> bool {
        *self == AHBCMDERRCODER::AHBCMDERRCODE_3
    }
    #[doc = "Checks if the value of the field is `AHBCMDERRCODE_4`"]
    #[inline]
    pub fn is_ahbcmderrcode_4(&self) -> bool {
        *self == AHBCMDERRCODER::AHBCMDERRCODE_4
    }
    #[doc = "Checks if the value of the field is `AHBCMDERRCODE_5`"]
    #[inline]
    pub fn is_ahbcmderrcode_5(&self) -> bool {
        *self == AHBCMDERRCODER::AHBCMDERRCODE_5
    }
    #[doc = "Checks if the value of the field is `AHBCMDERRCODE_14`"]
    #[inline]
    pub fn is_ahbcmderrcode_14(&self) -> bool {
        *self == AHBCMDERRCODER::AHBCMDERRCODE_14
    }
}
#[doc = r" Value of the field"]
pub struct IPCMDERRIDR {
    bits: u8,
}
impl IPCMDERRIDR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Possible values of the field `IPCMDERRCODE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IPCMDERRCODER {
    #[doc = "No error."]
    IPCMDERRCODE_0,
    #[doc = "IP command with JMP_ON_CS instruction used in the sequence."]
    IPCMDERRCODE_2,
    #[doc = "There is unknown instruction opcode in the sequence."]
    IPCMDERRCODE_3,
    #[doc = "Instruction DUMMY_SDR/DUMMY_RWDS_SDR used in DDR sequence."]
    IPCMDERRCODE_4,
    #[doc = "Instruction DUMMY_DDR/DUMMY_RWDS_DDR used in SDR sequence."]
    IPCMDERRCODE_5,
    #[doc = "Flash access start address exceed the whole flash address range (A1/A2/B1/B2)."]
    IPCMDERRCODE_6,
    #[doc = "Sequence execution timeout."]
    IPCMDERRCODE_14,
    #[doc = "Flash boundary crossed."]
    IPCMDERRCODE_15,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl IPCMDERRCODER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            IPCMDERRCODER::IPCMDERRCODE_0 => 0,
            IPCMDERRCODER::IPCMDERRCODE_2 => 2,
            IPCMDERRCODER::IPCMDERRCODE_3 => 3,
            IPCMDERRCODER::IPCMDERRCODE_4 => 4,
            IPCMDERRCODER::IPCMDERRCODE_5 => 5,
            IPCMDERRCODER::IPCMDERRCODE_6 => 6,
            IPCMDERRCODER::IPCMDERRCODE_14 => 14,
            IPCMDERRCODER::IPCMDERRCODE_15 => 15,
            IPCMDERRCODER::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> IPCMDERRCODER {
        match value {
            0 => IPCMDERRCODER::IPCMDERRCODE_0,
            2 => IPCMDERRCODER::IPCMDERRCODE_2,
            3 => IPCMDERRCODER::IPCMDERRCODE_3,
            4 => IPCMDERRCODER::IPCMDERRCODE_4,
            5 => IPCMDERRCODER::IPCMDERRCODE_5,
            6 => IPCMDERRCODER::IPCMDERRCODE_6,
            14 => IPCMDERRCODER::IPCMDERRCODE_14,
            15 => IPCMDERRCODER::IPCMDERRCODE_15,
            i => IPCMDERRCODER::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `IPCMDERRCODE_0`"]
    #[inline]
    pub fn is_ipcmderrcode_0(&self) -> bool {
        *self == IPCMDERRCODER::IPCMDERRCODE_0
    }
    #[doc = "Checks if the value of the field is `IPCMDERRCODE_2`"]
    #[inline]
    pub fn is_ipcmderrcode_2(&self) -> bool {
        *self == IPCMDERRCODER::IPCMDERRCODE_2
    }
    #[doc = "Checks if the value of the field is `IPCMDERRCODE_3`"]
    #[inline]
    pub fn is_ipcmderrcode_3(&self) -> bool {
        *self == IPCMDERRCODER::IPCMDERRCODE_3
    }
    #[doc = "Checks if the value of the field is `IPCMDERRCODE_4`"]
    #[inline]
    pub fn is_ipcmderrcode_4(&self) -> bool {
        *self == IPCMDERRCODER::IPCMDERRCODE_4
    }
    #[doc = "Checks if the value of the field is `IPCMDERRCODE_5`"]
    #[inline]
    pub fn is_ipcmderrcode_5(&self) -> bool {
        *self == IPCMDERRCODER::IPCMDERRCODE_5
    }
    #[doc = "Checks if the value of the field is `IPCMDERRCODE_6`"]
    #[inline]
    pub fn is_ipcmderrcode_6(&self) -> bool {
        *self == IPCMDERRCODER::IPCMDERRCODE_6
    }
    #[doc = "Checks if the value of the field is `IPCMDERRCODE_14`"]
    #[inline]
    pub fn is_ipcmderrcode_14(&self) -> bool {
        *self == IPCMDERRCODER::IPCMDERRCODE_14
    }
    #[doc = "Checks if the value of the field is `IPCMDERRCODE_15`"]
    #[inline]
    pub fn is_ipcmderrcode_15(&self) -> bool {
        *self == IPCMDERRCODER::IPCMDERRCODE_15
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:3 - Indicates the sequence index when an AHB command error is detected. This field will be cleared when INTR[AHBCMDERR] is write-1-clear(w1c)."]
    #[inline]
    pub fn ahbcmderrid(&self) -> AHBCMDERRIDR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        AHBCMDERRIDR { bits }
    }
    #[doc = "Bits 8:11 - Indicates the Error Code when AHB command Error detected. This field will be cleared when INTR[AHBCMDERR] is write-1-clear(w1c)."]
    #[inline]
    pub fn ahbcmderrcode(&self) -> AHBCMDERRCODER {
        AHBCMDERRCODER::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 16:19 - Indicates the sequence Index when IP command error detected. This field will be cleared when INTR[IPCMDERR] is write-1-clear(w1c)."]
    #[inline]
    pub fn ipcmderrid(&self) -> IPCMDERRIDR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        IPCMDERRIDR { bits }
    }
    #[doc = "Bits 24:27 - Indicates the Error Code when IP command Error detected. This field will be cleared when INTR[IPCMDERR] is write-1-clear(w1c)."]
    #[inline]
    pub fn ipcmderrcode(&self) -> IPCMDERRCODER {
        IPCMDERRCODER::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
}
