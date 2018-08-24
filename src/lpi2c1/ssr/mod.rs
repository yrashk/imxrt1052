#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::SSR {
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
#[doc = "Possible values of the field `TDF`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TDFR {
    #[doc = "Transmit data not requested"]
    TDF_0,
    #[doc = "Transmit data is requested"]
    TDF_1,
}
impl TDFR {
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
            TDFR::TDF_0 => false,
            TDFR::TDF_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TDFR {
        match value {
            false => TDFR::TDF_0,
            true => TDFR::TDF_1,
        }
    }
    #[doc = "Checks if the value of the field is `TDF_0`"]
    #[inline]
    pub fn is_tdf_0(&self) -> bool {
        *self == TDFR::TDF_0
    }
    #[doc = "Checks if the value of the field is `TDF_1`"]
    #[inline]
    pub fn is_tdf_1(&self) -> bool {
        *self == TDFR::TDF_1
    }
}
#[doc = "Possible values of the field `RDF`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RDFR {
    #[doc = "Receive data is not ready"]
    RDF_0,
    #[doc = "Receive data is ready"]
    RDF_1,
}
impl RDFR {
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
            RDFR::RDF_0 => false,
            RDFR::RDF_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RDFR {
        match value {
            false => RDFR::RDF_0,
            true => RDFR::RDF_1,
        }
    }
    #[doc = "Checks if the value of the field is `RDF_0`"]
    #[inline]
    pub fn is_rdf_0(&self) -> bool {
        *self == RDFR::RDF_0
    }
    #[doc = "Checks if the value of the field is `RDF_1`"]
    #[inline]
    pub fn is_rdf_1(&self) -> bool {
        *self == RDFR::RDF_1
    }
}
#[doc = "Possible values of the field `AVF`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AVFR {
    #[doc = "Address Status Register is not valid"]
    AVF_0,
    #[doc = "Address Status Register is valid"]
    AVF_1,
}
impl AVFR {
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
            AVFR::AVF_0 => false,
            AVFR::AVF_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> AVFR {
        match value {
            false => AVFR::AVF_0,
            true => AVFR::AVF_1,
        }
    }
    #[doc = "Checks if the value of the field is `AVF_0`"]
    #[inline]
    pub fn is_avf_0(&self) -> bool {
        *self == AVFR::AVF_0
    }
    #[doc = "Checks if the value of the field is `AVF_1`"]
    #[inline]
    pub fn is_avf_1(&self) -> bool {
        *self == AVFR::AVF_1
    }
}
#[doc = "Possible values of the field `TAF`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TAFR {
    #[doc = "Transmit ACK/NACK is not required"]
    TAF_0,
    #[doc = "Transmit ACK/NACK is required"]
    TAF_1,
}
impl TAFR {
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
            TAFR::TAF_0 => false,
            TAFR::TAF_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TAFR {
        match value {
            false => TAFR::TAF_0,
            true => TAFR::TAF_1,
        }
    }
    #[doc = "Checks if the value of the field is `TAF_0`"]
    #[inline]
    pub fn is_taf_0(&self) -> bool {
        *self == TAFR::TAF_0
    }
    #[doc = "Checks if the value of the field is `TAF_1`"]
    #[inline]
    pub fn is_taf_1(&self) -> bool {
        *self == TAFR::TAF_1
    }
}
#[doc = "Possible values of the field `RSF`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RSFR {
    #[doc = "Slave has not detected a Repeated START condition"]
    RSF_0,
    #[doc = "Slave has detected a Repeated START condition"]
    RSF_1,
}
impl RSFR {
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
            RSFR::RSF_0 => false,
            RSFR::RSF_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RSFR {
        match value {
            false => RSFR::RSF_0,
            true => RSFR::RSF_1,
        }
    }
    #[doc = "Checks if the value of the field is `RSF_0`"]
    #[inline]
    pub fn is_rsf_0(&self) -> bool {
        *self == RSFR::RSF_0
    }
    #[doc = "Checks if the value of the field is `RSF_1`"]
    #[inline]
    pub fn is_rsf_1(&self) -> bool {
        *self == RSFR::RSF_1
    }
}
#[doc = "Possible values of the field `SDF`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SDFR {
    #[doc = "Slave has not detected a STOP condition"]
    SDF_0,
    #[doc = "Slave has detected a STOP condition"]
    SDF_1,
}
impl SDFR {
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
            SDFR::SDF_0 => false,
            SDFR::SDF_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SDFR {
        match value {
            false => SDFR::SDF_0,
            true => SDFR::SDF_1,
        }
    }
    #[doc = "Checks if the value of the field is `SDF_0`"]
    #[inline]
    pub fn is_sdf_0(&self) -> bool {
        *self == SDFR::SDF_0
    }
    #[doc = "Checks if the value of the field is `SDF_1`"]
    #[inline]
    pub fn is_sdf_1(&self) -> bool {
        *self == SDFR::SDF_1
    }
}
#[doc = "Possible values of the field `BEF`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BEFR {
    #[doc = "Slave has not detected a bit error"]
    BEF_0,
    #[doc = "Slave has detected a bit error"]
    BEF_1,
}
impl BEFR {
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
            BEFR::BEF_0 => false,
            BEFR::BEF_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> BEFR {
        match value {
            false => BEFR::BEF_0,
            true => BEFR::BEF_1,
        }
    }
    #[doc = "Checks if the value of the field is `BEF_0`"]
    #[inline]
    pub fn is_bef_0(&self) -> bool {
        *self == BEFR::BEF_0
    }
    #[doc = "Checks if the value of the field is `BEF_1`"]
    #[inline]
    pub fn is_bef_1(&self) -> bool {
        *self == BEFR::BEF_1
    }
}
#[doc = "Possible values of the field `FEF`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FEFR {
    #[doc = "FIFO underflow or overflow was not detected"]
    FEF_0,
    #[doc = "FIFO underflow or overflow was detected"]
    FEF_1,
}
impl FEFR {
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
            FEFR::FEF_0 => false,
            FEFR::FEF_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> FEFR {
        match value {
            false => FEFR::FEF_0,
            true => FEFR::FEF_1,
        }
    }
    #[doc = "Checks if the value of the field is `FEF_0`"]
    #[inline]
    pub fn is_fef_0(&self) -> bool {
        *self == FEFR::FEF_0
    }
    #[doc = "Checks if the value of the field is `FEF_1`"]
    #[inline]
    pub fn is_fef_1(&self) -> bool {
        *self == FEFR::FEF_1
    }
}
#[doc = "Possible values of the field `AM0F`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AM0FR {
    #[doc = "Have not received an ADDR0 matching address"]
    AM0F_0,
    #[doc = "Have received an ADDR0 matching address"]
    AM0F_1,
}
impl AM0FR {
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
            AM0FR::AM0F_0 => false,
            AM0FR::AM0F_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> AM0FR {
        match value {
            false => AM0FR::AM0F_0,
            true => AM0FR::AM0F_1,
        }
    }
    #[doc = "Checks if the value of the field is `AM0F_0`"]
    #[inline]
    pub fn is_am0f_0(&self) -> bool {
        *self == AM0FR::AM0F_0
    }
    #[doc = "Checks if the value of the field is `AM0F_1`"]
    #[inline]
    pub fn is_am0f_1(&self) -> bool {
        *self == AM0FR::AM0F_1
    }
}
#[doc = "Possible values of the field `AM1F`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AM1FR {
    #[doc = "Have not received an ADDR1 or ADDR0/ADDR1 range matching address"]
    AM1F_0,
    #[doc = "Have received an ADDR1 or ADDR0/ADDR1 range matching address"]
    AM1F_1,
}
impl AM1FR {
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
            AM1FR::AM1F_0 => false,
            AM1FR::AM1F_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> AM1FR {
        match value {
            false => AM1FR::AM1F_0,
            true => AM1FR::AM1F_1,
        }
    }
    #[doc = "Checks if the value of the field is `AM1F_0`"]
    #[inline]
    pub fn is_am1f_0(&self) -> bool {
        *self == AM1FR::AM1F_0
    }
    #[doc = "Checks if the value of the field is `AM1F_1`"]
    #[inline]
    pub fn is_am1f_1(&self) -> bool {
        *self == AM1FR::AM1F_1
    }
}
#[doc = "Possible values of the field `GCF`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GCFR {
    #[doc = "Slave has not detected the General Call Address or the General Call Address is disabled"]
    GCF_0,
    #[doc = "Slave has detected the General Call Address"]
    GCF_1,
}
impl GCFR {
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
            GCFR::GCF_0 => false,
            GCFR::GCF_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> GCFR {
        match value {
            false => GCFR::GCF_0,
            true => GCFR::GCF_1,
        }
    }
    #[doc = "Checks if the value of the field is `GCF_0`"]
    #[inline]
    pub fn is_gcf_0(&self) -> bool {
        *self == GCFR::GCF_0
    }
    #[doc = "Checks if the value of the field is `GCF_1`"]
    #[inline]
    pub fn is_gcf_1(&self) -> bool {
        *self == GCFR::GCF_1
    }
}
#[doc = "Possible values of the field `SARF`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SARFR {
    #[doc = "SMBus Alert Response is disabled or not detected"]
    SARF_0,
    #[doc = "SMBus Alert Response is enabled and detected"]
    SARF_1,
}
impl SARFR {
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
            SARFR::SARF_0 => false,
            SARFR::SARF_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SARFR {
        match value {
            false => SARFR::SARF_0,
            true => SARFR::SARF_1,
        }
    }
    #[doc = "Checks if the value of the field is `SARF_0`"]
    #[inline]
    pub fn is_sarf_0(&self) -> bool {
        *self == SARFR::SARF_0
    }
    #[doc = "Checks if the value of the field is `SARF_1`"]
    #[inline]
    pub fn is_sarf_1(&self) -> bool {
        *self == SARFR::SARF_1
    }
}
#[doc = "Possible values of the field `SBF`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SBFR {
    #[doc = "I2C Slave is idle"]
    SBF_0,
    #[doc = "I2C Slave is busy"]
    SBF_1,
}
impl SBFR {
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
            SBFR::SBF_0 => false,
            SBFR::SBF_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SBFR {
        match value {
            false => SBFR::SBF_0,
            true => SBFR::SBF_1,
        }
    }
    #[doc = "Checks if the value of the field is `SBF_0`"]
    #[inline]
    pub fn is_sbf_0(&self) -> bool {
        *self == SBFR::SBF_0
    }
    #[doc = "Checks if the value of the field is `SBF_1`"]
    #[inline]
    pub fn is_sbf_1(&self) -> bool {
        *self == SBFR::SBF_1
    }
}
#[doc = "Possible values of the field `BBF`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BBFR {
    #[doc = "I2C Bus is idle"]
    BBF_0,
    #[doc = "I2C Bus is busy"]
    BBF_1,
}
impl BBFR {
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
            BBFR::BBF_0 => false,
            BBFR::BBF_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> BBFR {
        match value {
            false => BBFR::BBF_0,
            true => BBFR::BBF_1,
        }
    }
    #[doc = "Checks if the value of the field is `BBF_0`"]
    #[inline]
    pub fn is_bbf_0(&self) -> bool {
        *self == BBFR::BBF_0
    }
    #[doc = "Checks if the value of the field is `BBF_1`"]
    #[inline]
    pub fn is_bbf_1(&self) -> bool {
        *self == BBFR::BBF_1
    }
}
#[doc = "Values that can be written to the field `RSF`"]
pub enum RSFW {
    #[doc = "Slave has not detected a Repeated START condition"]
    RSF_0,
    #[doc = "Slave has detected a Repeated START condition"]
    RSF_1,
}
impl RSFW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RSFW::RSF_0 => false,
            RSFW::RSF_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RSFW<'a> {
    w: &'a mut W,
}
impl<'a> _RSFW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RSFW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Slave has not detected a Repeated START condition"]
    #[inline]
    pub fn rsf_0(self) -> &'a mut W {
        self.variant(RSFW::RSF_0)
    }
    #[doc = "Slave has detected a Repeated START condition"]
    #[inline]
    pub fn rsf_1(self) -> &'a mut W {
        self.variant(RSFW::RSF_1)
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
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `SDF`"]
pub enum SDFW {
    #[doc = "Slave has not detected a STOP condition"]
    SDF_0,
    #[doc = "Slave has detected a STOP condition"]
    SDF_1,
}
impl SDFW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SDFW::SDF_0 => false,
            SDFW::SDF_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SDFW<'a> {
    w: &'a mut W,
}
impl<'a> _SDFW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SDFW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Slave has not detected a STOP condition"]
    #[inline]
    pub fn sdf_0(self) -> &'a mut W {
        self.variant(SDFW::SDF_0)
    }
    #[doc = "Slave has detected a STOP condition"]
    #[inline]
    pub fn sdf_1(self) -> &'a mut W {
        self.variant(SDFW::SDF_1)
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
        const OFFSET: u8 = 9;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `BEF`"]
