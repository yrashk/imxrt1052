#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::SCR {
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
#[doc = "Possible values of the field `lockup_rst`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LOCKUP_RSTR {
    #[doc = "disabled"]
    LOCKUP_RST_0,
    #[doc = "enabled"]
    LOCKUP_RST_1,
}
impl LOCKUP_RSTR {
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
            LOCKUP_RSTR::LOCKUP_RST_0 => false,
            LOCKUP_RSTR::LOCKUP_RST_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> LOCKUP_RSTR {
        match value {
            false => LOCKUP_RSTR::LOCKUP_RST_0,
            true => LOCKUP_RSTR::LOCKUP_RST_1,
        }
    }
    #[doc = "Checks if the value of the field is `LOCKUP_RST_0`"]
    #[inline]
    pub fn is_lockup_rst_0(&self) -> bool {
        *self == LOCKUP_RSTR::LOCKUP_RST_0
    }
    #[doc = "Checks if the value of the field is `LOCKUP_RST_1`"]
    #[inline]
    pub fn is_lockup_rst_1(&self) -> bool {
        *self == LOCKUP_RSTR::LOCKUP_RST_1
    }
}
#[doc = "Possible values of the field `mask_wdog_rst`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MASK_WDOG_RSTR {
    #[doc = "wdog_rst_b is masked"]
    MASK_WDOG_RST_5,
    #[doc = "wdog_rst_b is not masked (default)"]
    MASK_WDOG_RST_10,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl MASK_WDOG_RSTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            MASK_WDOG_RSTR::MASK_WDOG_RST_5 => 5,
            MASK_WDOG_RSTR::MASK_WDOG_RST_10 => 10,
            MASK_WDOG_RSTR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> MASK_WDOG_RSTR {
        match value {
            5 => MASK_WDOG_RSTR::MASK_WDOG_RST_5,
            10 => MASK_WDOG_RSTR::MASK_WDOG_RST_10,
            i => MASK_WDOG_RSTR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `MASK_WDOG_RST_5`"]
    #[inline]
    pub fn is_mask_wdog_rst_5(&self) -> bool {
        *self == MASK_WDOG_RSTR::MASK_WDOG_RST_5
    }
    #[doc = "Checks if the value of the field is `MASK_WDOG_RST_10`"]
    #[inline]
    pub fn is_mask_wdog_rst_10(&self) -> bool {
        *self == MASK_WDOG_RSTR::MASK_WDOG_RST_10
    }
}
#[doc = "Possible values of the field `core0_rst`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CORE0_RSTR {
    #[doc = "do not assert core0 reset"]
    CORE0_RST_0,
    #[doc = "assert core0 reset"]
    CORE0_RST_1,
}
impl CORE0_RSTR {
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
            CORE0_RSTR::CORE0_RST_0 => false,
            CORE0_RSTR::CORE0_RST_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CORE0_RSTR {
        match value {
            false => CORE0_RSTR::CORE0_RST_0,
            true => CORE0_RSTR::CORE0_RST_1,
        }
    }
    #[doc = "Checks if the value of the field is `CORE0_RST_0`"]
    #[inline]
    pub fn is_core0_rst_0(&self) -> bool {
        *self == CORE0_RSTR::CORE0_RST_0
    }
    #[doc = "Checks if the value of the field is `CORE0_RST_1`"]
    #[inline]
    pub fn is_core0_rst_1(&self) -> bool {
        *self == CORE0_RSTR::CORE0_RST_1
    }
}
#[doc = "Possible values of the field `core0_dbg_rst`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CORE0_DBG_RSTR {
    #[doc = "do not assert core0 debug reset"]
    CORE0_DBG_RST_0,
    #[doc = "assert core0 debug reset"]
    CORE0_DBG_RST_1,
}
impl CORE0_DBG_RSTR {
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
            CORE0_DBG_RSTR::CORE0_DBG_RST_0 => false,
            CORE0_DBG_RSTR::CORE0_DBG_RST_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CORE0_DBG_RSTR {
        match value {
            false => CORE0_DBG_RSTR::CORE0_DBG_RST_0,
            true => CORE0_DBG_RSTR::CORE0_DBG_RST_1,
        }
    }
    #[doc = "Checks if the value of the field is `CORE0_DBG_RST_0`"]
    #[inline]
    pub fn is_core0_dbg_rst_0(&self) -> bool {
        *self == CORE0_DBG_RSTR::CORE0_DBG_RST_0
    }
    #[doc = "Checks if the value of the field is `CORE0_DBG_RST_1`"]
    #[inline]
    pub fn is_core0_dbg_rst_1(&self) -> bool {
        *self == CORE0_DBG_RSTR::CORE0_DBG_RST_1
    }
}
#[doc = "Possible values of the field `dbg_rst_msk_pg`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DBG_RST_MSK_PGR {
    #[doc = "do not mask core debug resets (debug resets will be asserted after power gating event)"]
    DBG_RST_MSK_PG_0,
    #[doc = "mask core debug resets (debug resets won't be asserted after power gating event)"]
    DBG_RST_MSK_PG_1,
}
impl DBG_RST_MSK_PGR {
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
            DBG_RST_MSK_PGR::DBG_RST_MSK_PG_0 => false,
            DBG_RST_MSK_PGR::DBG_RST_MSK_PG_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DBG_RST_MSK_PGR {
        match value {
            false => DBG_RST_MSK_PGR::DBG_RST_MSK_PG_0,
            true => DBG_RST_MSK_PGR::DBG_RST_MSK_PG_1,
        }
    }
    #[doc = "Checks if the value of the field is `DBG_RST_MSK_PG_0`"]
    #[inline]
    pub fn is_dbg_rst_msk_pg_0(&self) -> bool {
        *self == DBG_RST_MSK_PGR::DBG_RST_MSK_PG_0
    }
    #[doc = "Checks if the value of the field is `DBG_RST_MSK_PG_1`"]
    #[inline]
    pub fn is_dbg_rst_msk_pg_1(&self) -> bool {
        *self == DBG_RST_MSK_PGR::DBG_RST_MSK_PG_1
    }
}
#[doc = "Possible values of the field `mask_wdog3_rst`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MASK_WDOG3_RSTR {
    #[doc = "wdog3_rst_b is masked"]
    MASK_WDOG3_RST_5,
    #[doc = "wdog3_rst_b is not masked"]
    MASK_WDOG3_RST_10,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl MASK_WDOG3_RSTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            MASK_WDOG3_RSTR::MASK_WDOG3_RST_5 => 5,
            MASK_WDOG3_RSTR::MASK_WDOG3_RST_10 => 10,
            MASK_WDOG3_RSTR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> MASK_WDOG3_RSTR {
        match value {
            5 => MASK_WDOG3_RSTR::MASK_WDOG3_RST_5,
            10 => MASK_WDOG3_RSTR::MASK_WDOG3_RST_10,
            i => MASK_WDOG3_RSTR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `MASK_WDOG3_RST_5`"]
    #[inline]
    pub fn is_mask_wdog3_rst_5(&self) -> bool {
        *self == MASK_WDOG3_RSTR::MASK_WDOG3_RST_5
    }
    #[doc = "Checks if the value of the field is `MASK_WDOG3_RST_10`"]
    #[inline]
    pub fn is_mask_wdog3_rst_10(&self) -> bool {
        *self == MASK_WDOG3_RSTR::MASK_WDOG3_RST_10
    }
}
#[doc = "Values that can be written to the field `lockup_rst`"]
pub enum LOCKUP_RSTW {
    #[doc = "disabled"]
    LOCKUP_RST_0,
    #[doc = "enabled"]
    LOCKUP_RST_1,
}
impl LOCKUP_RSTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            LOCKUP_RSTW::LOCKUP_RST_0 => false,
            LOCKUP_RSTW::LOCKUP_RST_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _LOCKUP_RSTW<'a> {
    w: &'a mut W,
}
impl<'a> _LOCKUP_RSTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: LOCKUP_RSTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "disabled"]
    #[inline]
    pub fn lockup_rst_0(self) -> &'a mut W {
        self.variant(LOCKUP_RSTW::LOCKUP_RST_0)
    }
    #[doc = "enabled"]
    #[inline]
    pub fn lockup_rst_1(self) -> &'a mut W {
        self.variant(LOCKUP_RSTW::LOCKUP_RST_1)
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
#[doc = "Values that can be written to the field `mask_wdog_rst`"]
pub enum MASK_WDOG_RSTW {
    #[doc = "wdog_rst_b is masked"]
    MASK_WDOG_RST_5,
    #[doc = "wdog_rst_b is not masked (default)"]
    MASK_WDOG_RST_10,
}
impl MASK_WDOG_RSTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            MASK_WDOG_RSTW::MASK_WDOG_RST_5 => 5,
            MASK_WDOG_RSTW::MASK_WDOG_RST_10 => 10,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MASK_WDOG_RSTW<'a> {
    w: &'a mut W,
}
impl<'a> _MASK_WDOG_RSTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MASK_WDOG_RSTW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "wdog_rst_b is masked"]
    #[inline]
    pub fn mask_wdog_rst_5(self) -> &'a mut W {
        self.variant(MASK_WDOG_RSTW::MASK_WDOG_RST_5)
    }
    #[doc = "wdog_rst_b is not masked (default)"]
    #[inline]
    pub fn mask_wdog_rst_10(self) -> &'a mut W {
        self.variant(MASK_WDOG_RSTW::MASK_WDOG_RST_10)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 7;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `core0_rst`"]
