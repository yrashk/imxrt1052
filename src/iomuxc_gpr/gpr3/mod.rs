#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::GPR3 {
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
pub struct OCRAM_CTLR {
    bits: u8,
}
impl OCRAM_CTLR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Possible values of the field `DCP_KEY_SEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DCP_KEY_SELR {
    #[doc = "Select [127:0] from snvs/ocotp key as dcp key"]
    DCP_KEY_SEL_0,
    #[doc = "Select [255:128] from snvs/ocotp key as dcp key"]
    DCP_KEY_SEL_1,
}
impl DCP_KEY_SELR {
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
            DCP_KEY_SELR::DCP_KEY_SEL_0 => false,
            DCP_KEY_SELR::DCP_KEY_SEL_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DCP_KEY_SELR {
        match value {
            false => DCP_KEY_SELR::DCP_KEY_SEL_0,
            true => DCP_KEY_SELR::DCP_KEY_SEL_1,
        }
    }
    #[doc = "Checks if the value of the field is `DCP_KEY_SEL_0`"]
    #[inline]
    pub fn is_dcp_key_sel_0(&self) -> bool {
        *self == DCP_KEY_SELR::DCP_KEY_SEL_0
    }
    #[doc = "Checks if the value of the field is `DCP_KEY_SEL_1`"]
    #[inline]
    pub fn is_dcp_key_sel_1(&self) -> bool {
        *self == DCP_KEY_SELR::DCP_KEY_SEL_1
    }
}
#[doc = "Possible values of the field `OCRAM_STATUS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OCRAM_STATUSR {
    #[doc = "read data pipeline configuration valid"]
    OCRAM_STATUS_0,
    #[doc = "read data pipeline control bit changed"]
    OCRAM_STATUS_1,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl OCRAM_STATUSR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            OCRAM_STATUSR::OCRAM_STATUS_0 => 0,
            OCRAM_STATUSR::OCRAM_STATUS_1 => 1,
            OCRAM_STATUSR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> OCRAM_STATUSR {
        match value {
            0 => OCRAM_STATUSR::OCRAM_STATUS_0,
            1 => OCRAM_STATUSR::OCRAM_STATUS_1,
            i => OCRAM_STATUSR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `OCRAM_STATUS_0`"]
    #[inline]
    pub fn is_ocram_status_0(&self) -> bool {
        *self == OCRAM_STATUSR::OCRAM_STATUS_0
    }
    #[doc = "Checks if the value of the field is `OCRAM_STATUS_1`"]
    #[inline]
    pub fn is_ocram_status_1(&self) -> bool {
        *self == OCRAM_STATUSR::OCRAM_STATUS_1
    }
}
#[doc = r" Proxy"]
pub struct _OCRAM_CTLW<'a> {
    w: &'a mut W,
}
impl<'a> _OCRAM_CTLW<'a> {
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
#[doc = "Values that can be written to the field `DCP_KEY_SEL`"]
pub enum DCP_KEY_SELW {
    #[doc = "Select [127:0] from snvs/ocotp key as dcp key"]
    DCP_KEY_SEL_0,
    #[doc = "Select [255:128] from snvs/ocotp key as dcp key"]
    DCP_KEY_SEL_1,
}
impl DCP_KEY_SELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DCP_KEY_SELW::DCP_KEY_SEL_0 => false,
            DCP_KEY_SELW::DCP_KEY_SEL_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DCP_KEY_SELW<'a> {
    w: &'a mut W,
}
impl<'a> _DCP_KEY_SELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DCP_KEY_SELW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Select [127:0] from snvs/ocotp key as dcp key"]
    #[inline]
    pub fn dcp_key_sel_0(self) -> &'a mut W {
        self.variant(DCP_KEY_SELW::DCP_KEY_SEL_0)
    }
    #[doc = "Select [255:128] from snvs/ocotp key as dcp key"]
    #[inline]
    pub fn dcp_key_sel_1(self) -> &'a mut W {
        self.variant(DCP_KEY_SELW::DCP_KEY_SEL_1)
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
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:3 - OCRAM_CTL[3] - write address pipeline control bit"]
    #[inline]
    pub fn ocram_ctl(&self) -> OCRAM_CTLR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        OCRAM_CTLR { bits }
    }
    #[doc = "Bit 4 - Select 128-bit dcp key from 256-bit key from snvs/ocotp"]
    #[inline]
    pub fn dcp_key_sel(&self) -> DCP_KEY_SELR {
        DCP_KEY_SELR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 16:19 - This field shows the OCRAM pipeline settings status, controlled by OCRAM_CTL bits respectively"]
    #[inline]
    pub fn ocram_status(&self) -> OCRAM_STATUSR {
        OCRAM_STATUSR::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 4080 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:3 - OCRAM_CTL[3] - write address pipeline control bit"]
    #[inline]
    pub fn ocram_ctl(&mut self) -> _OCRAM_CTLW {
        _OCRAM_CTLW { w: self }
    }
    #[doc = "Bit 4 - Select 128-bit dcp key from 256-bit key from snvs/ocotp"]
    #[inline]
    pub fn dcp_key_sel(&mut self) -> _DCP_KEY_SELW {
        _DCP_KEY_SELW { w: self }
    }
}
