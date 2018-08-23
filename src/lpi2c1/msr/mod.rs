#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::MSR {
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
    #[doc = "Transmit data is not requested"]
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
    #[doc = "Receive Data is not ready"]
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
#[doc = "Possible values of the field `EPF`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EPFR {
    #[doc = "Master has not generated a STOP or Repeated START condition"]
    EPF_0,
    #[doc = "Master has generated a STOP or Repeated START condition"]
    EPF_1,
}
impl EPFR {
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
            EPFR::EPF_0 => false,
            EPFR::EPF_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> EPFR {
        match value {
            false => EPFR::EPF_0,
            true => EPFR::EPF_1,
        }
    }
    #[doc = "Checks if the value of the field is `EPF_0`"]
    #[inline]
    pub fn is_epf_0(&self) -> bool {
        *self == EPFR::EPF_0
    }
    #[doc = "Checks if the value of the field is `EPF_1`"]
    #[inline]
    pub fn is_epf_1(&self) -> bool {
        *self == EPFR::EPF_1
    }
}
#[doc = "Possible values of the field `SDF`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SDFR {
    #[doc = "Master has not generated a STOP condition"]
    SDF_0,
    #[doc = "Master has generated a STOP condition"]
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
#[doc = "Possible values of the field `NDF`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NDFR {
    #[doc = "Unexpected NACK was not detected"]
    NDF_0,
    #[doc = "Unexpected NACK was detected"]
    NDF_1,
}
impl NDFR {
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
            NDFR::NDF_0 => false,
            NDFR::NDF_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> NDFR {
        match value {
            false => NDFR::NDF_0,
            true => NDFR::NDF_1,
        }
    }
    #[doc = "Checks if the value of the field is `NDF_0`"]
    #[inline]
    pub fn is_ndf_0(&self) -> bool {
        *self == NDFR::NDF_0
    }
    #[doc = "Checks if the value of the field is `NDF_1`"]
    #[inline]
    pub fn is_ndf_1(&self) -> bool {
        *self == NDFR::NDF_1
    }
}
#[doc = "Possible values of the field `ALF`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ALFR {
    #[doc = "Master has not lost arbitration"]
    ALF_0,
    #[doc = "Master has lost arbitration"]
    ALF_1,
}
impl ALFR {
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
            ALFR::ALF_0 => false,
            ALFR::ALF_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ALFR {
        match value {
            false => ALFR::ALF_0,
            true => ALFR::ALF_1,
        }
    }
    #[doc = "Checks if the value of the field is `ALF_0`"]
    #[inline]
    pub fn is_alf_0(&self) -> bool {
        *self == ALFR::ALF_0
    }
    #[doc = "Checks if the value of the field is `ALF_1`"]
    #[inline]
    pub fn is_alf_1(&self) -> bool {
        *self == ALFR::ALF_1
    }
}
#[doc = "Possible values of the field `FEF`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FEFR {
    #[doc = "No error"]
    FEF_0,
    #[doc = "Master sending or receiving data without a START condition"]
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
#[doc = "Possible values of the field `PLTF`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PLTFR {
    #[doc = "Pin low timeout has not occurred or is disabled"]
    PLTF_0,
    #[doc = "Pin low timeout has occurred"]
    PLTF_1,
}
impl PLTFR {
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
            PLTFR::PLTF_0 => false,
            PLTFR::PLTF_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PLTFR {
        match value {
            false => PLTFR::PLTF_0,
            true => PLTFR::PLTF_1,
        }
    }
    #[doc = "Checks if the value of the field is `PLTF_0`"]
    #[inline]
    pub fn is_pltf_0(&self) -> bool {
        *self == PLTFR::PLTF_0
    }
    #[doc = "Checks if the value of the field is `PLTF_1`"]
    #[inline]
    pub fn is_pltf_1(&self) -> bool {
        *self == PLTFR::PLTF_1
    }
}
#[doc = "Possible values of the field `DMF`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DMFR {
    #[doc = "Have not received matching data"]
    DMF_0,
    #[doc = "Have received matching data"]
    DMF_1,
}
impl DMFR {
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
            DMFR::DMF_0 => false,
            DMFR::DMF_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DMFR {
        match value {
            false => DMFR::DMF_0,
            true => DMFR::DMF_1,
        }
    }
    #[doc = "Checks if the value of the field is `DMF_0`"]
    #[inline]
    pub fn is_dmf_0(&self) -> bool {
        *self == DMFR::DMF_0
    }
    #[doc = "Checks if the value of the field is `DMF_1`"]
    #[inline]
    pub fn is_dmf_1(&self) -> bool {
        *self == DMFR::DMF_1
    }
}
#[doc = "Possible values of the field `MBF`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MBFR {
    #[doc = "I2C Master is idle"]
    MBF_0,
    #[doc = "I2C Master is busy"]
    MBF_1,
}
impl MBFR {
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
            MBFR::MBF_0 => false,
            MBFR::MBF_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> MBFR {
        match value {
            false => MBFR::MBF_0,
            true => MBFR::MBF_1,
        }
    }
    #[doc = "Checks if the value of the field is `MBF_0`"]
    #[inline]
    pub fn is_mbf_0(&self) -> bool {
        *self == MBFR::MBF_0
    }
    #[doc = "Checks if the value of the field is `MBF_1`"]
    #[inline]
    pub fn is_mbf_1(&self) -> bool {
        *self == MBFR::MBF_1
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
#[doc = "Values that can be written to the field `EPF`"]
pub enum EPFW {
    #[doc = "Master has not generated a STOP or Repeated START condition"]
    EPF_0,
    #[doc = "Master has generated a STOP or Repeated START condition"]
    EPF_1,
}
impl EPFW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            EPFW::EPF_0 => false,
            EPFW::EPF_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _EPFW<'a> {
    w: &'a mut W,
}
impl<'a> _EPFW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: EPFW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Master has not generated a STOP or Repeated START condition"]
    #[inline]
    pub fn epf_0(self) -> &'a mut W {
        self.variant(EPFW::EPF_0)
    }
    #[doc = "Master has generated a STOP or Repeated START condition"]
    #[inline]
    pub fn epf_1(self) -> &'a mut W {
        self.variant(EPFW::EPF_1)
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
    #[doc = "Master has not generated a STOP condition"]
    SDF_0,
    #[doc = "Master has generated a STOP condition"]
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
    #[doc = "Master has not generated a STOP condition"]
    #[inline]
    pub fn sdf_0(self) -> &'a mut W {
        self.variant(SDFW::SDF_0)
    }
    #[doc = "Master has generated a STOP condition"]
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
#[doc = "Values that can be written to the field `NDF`"]
pub enum NDFW {
    #[doc = "Unexpected NACK was not detected"]
    NDF_0,
    #[doc = "Unexpected NACK was detected"]
    NDF_1,
}
impl NDFW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            NDFW::NDF_0 => false,
            NDFW::NDF_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _NDFW<'a> {
    w: &'a mut W,
}
impl<'a> _NDFW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: NDFW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Unexpected NACK was not detected"]
    #[inline]
    pub fn ndf_0(self) -> &'a mut W {
        self.variant(NDFW::NDF_0)
    }
    #[doc = "Unexpected NACK was detected"]
    #[inline]
    pub fn ndf_1(self) -> &'a mut W {
        self.variant(NDFW::NDF_1)
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
#[doc = "Values that can be written to the field `ALF`"]
pub enum ALFW {
    #[doc = "Master has not lost arbitration"]
    ALF_0,
    #[doc = "Master has lost arbitration"]
    ALF_1,
}
impl ALFW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ALFW::ALF_0 => false,
            ALFW::ALF_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ALFW<'a> {
    w: &'a mut W,
}
impl<'a> _ALFW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ALFW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Master has not lost arbitration"]
    #[inline]
    pub fn alf_0(self) -> &'a mut W {
        self.variant(ALFW::ALF_0)
    }
    #[doc = "Master has lost arbitration"]
    #[inline]
    pub fn alf_1(self) -> &'a mut W {
        self.variant(ALFW::ALF_1)
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
#[doc = "Values that can be written to the field `FEF`"]
pub enum FEFW {
    #[doc = "No error"]
    FEF_0,
    #[doc = "Master sending or receiving data without a START condition"]
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
    #[doc = "No error"]
    #[inline]
    pub fn fef_0(self) -> &'a mut W {
        self.variant(FEFW::FEF_0)
    }
    #[doc = "Master sending or receiving data without a START condition"]
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
        const OFFSET: u8 = 12;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `PLTF`"]
