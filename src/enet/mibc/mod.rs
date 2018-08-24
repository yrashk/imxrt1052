#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::MIBC {
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
#[doc = "Possible values of the field `MIB_CLEAR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MIB_CLEARR {
    #[doc = "See note above."]
    MIB_CLEAR_0,
    #[doc = "All statistics counters are reset to 0."]
    MIB_CLEAR_1,
}
impl MIB_CLEARR {
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
            MIB_CLEARR::MIB_CLEAR_0 => false,
            MIB_CLEARR::MIB_CLEAR_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> MIB_CLEARR {
        match value {
            false => MIB_CLEARR::MIB_CLEAR_0,
            true => MIB_CLEARR::MIB_CLEAR_1,
        }
    }
    #[doc = "Checks if the value of the field is `MIB_CLEAR_0`"]
    #[inline]
    pub fn is_mib_clear_0(&self) -> bool {
        *self == MIB_CLEARR::MIB_CLEAR_0
    }
    #[doc = "Checks if the value of the field is `MIB_CLEAR_1`"]
    #[inline]
    pub fn is_mib_clear_1(&self) -> bool {
        *self == MIB_CLEARR::MIB_CLEAR_1
    }
}
#[doc = "Possible values of the field `MIB_IDLE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MIB_IDLER {
    #[doc = "The MIB block is updating MIB counters."]
    MIB_IDLE_0,
    #[doc = "The MIB block is not currently updating any MIB counters."]
    MIB_IDLE_1,
}
impl MIB_IDLER {
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
            MIB_IDLER::MIB_IDLE_0 => false,
            MIB_IDLER::MIB_IDLE_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> MIB_IDLER {
        match value {
            false => MIB_IDLER::MIB_IDLE_0,
            true => MIB_IDLER::MIB_IDLE_1,
        }
    }
    #[doc = "Checks if the value of the field is `MIB_IDLE_0`"]
    #[inline]
    pub fn is_mib_idle_0(&self) -> bool {
        *self == MIB_IDLER::MIB_IDLE_0
    }
    #[doc = "Checks if the value of the field is `MIB_IDLE_1`"]
    #[inline]
    pub fn is_mib_idle_1(&self) -> bool {
        *self == MIB_IDLER::MIB_IDLE_1
    }
}
#[doc = "Possible values of the field `MIB_DIS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MIB_DISR {
    #[doc = "MIB logic is enabled."]
    MIB_DIS_0,
    #[doc = "MIB logic is disabled. The MIB logic halts and does not update any MIB counters."]
    MIB_DIS_1,
}
impl MIB_DISR {
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
            MIB_DISR::MIB_DIS_0 => false,
            MIB_DISR::MIB_DIS_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> MIB_DISR {
        match value {
            false => MIB_DISR::MIB_DIS_0,
            true => MIB_DISR::MIB_DIS_1,
        }
    }
    #[doc = "Checks if the value of the field is `MIB_DIS_0`"]
    #[inline]
    pub fn is_mib_dis_0(&self) -> bool {
        *self == MIB_DISR::MIB_DIS_0
    }
    #[doc = "Checks if the value of the field is `MIB_DIS_1`"]
    #[inline]
    pub fn is_mib_dis_1(&self) -> bool {
        *self == MIB_DISR::MIB_DIS_1
    }
}
#[doc = "Values that can be written to the field `MIB_CLEAR`"]
pub enum MIB_CLEARW {
    #[doc = "See note above."]
    MIB_CLEAR_0,
    #[doc = "All statistics counters are reset to 0."]
    MIB_CLEAR_1,
}
impl MIB_CLEARW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            MIB_CLEARW::MIB_CLEAR_0 => false,
            MIB_CLEARW::MIB_CLEAR_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MIB_CLEARW<'a> {
    w: &'a mut W,
}
impl<'a> _MIB_CLEARW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MIB_CLEARW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "See note above."]
    #[inline]
    pub fn mib_clear_0(self) -> &'a mut W {
        self.variant(MIB_CLEARW::MIB_CLEAR_0)
    }
    #[doc = "All statistics counters are reset to 0."]
    #[inline]
    pub fn mib_clear_1(self) -> &'a mut W {
        self.variant(MIB_CLEARW::MIB_CLEAR_1)
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
        const OFFSET: u8 = 29;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `MIB_DIS`"]
pub enum MIB_DISW {
    #[doc = "MIB logic is enabled."]
    MIB_DIS_0,
    #[doc = "MIB logic is disabled. The MIB logic halts and does not update any MIB counters."]
    MIB_DIS_1,
}
impl MIB_DISW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            MIB_DISW::MIB_DIS_0 => false,
            MIB_DISW::MIB_DIS_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MIB_DISW<'a> {
    w: &'a mut W,
}
impl<'a> _MIB_DISW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MIB_DISW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "MIB logic is enabled."]
    #[inline]
    pub fn mib_dis_0(self) -> &'a mut W {
        self.variant(MIB_DISW::MIB_DIS_0)
    }
    #[doc = "MIB logic is disabled. The MIB logic halts and does not update any MIB counters."]
    #[inline]
    pub fn mib_dis_1(self) -> &'a mut W {
        self.variant(MIB_DISW::MIB_DIS_1)
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
        const OFFSET: u8 = 31;
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
    #[doc = "Bit 29 - MIB Clear"]
    #[inline]
    pub fn mib_clear(&self) -> MIB_CLEARR {
        MIB_CLEARR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 29;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 30 - MIB Idle"]
    #[inline]
    pub fn mib_idle(&self) -> MIB_IDLER {
        MIB_IDLER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 30;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 31 - Disable MIB Logic"]
    #[inline]
    pub fn mib_dis(&self) -> MIB_DISR {
        MIB_DISR::_from({
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
        W { bits: 3221225472 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 29 - MIB Clear"]
    #[inline]
    pub fn mib_clear(&mut self) -> _MIB_CLEARW {
        _MIB_CLEARW { w: self }
    }
    #[doc = "Bit 31 - Disable MIB Logic"]
    #[inline]
    pub fn mib_dis(&mut self) -> _MIB_DISW {
        _MIB_DISW { w: self }
    }
}
