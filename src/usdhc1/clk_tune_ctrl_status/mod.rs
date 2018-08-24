#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CLK_TUNE_CTRL_STATUS {
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
#[doc = r" Value of the field"]
pub struct DLY_CELL_SET_POSTR {
    bits: u8,
}
impl DLY_CELL_SET_POSTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct DLY_CELL_SET_OUTR {
    bits: u8,
}
impl DLY_CELL_SET_OUTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct DLY_CELL_SET_PRER {
    bits: u8,
}
impl DLY_CELL_SET_PRER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct NXT_ERRR {
    bits: bool,
}
impl NXT_ERRR {
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
pub struct TAP_SEL_POSTR {
    bits: u8,
}
impl TAP_SEL_POSTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct TAP_SEL_OUTR {
    bits: u8,
}
impl TAP_SEL_OUTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct TAP_SEL_PRER {
    bits: u8,
}
impl TAP_SEL_PRER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct PRE_ERRR {
    bits: bool,
}
impl PRE_ERRR {
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
#[doc = r" Proxy"]
pub struct _DLY_CELL_SET_POSTW<'a> {
    w: &'a mut W,
}
impl<'a> _DLY_CELL_SET_POSTW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _DLY_CELL_SET_OUTW<'a> {
    w: &'a mut W,
}
impl<'a> _DLY_CELL_SET_OUTW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 4;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _DLY_CELL_SET_PREW<'a> {
    w: &'a mut W,
}
impl<'a> _DLY_CELL_SET_PREW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 127;
        const OFFSET: u8 = 8;
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
    #[doc = "Bits 0:3 - DLY_CELL_SET_POST"]
    #[inline]
    pub fn dly_cell_set_post(&self) -> DLY_CELL_SET_POSTR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        DLY_CELL_SET_POSTR { bits }
    }
    #[doc = "Bits 4:7 - DLY_CELL_SET_OUT"]
    #[inline]
    pub fn dly_cell_set_out(&self) -> DLY_CELL_SET_OUTR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        DLY_CELL_SET_OUTR { bits }
    }
    #[doc = "Bits 8:14 - DLY_CELL_SET_PRE"]
    #[inline]
    pub fn dly_cell_set_pre(&self) -> DLY_CELL_SET_PRER {
        let bits = {
            const MASK: u8 = 127;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        DLY_CELL_SET_PRER { bits }
    }
    #[doc = "Bit 15 - NXT_ERR"]
    #[inline]
    pub fn nxt_err(&self) -> NXT_ERRR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 15;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        NXT_ERRR { bits }
    }
    #[doc = "Bits 16:19 - TAP_SEL_POST"]
    #[inline]
    pub fn tap_sel_post(&self) -> TAP_SEL_POSTR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        TAP_SEL_POSTR { bits }
    }
    #[doc = "Bits 20:23 - TAP_SEL_OUT"]
    #[inline]
    pub fn tap_sel_out(&self) -> TAP_SEL_OUTR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        TAP_SEL_OUTR { bits }
    }
    #[doc = "Bits 24:30 - TAP_SEL_PRE"]
    #[inline]
    pub fn tap_sel_pre(&self) -> TAP_SEL_PRER {
        let bits = {
            const MASK: u8 = 127;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        TAP_SEL_PRER { bits }
    }
    #[doc = "Bit 31 - PRE_ERR"]
    #[inline]
    pub fn pre_err(&self) -> PRE_ERRR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 31;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        PRE_ERRR { bits }
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
    #[doc = "Bits 0:3 - DLY_CELL_SET_POST"]
    #[inline]
    pub fn dly_cell_set_post(&mut self) -> _DLY_CELL_SET_POSTW {
        _DLY_CELL_SET_POSTW { w: self }
    }
    #[doc = "Bits 4:7 - DLY_CELL_SET_OUT"]
    #[inline]
    pub fn dly_cell_set_out(&mut self) -> _DLY_CELL_SET_OUTW {
        _DLY_CELL_SET_OUTW { w: self }
    }
    #[doc = "Bits 8:14 - DLY_CELL_SET_PRE"]
    #[inline]
    pub fn dly_cell_set_pre(&mut self) -> _DLY_CELL_SET_PREW {
        _DLY_CELL_SET_PREW { w: self }
    }
}
