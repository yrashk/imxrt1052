#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::ES {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = "Possible values of the field `DBE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DBER {
    #[doc = "No destination bus error"]
    DBE_0,
    #[doc = "The last recorded error was a bus error on a destination write"]
    DBE_1,
}
impl DBER {
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
            DBER::DBE_0 => false,
            DBER::DBE_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DBER {
        match value {
            false => DBER::DBE_0,
            true => DBER::DBE_1,
        }
    }
    #[doc = "Checks if the value of the field is `DBE_0`"]
    #[inline]
    pub fn is_dbe_0(&self) -> bool {
        *self == DBER::DBE_0
    }
    #[doc = "Checks if the value of the field is `DBE_1`"]
    #[inline]
    pub fn is_dbe_1(&self) -> bool {
        *self == DBER::DBE_1
    }
}
#[doc = "Possible values of the field `SBE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SBER {
    #[doc = "No source bus error"]
    SBE_0,
    #[doc = "The last recorded error was a bus error on a source read"]
    SBE_1,
}
impl SBER {
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
            SBER::SBE_0 => false,
            SBER::SBE_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SBER {
        match value {
            false => SBER::SBE_0,
            true => SBER::SBE_1,
        }
    }
    #[doc = "Checks if the value of the field is `SBE_0`"]
    #[inline]
    pub fn is_sbe_0(&self) -> bool {
        *self == SBER::SBE_0
    }
    #[doc = "Checks if the value of the field is `SBE_1`"]
    #[inline]
    pub fn is_sbe_1(&self) -> bool {
        *self == SBER::SBE_1
    }
}
#[doc = "Possible values of the field `SGE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SGER {
    #[doc = "No scatter/gather configuration error"]
    SGE_0,
    #[doc = "The last recorded error was a configuration error detected in the TCDn_DLASTSGA field. This field is checked at the beginning of a scatter/gather operation after major loop completion if TCDn_CSR[ESG] is enabled. TCDn_DLASTSGA is not on a 32 byte boundary."]
    SGE_1,
}
impl SGER {
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
            SGER::SGE_0 => false,
            SGER::SGE_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SGER {
        match value {
            false => SGER::SGE_0,
            true => SGER::SGE_1,
        }
    }
    #[doc = "Checks if the value of the field is `SGE_0`"]
    #[inline]
    pub fn is_sge_0(&self) -> bool {
        *self == SGER::SGE_0
    }
    #[doc = "Checks if the value of the field is `SGE_1`"]
    #[inline]
    pub fn is_sge_1(&self) -> bool {
        *self == SGER::SGE_1
    }
}
#[doc = "Possible values of the field `NCE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NCER {
    #[doc = "No NBYTES/CITER configuration error"]
    NCE_0,
    #[doc = "no description available"]
    NCE_1,
}
impl NCER {
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
            NCER::NCE_0 => false,
            NCER::NCE_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> NCER {
        match value {
            false => NCER::NCE_0,
            true => NCER::NCE_1,
        }
    }
    #[doc = "Checks if the value of the field is `NCE_0`"]
    #[inline]
    pub fn is_nce_0(&self) -> bool {
        *self == NCER::NCE_0
    }
    #[doc = "Checks if the value of the field is `NCE_1`"]
    #[inline]
    pub fn is_nce_1(&self) -> bool {
        *self == NCER::NCE_1
    }
}
#[doc = "Possible values of the field `DOE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DOER {
    #[doc = "No destination offset configuration error"]
    DOE_0,
    #[doc = "The last recorded error was a configuration error detected in the TCDn_DOFF field. TCDn_DOFF is inconsistent with TCDn_ATTR[DSIZE]."]
    DOE_1,
}
impl DOER {
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
            DOER::DOE_0 => false,
            DOER::DOE_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DOER {
        match value {
            false => DOER::DOE_0,
            true => DOER::DOE_1,
        }
    }
    #[doc = "Checks if the value of the field is `DOE_0`"]
    #[inline]
    pub fn is_doe_0(&self) -> bool {
        *self == DOER::DOE_0
    }
    #[doc = "Checks if the value of the field is `DOE_1`"]
    #[inline]
    pub fn is_doe_1(&self) -> bool {
        *self == DOER::DOE_1
    }
}
#[doc = "Possible values of the field `DAE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DAER {
    #[doc = "No destination address configuration error"]
    DAE_0,
    #[doc = "The last recorded error was a configuration error detected in the TCDn_DADDR field. TCDn_DADDR is inconsistent with TCDn_ATTR[DSIZE]."]
    DAE_1,
}
impl DAER {
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
            DAER::DAE_0 => false,
            DAER::DAE_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DAER {
        match value {
            false => DAER::DAE_0,
            true => DAER::DAE_1,
        }
    }
    #[doc = "Checks if the value of the field is `DAE_0`"]
    #[inline]
    pub fn is_dae_0(&self) -> bool {
        *self == DAER::DAE_0
    }
    #[doc = "Checks if the value of the field is `DAE_1`"]
    #[inline]
    pub fn is_dae_1(&self) -> bool {
        *self == DAER::DAE_1
    }
}
#[doc = "Possible values of the field `SOE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SOER {
    #[doc = "No source offset configuration error"]
    SOE_0,
    #[doc = "The last recorded error was a configuration error detected in the TCDn_SOFF field. TCDn_SOFF is inconsistent with TCDn_ATTR[SSIZE]."]
    SOE_1,
}
impl SOER {
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
            SOER::SOE_0 => false,
            SOER::SOE_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SOER {
        match value {
            false => SOER::SOE_0,
            true => SOER::SOE_1,
        }
    }
    #[doc = "Checks if the value of the field is `SOE_0`"]
    #[inline]
    pub fn is_soe_0(&self) -> bool {
        *self == SOER::SOE_0
    }
    #[doc = "Checks if the value of the field is `SOE_1`"]
    #[inline]
    pub fn is_soe_1(&self) -> bool {
        *self == SOER::SOE_1
    }
}
#[doc = "Possible values of the field `SAE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SAER {
    #[doc = "No source address configuration error."]
    SAE_0,
    #[doc = "The last recorded error was a configuration error detected in the TCDn_SADDR field. TCDn_SADDR is inconsistent with TCDn_ATTR[SSIZE]."]
    SAE_1,
}
impl SAER {
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
            SAER::SAE_0 => false,
            SAER::SAE_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SAER {
        match value {
            false => SAER::SAE_0,
            true => SAER::SAE_1,
        }
    }
    #[doc = "Checks if the value of the field is `SAE_0`"]
    #[inline]
    pub fn is_sae_0(&self) -> bool {
        *self == SAER::SAE_0
    }
    #[doc = "Checks if the value of the field is `SAE_1`"]
    #[inline]
    pub fn is_sae_1(&self) -> bool {
        *self == SAER::SAE_1
    }
}
#[doc = r" Value of the field"]
pub struct ERRCHNR {
    bits: u8,
}
impl ERRCHNR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Possible values of the field `CPE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CPER {
    #[doc = "No channel priority error"]
    CPE_0,
    #[doc = "no description available"]
    CPE_1,
}
impl CPER {
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
            CPER::CPE_0 => false,
            CPER::CPE_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CPER {
        match value {
            false => CPER::CPE_0,
            true => CPER::CPE_1,
        }
    }
    #[doc = "Checks if the value of the field is `CPE_0`"]
    #[inline]
    pub fn is_cpe_0(&self) -> bool {
        *self == CPER::CPE_0
    }
    #[doc = "Checks if the value of the field is `CPE_1`"]
    #[inline]
    pub fn is_cpe_1(&self) -> bool {
        *self == CPER::CPE_1
    }
}
#[doc = "Possible values of the field `GPE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPER {
    #[doc = "No group priority error"]
    GPE_0,
    #[doc = "The last recorded error was a configuration error among the group priorities. All group priorities are not unique."]
    GPE_1,
}
impl GPER {
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
            GPER::GPE_0 => false,
            GPER::GPE_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> GPER {
        match value {
            false => GPER::GPE_0,
            true => GPER::GPE_1,
        }
    }
    #[doc = "Checks if the value of the field is `GPE_0`"]
    #[inline]
    pub fn is_gpe_0(&self) -> bool {
        *self == GPER::GPE_0
    }
    #[doc = "Checks if the value of the field is `GPE_1`"]
    #[inline]
    pub fn is_gpe_1(&self) -> bool {
        *self == GPER::GPE_1
    }
}
#[doc = "Possible values of the field `ECX`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ECXR {
    #[doc = "No canceled transfers"]
    ECX_0,
    #[doc = "The last recorded entry was a canceled transfer by the error cancel transfer input"]
    ECX_1,
}
impl ECXR {
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
            ECXR::ECX_0 => false,
            ECXR::ECX_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ECXR {
        match value {
            false => ECXR::ECX_0,
            true => ECXR::ECX_1,
        }
    }
    #[doc = "Checks if the value of the field is `ECX_0`"]
    #[inline]
    pub fn is_ecx_0(&self) -> bool {
        *self == ECXR::ECX_0
    }
    #[doc = "Checks if the value of the field is `ECX_1`"]
    #[inline]
    pub fn is_ecx_1(&self) -> bool {
        *self == ECXR::ECX_1
    }
}
#[doc = "Possible values of the field `VLD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VLDR {
    #[doc = "No ERR bits are set."]
    VLD_0,
    #[doc = "At least one ERR bit is set indicating a valid error exists that has not been cleared."]
    VLD_1,
}
impl VLDR {
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
            VLDR::VLD_0 => false,
            VLDR::VLD_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> VLDR {
        match value {
            false => VLDR::VLD_0,
            true => VLDR::VLD_1,
        }
    }
    #[doc = "Checks if the value of the field is `VLD_0`"]
    #[inline]
    pub fn is_vld_0(&self) -> bool {
        *self == VLDR::VLD_0
    }
    #[doc = "Checks if the value of the field is `VLD_1`"]
    #[inline]
    pub fn is_vld_1(&self) -> bool {
        *self == VLDR::VLD_1
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - Destination Bus Error"]
    #[inline]
    pub fn dbe(&self) -> DBER {
        DBER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - Source Bus Error"]
    #[inline]
    pub fn sbe(&self) -> SBER {
        SBER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - Scatter/Gather Configuration Error"]
    #[inline]
    pub fn sge(&self) -> SGER {
        SGER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - NBYTES/CITER Configuration Error"]
    #[inline]
    pub fn nce(&self) -> NCER {
        NCER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 4 - Destination Offset Error"]
    #[inline]
    pub fn doe(&self) -> DOER {
        DOER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 5 - Destination Address Error"]
    #[inline]
    pub fn dae(&self) -> DAER {
        DAER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 6 - Source Offset Error"]
    #[inline]
    pub fn soe(&self) -> SOER {
        SOER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 7 - Source Address Error"]
    #[inline]
    pub fn sae(&self) -> SAER {
        SAER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 8:12 - Error Channel Number or Canceled Channel Number"]
    #[inline]
    pub fn errchn(&self) -> ERRCHNR {
        let bits = {
            const MASK: u8 = 31;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        ERRCHNR { bits }
    }
    #[doc = "Bit 14 - Channel Priority Error"]
    #[inline]
    pub fn cpe(&self) -> CPER {
        CPER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 15 - Group Priority Error"]
    #[inline]
    pub fn gpe(&self) -> GPER {
        GPER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 15;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 16 - Transfer Canceled"]
    #[inline]
    pub fn ecx(&self) -> ECXR {
        ECXR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 31 - VLD"]
    #[inline]
    pub fn vld(&self) -> VLDR {
        VLDR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 31;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
}