pub enum PLTFW {
    #[doc = "Pin low timeout has not occurred or is disabled"]
    PLTF_0,
    #[doc = "Pin low timeout has occurred"]
    PLTF_1,
}
impl PLTFW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PLTFW::PLTF_0 => false,
            PLTFW::PLTF_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PLTFW<'a> {
    w: &'a mut W,
}
impl<'a> _PLTFW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PLTFW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Pin low timeout has not occurred or is disabled"]
    #[inline]
    pub fn pltf_0(self) -> &'a mut W {
        self.variant(PLTFW::PLTF_0)
    }
    #[doc = "Pin low timeout has occurred"]
    #[inline]
    pub fn pltf_1(self) -> &'a mut W {
        self.variant(PLTFW::PLTF_1)
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
        const OFFSET: u8 = 13;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `DMF`"]
pub enum DMFW {
    #[doc = "Have not received matching data"]
    DMF_0,
    #[doc = "Have received matching data"]
    DMF_1,
}
impl DMFW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DMFW::DMF_0 => false,
            DMFW::DMF_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DMFW<'a> {
    w: &'a mut W,
}
impl<'a> _DMFW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DMFW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Have not received matching data"]
    #[inline]
    pub fn dmf_0(self) -> &'a mut W {
        self.variant(DMFW::DMF_0)
    }
    #[doc = "Have received matching data"]
    #[inline]
    pub fn dmf_1(self) -> &'a mut W {
        self.variant(DMFW::DMF_1)
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
        const OFFSET: u8 = 14;
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
    #[doc = "Bit 8 - End Packet Flag"]
    #[inline]
    pub fn epf(&self) -> EPFR {
        EPFR::_from({
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
    #[doc = "Bit 10 - NACK Detect Flag"]
    #[inline]
    pub fn ndf(&self) -> NDFR {
        NDFR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 11 - Arbitration Lost Flag"]
    #[inline]
    pub fn alf(&self) -> ALFR {
        ALFR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 12 - FIFO Error Flag"]
    #[inline]
    pub fn fef(&self) -> FEFR {
        FEFR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 13 - Pin Low Timeout Flag"]
    #[inline]
    pub fn pltf(&self) -> PLTFR {
        PLTFR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 14 - Data Match Flag"]
    #[inline]
    pub fn dmf(&self) -> DMFR {
        DMFR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 24 - Master Busy Flag"]
    #[inline]
    pub fn mbf(&self) -> MBFR {
        MBFR::_from({
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
        W { bits: 1 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 8 - End Packet Flag"]
    #[inline]
    pub fn epf(&mut self) -> _EPFW {
        _EPFW { w: self }
    }
    #[doc = "Bit 9 - STOP Detect Flag"]
    #[inline]
    pub fn sdf(&mut self) -> _SDFW {
        _SDFW { w: self }
    }
    #[doc = "Bit 10 - NACK Detect Flag"]
    #[inline]
    pub fn ndf(&mut self) -> _NDFW {
        _NDFW { w: self }
    }
    #[doc = "Bit 11 - Arbitration Lost Flag"]
    #[inline]
    pub fn alf(&mut self) -> _ALFW {
        _ALFW { w: self }
    }
    #[doc = "Bit 12 - FIFO Error Flag"]
    #[inline]
    pub fn fef(&mut self) -> _FEFW {
        _FEFW { w: self }
    }
    #[doc = "Bit 13 - Pin Low Timeout Flag"]
    #[inline]
    pub fn pltf(&mut self) -> _PLTFW {
        _PLTFW { w: self }
    }
    #[doc = "Bit 14 - Data Match Flag"]
    #[inline]
    pub fn dmf(&mut self) -> _DMFW {
        _DMFW { w: self }
    }
}
