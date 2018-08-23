#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::SBUSCFG {
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
#[doc = "Possible values of the field `AHBBRST`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AHBBRSTR {
    #[doc = "Incremental burst of unspecified length only"]
    AHBBRST_0,
    #[doc = "INCR4 burst, then single transfer"]
    AHBBRST_1,
    #[doc = "INCR8 burst, INCR4 burst, then single transfer"]
    AHBBRST_2,
    #[doc = "INCR16 burst, INCR8 burst, INCR4 burst, then single transfer"]
    AHBBRST_3,
    #[doc = "INCR4 burst, then incremental burst of unspecified length"]
    AHBBRST_5,
    #[doc = "INCR8 burst, INCR4 burst, then incremental burst of unspecified length"]
    AHBBRST_6,
    #[doc = "INCR16 burst, INCR8 burst, INCR4 burst, then incremental burst of unspecified length"]
    AHBBRST_7,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl AHBBRSTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            AHBBRSTR::AHBBRST_0 => 0,
            AHBBRSTR::AHBBRST_1 => 1,
            AHBBRSTR::AHBBRST_2 => 2,
            AHBBRSTR::AHBBRST_3 => 3,
            AHBBRSTR::AHBBRST_5 => 5,
            AHBBRSTR::AHBBRST_6 => 6,
            AHBBRSTR::AHBBRST_7 => 7,
            AHBBRSTR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> AHBBRSTR {
        match value {
            0 => AHBBRSTR::AHBBRST_0,
            1 => AHBBRSTR::AHBBRST_1,
            2 => AHBBRSTR::AHBBRST_2,
            3 => AHBBRSTR::AHBBRST_3,
            5 => AHBBRSTR::AHBBRST_5,
            6 => AHBBRSTR::AHBBRST_6,
            7 => AHBBRSTR::AHBBRST_7,
            i => AHBBRSTR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `AHBBRST_0`"]
    #[inline]
    pub fn is_ahbbrst_0(&self) -> bool {
        *self == AHBBRSTR::AHBBRST_0
    }
    #[doc = "Checks if the value of the field is `AHBBRST_1`"]
    #[inline]
    pub fn is_ahbbrst_1(&self) -> bool {
        *self == AHBBRSTR::AHBBRST_1
    }
    #[doc = "Checks if the value of the field is `AHBBRST_2`"]
    #[inline]
    pub fn is_ahbbrst_2(&self) -> bool {
        *self == AHBBRSTR::AHBBRST_2
    }
    #[doc = "Checks if the value of the field is `AHBBRST_3`"]
    #[inline]
    pub fn is_ahbbrst_3(&self) -> bool {
        *self == AHBBRSTR::AHBBRST_3
    }
    #[doc = "Checks if the value of the field is `AHBBRST_5`"]
    #[inline]
    pub fn is_ahbbrst_5(&self) -> bool {
        *self == AHBBRSTR::AHBBRST_5
    }
    #[doc = "Checks if the value of the field is `AHBBRST_6`"]
    #[inline]
    pub fn is_ahbbrst_6(&self) -> bool {
        *self == AHBBRSTR::AHBBRST_6
    }
    #[doc = "Checks if the value of the field is `AHBBRST_7`"]
    #[inline]
    pub fn is_ahbbrst_7(&self) -> bool {
        *self == AHBBRSTR::AHBBRST_7
    }
}
#[doc = "Values that can be written to the field `AHBBRST`"]
pub enum AHBBRSTW {
    #[doc = "Incremental burst of unspecified length only"]
    AHBBRST_0,
    #[doc = "INCR4 burst, then single transfer"]
    AHBBRST_1,
    #[doc = "INCR8 burst, INCR4 burst, then single transfer"]
    AHBBRST_2,
    #[doc = "INCR16 burst, INCR8 burst, INCR4 burst, then single transfer"]
    AHBBRST_3,
    #[doc = "INCR4 burst, then incremental burst of unspecified length"]
    AHBBRST_5,
    #[doc = "INCR8 burst, INCR4 burst, then incremental burst of unspecified length"]
    AHBBRST_6,
    #[doc = "INCR16 burst, INCR8 burst, INCR4 burst, then incremental burst of unspecified length"]
    AHBBRST_7,
}
impl AHBBRSTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            AHBBRSTW::AHBBRST_0 => 0,
            AHBBRSTW::AHBBRST_1 => 1,
            AHBBRSTW::AHBBRST_2 => 2,
            AHBBRSTW::AHBBRST_3 => 3,
            AHBBRSTW::AHBBRST_5 => 5,
            AHBBRSTW::AHBBRST_6 => 6,
            AHBBRSTW::AHBBRST_7 => 7,
        }
    }
}
#[doc = r" Proxy"]
pub struct _AHBBRSTW<'a> {
    w: &'a mut W,
}
impl<'a> _AHBBRSTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: AHBBRSTW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Incremental burst of unspecified length only"]
    #[inline]
    pub fn ahbbrst_0(self) -> &'a mut W {
        self.variant(AHBBRSTW::AHBBRST_0)
    }
    #[doc = "INCR4 burst, then single transfer"]
    #[inline]
    pub fn ahbbrst_1(self) -> &'a mut W {
        self.variant(AHBBRSTW::AHBBRST_1)
    }
    #[doc = "INCR8 burst, INCR4 burst, then single transfer"]
    #[inline]
    pub fn ahbbrst_2(self) -> &'a mut W {
        self.variant(AHBBRSTW::AHBBRST_2)
    }
    #[doc = "INCR16 burst, INCR8 burst, INCR4 burst, then single transfer"]
    #[inline]
    pub fn ahbbrst_3(self) -> &'a mut W {
        self.variant(AHBBRSTW::AHBBRST_3)
    }
    #[doc = "INCR4 burst, then incremental burst of unspecified length"]
    #[inline]
    pub fn ahbbrst_5(self) -> &'a mut W {
        self.variant(AHBBRSTW::AHBBRST_5)
    }
    #[doc = "INCR8 burst, INCR4 burst, then incremental burst of unspecified length"]
    #[inline]
    pub fn ahbbrst_6(self) -> &'a mut W {
        self.variant(AHBBRSTW::AHBBRST_6)
    }
    #[doc = "INCR16 burst, INCR8 burst, INCR4 burst, then incremental burst of unspecified length"]
    #[inline]
    pub fn ahbbrst_7(self) -> &'a mut W {
        self.variant(AHBBRSTW::AHBBRST_7)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 0;
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
    #[doc = "Bits 0:2 - AHB master interface Burst configuration These bits control AHB master transfer type sequence (or priority)"]
    #[inline]
    pub fn ahbbrst(&self) -> AHBBRSTR {
        AHBBRSTR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 2 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:2 - AHB master interface Burst configuration These bits control AHB master transfer type sequence (or priority)"]
    #[inline]
    pub fn ahbbrst(&mut self) -> _AHBBRSTW {
        _AHBBRSTW { w: self }
    }
}
