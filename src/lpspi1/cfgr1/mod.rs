#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CFGR1 {
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
#[doc = "Possible values of the field `MASTER`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MASTERR {
    #[doc = "Slave mode"]
    MASTER_0,
    #[doc = "Master mode"]
    MASTER_1,
}
impl MASTERR {
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
            MASTERR::MASTER_0 => false,
            MASTERR::MASTER_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> MASTERR {
        match value {
            false => MASTERR::MASTER_0,
            true => MASTERR::MASTER_1,
        }
    }
    #[doc = "Checks if the value of the field is `MASTER_0`"]
    #[inline]
    pub fn is_master_0(&self) -> bool {
        *self == MASTERR::MASTER_0
    }
    #[doc = "Checks if the value of the field is `MASTER_1`"]
    #[inline]
    pub fn is_master_1(&self) -> bool {
        *self == MASTERR::MASTER_1
    }
}
#[doc = "Possible values of the field `SAMPLE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SAMPLER {
    #[doc = "Input data is sampled on SCK edge"]
    SAMPLE_0,
    #[doc = "Input data is sampled on delayed SCK edge"]
    SAMPLE_1,
}
impl SAMPLER {
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
            SAMPLER::SAMPLE_0 => false,
            SAMPLER::SAMPLE_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SAMPLER {
        match value {
            false => SAMPLER::SAMPLE_0,
            true => SAMPLER::SAMPLE_1,
        }
    }
    #[doc = "Checks if the value of the field is `SAMPLE_0`"]
    #[inline]
    pub fn is_sample_0(&self) -> bool {
        *self == SAMPLER::SAMPLE_0
    }
    #[doc = "Checks if the value of the field is `SAMPLE_1`"]
    #[inline]
    pub fn is_sample_1(&self) -> bool {
        *self == SAMPLER::SAMPLE_1
    }
}
#[doc = "Possible values of the field `AUTOPCS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AUTOPCSR {
    #[doc = "Automatic PCS generation is disabled"]
    AUTOPCS_0,
    #[doc = "Automatic PCS generation is enabled"]
    AUTOPCS_1,
}
impl AUTOPCSR {
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
            AUTOPCSR::AUTOPCS_0 => false,
            AUTOPCSR::AUTOPCS_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> AUTOPCSR {
        match value {
            false => AUTOPCSR::AUTOPCS_0,
            true => AUTOPCSR::AUTOPCS_1,
        }
    }
    #[doc = "Checks if the value of the field is `AUTOPCS_0`"]
    #[inline]
    pub fn is_autopcs_0(&self) -> bool {
        *self == AUTOPCSR::AUTOPCS_0
    }
    #[doc = "Checks if the value of the field is `AUTOPCS_1`"]
    #[inline]
    pub fn is_autopcs_1(&self) -> bool {
        *self == AUTOPCSR::AUTOPCS_1
    }
}
#[doc = "Possible values of the field `NOSTALL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NOSTALLR {
    #[doc = "Transfers will stall when the transmit FIFO is empty or the receive FIFO is full"]
    NOSTALL_0,
    #[doc = "Transfers will not stall, allowing transmit FIFO underruns or receive FIFO overruns to occur"]
    NOSTALL_1,
}
impl NOSTALLR {
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
            NOSTALLR::NOSTALL_0 => false,
            NOSTALLR::NOSTALL_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> NOSTALLR {
        match value {
            false => NOSTALLR::NOSTALL_0,
            true => NOSTALLR::NOSTALL_1,
        }
    }
    #[doc = "Checks if the value of the field is `NOSTALL_0`"]
    #[inline]
    pub fn is_nostall_0(&self) -> bool {
        *self == NOSTALLR::NOSTALL_0
    }
    #[doc = "Checks if the value of the field is `NOSTALL_1`"]
    #[inline]
    pub fn is_nostall_1(&self) -> bool {
        *self == NOSTALLR::NOSTALL_1
    }
}
#[doc = "Possible values of the field `PCSPOL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PCSPOLR {
    #[doc = "The Peripheral Chip Select pin PCSx is active low"]
    PCSPOL_0,
    #[doc = "The Peripheral Chip Select pin PCSx is active high"]
    PCSPOL_1,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl PCSPOLR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            PCSPOLR::PCSPOL_0 => 0,
            PCSPOLR::PCSPOL_1 => 1,
            PCSPOLR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> PCSPOLR {
        match value {
            0 => PCSPOLR::PCSPOL_0,
            1 => PCSPOLR::PCSPOL_1,
            i => PCSPOLR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `PCSPOL_0`"]
    #[inline]
    pub fn is_pcspol_0(&self) -> bool {
        *self == PCSPOLR::PCSPOL_0
    }
    #[doc = "Checks if the value of the field is `PCSPOL_1`"]
    #[inline]
    pub fn is_pcspol_1(&self) -> bool {
        *self == PCSPOLR::PCSPOL_1
    }
}
#[doc = "Possible values of the field `MATCFG`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MATCFGR {
    #[doc = "Match is disabled"]
    MATCFG_0,
    #[doc = "010b - Match is enabled, if 1st data word equals MATCH0 OR MATCH1, i.e., (1st data word = MATCH0 + MATCH1)"]
    MATCFG_2,
    #[doc = "011b - Match is enabled, if any data word equals MATCH0 OR MATCH1, i.e., (any data word = MATCH0 + MATCH1)"]
    MATCFG_3,
    #[doc = "100b - Match is enabled, if 1st data word equals MATCH0 AND 2nd data word equals MATCH1, i.e., [(1st data word = MATCH0) * (2nd data word = MATCH1)]"]
    MATCFG_4,
    #[doc = "101b - Match is enabled, if any data word equals MATCH0 AND the next data word equals MATCH1, i.e., [(any data word = MATCH0) * (next data word = MATCH1)]"]
    MATCFG_5,
    #[doc = "110b - Match is enabled, if (1st data word AND MATCH1) equals (MATCH0 AND MATCH1), i.e., [(1st data word * MATCH1) = (MATCH0 * MATCH1)]"]
    MATCFG_6,
    #[doc = "111b - Match is enabled, if (any data word AND MATCH1) equals (MATCH0 AND MATCH1), i.e., [(any data word * MATCH1) = (MATCH0 * MATCH1)]"]
    MATCFG_7,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl MATCFGR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            MATCFGR::MATCFG_0 => 0,
            MATCFGR::MATCFG_2 => 2,
            MATCFGR::MATCFG_3 => 3,
            MATCFGR::MATCFG_4 => 4,
            MATCFGR::MATCFG_5 => 5,
            MATCFGR::MATCFG_6 => 6,
            MATCFGR::MATCFG_7 => 7,
            MATCFGR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> MATCFGR {
        match value {
            0 => MATCFGR::MATCFG_0,
            2 => MATCFGR::MATCFG_2,
            3 => MATCFGR::MATCFG_3,
            4 => MATCFGR::MATCFG_4,
            5 => MATCFGR::MATCFG_5,
            6 => MATCFGR::MATCFG_6,
            7 => MATCFGR::MATCFG_7,
            i => MATCFGR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `MATCFG_0`"]
    #[inline]
    pub fn is_matcfg_0(&self) -> bool {
        *self == MATCFGR::MATCFG_0
    }
    #[doc = "Checks if the value of the field is `MATCFG_2`"]
    #[inline]
    pub fn is_matcfg_2(&self) -> bool {
        *self == MATCFGR::MATCFG_2
    }
    #[doc = "Checks if the value of the field is `MATCFG_3`"]
    #[inline]
    pub fn is_matcfg_3(&self) -> bool {
        *self == MATCFGR::MATCFG_3
    }
    #[doc = "Checks if the value of the field is `MATCFG_4`"]
    #[inline]
    pub fn is_matcfg_4(&self) -> bool {
        *self == MATCFGR::MATCFG_4
    }
    #[doc = "Checks if the value of the field is `MATCFG_5`"]
    #[inline]
    pub fn is_matcfg_5(&self) -> bool {
        *self == MATCFGR::MATCFG_5
    }
    #[doc = "Checks if the value of the field is `MATCFG_6`"]
    #[inline]
    pub fn is_matcfg_6(&self) -> bool {
        *self == MATCFGR::MATCFG_6
    }
    #[doc = "Checks if the value of the field is `MATCFG_7`"]
    #[inline]
    pub fn is_matcfg_7(&self) -> bool {
        *self == MATCFGR::MATCFG_7
    }
}
#[doc = "Possible values of the field `PINCFG`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PINCFGR {
    #[doc = "SIN is used for input data and SOUT is used for output data"]
    PINCFG_0,
    #[doc = "SIN is used for both input and output data"]
    PINCFG_1,
    #[doc = "SOUT is used for both input and output data"]
    PINCFG_2,
    #[doc = "SOUT is used for input data and SIN is used for output data"]
    PINCFG_3,
}
impl PINCFGR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            PINCFGR::PINCFG_0 => 0,
            PINCFGR::PINCFG_1 => 1,
            PINCFGR::PINCFG_2 => 2,
            PINCFGR::PINCFG_3 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> PINCFGR {
        match value {
            0 => PINCFGR::PINCFG_0,
            1 => PINCFGR::PINCFG_1,
            2 => PINCFGR::PINCFG_2,
            3 => PINCFGR::PINCFG_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `PINCFG_0`"]
    #[inline]
    pub fn is_pincfg_0(&self) -> bool {
        *self == PINCFGR::PINCFG_0
    }
    #[doc = "Checks if the value of the field is `PINCFG_1`"]
    #[inline]
    pub fn is_pincfg_1(&self) -> bool {
        *self == PINCFGR::PINCFG_1
    }
    #[doc = "Checks if the value of the field is `PINCFG_2`"]
    #[inline]
    pub fn is_pincfg_2(&self) -> bool {
        *self == PINCFGR::PINCFG_2
    }
    #[doc = "Checks if the value of the field is `PINCFG_3`"]
    #[inline]
    pub fn is_pincfg_3(&self) -> bool {
        *self == PINCFGR::PINCFG_3
    }
}
#[doc = "Possible values of the field `OUTCFG`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OUTCFGR {
    #[doc = "Output data retains last value when chip select is negated"]
    OUTCFG_0,
    #[doc = "Output data is tristated when chip select is negated"]
    OUTCFG_1,
}
impl OUTCFGR {
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
            OUTCFGR::OUTCFG_0 => false,
            OUTCFGR::OUTCFG_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> OUTCFGR {
        match value {
            false => OUTCFGR::OUTCFG_0,
            true => OUTCFGR::OUTCFG_1,
        }
    }
    #[doc = "Checks if the value of the field is `OUTCFG_0`"]
    #[inline]
    pub fn is_outcfg_0(&self) -> bool {
        *self == OUTCFGR::OUTCFG_0
    }
    #[doc = "Checks if the value of the field is `OUTCFG_1`"]
    #[inline]
    pub fn is_outcfg_1(&self) -> bool {
        *self == OUTCFGR::OUTCFG_1
    }
}
#[doc = "Possible values of the field `PCSCFG`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PCSCFGR {
    #[doc = "PCS[3:2] are enabled"]
    PCSCFG_0,
    #[doc = "PCS[3:2] are disabled"]
    PCSCFG_1,
}
impl PCSCFGR {
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
            PCSCFGR::PCSCFG_0 => false,
            PCSCFGR::PCSCFG_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PCSCFGR {
        match value {
            false => PCSCFGR::PCSCFG_0,
            true => PCSCFGR::PCSCFG_1,
        }
    }
    #[doc = "Checks if the value of the field is `PCSCFG_0`"]
    #[inline]
    pub fn is_pcscfg_0(&self) -> bool {
        *self == PCSCFGR::PCSCFG_0
    }
    #[doc = "Checks if the value of the field is `PCSCFG_1`"]
    #[inline]
    pub fn is_pcscfg_1(&self) -> bool {
        *self == PCSCFGR::PCSCFG_1
    }
}
#[doc = "Values that can be written to the field `MASTER`"]
pub enum MASTERW {
    #[doc = "Slave mode"]
    MASTER_0,
    #[doc = "Master mode"]
    MASTER_1,
}
impl MASTERW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            MASTERW::MASTER_0 => false,
            MASTERW::MASTER_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MASTERW<'a> {
    w: &'a mut W,
}
impl<'a> _MASTERW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MASTERW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Slave mode"]
    #[inline]
    pub fn master_0(self) -> &'a mut W {
        self.variant(MASTERW::MASTER_0)
    }
    #[doc = "Master mode"]
    #[inline]
    pub fn master_1(self) -> &'a mut W {
        self.variant(MASTERW::MASTER_1)
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
#[doc = "Values that can be written to the field `SAMPLE`"]
pub enum SAMPLEW {
    #[doc = "Input data is sampled on SCK edge"]
    SAMPLE_0,
    #[doc = "Input data is sampled on delayed SCK edge"]
    SAMPLE_1,
}
impl SAMPLEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SAMPLEW::SAMPLE_0 => false,
            SAMPLEW::SAMPLE_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SAMPLEW<'a> {
    w: &'a mut W,
}
impl<'a> _SAMPLEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SAMPLEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Input data is sampled on SCK edge"]
    #[inline]
    pub fn sample_0(self) -> &'a mut W {
        self.variant(SAMPLEW::SAMPLE_0)
    }
    #[doc = "Input data is sampled on delayed SCK edge"]
    #[inline]
    pub fn sample_1(self) -> &'a mut W {
        self.variant(SAMPLEW::SAMPLE_1)
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
#[doc = "Values that can be written to the field `AUTOPCS`"]
pub enum AUTOPCSW {
    #[doc = "Automatic PCS generation is disabled"]
    AUTOPCS_0,
    #[doc = "Automatic PCS generation is enabled"]
    AUTOPCS_1,
}
impl AUTOPCSW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            AUTOPCSW::AUTOPCS_0 => false,
            AUTOPCSW::AUTOPCS_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _AUTOPCSW<'a> {
    w: &'a mut W,
}
impl<'a> _AUTOPCSW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: AUTOPCSW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Automatic PCS generation is disabled"]
    #[inline]
    pub fn autopcs_0(self) -> &'a mut W {
        self.variant(AUTOPCSW::AUTOPCS_0)
    }
    #[doc = "Automatic PCS generation is enabled"]
    #[inline]
    pub fn autopcs_1(self) -> &'a mut W {
        self.variant(AUTOPCSW::AUTOPCS_1)
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
#[doc = "Values that can be written to the field `NOSTALL`"]
pub enum NOSTALLW {
    #[doc = "Transfers will stall when the transmit FIFO is empty or the receive FIFO is full"]
    NOSTALL_0,
    #[doc = "Transfers will not stall, allowing transmit FIFO underruns or receive FIFO overruns to occur"]
    NOSTALL_1,
}
impl NOSTALLW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            NOSTALLW::NOSTALL_0 => false,
            NOSTALLW::NOSTALL_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _NOSTALLW<'a> {
    w: &'a mut W,
}
impl<'a> _NOSTALLW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: NOSTALLW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Transfers will stall when the transmit FIFO is empty or the receive FIFO is full"]
    #[inline]
    pub fn nostall_0(self) -> &'a mut W {
        self.variant(NOSTALLW::NOSTALL_0)
    }
    #[doc = "Transfers will not stall, allowing transmit FIFO underruns or receive FIFO overruns to occur"]
    #[inline]
    pub fn nostall_1(self) -> &'a mut W {
        self.variant(NOSTALLW::NOSTALL_1)
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
        const OFFSET: u8 = 3;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `PCSPOL`"]
