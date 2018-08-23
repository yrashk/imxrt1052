#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::FLSHCR1 {
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
pub struct TCSSR {
    bits: u8,
}
impl TCSSR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct TCSHR {
    bits: u8,
}
impl TCSHR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct WAR {
    bits: bool,
}
impl WAR {
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
pub struct CASR {
    bits: u8,
}
impl CASR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Possible values of the field `CSINTERVALUNIT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CSINTERVALUNITR {
    #[doc = "The CS interval unit is 1 serial clock cycle"]
    CSINTERVALUNIT_0,
    #[doc = "The CS interval unit is 256 serial clock cycle"]
    CSINTERVALUNIT_1,
}
impl CSINTERVALUNITR {
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
            CSINTERVALUNITR::CSINTERVALUNIT_0 => false,
            CSINTERVALUNITR::CSINTERVALUNIT_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CSINTERVALUNITR {
        match value {
            false => CSINTERVALUNITR::CSINTERVALUNIT_0,
            true => CSINTERVALUNITR::CSINTERVALUNIT_1,
        }
    }
    #[doc = "Checks if the value of the field is `CSINTERVALUNIT_0`"]
    #[inline]
    pub fn is_csintervalunit_0(&self) -> bool {
        *self == CSINTERVALUNITR::CSINTERVALUNIT_0
    }
    #[doc = "Checks if the value of the field is `CSINTERVALUNIT_1`"]
    #[inline]
    pub fn is_csintervalunit_1(&self) -> bool {
        *self == CSINTERVALUNITR::CSINTERVALUNIT_1
    }
}
#[doc = r" Value of the field"]
pub struct CSINTERVALR {
    bits: u16,
}
impl CSINTERVALR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }
}
#[doc = r" Proxy"]
pub struct _TCSSW<'a> {
    w: &'a mut W,
}
impl<'a> _TCSSW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 31;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _TCSHW<'a> {
    w: &'a mut W,
}
impl<'a> _TCSHW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 31;
        const OFFSET: u8 = 5;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _WAW<'a> {
    w: &'a mut W,
}
impl<'a> _WAW<'a> {
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
        const OFFSET: u8 = 10;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _CASW<'a> {
    w: &'a mut W,
}
impl<'a> _CASW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 11;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `CSINTERVALUNIT`"]
pub enum CSINTERVALUNITW {
    #[doc = "The CS interval unit is 1 serial clock cycle"]
    CSINTERVALUNIT_0,
    #[doc = "The CS interval unit is 256 serial clock cycle"]
    CSINTERVALUNIT_1,
}
impl CSINTERVALUNITW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CSINTERVALUNITW::CSINTERVALUNIT_0 => false,
            CSINTERVALUNITW::CSINTERVALUNIT_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CSINTERVALUNITW<'a> {
    w: &'a mut W,
}
impl<'a> _CSINTERVALUNITW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CSINTERVALUNITW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The CS interval unit is 1 serial clock cycle"]
    #[inline]
    pub fn csintervalunit_0(self) -> &'a mut W {
        self.variant(CSINTERVALUNITW::CSINTERVALUNIT_0)
    }
    #[doc = "The CS interval unit is 256 serial clock cycle"]
    #[inline]
    pub fn csintervalunit_1(self) -> &'a mut W {
        self.variant(CSINTERVALUNITW::CSINTERVALUNIT_1)
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
        const OFFSET: u8 = 15;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _CSINTERVALW<'a> {
    w: &'a mut W,
}
impl<'a> _CSINTERVALW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        const MASK: u16 = 65535;
        const OFFSET: u8 = 16;
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
    #[doc = "Bits 0:4 - Serial Flash CS setup time."]
    #[inline]
    pub fn tcss(&self) -> TCSSR {
        let bits = {
            const MASK: u8 = 31;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        TCSSR { bits }
    }
    #[doc = "Bits 5:9 - Serial Flash CS Hold time."]
    #[inline]
    pub fn tcsh(&self) -> TCSHR {
        let bits = {
            const MASK: u8 = 31;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        TCSHR { bits }
    }
    #[doc = "Bit 10 - Word Addressable."]
    #[inline]
    pub fn wa(&self) -> WAR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        WAR { bits }
    }
    #[doc = "Bits 11:14 - Column Address Size."]
    #[inline]
    pub fn cas(&self) -> CASR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        CASR { bits }
    }
    #[doc = "Bit 15 - CS interval unit"]
    #[inline]
    pub fn csintervalunit(&self) -> CSINTERVALUNITR {
        CSINTERVALUNITR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 15;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 16:31 - This field is used to set the minimum interval between flash device Chip selection deassertion and flash device Chip selection assertion. If external flash has a limitation on the interval between command sequences, this field should be set accordingly. If there is no limitation, set this field with value 0x0."]
    #[inline]
    pub fn csinterval(&self) -> CSINTERVALR {
        let bits = {
            const MASK: u16 = 65535;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        };
        CSINTERVALR { bits }
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 99 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:4 - Serial Flash CS setup time."]
    #[inline]
    pub fn tcss(&mut self) -> _TCSSW {
        _TCSSW { w: self }
    }
    #[doc = "Bits 5:9 - Serial Flash CS Hold time."]
    #[inline]
    pub fn tcsh(&mut self) -> _TCSHW {
        _TCSHW { w: self }
    }
    #[doc = "Bit 10 - Word Addressable."]
    #[inline]
    pub fn wa(&mut self) -> _WAW {
        _WAW { w: self }
    }
    #[doc = "Bits 11:14 - Column Address Size."]
    #[inline]
    pub fn cas(&mut self) -> _CASW {
        _CASW { w: self }
    }
    #[doc = "Bit 15 - CS interval unit"]
    #[inline]
    pub fn csintervalunit(&mut self) -> _CSINTERVALUNITW {
        _CSINTERVALUNITW { w: self }
    }
    #[doc = "Bits 16:31 - This field is used to set the minimum interval between flash device Chip selection deassertion and flash device Chip selection assertion. If external flash has a limitation on the interval between command sequences, this field should be set accordingly. If there is no limitation, set this field with value 0x0."]
    #[inline]
    pub fn csinterval(&mut self) -> _CSINTERVALW {
        _CSINTERVALW { w: self }
    }
}
