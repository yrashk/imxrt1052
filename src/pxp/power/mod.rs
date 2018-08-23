#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::POWER {
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
#[doc = "Possible values of the field `ROT_MEM_LP_STATE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ROT_MEM_LP_STATER {
    #[doc = "Memory is not in low power state."]
    NONE,
    #[doc = "Light Sleep Mode. Low leakage mode, maintain memory contents."]
    LS,
    #[doc = "Deep Sleep Mode. Low leakage mode, maintain memory contents."]
    DS,
    #[doc = "Shut Down Mode. Shut Down periphery and core, no memory retention."]
    SD,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl ROT_MEM_LP_STATER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            ROT_MEM_LP_STATER::NONE => 0,
            ROT_MEM_LP_STATER::LS => 1,
            ROT_MEM_LP_STATER::DS => 2,
            ROT_MEM_LP_STATER::SD => 4,
            ROT_MEM_LP_STATER::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> ROT_MEM_LP_STATER {
        match value {
            0 => ROT_MEM_LP_STATER::NONE,
            1 => ROT_MEM_LP_STATER::LS,
            2 => ROT_MEM_LP_STATER::DS,
            4 => ROT_MEM_LP_STATER::SD,
            i => ROT_MEM_LP_STATER::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline]
    pub fn is_none(&self) -> bool {
        *self == ROT_MEM_LP_STATER::NONE
    }
    #[doc = "Checks if the value of the field is `LS`"]
    #[inline]
    pub fn is_ls(&self) -> bool {
        *self == ROT_MEM_LP_STATER::LS
    }
    #[doc = "Checks if the value of the field is `DS`"]
    #[inline]
    pub fn is_ds(&self) -> bool {
        *self == ROT_MEM_LP_STATER::DS
    }
    #[doc = "Checks if the value of the field is `SD`"]
    #[inline]
    pub fn is_sd(&self) -> bool {
        *self == ROT_MEM_LP_STATER::SD
    }
}
#[doc = r" Value of the field"]
pub struct CTRLR {
    bits: u32,
}
impl CTRLR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
}
#[doc = "Values that can be written to the field `ROT_MEM_LP_STATE`"]
pub enum ROT_MEM_LP_STATEW {
    #[doc = "Memory is not in low power state."]
    NONE,
    #[doc = "Light Sleep Mode. Low leakage mode, maintain memory contents."]
    LS,
    #[doc = "Deep Sleep Mode. Low leakage mode, maintain memory contents."]
    DS,
    #[doc = "Shut Down Mode. Shut Down periphery and core, no memory retention."]
    SD,
}
impl ROT_MEM_LP_STATEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            ROT_MEM_LP_STATEW::NONE => 0,
            ROT_MEM_LP_STATEW::LS => 1,
            ROT_MEM_LP_STATEW::DS => 2,
            ROT_MEM_LP_STATEW::SD => 4,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ROT_MEM_LP_STATEW<'a> {
    w: &'a mut W,
}
impl<'a> _ROT_MEM_LP_STATEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ROT_MEM_LP_STATEW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Memory is not in low power state."]
    #[inline]
    pub fn none(self) -> &'a mut W {
        self.variant(ROT_MEM_LP_STATEW::NONE)
    }
    #[doc = "Light Sleep Mode. Low leakage mode, maintain memory contents."]
    #[inline]
    pub fn ls(self) -> &'a mut W {
        self.variant(ROT_MEM_LP_STATEW::LS)
    }
    #[doc = "Deep Sleep Mode. Low leakage mode, maintain memory contents."]
    #[inline]
    pub fn ds(self) -> &'a mut W {
        self.variant(ROT_MEM_LP_STATEW::DS)
    }
    #[doc = "Shut Down Mode. Shut Down periphery and core, no memory retention."]
    #[inline]
    pub fn sd(self) -> &'a mut W {
        self.variant(ROT_MEM_LP_STATEW::SD)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 9;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _CTRLW<'a> {
    w: &'a mut W,
}
impl<'a> _CTRLW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        const MASK: u32 = 1048575;
        const OFFSET: u8 = 12;
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
    #[doc = "Bits 9:11 - Select the low power state of the ROT memory."]
    #[inline]
    pub fn rot_mem_lp_state(&self) -> ROT_MEM_LP_STATER {
        ROT_MEM_LP_STATER::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 12:31 - Power control for the PXP."]
    #[inline]
    pub fn ctrl(&self) -> CTRLR {
        let bits = {
            const MASK: u32 = 1048575;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) as u32
        };
        CTRLR { bits }
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
    #[doc = "Bits 9:11 - Select the low power state of the ROT memory."]
    #[inline]
    pub fn rot_mem_lp_state(&mut self) -> _ROT_MEM_LP_STATEW {
        _ROT_MEM_LP_STATEW { w: self }
    }
    #[doc = "Bits 12:31 - Power control for the PXP."]
    #[inline]
    pub fn ctrl(&mut self) -> _CTRLW {
        _CTRLW { w: self }
    }
}