pub enum PCSPOLW {
    #[doc = "The Peripheral Chip Select pin PCSx is active low"]
    PCSPOL_0,
    #[doc = "The Peripheral Chip Select pin PCSx is active high"]
    PCSPOL_1,
}
impl PCSPOLW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            PCSPOLW::PCSPOL_0 => 0,
            PCSPOLW::PCSPOL_1 => 1,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PCSPOLW<'a> {
    w: &'a mut W,
}
impl<'a> _PCSPOLW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PCSPOLW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "The Peripheral Chip Select pin PCSx is active low"]
    #[inline]
    pub fn pcspol_0(self) -> &'a mut W {
        self.variant(PCSPOLW::PCSPOL_0)
    }
    #[doc = "The Peripheral Chip Select pin PCSx is active high"]
    #[inline]
    pub fn pcspol_1(self) -> &'a mut W {
        self.variant(PCSPOLW::PCSPOL_1)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `MATCFG`"]
pub enum MATCFGW {
    #[doc = "Match is disabled"]
    MATCFG_0,
    #[doc = "010b - Match is enabled, if 1st data word equals MATCH0 OR MATCH1, i.e., (1st data word = MATCH0 + MATCH1)"]
    MATCFG_2,
    #[doc = "011b - Match is enabled, if any data word equals MATCH0 OR MATCH1, i.e., (any data word = MATCH0 + MATCH1)"]
    MATCFG_3,
    #[doc = "100b - Match is enabled, if 1st data word equals MATCH0 AND 2nd data word equals MATCH1, i.e., [(1st data word = MATCH0) * (2nd data word = MATCH1)]"]
    MATCFG_4,
    #[doc = "101b - Match is enabled, if any data word equals MATCH0 AND the next data word equals MATCH1, i.e., [(any data word = MATCH0) * (next data word = MATCH1)]"]
    MATCFG_5,
    #[doc = "110b - Match is enabled, if (1st data word AND MATCH1) equals (MATCH0 AND MATCH1), i.e., [(1st data word * MATCH1) = (MATCH0 * MATCH1)]"]
    MATCFG_6,
    #[doc = "111b - Match is enabled, if (any data word AND MATCH1) equals (MATCH0 AND MATCH1), i.e., [(any data word * MATCH1) = (MATCH0 * MATCH1)]"]
    MATCFG_7,
}
impl MATCFGW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            MATCFGW::MATCFG_0 => 0,
            MATCFGW::MATCFG_2 => 2,
            MATCFGW::MATCFG_3 => 3,
            MATCFGW::MATCFG_4 => 4,
            MATCFGW::MATCFG_5 => 5,
            MATCFGW::MATCFG_6 => 6,
            MATCFGW::MATCFG_7 => 7,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MATCFGW<'a> {
    w: &'a mut W,
}
impl<'a> _MATCFGW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MATCFGW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Match is disabled"]
    #[inline]
    pub fn matcfg_0(self) -> &'a mut W {
        self.variant(MATCFGW::MATCFG_0)
    }
    #[doc = "010b - Match is enabled, if 1st data word equals MATCH0 OR MATCH1, i.e., (1st data word = MATCH0 + MATCH1)"]
    #[inline]
    pub fn matcfg_2(self) -> &'a mut W {
        self.variant(MATCFGW::MATCFG_2)
    }
    #[doc = "011b - Match is enabled, if any data word equals MATCH0 OR MATCH1, i.e., (any data word = MATCH0 + MATCH1)"]
    #[inline]
    pub fn matcfg_3(self) -> &'a mut W {
        self.variant(MATCFGW::MATCFG_3)
    }
    #[doc = "100b - Match is enabled, if 1st data word equals MATCH0 AND 2nd data word equals MATCH1, i.e., [(1st data word = MATCH0) * (2nd data word = MATCH1)]"]
    #[inline]
    pub fn matcfg_4(self) -> &'a mut W {
        self.variant(MATCFGW::MATCFG_4)
    }
    #[doc = "101b - Match is enabled, if any data word equals MATCH0 AND the next data word equals MATCH1, i.e., [(any data word = MATCH0) * (next data word = MATCH1)]"]
    #[inline]
    pub fn matcfg_5(self) -> &'a mut W {
        self.variant(MATCFGW::MATCFG_5)
    }
    #[doc = "110b - Match is enabled, if (1st data word AND MATCH1) equals (MATCH0 AND MATCH1), i.e., [(1st data word * MATCH1) = (MATCH0 * MATCH1)]"]
    #[inline]
    pub fn matcfg_6(self) -> &'a mut W {
        self.variant(MATCFGW::MATCFG_6)
    }
    #[doc = "111b - Match is enabled, if (any data word AND MATCH1) equals (MATCH0 AND MATCH1), i.e., [(any data word * MATCH1) = (MATCH0 * MATCH1)]"]
    #[inline]
    pub fn matcfg_7(self) -> &'a mut W {
        self.variant(MATCFGW::MATCFG_7)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 16;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `PINCFG`"]