pub enum CORE0_RSTW {
    #[doc = "do not assert core0 reset"]
    CORE0_RST_0,
    #[doc = "assert core0 reset"]
    CORE0_RST_1,
}
impl CORE0_RSTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CORE0_RSTW::CORE0_RST_0 => false,
            CORE0_RSTW::CORE0_RST_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CORE0_RSTW<'a> {
    w: &'a mut W,
}
impl<'a> _CORE0_RSTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CORE0_RSTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "do not assert core0 reset"]
    #[inline]
    pub fn core0_rst_0(self) -> &'a mut W {
        self.variant(CORE0_RSTW::CORE0_RST_0)
    }
    #[doc = "assert core0 reset"]
    #[inline]
    pub fn core0_rst_1(self) -> &'a mut W {
        self.variant(CORE0_RSTW::CORE0_RST_1)
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
#[doc = "Values that can be written to the field `core0_dbg_rst`"]
pub enum CORE0_DBG_RSTW {
    #[doc = "do not assert core0 debug reset"]
    CORE0_DBG_RST_0,
    #[doc = "assert core0 debug reset"]
    CORE0_DBG_RST_1,
}
impl CORE0_DBG_RSTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CORE0_DBG_RSTW::CORE0_DBG_RST_0 => false,
            CORE0_DBG_RSTW::CORE0_DBG_RST_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CORE0_DBG_RSTW<'a> {
    w: &'a mut W,
}
impl<'a> _CORE0_DBG_RSTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CORE0_DBG_RSTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "do not assert core0 debug reset"]
    #[inline]
    pub fn core0_dbg_rst_0(self) -> &'a mut W {
        self.variant(CORE0_DBG_RSTW::CORE0_DBG_RST_0)
    }
    #[doc = "assert core0 debug reset"]
    #[inline]
    pub fn core0_dbg_rst_1(self) -> &'a mut W {
        self.variant(CORE0_DBG_RSTW::CORE0_DBG_RST_1)
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
#[doc = "Values that can be written to the field `dbg_rst_msk_pg`"]
pub enum DBG_RST_MSK_PGW {
    #[doc = "do not mask core debug resets (debug resets will be asserted after power gating event)"]
    DBG_RST_MSK_PG_0,
    #[doc = "mask core debug resets (debug resets won't be asserted after power gating event)"]
    DBG_RST_MSK_PG_1,
}
impl DBG_RST_MSK_PGW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DBG_RST_MSK_PGW::DBG_RST_MSK_PG_0 => false,
            DBG_RST_MSK_PGW::DBG_RST_MSK_PG_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DBG_RST_MSK_PGW<'a> {
    w: &'a mut W,
}
impl<'a> _DBG_RST_MSK_PGW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DBG_RST_MSK_PGW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "do not mask core debug resets (debug resets will be asserted after power gating event)"]
    #[inline]
    pub fn dbg_rst_msk_pg_0(self) -> &'a mut W {
        self.variant(DBG_RST_MSK_PGW::DBG_RST_MSK_PG_0)
    }
    #[doc = "mask core debug resets (debug resets won't be asserted after power gating event)"]
    #[inline]
    pub fn dbg_rst_msk_pg_1(self) -> &'a mut W {
        self.variant(DBG_RST_MSK_PGW::DBG_RST_MSK_PG_1)
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
        const OFFSET: u8 = 25;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `mask_wdog3_rst`"]
