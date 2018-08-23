#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::BLK_ATT {
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
#[doc = "Possible values of the field `BLKSIZE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BLKSIZER {
    #[doc = "No data transfer"]
    BLKSIZE_0,
    #[doc = "1 Byte"]
    BLKSIZE_1,
    #[doc = "2 Bytes"]
    BLKSIZE_2,
    #[doc = "3 Bytes"]
    BLKSIZE_3,
    #[doc = "4 Bytes"]
    BLKSIZE_4,
    #[doc = "511 Bytes"]
    BLKSIZE_511,
    #[doc = "512 Bytes"]
    BLKSIZE_512,
    #[doc = "2048 Bytes"]
    BLKSIZE_2048,
    #[doc = "4096 Bytes"]
    BLKSIZE_4096,
    #[doc = r" Reserved"]
    _Reserved(u16),
}
impl BLKSIZER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        match *self {
            BLKSIZER::BLKSIZE_0 => 0,
            BLKSIZER::BLKSIZE_1 => 1,
            BLKSIZER::BLKSIZE_2 => 2,
            BLKSIZER::BLKSIZE_3 => 3,
            BLKSIZER::BLKSIZE_4 => 4,
            BLKSIZER::BLKSIZE_511 => 511,
            BLKSIZER::BLKSIZE_512 => 512,
            BLKSIZER::BLKSIZE_2048 => 2048,
            BLKSIZER::BLKSIZE_4096 => 4096,
            BLKSIZER::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u16) -> BLKSIZER {
        match value {
            0 => BLKSIZER::BLKSIZE_0,
            1 => BLKSIZER::BLKSIZE_1,
            2 => BLKSIZER::BLKSIZE_2,
            3 => BLKSIZER::BLKSIZE_3,
            4 => BLKSIZER::BLKSIZE_4,
            511 => BLKSIZER::BLKSIZE_511,
            512 => BLKSIZER::BLKSIZE_512,
            2048 => BLKSIZER::BLKSIZE_2048,
            4096 => BLKSIZER::BLKSIZE_4096,
            i => BLKSIZER::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `BLKSIZE_0`"]
    #[inline]
    pub fn is_blksize_0(&self) -> bool {
        *self == BLKSIZER::BLKSIZE_0
    }
    #[doc = "Checks if the value of the field is `BLKSIZE_1`"]
    #[inline]
    pub fn is_blksize_1(&self) -> bool {
        *self == BLKSIZER::BLKSIZE_1
    }
    #[doc = "Checks if the value of the field is `BLKSIZE_2`"]
    #[inline]
    pub fn is_blksize_2(&self) -> bool {
        *self == BLKSIZER::BLKSIZE_2
    }
    #[doc = "Checks if the value of the field is `BLKSIZE_3`"]
    #[inline]
    pub fn is_blksize_3(&self) -> bool {
        *self == BLKSIZER::BLKSIZE_3
    }
    #[doc = "Checks if the value of the field is `BLKSIZE_4`"]
    #[inline]
    pub fn is_blksize_4(&self) -> bool {
        *self == BLKSIZER::BLKSIZE_4
    }
    #[doc = "Checks if the value of the field is `BLKSIZE_511`"]
    #[inline]
    pub fn is_blksize_511(&self) -> bool {
        *self == BLKSIZER::BLKSIZE_511
    }
    #[doc = "Checks if the value of the field is `BLKSIZE_512`"]
    #[inline]
    pub fn is_blksize_512(&self) -> bool {
        *self == BLKSIZER::BLKSIZE_512
    }
    #[doc = "Checks if the value of the field is `BLKSIZE_2048`"]
    #[inline]
    pub fn is_blksize_2048(&self) -> bool {
        *self == BLKSIZER::BLKSIZE_2048
    }
    #[doc = "Checks if the value of the field is `BLKSIZE_4096`"]
    #[inline]
    pub fn is_blksize_4096(&self) -> bool {
        *self == BLKSIZER::BLKSIZE_4096
    }
}
#[doc = "Possible values of the field `BLKCNT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BLKCNTR {
    #[doc = "Stop Count"]
    BLKCNT_0,
    #[doc = "1 block"]
    BLKCNT_1,
    #[doc = "2 blocks"]
    BLKCNT_2,
    #[doc = "65535 blocks"]
    BLKCNT_65535,
    #[doc = r" Reserved"]
    _Reserved(u16),
}
impl BLKCNTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        match *self {
            BLKCNTR::BLKCNT_0 => 0,
            BLKCNTR::BLKCNT_1 => 1,
            BLKCNTR::BLKCNT_2 => 2,
            BLKCNTR::BLKCNT_65535 => 65535,
            BLKCNTR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u16) -> BLKCNTR {
        match value {
            0 => BLKCNTR::BLKCNT_0,
            1 => BLKCNTR::BLKCNT_1,
            2 => BLKCNTR::BLKCNT_2,
            65535 => BLKCNTR::BLKCNT_65535,
            i => BLKCNTR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `BLKCNT_0`"]
    #[inline]
    pub fn is_blkcnt_0(&self) -> bool {
        *self == BLKCNTR::BLKCNT_0
    }
    #[doc = "Checks if the value of the field is `BLKCNT_1`"]
    #[inline]
    pub fn is_blkcnt_1(&self) -> bool {
        *self == BLKCNTR::BLKCNT_1
    }
    #[doc = "Checks if the value of the field is `BLKCNT_2`"]
    #[inline]
    pub fn is_blkcnt_2(&self) -> bool {
        *self == BLKCNTR::BLKCNT_2
    }
    #[doc = "Checks if the value of the field is `BLKCNT_65535`"]
    #[inline]
    pub fn is_blkcnt_65535(&self) -> bool {
        *self == BLKCNTR::BLKCNT_65535
    }
}
#[doc = "Values that can be written to the field `BLKSIZE`"]
pub enum BLKSIZEW {
    #[doc = "No data transfer"]
    BLKSIZE_0,
    #[doc = "1 Byte"]
    BLKSIZE_1,
    #[doc = "2 Bytes"]
    BLKSIZE_2,
    #[doc = "3 Bytes"]
    BLKSIZE_3,
    #[doc = "4 Bytes"]
    BLKSIZE_4,
    #[doc = "511 Bytes"]
    BLKSIZE_511,
    #[doc = "512 Bytes"]
    BLKSIZE_512,
    #[doc = "2048 Bytes"]
    BLKSIZE_2048,
    #[doc = "4096 Bytes"]
    BLKSIZE_4096,
}
impl BLKSIZEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u16 {
        match *self {
            BLKSIZEW::BLKSIZE_0 => 0,
            BLKSIZEW::BLKSIZE_1 => 1,
            BLKSIZEW::BLKSIZE_2 => 2,
            BLKSIZEW::BLKSIZE_3 => 3,
            BLKSIZEW::BLKSIZE_4 => 4,
            BLKSIZEW::BLKSIZE_511 => 511,
            BLKSIZEW::BLKSIZE_512 => 512,
            BLKSIZEW::BLKSIZE_2048 => 2048,
            BLKSIZEW::BLKSIZE_4096 => 4096,
        }
    }
}
#[doc = r" Proxy"]
pub struct _BLKSIZEW<'a> {
    w: &'a mut W,
}
impl<'a> _BLKSIZEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: BLKSIZEW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "No data transfer"]
    #[inline]
    pub fn blksize_0(self) -> &'a mut W {
        self.variant(BLKSIZEW::BLKSIZE_0)
    }
    #[doc = "1 Byte"]
    #[inline]
    pub fn blksize_1(self) -> &'a mut W {
        self.variant(BLKSIZEW::BLKSIZE_1)
    }
    #[doc = "2 Bytes"]
    #[inline]
    pub fn blksize_2(self) -> &'a mut W {
        self.variant(BLKSIZEW::BLKSIZE_2)
    }
    #[doc = "3 Bytes"]
    #[inline]
    pub fn blksize_3(self) -> &'a mut W {
        self.variant(BLKSIZEW::BLKSIZE_3)
    }
    #[doc = "4 Bytes"]
    #[inline]
    pub fn blksize_4(self) -> &'a mut W {
        self.variant(BLKSIZEW::BLKSIZE_4)
    }
    #[doc = "511 Bytes"]
    #[inline]
    pub fn blksize_511(self) -> &'a mut W {
        self.variant(BLKSIZEW::BLKSIZE_511)
    }
    #[doc = "512 Bytes"]
    #[inline]
    pub fn blksize_512(self) -> &'a mut W {
        self.variant(BLKSIZEW::BLKSIZE_512)
    }
    #[doc = "2048 Bytes"]
    #[inline]
    pub fn blksize_2048(self) -> &'a mut W {
        self.variant(BLKSIZEW::BLKSIZE_2048)
    }
    #[doc = "4096 Bytes"]
    #[inline]
    pub fn blksize_4096(self) -> &'a mut W {
        self.variant(BLKSIZEW::BLKSIZE_4096)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        const MASK: u16 = 8191;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `BLKCNT`"]