pub enum PINCFGW {
    #[doc = "SIN is used for input data and SOUT is used for output data"]
    PINCFG_0,
    #[doc = "SIN is used for both input and output data"]
    PINCFG_1,
    #[doc = "SOUT is used for both input and output data"]
    PINCFG_2,
    #[doc = "SOUT is used for input data and SIN is used for output data"]
    PINCFG_3,
}
impl PINCFGW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            PINCFGW::PINCFG_0 => 0,
            PINCFGW::PINCFG_1 => 1,
            PINCFGW::PINCFG_2 => 2,
            PINCFGW::PINCFG_3 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PINCFGW<'a> {
    w: &'a mut W,
}
impl<'a> _PINCFGW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PINCFGW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "SIN is used for input data and SOUT is used for output data"]
    #[inline]
    pub fn pincfg_0(self) -> &'a mut W {
        self.variant(PINCFGW::PINCFG_0)
    }
    #[doc = "SIN is used for both input and output data"]
    #[inline]
    pub fn pincfg_1(self) -> &'a mut W {
        self.variant(PINCFGW::PINCFG_1)
    }
    #[doc = "SOUT is used for both input and output data"]
    #[inline]
    pub fn pincfg_2(self) -> &'a mut W {
        self.variant(PINCFGW::PINCFG_2)
    }
    #[doc = "SOUT is used for input data and SIN is used for output data"]
    #[inline]
    pub fn pincfg_3(self) -> &'a mut W {
        self.variant(PINCFGW::PINCFG_3)
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
#[doc = "Values that can be written to the field `OUTCFG`"]
pub enum OUTCFGW {
    #[doc = "Output data retains last value when chip select is negated"]
    OUTCFG_0,
    #[doc = "Output data is tristated when chip select is negated"]
    OUTCFG_1,
}
impl OUTCFGW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            OUTCFGW::OUTCFG_0 => false,
            OUTCFGW::OUTCFG_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _OUTCFGW<'a> {
    w: &'a mut W,
}
impl<'a> _OUTCFGW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: OUTCFGW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Output data retains last value when chip select is negated"]
    #[inline]
    pub fn outcfg_0(self) -> &'a mut W {
        self.variant(OUTCFGW::OUTCFG_0)
    }
    #[doc = "Output data is tristated when chip select is negated"]
    #[inline]
    pub fn outcfg_1(self) -> &'a mut W {
        self.variant(OUTCFGW::OUTCFG_1)
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
        const OFFSET: u8 = 26;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `PCSCFG`"]
