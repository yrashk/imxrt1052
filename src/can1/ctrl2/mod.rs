#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CTRL2 {
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
#[doc = "Possible values of the field `EACEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EACENR {
    #[doc = "Rx Mailbox filter's IDE bit is always compared and RTR is never compared despite mask bits."]
    EACEN_0,
    #[doc = "Enables the comparison of both Rx Mailbox filter's IDE and RTR bit with their corresponding bits within the incoming frame. Mask bits do apply."]
    EACEN_1,
}
impl EACENR {
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
            EACENR::EACEN_0 => false,
            EACENR::EACEN_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> EACENR {
        match value {
            false => EACENR::EACEN_0,
            true => EACENR::EACEN_1,
        }
    }
    #[doc = "Checks if the value of the field is `EACEN_0`"]
    #[inline]
    pub fn is_eacen_0(&self) -> bool {
        *self == EACENR::EACEN_0
    }
    #[doc = "Checks if the value of the field is `EACEN_1`"]
    #[inline]
    pub fn is_eacen_1(&self) -> bool {
        *self == EACENR::EACEN_1
    }
}
#[doc = "Possible values of the field `RRS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RRSR {
    #[doc = "Remote Response Frame is generated"]
    RRS_0,
    #[doc = "Remote Request Frame is stored"]
    RRS_1,
}
impl RRSR {
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
            RRSR::RRS_0 => false,
            RRSR::RRS_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RRSR {
        match value {
            false => RRSR::RRS_0,
            true => RRSR::RRS_1,
        }
    }
    #[doc = "Checks if the value of the field is `RRS_0`"]
    #[inline]
    pub fn is_rrs_0(&self) -> bool {
        *self == RRSR::RRS_0
    }
    #[doc = "Checks if the value of the field is `RRS_1`"]
    #[inline]
    pub fn is_rrs_1(&self) -> bool {
        *self == RRSR::RRS_1
    }
}
#[doc = "Possible values of the field `MRP`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MRPR {
    #[doc = "Matching starts from Rx FIFO and continues on Mailboxes"]
    MRP_0,
    #[doc = "Matching starts from Mailboxes and continues on Rx FIFO"]
    MRP_1,
}
impl MRPR {
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
            MRPR::MRP_0 => false,
            MRPR::MRP_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> MRPR {
        match value {
            false => MRPR::MRP_0,
            true => MRPR::MRP_1,
        }
    }
    #[doc = "Checks if the value of the field is `MRP_0`"]
    #[inline]
    pub fn is_mrp_0(&self) -> bool {
        *self == MRPR::MRP_0
    }
    #[doc = "Checks if the value of the field is `MRP_1`"]
    #[inline]
    pub fn is_mrp_1(&self) -> bool {
        *self == MRPR::MRP_1
    }
}
#[doc = r" Value of the field"]
pub struct TASDR {
    bits: u8,
}
impl TASDR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct RFFNR {
    bits: u8,
}
impl RFFNR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Possible values of the field `WRMFRZ`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WRMFRZR {
    #[doc = "Keep the write access restricted in some regions of FlexCAN memory"]
    WRMFRZ_0,
    #[doc = "Enable unrestricted write access to FlexCAN memory"]
    WRMFRZ_1,
}
impl WRMFRZR {
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
            WRMFRZR::WRMFRZ_0 => false,
            WRMFRZR::WRMFRZ_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> WRMFRZR {
        match value {
            false => WRMFRZR::WRMFRZ_0,
            true => WRMFRZR::WRMFRZ_1,
        }
    }
    #[doc = "Checks if the value of the field is `WRMFRZ_0`"]
    #[inline]
    pub fn is_wrmfrz_0(&self) -> bool {
        *self == WRMFRZR::WRMFRZ_0
    }
    #[doc = "Checks if the value of the field is `WRMFRZ_1`"]
    #[inline]
    pub fn is_wrmfrz_1(&self) -> bool {
        *self == WRMFRZR::WRMFRZ_1
    }
}
#[doc = "Values that can be written to the field `EACEN`"]
pub enum EACENW {
    #[doc = "Rx Mailbox filter's IDE bit is always compared and RTR is never compared despite mask bits."]
    EACEN_0,
    #[doc = "Enables the comparison of both Rx Mailbox filter's IDE and RTR bit with their corresponding bits within the incoming frame. Mask bits do apply."]
    EACEN_1,
}
impl EACENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            EACENW::EACEN_0 => false,
            EACENW::EACEN_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _EACENW<'a> {
    w: &'a mut W,
}
impl<'a> _EACENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: EACENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Rx Mailbox filter's IDE bit is always compared and RTR is never compared despite mask bits."]
    #[inline]
    pub fn eacen_0(self) -> &'a mut W {
        self.variant(EACENW::EACEN_0)
    }
    #[doc = "Enables the comparison of both Rx Mailbox filter's IDE and RTR bit with their corresponding bits within the incoming frame. Mask bits do apply."]
    #[inline]
    pub fn eacen_1(self) -> &'a mut W {
        self.variant(EACENW::EACEN_1)
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
#[doc = "Values that can be written to the field `RRS`"]
pub enum RRSW {
    #[doc = "Remote Response Frame is generated"]
    RRS_0,
    #[doc = "Remote Request Frame is stored"]
    RRS_1,
}
impl RRSW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RRSW::RRS_0 => false,
            RRSW::RRS_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RRSW<'a> {
    w: &'a mut W,
}
impl<'a> _RRSW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RRSW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Remote Response Frame is generated"]
    #[inline]
    pub fn rrs_0(self) -> &'a mut W {
        self.variant(RRSW::RRS_0)
    }
    #[doc = "Remote Request Frame is stored"]
    #[inline]
    pub fn rrs_1(self) -> &'a mut W {
        self.variant(RRSW::RRS_1)
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
#[doc = "Values that can be written to the field `MRP`"]
pub enum MRPW {
    #[doc = "Matching starts from Rx FIFO and continues on Mailboxes"]
    MRP_0,
    #[doc = "Matching starts from Mailboxes and continues on Rx FIFO"]
    MRP_1,
}
impl MRPW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            MRPW::MRP_0 => false,
            MRPW::MRP_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MRPW<'a> {
    w: &'a mut W,
}
impl<'a> _MRPW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MRPW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Matching starts from Rx FIFO and continues on Mailboxes"]
    #[inline]
    pub fn mrp_0(self) -> &'a mut W {
        self.variant(MRPW::MRP_0)
    }
    #[doc = "Matching starts from Mailboxes and continues on Rx FIFO"]
    #[inline]
    pub fn mrp_1(self) -> &'a mut W {
        self.variant(MRPW::MRP_1)
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
        const OFFSET: u8 = 18;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _TASDW<'a> {
    w: &'a mut W,
}
impl<'a> _TASDW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 31;
        const OFFSET: u8 = 19;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _RFFNW<'a> {
    w: &'a mut W,
}
impl<'a> _RFFNW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 24;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `WRMFRZ`"]
pub enum WRMFRZW {
    #[doc = "Keep the write access restricted in some regions of FlexCAN memory"]
    WRMFRZ_0,
    #[doc = "Enable unrestricted write access to FlexCAN memory"]
    WRMFRZ_1,
}
impl WRMFRZW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            WRMFRZW::WRMFRZ_0 => false,
            WRMFRZW::WRMFRZ_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _WRMFRZW<'a> {
    w: &'a mut W,
}
impl<'a> _WRMFRZW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: WRMFRZW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Keep the write access restricted in some regions of FlexCAN memory"]
    #[inline]
    pub fn wrmfrz_0(self) -> &'a mut W {
        self.variant(WRMFRZW::WRMFRZ_0)
    }
    #[doc = "Enable unrestricted write access to FlexCAN memory"]
    #[inline]
    pub fn wrmfrz_1(self) -> &'a mut W {
        self.variant(WRMFRZW::WRMFRZ_1)
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
        const OFFSET: u8 = 28;
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
    #[doc = "Bit 16 - This bit controls the comparison of IDE and RTR bits within Rx Mailboxes filters with their corresponding bits in the incoming frame by the matching process"]
    #[inline]
    pub fn eacen(&self) -> EACENR {
        EACENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 17 - If this bit is asserted Remote Request Frame is submitted to a matching process and stored in the corresponding Message Buffer in the same fashion of a Data Frame"]
    #[inline]
    pub fn rrs(&self) -> RRSR {
        RRSR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 17;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 18 - If this bit is set the matching process starts from the Mailboxes and if no match occurs the matching continues on the Rx FIFO"]
    #[inline]
    pub fn mrp(&self) -> MRPR {
        MRPR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 19:23 - This 5-bit field indicates how many CAN bits the Tx arbitration process start point can be delayed from the first bit of CRC field on CAN bus"]
    #[inline]
    pub fn tasd(&self) -> TASDR {
        let bits = {
            const MASK: u8 = 31;
            const OFFSET: u8 = 19;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        TASDR { bits }
    }
    #[doc = "Bits 24:27 - This 4-bit field defines the number of Rx FIFO filters according to"]
    #[inline]
    pub fn rffn(&self) -> RFFNR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        RFFNR { bits }
    }
    #[doc = "Bit 28 - Enable unrestricted write access to FlexCAN memory in Freeze mode"]
    #[inline]
    pub fn wrmfrz(&self) -> WRMFRZR {
        WRMFRZR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 28;
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
    #[doc = "Bit 16 - This bit controls the comparison of IDE and RTR bits within Rx Mailboxes filters with their corresponding bits in the incoming frame by the matching process"]
    #[inline]
    pub fn eacen(&mut self) -> _EACENW {
        _EACENW { w: self }
    }
    #[doc = "Bit 17 - If this bit is asserted Remote Request Frame is submitted to a matching process and stored in the corresponding Message Buffer in the same fashion of a Data Frame"]
    #[inline]
    pub fn rrs(&mut self) -> _RRSW {
        _RRSW { w: self }
    }
    #[doc = "Bit 18 - If this bit is set the matching process starts from the Mailboxes and if no match occurs the matching continues on the Rx FIFO"]
    #[inline]
    pub fn mrp(&mut self) -> _MRPW {
        _MRPW { w: self }
    }
    #[doc = "Bits 19:23 - This 5-bit field indicates how many CAN bits the Tx arbitration process start point can be delayed from the first bit of CRC field on CAN bus"]
    #[inline]
    pub fn tasd(&mut self) -> _TASDW {
        _TASDW { w: self }
    }
    #[doc = "Bits 24:27 - This 4-bit field defines the number of Rx FIFO filters according to"]
    #[inline]
    pub fn rffn(&mut self) -> _RFFNW {
        _RFFNW { w: self }
    }
    #[doc = "Bit 28 - Enable unrestricted write access to FlexCAN memory in Freeze mode"]
    #[inline]
    pub fn wrmfrz(&mut self) -> _WRMFRZW {
        _WRMFRZW { w: self }
    }
}
