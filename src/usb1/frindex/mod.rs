#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::FRINDEX {
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
#[doc = "Possible values of the field `FRINDEX`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FRINDEXR {
    #[doc = "(1024) 12"]
    FRINDEX_0,
    #[doc = "(512) 11"]
    FRINDEX_1,
    #[doc = "(256) 10"]
    FRINDEX_2,
    #[doc = "(128) 9"]
    FRINDEX_3,
    #[doc = "(64) 8"]
    FRINDEX_4,
    #[doc = "(32) 7"]
    FRINDEX_5,
    #[doc = "(16) 6"]
    FRINDEX_6,
    #[doc = "(8) 5"]
    FRINDEX_7,
    #[doc = r" Reserved"]
    _Reserved(u16),
}
impl FRINDEXR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        match *self {
            FRINDEXR::FRINDEX_0 => 0,
            FRINDEXR::FRINDEX_1 => 1,
            FRINDEXR::FRINDEX_2 => 2,
            FRINDEXR::FRINDEX_3 => 3,
            FRINDEXR::FRINDEX_4 => 4,
            FRINDEXR::FRINDEX_5 => 5,
            FRINDEXR::FRINDEX_6 => 6,
            FRINDEXR::FRINDEX_7 => 7,
            FRINDEXR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u16) -> FRINDEXR {
        match value {
            0 => FRINDEXR::FRINDEX_0,
            1 => FRINDEXR::FRINDEX_1,
            2 => FRINDEXR::FRINDEX_2,
            3 => FRINDEXR::FRINDEX_3,
            4 => FRINDEXR::FRINDEX_4,
            5 => FRINDEXR::FRINDEX_5,
            6 => FRINDEXR::FRINDEX_6,
            7 => FRINDEXR::FRINDEX_7,
            i => FRINDEXR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `FRINDEX_0`"]
    #[inline]
    pub fn is_frindex_0(&self) -> bool {
        *self == FRINDEXR::FRINDEX_0
    }
    #[doc = "Checks if the value of the field is `FRINDEX_1`"]
    #[inline]
    pub fn is_frindex_1(&self) -> bool {
        *self == FRINDEXR::FRINDEX_1
    }
    #[doc = "Checks if the value of the field is `FRINDEX_2`"]
    #[inline]
    pub fn is_frindex_2(&self) -> bool {
        *self == FRINDEXR::FRINDEX_2
    }
    #[doc = "Checks if the value of the field is `FRINDEX_3`"]
    #[inline]
    pub fn is_frindex_3(&self) -> bool {
        *self == FRINDEXR::FRINDEX_3
    }
    #[doc = "Checks if the value of the field is `FRINDEX_4`"]
    #[inline]
    pub fn is_frindex_4(&self) -> bool {
        *self == FRINDEXR::FRINDEX_4
    }
    #[doc = "Checks if the value of the field is `FRINDEX_5`"]
    #[inline]
    pub fn is_frindex_5(&self) -> bool {
        *self == FRINDEXR::FRINDEX_5
    }
    #[doc = "Checks if the value of the field is `FRINDEX_6`"]
    #[inline]
    pub fn is_frindex_6(&self) -> bool {
        *self == FRINDEXR::FRINDEX_6
    }
    #[doc = "Checks if the value of the field is `FRINDEX_7`"]
    #[inline]
    pub fn is_frindex_7(&self) -> bool {
        *self == FRINDEXR::FRINDEX_7
    }
}
#[doc = "Values that can be written to the field `FRINDEX`"]
pub enum FRINDEXW {
    #[doc = "(1024) 12"]
    FRINDEX_0,
    #[doc = "(512) 11"]
    FRINDEX_1,
    #[doc = "(256) 10"]
    FRINDEX_2,
    #[doc = "(128) 9"]
    FRINDEX_3,
    #[doc = "(64) 8"]
    FRINDEX_4,
    #[doc = "(32) 7"]
    FRINDEX_5,
    #[doc = "(16) 6"]
    FRINDEX_6,
    #[doc = "(8) 5"]
    FRINDEX_7,
}
impl FRINDEXW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u16 {
        match *self {
            FRINDEXW::FRINDEX_0 => 0,
            FRINDEXW::FRINDEX_1 => 1,
            FRINDEXW::FRINDEX_2 => 2,
            FRINDEXW::FRINDEX_3 => 3,
            FRINDEXW::FRINDEX_4 => 4,
            FRINDEXW::FRINDEX_5 => 5,
            FRINDEXW::FRINDEX_6 => 6,
            FRINDEXW::FRINDEX_7 => 7,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FRINDEXW<'a> {
    w: &'a mut W,
}
impl<'a> _FRINDEXW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FRINDEXW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "(1024) 12"]
    #[inline]
    pub fn frindex_0(self) -> &'a mut W {
        self.variant(FRINDEXW::FRINDEX_0)
    }
    #[doc = "(512) 11"]
    #[inline]
    pub fn frindex_1(self) -> &'a mut W {
        self.variant(FRINDEXW::FRINDEX_1)
    }
    #[doc = "(256) 10"]
    #[inline]
    pub fn frindex_2(self) -> &'a mut W {
        self.variant(FRINDEXW::FRINDEX_2)
    }
    #[doc = "(128) 9"]
    #[inline]
    pub fn frindex_3(self) -> &'a mut W {
        self.variant(FRINDEXW::FRINDEX_3)
    }
    #[doc = "(64) 8"]
    #[inline]
    pub fn frindex_4(self) -> &'a mut W {
        self.variant(FRINDEXW::FRINDEX_4)
    }
    #[doc = "(32) 7"]
    #[inline]
    pub fn frindex_5(self) -> &'a mut W {
        self.variant(FRINDEXW::FRINDEX_5)
    }
    #[doc = "(16) 6"]
    #[inline]
    pub fn frindex_6(self) -> &'a mut W {
        self.variant(FRINDEXW::FRINDEX_6)
    }
    #[doc = "(8) 5"]
    #[inline]
    pub fn frindex_7(self) -> &'a mut W {
        self.variant(FRINDEXW::FRINDEX_7)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        const MASK: u16 = 16383;
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
    #[doc = "Bits 0:13 - Frame Index"]
    #[inline]
    pub fn frindex(&self) -> FRINDEXR {
        FRINDEXR::_from({
            const MASK: u16 = 16383;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u16
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
    #[doc = "Bits 0:13 - Frame Index"]
    #[inline]
    pub fn frindex(&mut self) -> _FRINDEXW {
        _FRINDEXW { w: self }
    }
}
