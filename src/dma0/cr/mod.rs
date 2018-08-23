#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CR {
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
#[doc = "Possible values of the field `EDBG`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EDBGR {
    #[doc = "no description available"]
    EDBG_0,
    #[doc = "no description available"]
    EDBG_1,
}
impl EDBGR {
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
            EDBGR::EDBG_0 => false,
            EDBGR::EDBG_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> EDBGR {
        match value {
            false => EDBGR::EDBG_0,
            true => EDBGR::EDBG_1,
        }
    }
    #[doc = "Checks if the value of the field is `EDBG_0`"]
    #[inline]
    pub fn is_edbg_0(&self) -> bool {
        *self == EDBGR::EDBG_0
    }
    #[doc = "Checks if the value of the field is `EDBG_1`"]
    #[inline]
    pub fn is_edbg_1(&self) -> bool {
        *self == EDBGR::EDBG_1
    }
}
#[doc = "Possible values of the field `ERCA`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ERCAR {
    #[doc = "no description available"]
    ERCA_0,
    #[doc = "no description available"]
    ERCA_1,
}
impl ERCAR {
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
            ERCAR::ERCA_0 => false,
            ERCAR::ERCA_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ERCAR {
        match value {
            false => ERCAR::ERCA_0,
            true => ERCAR::ERCA_1,
        }
    }
    #[doc = "Checks if the value of the field is `ERCA_0`"]
    #[inline]
    pub fn is_erca_0(&self) -> bool {
        *self == ERCAR::ERCA_0
    }
    #[doc = "Checks if the value of the field is `ERCA_1`"]
    #[inline]
    pub fn is_erca_1(&self) -> bool {
        *self == ERCAR::ERCA_1
    }
}
#[doc = "Possible values of the field `ERGA`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ERGAR {
    #[doc = "Fixed priority arbitration is used for selection among the groups."]
    ERGA_0,
    #[doc = "Round robin arbitration is used for selection among the groups."]
    ERGA_1,
}
impl ERGAR {
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
            ERGAR::ERGA_0 => false,
            ERGAR::ERGA_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ERGAR {
        match value {
            false => ERGAR::ERGA_0,
            true => ERGAR::ERGA_1,
        }
    }
    #[doc = "Checks if the value of the field is `ERGA_0`"]
    #[inline]
    pub fn is_erga_0(&self) -> bool {
        *self == ERGAR::ERGA_0
    }
    #[doc = "Checks if the value of the field is `ERGA_1`"]
    #[inline]
    pub fn is_erga_1(&self) -> bool {
        *self == ERGAR::ERGA_1
    }
}
#[doc = "Possible values of the field `HOE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HOER {
    #[doc = "Normal operation"]
    HOE_0,
    #[doc = "Any error causes the HALT bit to set. Subsequently, all service requests are ignored until the HALT bit is cleared."]
    HOE_1,
}
impl HOER {
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
            HOER::HOE_0 => false,
            HOER::HOE_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> HOER {
        match value {
            false => HOER::HOE_0,
            true => HOER::HOE_1,
        }
    }
    #[doc = "Checks if the value of the field is `HOE_0`"]
    #[inline]
    pub fn is_hoe_0(&self) -> bool {
        *self == HOER::HOE_0
    }
    #[doc = "Checks if the value of the field is `HOE_1`"]
    #[inline]
    pub fn is_hoe_1(&self) -> bool {
        *self == HOER::HOE_1
    }
}
#[doc = "Possible values of the field `HALT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HALTR {
    #[doc = "Normal operation"]
    HALT_0,
    #[doc = "Stall the start of any new channels. Executing channels are allowed to complete. Channel execution resumes when this bit is cleared."]
    HALT_1,
}
impl HALTR {
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
            HALTR::HALT_0 => false,
            HALTR::HALT_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> HALTR {
        match value {
            false => HALTR::HALT_0,
            true => HALTR::HALT_1,
        }
    }
    #[doc = "Checks if the value of the field is `HALT_0`"]
    #[inline]
    pub fn is_halt_0(&self) -> bool {
        *self == HALTR::HALT_0
    }
    #[doc = "Checks if the value of the field is `HALT_1`"]
    #[inline]
    pub fn is_halt_1(&self) -> bool {
        *self == HALTR::HALT_1
    }
}
#[doc = "Possible values of the field `CLM`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CLMR {
    #[doc = "A minor loop channel link made to itself goes through channel arbitration before being activated again."]
    CLM_0,
    #[doc = "A minor loop channel link made to itself does not go through channel arbitration before being activated again. Upon minor loop completion, the channel activates again if that channel has a minor loop channel link enabled and the link channel is itself. This effectively applies the minor loop offsets and restarts the next minor loop."]
    CLM_1,
}
impl CLMR {
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
            CLMR::CLM_0 => false,
            CLMR::CLM_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CLMR {
        match value {
            false => CLMR::CLM_0,
            true => CLMR::CLM_1,
        }
    }
    #[doc = "Checks if the value of the field is `CLM_0`"]
    #[inline]
    pub fn is_clm_0(&self) -> bool {
        *self == CLMR::CLM_0
    }
    #[doc = "Checks if the value of the field is `CLM_1`"]
    #[inline]
    pub fn is_clm_1(&self) -> bool {
        *self == CLMR::CLM_1
    }
}
#[doc = "Possible values of the field `EMLM`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EMLMR {
    #[doc = "Disabled. TCDn.word2 is defined as a 32-bit NBYTES field."]
    EMLM_0,
    #[doc = "Enabled. TCDn.word2 is redefined to include individual enable fields, an offset field, and the NBYTES field. The individual enable fields allow the minor loop offset to be applied to the source address, the destination address, or both. The NBYTES field is reduced when either offset is enabled."]
    EMLM_1,
}
impl EMLMR {
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
            EMLMR::EMLM_0 => false,
            EMLMR::EMLM_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> EMLMR {
        match value {
            false => EMLMR::EMLM_0,
            true => EMLMR::EMLM_1,
        }
    }
    #[doc = "Checks if the value of the field is `EMLM_0`"]
    #[inline]
    pub fn is_emlm_0(&self) -> bool {
        *self == EMLMR::EMLM_0
    }
    #[doc = "Checks if the value of the field is `EMLM_1`"]
    #[inline]
    pub fn is_emlm_1(&self) -> bool {
        *self == EMLMR::EMLM_1
    }
}
#[doc = r" Value of the field"]
pub struct GRP0PRIR {
    bits: bool,
}
impl GRP0PRIR {
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
pub struct GRP1PRIR {
    bits: bool,
}
impl GRP1PRIR {
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
#[doc = "Possible values of the field `ECX`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ECXR {
    #[doc = "Normal operation"]
    ECX_0,
    #[doc = "Cancel the remaining data transfer in the same fashion as the CX bit. Stop the executing channel and force the minor loop to finish. The cancel takes effect after the last write of the current read/write sequence. The ECX bit clears itself after the cancel is honored. In addition to cancelling the transfer, ECX treats the cancel as an error condition, thus updating the Error Status register (DMAx_ES) and generating an optional error interrupt."]
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
#[doc = "Possible values of the field `CX`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CXR {
    #[doc = "Normal operation"]
    CX_0,
    #[doc = "Cancel the remaining data transfer. Stop the executing channel and force the minor loop to finish. The cancel takes effect after the last write of the current read/write sequence. The CX bit clears itself after the cancel has been honored. This cancel retires the channel normally as if the minor loop was completed."]
    CX_1,
}
impl CXR {
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
            CXR::CX_0 => false,
            CXR::CX_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CXR {
        match value {
            false => CXR::CX_0,
            true => CXR::CX_1,
        }
    }
    #[doc = "Checks if the value of the field is `CX_0`"]
    #[inline]
    pub fn is_cx_0(&self) -> bool {
        *self == CXR::CX_0
    }
    #[doc = "Checks if the value of the field is `CX_1`"]
    #[inline]
    pub fn is_cx_1(&self) -> bool {
        *self == CXR::CX_1
    }
}
#[doc = "Possible values of the field `ACTIVE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ACTIVER {
    #[doc = "eDMA is idle."]
    ACTIVE_0,
    #[doc = "eDMA is executing a channel."]
    ACTIVE_1,
}
impl ACTIVER {
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
            ACTIVER::ACTIVE_0 => false,
            ACTIVER::ACTIVE_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ACTIVER {
        match value {
            false => ACTIVER::ACTIVE_0,
            true => ACTIVER::ACTIVE_1,
        }
    }
    #[doc = "Checks if the value of the field is `ACTIVE_0`"]
    #[inline]
    pub fn is_active_0(&self) -> bool {
        *self == ACTIVER::ACTIVE_0
    }
    #[doc = "Checks if the value of the field is `ACTIVE_1`"]
    #[inline]
    pub fn is_active_1(&self) -> bool {
        *self == ACTIVER::ACTIVE_1
    }
}
#[doc = "Values that can be written to the field `EDBG`"]
pub enum EDBGW {
    #[doc = "no description available"]
    EDBG_0,
    #[doc = "no description available"]
    EDBG_1,
}
impl EDBGW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            EDBGW::EDBG_0 => false,
            EDBGW::EDBG_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _EDBGW<'a> {
    w: &'a mut W,
}
impl<'a> _EDBGW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: EDBGW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "no description available"]
    #[inline]
    pub fn edbg_0(self) -> &'a mut W {
        self.variant(EDBGW::EDBG_0)
    }
    #[doc = "no description available"]
    #[inline]
    pub fn edbg_1(self) -> &'a mut W {
        self.variant(EDBGW::EDBG_1)
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
#[doc = "Values that can be written to the field `ERCA`"]
pub enum ERCAW {
    #[doc = "no description available"]
    ERCA_0,
    #[doc = "no description available"]
    ERCA_1,
}
impl ERCAW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ERCAW::ERCA_0 => false,
            ERCAW::ERCA_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ERCAW<'a> {
    w: &'a mut W,
}
impl<'a> _ERCAW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ERCAW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "no description available"]
    #[inline]
    pub fn erca_0(self) -> &'a mut W {
        self.variant(ERCAW::ERCA_0)
    }
    #[doc = "no description available"]
    #[inline]
    pub fn erca_1(self) -> &'a mut W {
        self.variant(ERCAW::ERCA_1)
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
#[doc = "Values that can be written to the field `ERGA`"]
pub enum ERGAW {
    #[doc = "Fixed priority arbitration is used for selection among the groups."]
    ERGA_0,
    #[doc = "Round robin arbitration is used for selection among the groups."]
    ERGA_1,
}
impl ERGAW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ERGAW::ERGA_0 => false,
            ERGAW::ERGA_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ERGAW<'a> {
    w: &'a mut W,
}
impl<'a> _ERGAW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ERGAW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Fixed priority arbitration is used for selection among the groups."]
    #[inline]
    pub fn erga_0(self) -> &'a mut W {
        self.variant(ERGAW::ERGA_0)
    }
    #[doc = "Round robin arbitration is used for selection among the groups."]
    #[inline]
    pub fn erga_1(self) -> &'a mut W {
        self.variant(ERGAW::ERGA_1)
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
#[doc = "Values that can be written to the field `HOE`"]
pub enum HOEW {
    #[doc = "Normal operation"]
    HOE_0,
    #[doc = "Any error causes the HALT bit to set. Subsequently, all service requests are ignored until the HALT bit is cleared."]
    HOE_1,
}
impl HOEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            HOEW::HOE_0 => false,
            HOEW::HOE_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _HOEW<'a> {
    w: &'a mut W,
}
impl<'a> _HOEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: HOEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Normal operation"]
    #[inline]
    pub fn hoe_0(self) -> &'a mut W {
        self.variant(HOEW::HOE_0)
    }
    #[doc = "Any error causes the HALT bit to set. Subsequently, all service requests are ignored until the HALT bit is cleared."]
    #[inline]
    pub fn hoe_1(self) -> &'a mut W {
        self.variant(HOEW::HOE_1)
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
        const OFFSET: u8 = 4;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `HALT`"]
