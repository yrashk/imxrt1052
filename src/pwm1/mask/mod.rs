#[doc = r" Value read from the register"]
pub struct R {
    bits: u16,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u16,
}
impl super::MASK {
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
#[doc = "Possible values of the field `MASKX`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MASKXR {
    #[doc = "PWM_X output normal."]
    MASKX_0,
    #[doc = "PWM_X output masked."]
    MASKX_1,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl MASKXR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            MASKXR::MASKX_0 => 0,
            MASKXR::MASKX_1 => 1,
            MASKXR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> MASKXR {
        match value {
            0 => MASKXR::MASKX_0,
            1 => MASKXR::MASKX_1,
            i => MASKXR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `MASKX_0`"]
    #[inline]
    pub fn is_maskx_0(&self) -> bool {
        *self == MASKXR::MASKX_0
    }
    #[doc = "Checks if the value of the field is `MASKX_1`"]
    #[inline]
    pub fn is_maskx_1(&self) -> bool {
        *self == MASKXR::MASKX_1
    }
}
#[doc = "Possible values of the field `MASKB`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MASKBR {
    #[doc = "PWM_B output normal."]
    MASKB_0,
    #[doc = "PWM_B output masked."]
    MASKB_1,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl MASKBR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            MASKBR::MASKB_0 => 0,
            MASKBR::MASKB_1 => 1,
            MASKBR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> MASKBR {
        match value {
            0 => MASKBR::MASKB_0,
            1 => MASKBR::MASKB_1,
            i => MASKBR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `MASKB_0`"]
    #[inline]
    pub fn is_maskb_0(&self) -> bool {
        *self == MASKBR::MASKB_0
    }
    #[doc = "Checks if the value of the field is `MASKB_1`"]
    #[inline]
    pub fn is_maskb_1(&self) -> bool {
        *self == MASKBR::MASKB_1
    }
}
#[doc = "Possible values of the field `MASKA`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MASKAR {
    #[doc = "PWM_A output normal."]
    MASKA_0,
    #[doc = "PWM_A output masked."]
    MASKA_1,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl MASKAR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            MASKAR::MASKA_0 => 0,
            MASKAR::MASKA_1 => 1,
            MASKAR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> MASKAR {
        match value {
            0 => MASKAR::MASKA_0,
            1 => MASKAR::MASKA_1,
            i => MASKAR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `MASKA_0`"]
    #[inline]
    pub fn is_maska_0(&self) -> bool {
        *self == MASKAR::MASKA_0
    }
    #[doc = "Checks if the value of the field is `MASKA_1`"]
    #[inline]
    pub fn is_maska_1(&self) -> bool {
        *self == MASKAR::MASKA_1
    }
}
#[doc = "Values that can be written to the field `MASKX`"]
pub enum MASKXW {
    #[doc = "PWM_X output normal."]
    MASKX_0,
    #[doc = "PWM_X output masked."]
    MASKX_1,
}
impl MASKXW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            MASKXW::MASKX_0 => 0,
            MASKXW::MASKX_1 => 1,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MASKXW<'a> {
    w: &'a mut W,
}
impl<'a> _MASKXW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MASKXW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "PWM_X output normal."]
    #[inline]
    pub fn maskx_0(self) -> &'a mut W {
        self.variant(MASKXW::MASKX_0)
    }
    #[doc = "PWM_X output masked."]
    #[inline]
    pub fn maskx_1(self) -> &'a mut W {
        self.variant(MASKXW::MASKX_1)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u16) << OFFSET);
        self.w.bits |= ((value & MASK) as u16) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `MASKB`"]
pub enum MASKBW {
    #[doc = "PWM_B output normal."]
    MASKB_0,
    #[doc = "PWM_B output masked."]
    MASKB_1,
}
impl MASKBW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            MASKBW::MASKB_0 => 0,
            MASKBW::MASKB_1 => 1,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MASKBW<'a> {
    w: &'a mut W,
}
impl<'a> _MASKBW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MASKBW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "PWM_B output normal."]
    #[inline]
    pub fn maskb_0(self) -> &'a mut W {
        self.variant(MASKBW::MASKB_0)
    }
    #[doc = "PWM_B output masked."]
    #[inline]
    pub fn maskb_1(self) -> &'a mut W {
        self.variant(MASKBW::MASKB_1)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 4;
        self.w.bits &= !((MASK as u16) << OFFSET);
        self.w.bits |= ((value & MASK) as u16) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `MASKA`"]
