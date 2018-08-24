#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::OCRAM_MAGIC_ADDR {
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
#[doc = "Possible values of the field `OCRAM_WR_RD_SEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OCRAM_WR_RD_SELR {
    #[doc = "When OCRAM read access hits magic address, it will generate interrupt."]
    OCRAM_WR_RD_SEL_0,
    #[doc = "When OCRAM write access hits magic address, it will generate interrupt."]
    OCRAM_WR_RD_SEL_1,
}
impl OCRAM_WR_RD_SELR {
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
            OCRAM_WR_RD_SELR::OCRAM_WR_RD_SEL_0 => false,
            OCRAM_WR_RD_SELR::OCRAM_WR_RD_SEL_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> OCRAM_WR_RD_SELR {
        match value {
            false => OCRAM_WR_RD_SELR::OCRAM_WR_RD_SEL_0,
            true => OCRAM_WR_RD_SELR::OCRAM_WR_RD_SEL_1,
        }
    }
    #[doc = "Checks if the value of the field is `OCRAM_WR_RD_SEL_0`"]
    #[inline]
    pub fn is_ocram_wr_rd_sel_0(&self) -> bool {
        *self == OCRAM_WR_RD_SELR::OCRAM_WR_RD_SEL_0
    }
    #[doc = "Checks if the value of the field is `OCRAM_WR_RD_SEL_1`"]
    #[inline]
    pub fn is_ocram_wr_rd_sel_1(&self) -> bool {
        *self == OCRAM_WR_RD_SELR::OCRAM_WR_RD_SEL_1
    }
}
#[doc = r" Value of the field"]
pub struct OCRAM_MAGIC_ADDRR {
    bits: u16,
}
impl OCRAM_MAGIC_ADDRR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }
}
#[doc = "Values that can be written to the field `OCRAM_WR_RD_SEL`"]
pub enum OCRAM_WR_RD_SELW {
    #[doc = "When OCRAM read access hits magic address, it will generate interrupt."]
    OCRAM_WR_RD_SEL_0,
    #[doc = "When OCRAM write access hits magic address, it will generate interrupt."]
    OCRAM_WR_RD_SEL_1,
}
impl OCRAM_WR_RD_SELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            OCRAM_WR_RD_SELW::OCRAM_WR_RD_SEL_0 => false,
            OCRAM_WR_RD_SELW::OCRAM_WR_RD_SEL_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _OCRAM_WR_RD_SELW<'a> {
    w: &'a mut W,
}
impl<'a> _OCRAM_WR_RD_SELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: OCRAM_WR_RD_SELW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "When OCRAM read access hits magic address, it will generate interrupt."]
    #[inline]
    pub fn ocram_wr_rd_sel_0(self) -> &'a mut W {
        self.variant(OCRAM_WR_RD_SELW::OCRAM_WR_RD_SEL_0)
    }
    #[doc = "When OCRAM write access hits magic address, it will generate interrupt."]
    #[inline]
    pub fn ocram_wr_rd_sel_1(self) -> &'a mut W {
        self.variant(OCRAM_WR_RD_SELW::OCRAM_WR_RD_SEL_1)
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
pub struct _OCRAM_MAGIC_ADDRW<'a> {
    w: &'a mut W,
}
impl<'a> _OCRAM_MAGIC_ADDRW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        const MASK: u16 = 65535;
        const OFFSET: u8 = 1;
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
    #[doc = "Bit 0 - OCRAM Write Read Select"]
    #[inline]
    pub fn ocram_wr_rd_sel(&self) -> OCRAM_WR_RD_SELR {
        OCRAM_WR_RD_SELR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 1:16 - OCRAM Magic Address"]
    #[inline]
    pub fn ocram_magic_addr(&self) -> OCRAM_MAGIC_ADDRR {
        let bits = {
            const MASK: u16 = 65535;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        };
        OCRAM_MAGIC_ADDRR { bits }
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
    #[doc = "Bit 0 - OCRAM Write Read Select"]
    #[inline]
    pub fn ocram_wr_rd_sel(&mut self) -> _OCRAM_WR_RD_SELW {
        _OCRAM_WR_RD_SELW { w: self }
    }
    #[doc = "Bits 1:16 - OCRAM Magic Address"]
    #[inline]
    pub fn ocram_magic_addr(&mut self) -> _OCRAM_MAGIC_ADDRW {
        _OCRAM_MAGIC_ADDRW { w: self }
    }
}