pub enum HALTW {
    #[doc = "Normal operation"]
    HALT_0,
    #[doc = "Stall the start of any new channels. Executing channels are allowed to complete. Channel execution resumes when this bit is cleared."]
    HALT_1,
}
impl HALTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            HALTW::HALT_0 => false,
            HALTW::HALT_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _HALTW<'a> {
    w: &'a mut W,
}
impl<'a> _HALTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: HALTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Normal operation"]
    #[inline]
    pub fn halt_0(self) -> &'a mut W {
        self.variant(HALTW::HALT_0)
    }
    #[doc = "Stall the start of any new channels. Executing channels are allowed to complete. Channel execution resumes when this bit is cleared."]
    #[inline]
    pub fn halt_1(self) -> &'a mut W {
        self.variant(HALTW::HALT_1)
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
        const OFFSET: u8 = 5;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `CLM`"]
pub enum CLMW {
    #[doc = "A minor loop channel link made to itself goes through channel arbitration before being activated again."]
    CLM_0,
    #[doc = "A minor loop channel link made to itself does not go through channel arbitration before being activated again. Upon minor loop completion, the channel activates again if that channel has a minor loop channel link enabled and the link channel is itself. This effectively applies the minor loop offsets and restarts the next minor loop."]
    CLM_1,
}
impl CLMW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CLMW::CLM_0 => false,
            CLMW::CLM_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CLMW<'a> {
    w: &'a mut W,
}
impl<'a> _CLMW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CLMW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "A minor loop channel link made to itself goes through channel arbitration before being activated again."]
    #[inline]
    pub fn clm_0(self) -> &'a mut W {
        self.variant(CLMW::CLM_0)
    }
    #[doc = "A minor loop channel link made to itself does not go through channel arbitration before being activated again. Upon minor loop completion, the channel activates again if that channel has a minor loop channel link enabled and the link channel is itself. This effectively applies the minor loop offsets and restarts the next minor loop."]
    #[inline]
    pub fn clm_1(self) -> &'a mut W {
        self.variant(CLMW::CLM_1)
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
        const OFFSET: u8 = 6;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `EMLM`"]