pub enum BLKCNTW {
    #[doc = "Stop Count"]
    BLKCNT_0,
    #[doc = "1 block"]
    BLKCNT_1,
    #[doc = "2 blocks"]
    BLKCNT_2,
    #[doc = "65535 blocks"]
    BLKCNT_65535,
}
impl BLKCNTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u16 {
        match *self {
            BLKCNTW::BLKCNT_0 => 0,
            BLKCNTW::BLKCNT_1 => 1,
            BLKCNTW::BLKCNT_2 => 2,
            BLKCNTW::BLKCNT_65535 => 65535,
        }
    }
}
#[doc = r" Proxy"]
pub struct _BLKCNTW<'a> {
    w: &'a mut W,
}
impl<'a> _BLKCNTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: BLKCNTW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Stop Count"]
    #[inline]
    pub fn blkcnt_0(self) -> &'a mut W {
        self.variant(BLKCNTW::BLKCNT_0)
    }
    #[doc = "1 block"]
    #[inline]
    pub fn blkcnt_1(self) -> &'a mut W {
        self.variant(BLKCNTW::BLKCNT_1)
    }
    #[doc = "2 blocks"]
    #[inline]
    pub fn blkcnt_2(self) -> &'a mut W {
        self.variant(BLKCNTW::BLKCNT_2)
    }
    #[doc = "65535 blocks"]
    #[inline]
    pub fn blkcnt_65535(self) -> &'a mut W {
        self.variant(BLKCNTW::BLKCNT_65535)
    }
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
    #[doc = "Bits 0:12 - Block Size"]
    #[inline]
    pub fn blksize(&self) -> BLKSIZER {
        BLKSIZER::_from({
            const MASK: u16 = 8191;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        })
    }
    #[doc = "Bits 16:31 - Block Count"]
    #[inline]
    pub fn blkcnt(&self) -> BLKCNTR {
        BLKCNTR::_from({
            const MASK: u16 = 65535;
            const OFFSET: u8 = 16;
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
    #[doc = "Bits 0:12 - Block Size"]
    #[inline]
    pub fn blksize(&mut self) -> _BLKSIZEW {
        _BLKSIZEW { w: self }
    }
    #[doc = "Bits 16:31 - Block Count"]
    #[inline]
    pub fn blkcnt(&mut self) -> _BLKCNTW {
        _BLKCNTW { w: self }
    }
}
