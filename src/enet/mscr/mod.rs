#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::MSCR {
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
pub struct MII_SPEEDR {
    bits: u8,
}
impl MII_SPEEDR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Possible values of the field `DIS_PRE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DIS_PRER {
    #[doc = "Preamble enabled."]
    DIS_PRE_0,
    #[doc = "Preamble (32 ones) is not prepended to the MII management frame."]
    DIS_PRE_1,
}
impl DIS_PRER {
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
            DIS_PRER::DIS_PRE_0 => false,
            DIS_PRER::DIS_PRE_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DIS_PRER {
        match value {
            false => DIS_PRER::DIS_PRE_0,
            true => DIS_PRER::DIS_PRE_1,
        }
    }
    #[doc = "Checks if the value of the field is `DIS_PRE_0`"]
    #[inline]
    pub fn is_dis_pre_0(&self) -> bool {
        *self == DIS_PRER::DIS_PRE_0
    }
    #[doc = "Checks if the value of the field is `DIS_PRE_1`"]
    #[inline]
    pub fn is_dis_pre_1(&self) -> bool {
        *self == DIS_PRER::DIS_PRE_1
    }
}
#[doc = "Possible values of the field `HOLDTIME`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HOLDTIMER {
    #[doc = "1 internal module clock cycle"]
    HOLDTIME_0,
    #[doc = "2 internal module clock cycles"]
    HOLDTIME_1,
    #[doc = "3 internal module clock cycles"]
    HOLDTIME_2,
    #[doc = "8 internal module clock cycles"]
    HOLDTIME_7,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl HOLDTIMER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            HOLDTIMER::HOLDTIME_0 => 0,
            HOLDTIMER::HOLDTIME_1 => 1,
            HOLDTIMER::HOLDTIME_2 => 2,
            HOLDTIMER::HOLDTIME_7 => 7,
            HOLDTIMER::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> HOLDTIMER {
        match value {
            0 => HOLDTIMER::HOLDTIME_0,
            1 => HOLDTIMER::HOLDTIME_1,
            2 => HOLDTIMER::HOLDTIME_2,
            7 => HOLDTIMER::HOLDTIME_7,
            i => HOLDTIMER::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `HOLDTIME_0`"]
    #[inline]
    pub fn is_holdtime_0(&self) -> bool {
        *self == HOLDTIMER::HOLDTIME_0
    }
    #[doc = "Checks if the value of the field is `HOLDTIME_1`"]
    #[inline]
    pub fn is_holdtime_1(&self) -> bool {
        *self == HOLDTIMER::HOLDTIME_1
    }
    #[doc = "Checks if the value of the field is `HOLDTIME_2`"]
    #[inline]
    pub fn is_holdtime_2(&self) -> bool {
        *self == HOLDTIMER::HOLDTIME_2
    }
    #[doc = "Checks if the value of the field is `HOLDTIME_7`"]
    #[inline]
    pub fn is_holdtime_7(&self) -> bool {
        *self == HOLDTIMER::HOLDTIME_7
    }
}
#[doc = r" Proxy"]
pub struct _MII_SPEEDW<'a> {
    w: &'a mut W,
}
impl<'a> _MII_SPEEDW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 63;
        const OFFSET: u8 = 1;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `DIS_PRE`"]
pub enum DIS_PREW {
    #[doc = "Preamble enabled."]
    DIS_PRE_0,
    #[doc = "Preamble (32 ones) is not prepended to the MII management frame."]
    DIS_PRE_1,
}
impl DIS_PREW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DIS_PREW::DIS_PRE_0 => false,
            DIS_PREW::DIS_PRE_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DIS_PREW<'a> {
    w: &'a mut W,
}
impl<'a> _DIS_PREW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DIS_PREW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Preamble enabled."]
    #[inline]
    pub fn dis_pre_0(self) -> &'a mut W {
        self.variant(DIS_PREW::DIS_PRE_0)
    }
    #[doc = "Preamble (32 ones) is not prepended to the MII management frame."]
    #[inline]
    pub fn dis_pre_1(self) -> &'a mut W {
        self.variant(DIS_PREW::DIS_PRE_1)
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
        const OFFSET: u8 = 7;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `HOLDTIME`"]
pub enum HOLDTIMEW {
    #[doc = "1 internal module clock cycle"]
    HOLDTIME_0,
    #[doc = "2 internal module clock cycles"]
    HOLDTIME_1,
    #[doc = "3 internal module clock cycles"]
    HOLDTIME_2,
    #[doc = "8 internal module clock cycles"]
    HOLDTIME_7,
}
impl HOLDTIMEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            HOLDTIMEW::HOLDTIME_0 => 0,
            HOLDTIMEW::HOLDTIME_1 => 1,
            HOLDTIMEW::HOLDTIME_2 => 2,
            HOLDTIMEW::HOLDTIME_7 => 7,
        }
    }
}
#[doc = r" Proxy"]
pub struct _HOLDTIMEW<'a> {
    w: &'a mut W,
}
impl<'a> _HOLDTIMEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: HOLDTIMEW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "1 internal module clock cycle"]
    #[inline]
    pub fn holdtime_0(self) -> &'a mut W {
        self.variant(HOLDTIMEW::HOLDTIME_0)
    }
    #[doc = "2 internal module clock cycles"]
    #[inline]
    pub fn holdtime_1(self) -> &'a mut W {
        self.variant(HOLDTIMEW::HOLDTIME_1)
    }
    #[doc = "3 internal module clock cycles"]
    #[inline]
    pub fn holdtime_2(self) -> &'a mut W {
        self.variant(HOLDTIMEW::HOLDTIME_2)
    }
    #[doc = "8 internal module clock cycles"]
    #[inline]
    pub fn holdtime_7(self) -> &'a mut W {
        self.variant(HOLDTIMEW::HOLDTIME_7)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
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
    #[doc = "Bits 1:6 - MII Speed"]
    #[inline]
    pub fn mii_speed(&self) -> MII_SPEEDR {
        let bits = {
            const MASK: u8 = 63;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        MII_SPEEDR { bits }
    }
    #[doc = "Bit 7 - Disable Preamble"]
    #[inline]
    pub fn dis_pre(&self) -> DIS_PRER {
        DIS_PRER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 8:10 - Hold time On MDIO Output"]
    #[inline]
    pub fn holdtime(&self) -> HOLDTIMER {
        HOLDTIMER::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
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
    #[doc = "Bits 1:6 - MII Speed"]
    #[inline]
    pub fn mii_speed(&mut self) -> _MII_SPEEDW {
        _MII_SPEEDW { w: self }
    }
    #[doc = "Bit 7 - Disable Preamble"]
    #[inline]
    pub fn dis_pre(&mut self) -> _DIS_PREW {
        _DIS_PREW { w: self }
    }
    #[doc = "Bits 8:10 - Hold time On MDIO Output"]
    #[inline]
    pub fn holdtime(&mut self) -> _HOLDTIMEW {
        _HOLDTIMEW { w: self }
    }
}