pub enum EMLMW {
    #[doc = "Disabled. TCDn.word2 is defined as a 32-bit NBYTES field."]
    EMLM_0,
    #[doc = "Enabled. TCDn.word2 is redefined to include individual enable fields, an offset field, and the NBYTES field. The individual enable fields allow the minor loop offset to be applied to the source address, the destination address, or both. The NBYTES field is reduced when either offset is enabled."]
    EMLM_1,
}
impl EMLMW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            EMLMW::EMLM_0 => false,
            EMLMW::EMLM_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _EMLMW<'a> {
    w: &'a mut W,
}
impl<'a> _EMLMW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: EMLMW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disabled. TCDn.word2 is defined as a 32-bit NBYTES field."]
    #[inline]
    pub fn emlm_0(self) -> &'a mut W {
        self.variant(EMLMW::EMLM_0)
    }
    #[doc = "Enabled. TCDn.word2 is redefined to include individual enable fields, an offset field, and the NBYTES field. The individual enable fields allow the minor loop offset to be applied to the source address, the destination address, or both. The NBYTES field is reduced when either offset is enabled."]
    #[inline]
    pub fn emlm_1(self) -> &'a mut W {
        self.variant(EMLMW::EMLM_1)
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
        const OFFSET: u8 = 7;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _GRP0PRIW<'a> {
    w: &'a mut W,
}
impl<'a> _GRP0PRIW<'a> {
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
#[doc = r" Proxy"]
pub struct _GRP1PRIW<'a> {
    w: &'a mut W,
}
impl<'a> _GRP1PRIW<'a> {
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
#[doc = "Values that can be written to the field `ECX`"]
pub enum ECXW {
    #[doc = "Normal operation"]
    ECX_0,
    #[doc = "Cancel the remaining data transfer in the same fashion as the CX bit. Stop the executing channel and force the minor loop to finish. The cancel takes effect after the last write of the current read/write sequence. The ECX bit clears itself after the cancel is honored. In addition to cancelling the transfer, ECX treats the cancel as an error condition, thus updating the Error Status register (DMAx_ES) and generating an optional error interrupt."]
    ECX_1,
}
impl ECXW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ECXW::ECX_0 => false,
            ECXW::ECX_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ECXW<'a> {
    w: &'a mut W,
}
impl<'a> _ECXW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ECXW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Normal operation"]
    #[inline]
    pub fn ecx_0(self) -> &'a mut W {
        self.variant(ECXW::ECX_0)
    }
    #[doc = "Cancel the remaining data transfer in the same fashion as the CX bit. Stop the executing channel and force the minor loop to finish. The cancel takes effect after the last write of the current read/write sequence. The ECX bit clears itself after the cancel is honored. In addition to cancelling the transfer, ECX treats the cancel as an error condition, thus updating the Error Status register (DMAx_ES) and generating an optional error interrupt."]
    #[inline]
    pub fn ecx_1(self) -> &'a mut W {
        self.variant(ECXW::ECX_1)
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
        const OFFSET: u8 = 16;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `CX`"]
