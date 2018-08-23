#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::FLSHCR2 {
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
pub struct ARDSEQIDR {
    bits: u8,
}
impl ARDSEQIDR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct ARDSEQNUMR {
    bits: u8,
}
impl ARDSEQNUMR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct AWRSEQIDR {
    bits: u8,
}
impl AWRSEQIDR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct AWRSEQNUMR {
    bits: u8,
}
impl AWRSEQNUMR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct AWRWAITR {
    bits: u16,
}
impl AWRWAITR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }
}
#[doc = "Possible values of the field `AWRWAITUNIT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AWRWAITUNITR {
    #[doc = "The AWRWAIT unit is 2 ahb clock cycle"]
    AWRWAITUNIT_0,
    #[doc = "The AWRWAIT unit is 8 ahb clock cycle"]
    AWRWAITUNIT_1,
    #[doc = "The AWRWAIT unit is 32 ahb clock cycle"]
    AWRWAITUNIT_2,
    #[doc = "The AWRWAIT unit is 128 ahb clock cycle"]
    AWRWAITUNIT_3,
    #[doc = "The AWRWAIT unit is 512 ahb clock cycle"]
    AWRWAITUNIT_4,
    #[doc = "The AWRWAIT unit is 2048 ahb clock cycle"]
    AWRWAITUNIT_5,
    #[doc = "The AWRWAIT unit is 8192 ahb clock cycle"]
    AWRWAITUNIT_6,
    #[doc = "The AWRWAIT unit is 32768 ahb clock cycle"]
    AWRWAITUNIT_7,
}
impl AWRWAITUNITR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            AWRWAITUNITR::AWRWAITUNIT_0 => 0,
            AWRWAITUNITR::AWRWAITUNIT_1 => 1,
            AWRWAITUNITR::AWRWAITUNIT_2 => 2,
            AWRWAITUNITR::AWRWAITUNIT_3 => 3,
            AWRWAITUNITR::AWRWAITUNIT_4 => 4,
            AWRWAITUNITR::AWRWAITUNIT_5 => 5,
            AWRWAITUNITR::AWRWAITUNIT_6 => 6,
            AWRWAITUNITR::AWRWAITUNIT_7 => 7,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> AWRWAITUNITR {
        match value {
            0 => AWRWAITUNITR::AWRWAITUNIT_0,
            1 => AWRWAITUNITR::AWRWAITUNIT_1,
            2 => AWRWAITUNITR::AWRWAITUNIT_2,
            3 => AWRWAITUNITR::AWRWAITUNIT_3,
            4 => AWRWAITUNITR::AWRWAITUNIT_4,
            5 => AWRWAITUNITR::AWRWAITUNIT_5,
            6 => AWRWAITUNITR::AWRWAITUNIT_6,
            7 => AWRWAITUNITR::AWRWAITUNIT_7,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `AWRWAITUNIT_0`"]
    #[inline]
    pub fn is_awrwaitunit_0(&self) -> bool {
        *self == AWRWAITUNITR::AWRWAITUNIT_0
    }
    #[doc = "Checks if the value of the field is `AWRWAITUNIT_1`"]
    #[inline]
    pub fn is_awrwaitunit_1(&self) -> bool {
        *self == AWRWAITUNITR::AWRWAITUNIT_1
    }
    #[doc = "Checks if the value of the field is `AWRWAITUNIT_2`"]
    #[inline]
    pub fn is_awrwaitunit_2(&self) -> bool {
        *self == AWRWAITUNITR::AWRWAITUNIT_2
    }
    #[doc = "Checks if the value of the field is `AWRWAITUNIT_3`"]
    #[inline]
    pub fn is_awrwaitunit_3(&self) -> bool {
        *self == AWRWAITUNITR::AWRWAITUNIT_3
    }
    #[doc = "Checks if the value of the field is `AWRWAITUNIT_4`"]
    #[inline]
    pub fn is_awrwaitunit_4(&self) -> bool {
        *self == AWRWAITUNITR::AWRWAITUNIT_4
    }
    #[doc = "Checks if the value of the field is `AWRWAITUNIT_5`"]
    #[inline]
    pub fn is_awrwaitunit_5(&self) -> bool {
        *self == AWRWAITUNITR::AWRWAITUNIT_5
    }
    #[doc = "Checks if the value of the field is `AWRWAITUNIT_6`"]
    #[inline]
    pub fn is_awrwaitunit_6(&self) -> bool {
        *self == AWRWAITUNITR::AWRWAITUNIT_6
    }
    #[doc = "Checks if the value of the field is `AWRWAITUNIT_7`"]
    #[inline]
    pub fn is_awrwaitunit_7(&self) -> bool {
        *self == AWRWAITUNITR::AWRWAITUNIT_7
    }
}
#[doc = r" Value of the field"]
pub struct CLRINSTRPTRR {
    bits: bool,
}
impl CLRINSTRPTRR {
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
pub struct _ARDSEQIDW<'a> {
    w: &'a mut W,
}
impl<'a> _ARDSEQIDW<'a> {
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
pub struct _ARDSEQNUMW<'a> {
    w: &'a mut W,
}
impl<'a> _ARDSEQNUMW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 5;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _AWRSEQIDW<'a> {
    w: &'a mut W,
}
impl<'a> _AWRSEQIDW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _AWRSEQNUMW<'a> {
    w: &'a mut W,
}
impl<'a> _AWRSEQNUMW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 13;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _AWRWAITW<'a> {
    w: &'a mut W,
}
impl<'a> _AWRWAITW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        const MASK: u16 = 4095;
        const OFFSET: u8 = 16;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `AWRWAITUNIT`"]
pub enum AWRWAITUNITW {
    #[doc = "The AWRWAIT unit is 2 ahb clock cycle"]
    AWRWAITUNIT_0,
    #[doc = "The AWRWAIT unit is 8 ahb clock cycle"]
    AWRWAITUNIT_1,
    #[doc = "The AWRWAIT unit is 32 ahb clock cycle"]
    AWRWAITUNIT_2,
    #[doc = "The AWRWAIT unit is 128 ahb clock cycle"]
    AWRWAITUNIT_3,
    #[doc = "The AWRWAIT unit is 512 ahb clock cycle"]
    AWRWAITUNIT_4,
    #[doc = "The AWRWAIT unit is 2048 ahb clock cycle"]
    AWRWAITUNIT_5,
    #[doc = "The AWRWAIT unit is 8192 ahb clock cycle"]
    AWRWAITUNIT_6,
    #[doc = "The AWRWAIT unit is 32768 ahb clock cycle"]
    AWRWAITUNIT_7,
}
impl AWRWAITUNITW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            AWRWAITUNITW::AWRWAITUNIT_0 => 0,
            AWRWAITUNITW::AWRWAITUNIT_1 => 1,
            AWRWAITUNITW::AWRWAITUNIT_2 => 2,
            AWRWAITUNITW::AWRWAITUNIT_3 => 3,
            AWRWAITUNITW::AWRWAITUNIT_4 => 4,
            AWRWAITUNITW::AWRWAITUNIT_5 => 5,
            AWRWAITUNITW::AWRWAITUNIT_6 => 6,
            AWRWAITUNITW::AWRWAITUNIT_7 => 7,
        }
    }
}
#[doc = r" Proxy"]
pub struct _AWRWAITUNITW<'a> {
    w: &'a mut W,
}
impl<'a> _AWRWAITUNITW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: AWRWAITUNITW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "The AWRWAIT unit is 2 ahb clock cycle"]
    #[inline]
    pub fn awrwaitunit_0(self) -> &'a mut W {
        self.variant(AWRWAITUNITW::AWRWAITUNIT_0)
    }
    #[doc = "The AWRWAIT unit is 8 ahb clock cycle"]
    #[inline]
    pub fn awrwaitunit_1(self) -> &'a mut W {
        self.variant(AWRWAITUNITW::AWRWAITUNIT_1)
    }
    #[doc = "The AWRWAIT unit is 32 ahb clock cycle"]
    #[inline]
    pub fn awrwaitunit_2(self) -> &'a mut W {
        self.variant(AWRWAITUNITW::AWRWAITUNIT_2)
    }
    #[doc = "The AWRWAIT unit is 128 ahb clock cycle"]
    #[inline]
    pub fn awrwaitunit_3(self) -> &'a mut W {
        self.variant(AWRWAITUNITW::AWRWAITUNIT_3)
    }
    #[doc = "The AWRWAIT unit is 512 ahb clock cycle"]
    #[inline]
    pub fn awrwaitunit_4(self) -> &'a mut W {
        self.variant(AWRWAITUNITW::AWRWAITUNIT_4)
    }
    #[doc = "The AWRWAIT unit is 2048 ahb clock cycle"]
    #[inline]
    pub fn awrwaitunit_5(self) -> &'a mut W {
        self.variant(AWRWAITUNITW::AWRWAITUNIT_5)
    }
    #[doc = "The AWRWAIT unit is 8192 ahb clock cycle"]
    #[inline]
    pub fn awrwaitunit_6(self) -> &'a mut W {
        self.variant(AWRWAITUNITW::AWRWAITUNIT_6)
    }
    #[doc = "The AWRWAIT unit is 32768 ahb clock cycle"]
    #[inline]
    pub fn awrwaitunit_7(self) -> &'a mut W {
        self.variant(AWRWAITUNITW::AWRWAITUNIT_7)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 28;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _CLRINSTRPTRW<'a> {
    w: &'a mut W,
}
impl<'a> _CLRINSTRPTRW<'a> {
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
        const OFFSET: u8 = 31;
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
    #[doc = "Bits 0:3 - Sequence Index for AHB Read triggered Command in LUT."]
    #[inline]
    pub fn ardseqid(&self) -> ARDSEQIDR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        ARDSEQIDR { bits }
    }
    #[doc = "Bits 5:7 - Sequence Number for AHB Read triggered Command in LUT."]
    #[inline]
    pub fn ardseqnum(&self) -> ARDSEQNUMR {
        let bits = {
            const MASK: u8 = 7;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        ARDSEQNUMR { bits }
    }
    #[doc = "Bits 8:11 - Sequence Index for AHB Write triggered Command."]
    #[inline]
    pub fn awrseqid(&self) -> AWRSEQIDR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        AWRSEQIDR { bits }
    }
    #[doc = "Bits 13:15 - Sequence Number for AHB Write triggered Command."]
    #[inline]
    pub fn awrseqnum(&self) -> AWRSEQNUMR {
        let bits = {
            const MASK: u8 = 7;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        AWRSEQNUMR { bits }
    }
    #[doc = "Bits 16:27 - For certain devices (such as FPGA), it need some time to write data into internal memory after the command sequences finished on FlexSPI interface"]
    #[inline]
    pub fn awrwait(&self) -> AWRWAITR {
        let bits = {
            const MASK: u16 = 4095;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        };
        AWRWAITR { bits }
    }
    #[doc = "Bits 28:30 - AWRWAIT unit"]
    #[inline]
    pub fn awrwaitunit(&self) -> AWRWAITUNITR {
        AWRWAITUNITR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 28;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 31 - Clear the instruction pointer which is internally saved pointer by JMP_ON_CS. Refer Programmable Sequence Engine for details."]
    #[inline]
    pub fn clrinstrptr(&self) -> CLRINSTRPTRR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 31;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        CLRINSTRPTRR { bits }
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
    #[doc = "Bits 0:3 - Sequence Index for AHB Read triggered Command in LUT."]
    #[inline]
    pub fn ardseqid(&mut self) -> _ARDSEQIDW {
        _ARDSEQIDW { w: self }
    }
    #[doc = "Bits 5:7 - Sequence Number for AHB Read triggered Command in LUT."]
    #[inline]
    pub fn ardseqnum(&mut self) -> _ARDSEQNUMW {
        _ARDSEQNUMW { w: self }
    }
    #[doc = "Bits 8:11 - Sequence Index for AHB Write triggered Command."]
    #[inline]
    pub fn awrseqid(&mut self) -> _AWRSEQIDW {
        _AWRSEQIDW { w: self }
    }
    #[doc = "Bits 13:15 - Sequence Number for AHB Write triggered Command."]
    #[inline]
    pub fn awrseqnum(&mut self) -> _AWRSEQNUMW {
        _AWRSEQNUMW { w: self }
    }
    #[doc = "Bits 16:27 - For certain devices (such as FPGA), it need some time to write data into internal memory after the command sequences finished on FlexSPI interface"]
    #[inline]
    pub fn awrwait(&mut self) -> _AWRWAITW {
        _AWRWAITW { w: self }
    }
    #[doc = "Bits 28:30 - AWRWAIT unit"]
    #[inline]
    pub fn awrwaitunit(&mut self) -> _AWRWAITUNITW {
        _AWRWAITUNITW { w: self }
    }
    #[doc = "Bit 31 - Clear the instruction pointer which is internally saved pointer by JMP_ON_CS. Refer Programmable Sequence Engine for details."]
    #[inline]
    pub fn clrinstrptr(&mut self) -> _CLRINSTRPTRW {
        _CLRINSTRPTRW { w: self }
    }
}