pub enum PCSCFGW {
    #[doc = "PCS[3:2] are enabled"]
    PCSCFG_0,
    #[doc = "PCS[3:2] are disabled"]
    PCSCFG_1,
}
impl PCSCFGW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PCSCFGW::PCSCFG_0 => false,
            PCSCFGW::PCSCFG_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PCSCFGW<'a> {
    w: &'a mut W,
}
impl<'a> _PCSCFGW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PCSCFGW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "PCS[3:2] are enabled"]
    #[inline]
    pub fn pcscfg_0(self) -> &'a mut W {
        self.variant(PCSCFGW::PCSCFG_0)
    }
    #[doc = "PCS[3:2] are disabled"]
    #[inline]
    pub fn pcscfg_1(self) -> &'a mut W {
        self.variant(PCSCFGW::PCSCFG_1)
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
        const OFFSET: u8 = 27;
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
    #[doc = "Bit 0 - Master Mode"]
    #[inline]
    pub fn master(&self) -> MASTERR {
        MASTERR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - Sample Point"]
    #[inline]
    pub fn sample(&self) -> SAMPLER {
        SAMPLER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - Automatic PCS"]
    #[inline]
    pub fn autopcs(&self) -> AUTOPCSR {
        AUTOPCSR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - No Stall"]
    #[inline]
    pub fn nostall(&self) -> NOSTALLR {
        NOSTALLR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 8:11 - Peripheral Chip Select Polarity"]
    #[inline]
    pub fn pcspol(&self) -> PCSPOLR {
        PCSPOLR::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 16:18 - Match Configuration"]
    #[inline]
    pub fn matcfg(&self) -> MATCFGR {
        MATCFGR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 24:25 - Pin Configuration"]
    #[inline]
    pub fn pincfg(&self) -> PINCFGR {
        PINCFGR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 26 - Output Config"]
    #[inline]
    pub fn outcfg(&self) -> OUTCFGR {
        OUTCFGR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 26;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 27 - Peripheral Chip Select Configuration"]
    #[inline]
    pub fn pcscfg(&self) -> PCSCFGR {
        PCSCFGR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 27;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 0 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - Master Mode"]
    #[inline]
    pub fn master(&mut self) -> _MASTERW {
        _MASTERW { w: self }
    }
    #[doc = "Bit 1 - Sample Point"]
    #[inline]
    pub fn sample(&mut self) -> _SAMPLEW {
        _SAMPLEW { w: self }
    }
    #[doc = "Bit 2 - Automatic PCS"]
    #[inline]
    pub fn autopcs(&mut self) -> _AUTOPCSW {
        _AUTOPCSW { w: self }
    }
    #[doc = "Bit 3 - No Stall"]
    #[inline]
    pub fn nostall(&mut self) -> _NOSTALLW {
        _NOSTALLW { w: self }
    }
    #[doc = "Bits 8:11 - Peripheral Chip Select Polarity"]
    #[inline]
    pub fn pcspol(&mut self) -> _PCSPOLW {
        _PCSPOLW { w: self }
    }
    #[doc = "Bits 16:18 - Match Configuration"]
    #[inline]
    pub fn matcfg(&mut self) -> _MATCFGW {
        _MATCFGW { w: self }
    }
    #[doc = "Bits 24:25 - Pin Configuration"]
    #[inline]
    pub fn pincfg(&mut self) -> _PINCFGW {
        _PINCFGW { w: self }
    }
    #[doc = "Bit 26 - Output Config"]
    #[inline]
    pub fn outcfg(&mut self) -> _OUTCFGW {
        _OUTCFGW { w: self }
    }
    #[doc = "Bit 27 - Peripheral Chip Select Configuration"]
    #[inline]
    pub fn pcscfg(&mut self) -> _PCSCFGW {
        _PCSCFGW { w: self }
    }
}
