#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::DLL_CTRL {
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
pub struct DLL_CTRL_ENABLER {
    bits: bool,
}
impl DLL_CTRL_ENABLER {
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
pub struct DLL_CTRL_RESETR {
    bits: bool,
}
impl DLL_CTRL_RESETR {
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
pub struct DLL_CTRL_SLV_FORCE_UPDR {
    bits: bool,
}
impl DLL_CTRL_SLV_FORCE_UPDR {
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
pub struct DLL_CTRL_SLV_DLY_TARGET0R {
    bits: u8,
}
impl DLL_CTRL_SLV_DLY_TARGET0R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct DLL_CTRL_GATE_UPDATER {
    bits: bool,
}
impl DLL_CTRL_GATE_UPDATER {
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
pub struct DLL_CTRL_SLV_OVERRIDER {
    bits: bool,
}
impl DLL_CTRL_SLV_OVERRIDER {
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
pub struct DLL_CTRL_SLV_OVERRIDE_VALR {
    bits: u8,
}
impl DLL_CTRL_SLV_OVERRIDE_VALR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct DLL_CTRL_SLV_DLY_TARGET1R {
    bits: u8,
}
impl DLL_CTRL_SLV_DLY_TARGET1R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct DLL_CTRL_SLV_UPDATE_INTR {
    bits: u8,
}
impl DLL_CTRL_SLV_UPDATE_INTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct DLL_CTRL_REF_UPDATE_INTR {
    bits: u8,
}
impl DLL_CTRL_REF_UPDATE_INTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Proxy"]
pub struct _DLL_CTRL_ENABLEW<'a> {
    w: &'a mut W,
}
impl<'a> _DLL_CTRL_ENABLEW<'a> {
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
pub struct _DLL_CTRL_RESETW<'a> {
    w: &'a mut W,
}
impl<'a> _DLL_CTRL_RESETW<'a> {
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
#[doc = r" Proxy"]
pub struct _DLL_CTRL_SLV_FORCE_UPDW<'a> {
    w: &'a mut W,
}
impl<'a> _DLL_CTRL_SLV_FORCE_UPDW<'a> {
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
#[doc = r" Proxy"]
pub struct _DLL_CTRL_SLV_DLY_TARGET0W<'a> {
    w: &'a mut W,
}
impl<'a> _DLL_CTRL_SLV_DLY_TARGET0W<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 3;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _DLL_CTRL_GATE_UPDATEW<'a> {
    w: &'a mut W,
}
impl<'a> _DLL_CTRL_GATE_UPDATEW<'a> {
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
pub struct _DLL_CTRL_SLV_OVERRIDEW<'a> {
    w: &'a mut W,
}
impl<'a> _DLL_CTRL_SLV_OVERRIDEW<'a> {
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
pub struct _DLL_CTRL_SLV_OVERRIDE_VALW<'a> {
    w: &'a mut W,
}
impl<'a> _DLL_CTRL_SLV_OVERRIDE_VALW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 127;
        const OFFSET: u8 = 9;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _DLL_CTRL_SLV_DLY_TARGET1W<'a> {
    w: &'a mut W,
}
impl<'a> _DLL_CTRL_SLV_DLY_TARGET1W<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 16;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _DLL_CTRL_SLV_UPDATE_INTW<'a> {
    w: &'a mut W,
}
impl<'a> _DLL_CTRL_SLV_UPDATE_INTW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 255;
        const OFFSET: u8 = 20;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _DLL_CTRL_REF_UPDATE_INTW<'a> {
    w: &'a mut W,
}
impl<'a> _DLL_CTRL_REF_UPDATE_INTW<'a> {
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
    #[doc = "Bit 0 - DLL_CTRL_ENABLE"]
    #[inline]
    pub fn dll_ctrl_enable(&self) -> DLL_CTRL_ENABLER {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        DLL_CTRL_ENABLER { bits }
    }
    #[doc = "Bit 1 - DLL_CTRL_RESET"]
    #[inline]
    pub fn dll_ctrl_reset(&self) -> DLL_CTRL_RESETR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        DLL_CTRL_RESETR { bits }
    }
    #[doc = "Bit 2 - DLL_CTRL_SLV_FORCE_UPD"]
    #[inline]
    pub fn dll_ctrl_slv_force_upd(&self) -> DLL_CTRL_SLV_FORCE_UPDR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        DLL_CTRL_SLV_FORCE_UPDR { bits }
    }
    #[doc = "Bits 3:6 - DLL_CTRL_SLV_DLY_TARGET0"]
    #[inline]
    pub fn dll_ctrl_slv_dly_target0(&self) -> DLL_CTRL_SLV_DLY_TARGET0R {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        DLL_CTRL_SLV_DLY_TARGET0R { bits }
    }
    #[doc = "Bit 7 - DLL_CTRL_GATE_UPDATE"]
    #[inline]
    pub fn dll_ctrl_gate_update(&self) -> DLL_CTRL_GATE_UPDATER {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        DLL_CTRL_GATE_UPDATER { bits }
    }
    #[doc = "Bit 8 - DLL_CTRL_SLV_OVERRIDE"]
    #[inline]
    pub fn dll_ctrl_slv_override(&self) -> DLL_CTRL_SLV_OVERRIDER {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        DLL_CTRL_SLV_OVERRIDER { bits }
    }
    #[doc = "Bits 9:15 - DLL_CTRL_SLV_OVERRIDE_VAL"]
    #[inline]
    pub fn dll_ctrl_slv_override_val(&self) -> DLL_CTRL_SLV_OVERRIDE_VALR {
        let bits = {
            const MASK: u8 = 127;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        DLL_CTRL_SLV_OVERRIDE_VALR { bits }
    }
    #[doc = "Bits 16:18 - DLL_CTRL_SLV_DLY_TARGET1"]
    #[inline]
    pub fn dll_ctrl_slv_dly_target1(&self) -> DLL_CTRL_SLV_DLY_TARGET1R {
        let bits = {
            const MASK: u8 = 7;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        DLL_CTRL_SLV_DLY_TARGET1R { bits }
    }
    #[doc = "Bits 20:27 - DLL_CTRL_SLV_UPDATE_INT"]
    #[inline]
    pub fn dll_ctrl_slv_update_int(&self) -> DLL_CTRL_SLV_UPDATE_INTR {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        DLL_CTRL_SLV_UPDATE_INTR { bits }
    }
    #[doc = "Bits 28:31 - DLL_CTRL_REF_UPDATE_INT"]
    #[inline]
    pub fn dll_ctrl_ref_update_int(&self) -> DLL_CTRL_REF_UPDATE_INTR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 28;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        DLL_CTRL_REF_UPDATE_INTR { bits }
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 512 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - DLL_CTRL_ENABLE"]
    #[inline]
    pub fn dll_ctrl_enable(&mut self) -> _DLL_CTRL_ENABLEW {
        _DLL_CTRL_ENABLEW { w: self }
    }
    #[doc = "Bit 1 - DLL_CTRL_RESET"]
    #[inline]
    pub fn dll_ctrl_reset(&mut self) -> _DLL_CTRL_RESETW {
        _DLL_CTRL_RESETW { w: self }
    }
    #[doc = "Bit 2 - DLL_CTRL_SLV_FORCE_UPD"]
    #[inline]
    pub fn dll_ctrl_slv_force_upd(&mut self) -> _DLL_CTRL_SLV_FORCE_UPDW {
        _DLL_CTRL_SLV_FORCE_UPDW { w: self }
    }
    #[doc = "Bits 3:6 - DLL_CTRL_SLV_DLY_TARGET0"]
    #[inline]
    pub fn dll_ctrl_slv_dly_target0(&mut self) -> _DLL_CTRL_SLV_DLY_TARGET0W {
        _DLL_CTRL_SLV_DLY_TARGET0W { w: self }
    }
    #[doc = "Bit 7 - DLL_CTRL_GATE_UPDATE"]
    #[inline]
    pub fn dll_ctrl_gate_update(&mut self) -> _DLL_CTRL_GATE_UPDATEW {
        _DLL_CTRL_GATE_UPDATEW { w: self }
    }
    #[doc = "Bit 8 - DLL_CTRL_SLV_OVERRIDE"]
    #[inline]
    pub fn dll_ctrl_slv_override(&mut self) -> _DLL_CTRL_SLV_OVERRIDEW {
        _DLL_CTRL_SLV_OVERRIDEW { w: self }
    }
    #[doc = "Bits 9:15 - DLL_CTRL_SLV_OVERRIDE_VAL"]
    #[inline]
    pub fn dll_ctrl_slv_override_val(&mut self) -> _DLL_CTRL_SLV_OVERRIDE_VALW {
        _DLL_CTRL_SLV_OVERRIDE_VALW { w: self }
    }
    #[doc = "Bits 16:18 - DLL_CTRL_SLV_DLY_TARGET1"]
    #[inline]
    pub fn dll_ctrl_slv_dly_target1(&mut self) -> _DLL_CTRL_SLV_DLY_TARGET1W {
        _DLL_CTRL_SLV_DLY_TARGET1W { w: self }
    }
    #[doc = "Bits 20:27 - DLL_CTRL_SLV_UPDATE_INT"]
    #[inline]
    pub fn dll_ctrl_slv_update_int(&mut self) -> _DLL_CTRL_SLV_UPDATE_INTW {
        _DLL_CTRL_SLV_UPDATE_INTW { w: self }
    }
    #[doc = "Bits 28:31 - DLL_CTRL_REF_UPDATE_INT"]
    #[inline]
    pub fn dll_ctrl_ref_update_int(&mut self) -> _DLL_CTRL_REF_UPDATE_INTW {
        _DLL_CTRL_REF_UPDATE_INTW { w: self }
    }
}