pub enum CXW {
    #[doc = "Normal operation"]
    CX_0,
    #[doc = "Cancel the remaining data transfer. Stop the executing channel and force the minor loop to finish. The cancel takes effect after the last write of the current read/write sequence. The CX bit clears itself after the cancel has been honored. This cancel retires the channel normally as if the minor loop was completed."]
    CX_1,
}
impl CXW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CXW::CX_0 => false,
            CXW::CX_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CXW<'a> {
    w: &'a mut W,
}
impl<'a> _CXW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CXW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Normal operation"]
    #[inline]
    pub fn cx_0(self) -> &'a mut W {
        self.variant(CXW::CX_0)
    }
    #[doc = "Cancel the remaining data transfer. Stop the executing channel and force the minor loop to finish. The cancel takes effect after the last write of the current read/write sequence. The CX bit clears itself after the cancel has been honored. This cancel retires the channel normally as if the minor loop was completed."]
    #[inline]
    pub fn cx_1(self) -> &'a mut W {
        self.variant(CXW::CX_1)
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
        const OFFSET: u8 = 17;
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
    #[doc = "Bit 1 - Enable Debug"]
    #[inline]
    pub fn edbg(&self) -> EDBGR {
        EDBGR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - Enable Round Robin Channel Arbitration"]
    #[inline]
    pub fn erca(&self) -> ERCAR {
        ERCAR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - Enable Round Robin Group Arbitration"]
    #[inline]
    pub fn erga(&self) -> ERGAR {
        ERGAR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 4 - Halt On Error"]
    #[inline]
    pub fn hoe(&self) -> HOER {
        HOER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 5 - Halt DMA Operations"]
    #[inline]
    pub fn halt(&self) -> HALTR {
        HALTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 6 - Continuous Link Mode"]
    #[inline]
    pub fn clm(&self) -> CLMR {
        CLMR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 7 - Enable Minor Loop Mapping"]
    #[inline]
    pub fn emlm(&self) -> EMLMR {
        EMLMR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 8 - Channel Group 0 Priority"]
    #[inline]
    pub fn grp0pri(&self) -> GRP0PRIR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        GRP0PRIR { bits }
    }
    #[doc = "Bit 10 - Channel Group 1 Priority"]
    #[inline]
    pub fn grp1pri(&self) -> GRP1PRIR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        GRP1PRIR { bits }
    }
    #[doc = "Bit 16 - Error Cancel Transfer"]
    #[inline]
    pub fn ecx(&self) -> ECXR {
        ECXR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 17 - Cancel Transfer"]
    #[inline]
    pub fn cx(&self) -> CXR {
        CXR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 17;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 31 - DMA Active Status"]
    #[inline]
    pub fn active(&self) -> ACTIVER {
        ACTIVER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 31;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 1024 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 1 - Enable Debug"]
    #[inline]
    pub fn edbg(&mut self) -> _EDBGW {
        _EDBGW { w: self }
    }
    #[doc = "Bit 2 - Enable Round Robin Channel Arbitration"]
    #[inline]
    pub fn erca(&mut self) -> _ERCAW {
        _ERCAW { w: self }
    }
    #[doc = "Bit 3 - Enable Round Robin Group Arbitration"]
    #[inline]
    pub fn erga(&mut self) -> _ERGAW {
        _ERGAW { w: self }
    }
    #[doc = "Bit 4 - Halt On Error"]
    #[inline]
    pub fn hoe(&mut self) -> _HOEW {
        _HOEW { w: self }
    }
    #[doc = "Bit 5 - Halt DMA Operations"]
    #[inline]
    pub fn halt(&mut self) -> _HALTW {
        _HALTW { w: self }
    }
    #[doc = "Bit 6 - Continuous Link Mode"]
    #[inline]
    pub fn clm(&mut self) -> _CLMW {
        _CLMW { w: self }
    }
    #[doc = "Bit 7 - Enable Minor Loop Mapping"]
    #[inline]
    pub fn emlm(&mut self) -> _EMLMW {
        _EMLMW { w: self }
    }
    #[doc = "Bit 8 - Channel Group 0 Priority"]
    #[inline]
    pub fn grp0pri(&mut self) -> _GRP0PRIW {
        _GRP0PRIW { w: self }
    }
    #[doc = "Bit 10 - Channel Group 1 Priority"]
    #[inline]
    pub fn grp1pri(&mut self) -> _GRP1PRIW {
        _GRP1PRIW { w: self }
    }
    #[doc = "Bit 16 - Error Cancel Transfer"]
    #[inline]
    pub fn ecx(&mut self) -> _ECXW {
        _ECXW { w: self }
    }
    #[doc = "Bit 17 - Cancel Transfer"]
    #[inline]
    pub fn cx(&mut self) -> _CXW {
        _CXW { w: self }
    }
}
