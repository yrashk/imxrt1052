#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::GPR24 {
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
#[doc = "Possible values of the field `LOCK_M7_APC_AC_R3_BOT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LOCK_M7_APC_AC_R3_BOTR {
    #[doc = "Register field [31:1] is not locked"]
    LOCK_M7_APC_AC_R3_BOT_0,
    #[doc = "Register field [31:1] is locked (read access only)"]
    LOCK_M7_APC_AC_R3_BOT_1,
}
impl LOCK_M7_APC_AC_R3_BOTR {
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
            LOCK_M7_APC_AC_R3_BOTR::LOCK_M7_APC_AC_R3_BOT_0 => false,
            LOCK_M7_APC_AC_R3_BOTR::LOCK_M7_APC_AC_R3_BOT_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> LOCK_M7_APC_AC_R3_BOTR {
        match value {
            false => LOCK_M7_APC_AC_R3_BOTR::LOCK_M7_APC_AC_R3_BOT_0,
            true => LOCK_M7_APC_AC_R3_BOTR::LOCK_M7_APC_AC_R3_BOT_1,
        }
    }
    #[doc = "Checks if the value of the field is `LOCK_M7_APC_AC_R3_BOT_0`"]
    #[inline]
    pub fn is_lock_m7_apc_ac_r3_bot_0(&self) -> bool {
        *self == LOCK_M7_APC_AC_R3_BOTR::LOCK_M7_APC_AC_R3_BOT_0
    }
    #[doc = "Checks if the value of the field is `LOCK_M7_APC_AC_R3_BOT_1`"]
    #[inline]
    pub fn is_lock_m7_apc_ac_r3_bot_1(&self) -> bool {
        *self == LOCK_M7_APC_AC_R3_BOTR::LOCK_M7_APC_AC_R3_BOT_1
    }
}
#[doc = r" Value of the field"]
pub struct M7_APC_AC_R2_BOTR {
    bits: u32,
}
impl M7_APC_AC_R2_BOTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
}
#[doc = "Values that can be written to the field `LOCK_M7_APC_AC_R3_BOT`"]
pub enum LOCK_M7_APC_AC_R3_BOTW {
    #[doc = "Register field [31:1] is not locked"]
    LOCK_M7_APC_AC_R3_BOT_0,
    #[doc = "Register field [31:1] is locked (read access only)"]
    LOCK_M7_APC_AC_R3_BOT_1,
}
impl LOCK_M7_APC_AC_R3_BOTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            LOCK_M7_APC_AC_R3_BOTW::LOCK_M7_APC_AC_R3_BOT_0 => false,
            LOCK_M7_APC_AC_R3_BOTW::LOCK_M7_APC_AC_R3_BOT_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _LOCK_M7_APC_AC_R3_BOTW<'a> {
    w: &'a mut W,
}
impl<'a> _LOCK_M7_APC_AC_R3_BOTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: LOCK_M7_APC_AC_R3_BOTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Register field [31:1] is not locked"]
    #[inline]
    pub fn lock_m7_apc_ac_r3_bot_0(self) -> &'a mut W {
        self.variant(LOCK_M7_APC_AC_R3_BOTW::LOCK_M7_APC_AC_R3_BOT_0)
    }
    #[doc = "Register field [31:1] is locked (read access only)"]
    #[inline]
    pub fn lock_m7_apc_ac_r3_bot_1(self) -> &'a mut W {
        self.variant(LOCK_M7_APC_AC_R3_BOTW::LOCK_M7_APC_AC_R3_BOT_1)
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
#[doc = r" Proxy"]
pub struct _M7_APC_AC_R2_BOTW<'a> {
    w: &'a mut W,
}
impl<'a> _M7_APC_AC_R2_BOTW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        const MASK: u32 = 536870911;
        const OFFSET: u8 = 3;
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
    #[doc = "Bit 0 - lock M7_APC_AC_R3_BOT field for changes"]
    #[inline]
    pub fn lock_m7_apc_ac_r3_bot(&self) -> LOCK_M7_APC_AC_R3_BOTR {
        LOCK_M7_APC_AC_R3_BOTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 3:31 - APC end address of memory region-3"]
    #[inline]
    pub fn m7_apc_ac_r2_bot(&self) -> M7_APC_AC_R2_BOTR {
        let bits = {
            const MASK: u32 = 536870911;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) as u32
        };
        M7_APC_AC_R2_BOTR { bits }
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
    #[doc = "Bit 0 - lock M7_APC_AC_R3_BOT field for changes"]
    #[inline]
    pub fn lock_m7_apc_ac_r3_bot(&mut self) -> _LOCK_M7_APC_AC_R3_BOTW {
        _LOCK_M7_APC_AC_R3_BOTW { w: self }
    }
    #[doc = "Bits 3:31 - APC end address of memory region-3"]
    #[inline]
    pub fn m7_apc_ac_r2_bot(&mut self) -> _M7_APC_AC_R2_BOTW {
        _M7_APC_AC_R2_BOTW { w: self }
    }
}
