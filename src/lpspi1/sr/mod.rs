#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::SR {
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
#[doc = "Possible values of the field `WCF`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WCFR {
    #[doc = "Transfer of a received word has not yet completed"]
    WCF_0,
    #[doc = "Transfer of a received word has completed"]
    WCF_1,
}
impl WCFR {
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
            WCFR::WCF_0 => false,
            WCFR::WCF_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> WCFR {
        match value {
            false => WCFR::WCF_0,
            true => WCFR::WCF_1,
        }
    }
    #[doc = "Checks if the value of the field is `WCF_0`"]
    #[inline]
    pub fn is_wcf_0(&self) -> bool {
        *self == WCFR::WCF_0
    }
    #[doc = "Checks if the value of the field is `WCF_1`"]
    #[inline]
    pub fn is_wcf_1(&self) -> bool {
        *self == WCFR::WCF_1
    }
}
#[doc = "Possible values of the field `FCF`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FCFR {
    #[doc = "Frame transfer has not completed"]
    FCF_0,
    #[doc = "Frame transfer has completed"]
    FCF_1,
}
impl FCFR {
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
            FCFR::FCF_0 => false,
            FCFR::FCF_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> FCFR {
        match value {
            false => FCFR::FCF_0,
            true => FCFR::FCF_1,
        }
    }
    #[doc = "Checks if the value of the field is `FCF_0`"]
    #[inline]
    pub fn is_fcf_0(&self) -> bool {
        *self == FCFR::FCF_0
    }
    #[doc = "Checks if the value of the field is `FCF_1`"]
    #[inline]
    pub fn is_fcf_1(&self) -> bool {
        *self == FCFR::FCF_1
    }
}
#[doc = "Possible values of the field `TCF`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TCFR {
    #[doc = "All transfers have not completed"]
    TCF_0,
    #[doc = "All transfers have completed"]
    TCF_1,
}
impl TCFR {
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
            TCFR::TCF_0 => false,
            TCFR::TCF_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TCFR {
        match value {
            false => TCFR::TCF_0,
            true => TCFR::TCF_1,
        }
    }
    #[doc = "Checks if the value of the field is `TCF_0`"]
    #[inline]
    pub fn is_tcf_0(&self) -> bool {
        *self == TCFR::TCF_0
    }
    #[doc = "Checks if the value of the field is `TCF_1`"]
    #[inline]
    pub fn is_tcf_1(&self) -> bool {
        *self == TCFR::TCF_1
    }
}
#[doc = "Possible values of the field `TEF`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TEFR {
    #[doc = "Transmit FIFO underrun has not occurred"]
    TEF_0,
    #[doc = "Transmit FIFO underrun has occurred"]
    TEF_1,
}
impl TEFR {
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
            TEFR::TEF_0 => false,
            TEFR::TEF_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TEFR {
        match value {
            false => TEFR::TEF_0,
            true => TEFR::TEF_1,
        }
    }
    #[doc = "Checks if the value of the field is `TEF_0`"]
    #[inline]
    pub fn is_tef_0(&self) -> bool {
        *self == TEFR::TEF_0
    }
    #[doc = "Checks if the value of the field is `TEF_1`"]
    #[inline]
    pub fn is_tef_1(&self) -> bool {
        *self == TEFR::TEF_1
    }
}
#[doc = "Possible values of the field `REF`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REFR {
    #[doc = "Receive FIFO has not overflowed"]
    REF_0,
    #[doc = "Receive FIFO has overflowed"]
    REF_1,
}
impl REFR {
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
            REFR::REF_0 => false,
            REFR::REF_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> REFR {
        match value {
            false => REFR::REF_0,
            true => REFR::REF_1,
        }
    }
    #[doc = "Checks if the value of the field is `REF_0`"]
    #[inline]
    pub fn is_ref_0(&self) -> bool {
        *self == REFR::REF_0
    }
    #[doc = "Checks if the value of the field is `REF_1`"]
    #[inline]
    pub fn is_ref_1(&self) -> bool {
        *self == REFR::REF_1
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
    #[doc = "LPSPI is idle"]
    MBF_0,
    #[doc = "LPSPI is busy"]
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
#[doc = "Values that can be written to the field `WCF`"]
pub enum WCFW {
    #[doc = "Transfer of a received word has not yet completed"]
    WCF_0,
    #[doc = "Transfer of a received word has completed"]
    WCF_1,
}
impl WCFW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            WCFW::WCF_0 => false,
            WCFW::WCF_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _WCFW<'a> {
    w: &'a mut W,
}
impl<'a> _WCFW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: WCFW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Transfer of a received word has not yet completed"]
    #[inline]
    pub fn wcf_0(self) -> &'a mut W {
        self.variant(WCFW::WCF_0)
    }
    #[doc = "Transfer of a received word has completed"]
    #[inline]
    pub fn wcf_1(self) -> &'a mut W {
        self.variant(WCFW::WCF_1)
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
#[doc = "Values that can be written to the field `FCF`"]
pub enum FCFW {
    #[doc = "Frame transfer has not completed"]
    FCF_0,
    #[doc = "Frame transfer has completed"]
    FCF_1,
}
impl FCFW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            FCFW::FCF_0 => false,
            FCFW::FCF_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FCFW<'a> {
    w: &'a mut W,
}
impl<'a> _FCFW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FCFW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Frame transfer has not completed"]
    #[inline]
    pub fn fcf_0(self) -> &'a mut W {
        self.variant(FCFW::FCF_0)
    }
    #[doc = "Frame transfer has completed"]
    #[inline]
    pub fn fcf_1(self) -> &'a mut W {
        self.variant(FCFW::FCF_1)
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
#[doc = "Values that can be written to the field `TCF`"]
pub enum TCFW {
    #[doc = "All transfers have not completed"]
    TCF_0,
    #[doc = "All transfers have completed"]
    TCF_1,
}
impl TCFW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TCFW::TCF_0 => false,
            TCFW::TCF_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TCFW<'a> {
    w: &'a mut W,
}
impl<'a> _TCFW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TCFW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "All transfers have not completed"]
    #[inline]
    pub fn tcf_0(self) -> &'a mut W {
        self.variant(TCFW::TCF_0)
    }
    #[doc = "All transfers have completed"]
    #[inline]
    pub fn tcf_1(self) -> &'a mut W {
        self.variant(TCFW::TCF_1)
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
#[doc = "Values that can be written to the field `TEF`"]
pub enum TEFW {
    #[doc = "Transmit FIFO underrun has not occurred"]
    TEF_0,
    #[doc = "Transmit FIFO underrun has occurred"]
    TEF_1,
}
impl TEFW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TEFW::TEF_0 => false,
            TEFW::TEF_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TEFW<'a> {
    w: &'a mut W,
}
impl<'a> _TEFW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TEFW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Transmit FIFO underrun has not occurred"]
    #[inline]
    pub fn tef_0(self) -> &'a mut W {
        self.variant(TEFW::TEF_0)
    }
    #[doc = "Transmit FIFO underrun has occurred"]
    #[inline]
    pub fn tef_1(self) -> &'a mut W {
        self.variant(TEFW::TEF_1)
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
#[doc = "Values that can be written to the field `REF`"]
pub enum REFW {
    #[doc = "Receive FIFO has not overflowed"]
    REF_0,
    #[doc = "Receive FIFO has overflowed"]
    REF_1,
}
impl REFW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            REFW::REF_0 => false,
            REFW::REF_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _REFW<'a> {
    w: &'a mut W,
}
impl<'a> _REFW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: REFW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Receive FIFO has not overflowed"]
    #[inline]
    pub fn ref_0(self) -> &'a mut W {
        self.variant(REFW::REF_0)
    }
    #[doc = "Receive FIFO has overflowed"]
    #[inline]
    pub fn ref_1(self) -> &'a mut W {
        self.variant(REFW::REF_1)
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
        const OFFSET: u8 = 13;
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
    #[doc = "Bit 8 - Word Complete Flag"]
    #[inline]
    pub fn wcf(&self) -> WCFR {
        WCFR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 9 - Frame Complete Flag"]
    #[inline]
    pub fn fcf(&self) -> FCFR {
        FCFR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 10 - Transfer Complete Flag"]
    #[inline]
    pub fn tcf(&self) -> TCFR {
        TCFR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 11 - Transmit Error Flag"]
    #[inline]
    pub fn tef(&self) -> TEFR {
        TEFR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 12 - Receive Error Flag"]
    #[inline]
    pub fn ref_(&self) -> REFR {
        REFR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 13 - Data Match Flag"]
    #[inline]
    pub fn dmf(&self) -> DMFR {
        DMFR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 24 - Module Busy Flag"]
    #[inline]
    pub fn mbf(&self) -> MBFR {
        MBFR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 24;
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
    #[doc = "Bit 8 - Word Complete Flag"]
    #[inline]
    pub fn wcf(&mut self) -> _WCFW {
        _WCFW { w: self }
    }
    #[doc = "Bit 9 - Frame Complete Flag"]
    #[inline]
    pub fn fcf(&mut self) -> _FCFW {
        _FCFW { w: self }
    }
    #[doc = "Bit 10 - Transfer Complete Flag"]
    #[inline]
    pub fn tcf(&mut self) -> _TCFW {
        _TCFW { w: self }
    }
    #[doc = "Bit 11 - Transmit Error Flag"]
    #[inline]
    pub fn tef(&mut self) -> _TEFW {
        _TEFW { w: self }
    }
    #[doc = "Bit 12 - Receive Error Flag"]
    #[inline]
    pub fn ref_(&mut self) -> _REFW {
        _REFW { w: self }
    }
    #[doc = "Bit 13 - Data Match Flag"]
    #[inline]
    pub fn dmf(&mut self) -> _DMFW {
        _DMFW { w: self }
    }
}