pub enum MASKAW {
    #[doc = "PWM_A output normal."]
    MASKA_0,
    #[doc = "PWM_A output masked."]
    MASKA_1,
}
impl MASKAW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            MASKAW::MASKA_0 => 0,
            MASKAW::MASKA_1 => 1,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MASKAW<'a> {
    w: &'a mut W,
}
impl<'a> _MASKAW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MASKAW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "PWM_A output normal."]
    #[inline]
    pub fn maska_0(self) -> &'a mut W {
        self.variant(MASKAW::MASKA_0)
    }
    #[doc = "PWM_A output masked."]
    #[inline]
    pub fn maska_1(self) -> &'a mut W {
        self.variant(MASKAW::MASKA_1)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u16) << OFFSET);
        self.w.bits |= ((value & MASK) as u16) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `UPDATE_MASK`"]
pub enum UPDATE_MASKW {
    #[doc = "Normal operation. MASK* bits within the corresponding submodule are not updated until a FORCE_OUT event occurs within the submodule."]
    UPDATE_MASK_0,
    #[doc = "Immediate operation. MASK* bits within the corresponding submodule are updated on the following clock edge after setting this bit."]
    UPDATE_MASK_1,
}
impl UPDATE_MASKW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            UPDATE_MASKW::UPDATE_MASK_0 => 0,
            UPDATE_MASKW::UPDATE_MASK_1 => 1,
        }
    }
}
#[doc = r" Proxy"]
pub struct _UPDATE_MASKW<'a> {
    w: &'a mut W,
}
impl<'a> _UPDATE_MASKW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: UPDATE_MASKW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Normal operation. MASK* bits within the corresponding submodule are not updated until a FORCE_OUT event occurs within the submodule."]
    #[inline]
    pub fn update_mask_0(self) -> &'a mut W {
        self.variant(UPDATE_MASKW::UPDATE_MASK_0)
    }
    #[doc = "Immediate operation. MASK* bits within the corresponding submodule are updated on the following clock edge after setting this bit."]
    #[inline]
    pub fn update_mask_1(self) -> &'a mut W {
        self.variant(UPDATE_MASKW::UPDATE_MASK_1)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 12;
        self.w.bits &= !((MASK as u16) << OFFSET);
        self.w.bits |= ((value & MASK) as u16) << OFFSET;
        self.w
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }
    #[doc = "Bits 0:3 - PWM_X Masks"]
    #[inline]
    pub fn maskx(&self) -> MASKXR {
        MASKXR::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u16) as u8
        })
    }
    #[doc = "Bits 4:7 - PWM_B Masks"]
    #[inline]
    pub fn maskb(&self) -> MASKBR {
        MASKBR::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u16) as u8
        })
    }
    #[doc = "Bits 8:11 - PWM_A Masks"]
    #[inline]
    pub fn maska(&self) -> MASKAR {
        MASKAR::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u16) as u8
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
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:3 - PWM_X Masks"]
    #[inline]
    pub fn maskx(&mut self) -> _MASKXW {
        _MASKXW { w: self }
    }
    #[doc = "Bits 4:7 - PWM_B Masks"]
    #[inline]
    pub fn maskb(&mut self) -> _MASKBW {
        _MASKBW { w: self }
    }
    #[doc = "Bits 8:11 - PWM_A Masks"]
    #[inline]
    pub fn maska(&mut self) -> _MASKAW {
        _MASKAW { w: self }
    }
    #[doc = "Bits 12:15 - Update Mask Bits Immediately"]
    #[inline]
    pub fn update_mask(&mut self) -> _UPDATE_MASKW {
        _UPDATE_MASKW { w: self }
    }
}