pub enum BEFW {
    #[doc = "Slave has not detected a bit error"]
    BEF_0,
    #[doc = "Slave has detected a bit error"]
    BEF_1,
}
impl BEFW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            BEFW::BEF_0 => false,
            BEFW::BEF_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _BEFW<'a> {
    w: &'a mut W,
}
impl<'a> _BEFW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: BEFW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Slave has not detected a bit error"]
    #[inline]
    pub fn bef_0(self) -> &'a mut W {
        self.variant(BEFW::BEF_0)
    }
    #[doc = "Slave has detected a bit error"]
    #[inline]
    pub fn bef_1(self) -> &'a mut W {
        self.variant(BEFW::BEF_1)
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
#[doc = "Values that can be written to the field `FEF`"]
pub enum FEFW {
    #[doc = "FIFO underflow or overflow was not detected"]
    FEF_0,
    #[doc = "FIFO underflow or overflow was detected"]
    FEF_1,
}
impl FEFW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            FEFW::FEF_0 => false,
            FEFW::FEF_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FEFW<'a> {
    w: &'a mut W,
}
impl<'a> _FEFW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FEFW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "FIFO underflow or overflow was not detected"]
    #[inline]
    pub fn fef_0(self) -> &'a mut W {
        self.variant(FEFW::FEF_0)
    }
    #[doc = "FIFO underflow or overflow was detected"]
    #[inline]
    pub fn fef_1(self) -> &'a mut W {
        self.variant(FEFW::FEF_1)
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
        const OFFSET: u8 = 11;
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
    #[doc = "Bit 0 - Transmit Data Flag"]
    #[inline]
    pub fn tdf(&self) -> TDFR {
        TDFR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - Receive Data Flag"]
    #[inline]
    pub fn rdf(&self) -> RDFR {
        RDFR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - Address Valid Flag"]
    #[inline]
    pub fn avf(&self) -> AVFR {
        AVFR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - Transmit ACK Flag"]
    #[inline]
    pub fn taf(&self) -> TAFR {
        TAFR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 8 - Repeated Start Flag"]
    #[inline]
    pub fn rsf(&self) -> RSFR {
        RSFR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 9 - STOP Detect Flag"]
    #[inline]
    pub fn sdf(&self) -> SDFR {
        SDFR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 10 - Bit Error Flag"]
    #[inline]
    pub fn bef(&self) -> BEFR {
        BEFR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 11 - FIFO Error Flag"]
    #[inline]
    pub fn fef(&self) -> FEFR {
        FEFR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 12 - Address Match 0 Flag"]
    #[inline]
    pub fn am0f(&self) -> AM0FR {
        AM0FR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 13 - Address Match 1 Flag"]
    #[inline]
    pub fn am1f(&self) -> AM1FR {
        AM1FR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 14 - General Call Flag"]
    #[inline]
    pub fn gcf(&self) -> GCFR {
        GCFR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 15 - SMBus Alert Response Flag"]
    #[inline]
    pub fn sarf(&self) -> SARFR {
        SARFR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 15;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 24 - Slave Busy Flag"]
    #[inline]
    pub fn sbf(&self) -> SBFR {
        SBFR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 25 - Bus Busy Flag"]
    #[inline]
    pub fn bbf(&self) -> BBFR {
        BBFR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 25;
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
    #[doc = "Bit 8 - Repeated Start Flag"]
    #[inline]
    pub fn rsf(&mut self) -> _RSFW {
        _RSFW { w: self }
    }
    #[doc = "Bit 9 - STOP Detect Flag"]
    #[inline]
    pub fn sdf(&mut self) -> _SDFW {
        _SDFW { w: self }
    }
    #[doc = "Bit 10 - Bit Error Flag"]
    #[inline]
    pub fn bef(&mut self) -> _BEFW {
        _BEFW { w: self }
    }
    #[doc = "Bit 11 - FIFO Error Flag"]
    #[inline]
    pub fn fef(&mut self) -> _FEFW {
        _FEFW { w: self }
    }
}