pub enum MASK_WDOG3_RSTW {
    #[doc = "wdog3_rst_b is masked"]
    MASK_WDOG3_RST_5,
    #[doc = "wdog3_rst_b is not masked"]
    MASK_WDOG3_RST_10,
}
impl MASK_WDOG3_RSTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            MASK_WDOG3_RSTW::MASK_WDOG3_RST_5 => 5,
            MASK_WDOG3_RSTW::MASK_WDOG3_RST_10 => 10,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MASK_WDOG3_RSTW<'a> {
    w: &'a mut W,
}
impl<'a> _MASK_WDOG3_RSTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MASK_WDOG3_RSTW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "wdog3_rst_b is masked"]
    #[inline]
    pub fn mask_wdog3_rst_5(self) -> &'a mut W {
        self.variant(MASK_WDOG3_RSTW::MASK_WDOG3_RST_5)
    }
    #[doc = "wdog3_rst_b is not masked"]
    #[inline]
    pub fn mask_wdog3_rst_10(self) -> &'a mut W {
        self.variant(MASK_WDOG3_RSTW::MASK_WDOG3_RST_10)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
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
    #[doc = "Bit 4 - lockup reset enable bit"]
    #[inline]
    pub fn lockup_rst(&self) -> LOCKUP_RSTR {
        LOCKUP_RSTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 7:10 - Mask wdog_rst_b source"]
    #[inline]
    pub fn mask_wdog_rst(&self) -> MASK_WDOG_RSTR {
        MASK_WDOG_RSTR::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 13 - Software reset for core0 only"]
    #[inline]
    pub fn core0_rst(&self) -> CORE0_RSTR {
        CORE0_RSTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 17 - Software reset for core0 debug only"]
    #[inline]
    pub fn core0_dbg_rst(&self) -> CORE0_DBG_RSTR {
        CORE0_DBG_RSTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 17;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 25 - Do not assert debug resets after power gating event of core"]
    #[inline]
    pub fn dbg_rst_msk_pg(&self) -> DBG_RST_MSK_PGR {
        DBG_RST_MSK_PGR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 25;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 28:31 - Mask wdog3_rst_b source"]
    #[inline]
    pub fn mask_wdog3_rst(&self) -> MASK_WDOG3_RSTR {
        MASK_WDOG3_RSTR::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 28;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 1280 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 4 - lockup reset enable bit"]
    #[inline]
    pub fn lockup_rst(&mut self) -> _LOCKUP_RSTW {
        _LOCKUP_RSTW { w: self }
    }
    #[doc = "Bits 7:10 - Mask wdog_rst_b source"]
    #[inline]
    pub fn mask_wdog_rst(&mut self) -> _MASK_WDOG_RSTW {
        _MASK_WDOG_RSTW { w: self }
    }
    #[doc = "Bit 13 - Software reset for core0 only"]
    #[inline]
    pub fn core0_rst(&mut self) -> _CORE0_RSTW {
        _CORE0_RSTW { w: self }
    }
    #[doc = "Bit 17 - Software reset for core0 debug only"]
    #[inline]
    pub fn core0_dbg_rst(&mut self) -> _CORE0_DBG_RSTW {
        _CORE0_DBG_RSTW { w: self }
    }
    #[doc = "Bit 25 - Do not assert debug resets after power gating event of core"]
    #[inline]
    pub fn dbg_rst_msk_pg(&mut self) -> _DBG_RST_MSK_PGW {
        _DBG_RST_MSK_PGW { w: self }
    }
    #[doc = "Bits 28:31 - Mask wdog3_rst_b source"]
    #[inline]
    pub fn mask_wdog3_rst(&mut self) -> _MASK_WDOG3_RSTW {
        _MASK_WDOG3_RSTW { w: self }
    }
}
