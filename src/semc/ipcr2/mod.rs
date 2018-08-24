#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::IPCR2 {
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
#[doc = "Possible values of the field `BM0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BM0R {
    #[doc = "Byte Unmasked"]
    BM0_0,
    #[doc = "Byte Masked"]
    BM0_1,
}
impl BM0R {
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
            BM0R::BM0_0 => false,
            BM0R::BM0_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> BM0R {
        match value {
            false => BM0R::BM0_0,
            true => BM0R::BM0_1,
        }
    }
    #[doc = "Checks if the value of the field is `BM0_0`"]
    #[inline]
    pub fn is_bm0_0(&self) -> bool {
        *self == BM0R::BM0_0
    }
    #[doc = "Checks if the value of the field is `BM0_1`"]
    #[inline]
    pub fn is_bm0_1(&self) -> bool {
        *self == BM0R::BM0_1
    }
}
#[doc = "Possible values of the field `BM1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BM1R {
    #[doc = "Byte Unmasked"]
    BM1_0,
    #[doc = "Byte Masked"]
    BM1_1,
}
impl BM1R {
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
            BM1R::BM1_0 => false,
            BM1R::BM1_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> BM1R {
        match value {
            false => BM1R::BM1_0,
            true => BM1R::BM1_1,
        }
    }
    #[doc = "Checks if the value of the field is `BM1_0`"]
    #[inline]
    pub fn is_bm1_0(&self) -> bool {
        *self == BM1R::BM1_0
    }
    #[doc = "Checks if the value of the field is `BM1_1`"]
    #[inline]
    pub fn is_bm1_1(&self) -> bool {
        *self == BM1R::BM1_1
    }
}
#[doc = "Possible values of the field `BM2`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BM2R {
    #[doc = "Byte Unmasked"]
    BM2_0,
    #[doc = "Byte Masked"]
    BM2_1,
}
impl BM2R {
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
            BM2R::BM2_0 => false,
            BM2R::BM2_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> BM2R {
        match value {
            false => BM2R::BM2_0,
            true => BM2R::BM2_1,
        }
    }
    #[doc = "Checks if the value of the field is `BM2_0`"]
    #[inline]
    pub fn is_bm2_0(&self) -> bool {
        *self == BM2R::BM2_0
    }
    #[doc = "Checks if the value of the field is `BM2_1`"]
    #[inline]
    pub fn is_bm2_1(&self) -> bool {
        *self == BM2R::BM2_1
    }
}
#[doc = "Possible values of the field `BM3`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BM3R {
    #[doc = "Byte Unmasked"]
    BM3_0,
    #[doc = "Byte Masked"]
    BM3_1,
}
impl BM3R {
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
            BM3R::BM3_0 => false,
            BM3R::BM3_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> BM3R {
        match value {
            false => BM3R::BM3_0,
            true => BM3R::BM3_1,
        }
    }
    #[doc = "Checks if the value of the field is `BM3_0`"]
    #[inline]
    pub fn is_bm3_0(&self) -> bool {
        *self == BM3R::BM3_0
    }
    #[doc = "Checks if the value of the field is `BM3_1`"]
    #[inline]
    pub fn is_bm3_1(&self) -> bool {
        *self == BM3R::BM3_1
    }
}
#[doc = "Values that can be written to the field `BM0`"]
pub enum BM0W {
    #[doc = "Byte Unmasked"]
    BM0_0,
    #[doc = "Byte Masked"]
    BM0_1,
}
impl BM0W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            BM0W::BM0_0 => false,
            BM0W::BM0_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _BM0W<'a> {
    w: &'a mut W,
}
impl<'a> _BM0W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: BM0W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Byte Unmasked"]
    #[inline]
    pub fn bm0_0(self) -> &'a mut W {
        self.variant(BM0W::BM0_0)
    }
    #[doc = "Byte Masked"]
    #[inline]
    pub fn bm0_1(self) -> &'a mut W {
        self.variant(BM0W::BM0_1)
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
#[doc = "Values that can be written to the field `BM1`"]
pub enum BM1W {
    #[doc = "Byte Unmasked"]
    BM1_0,
    #[doc = "Byte Masked"]
    BM1_1,
}
impl BM1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            BM1W::BM1_0 => false,
            BM1W::BM1_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _BM1W<'a> {
    w: &'a mut W,
}
impl<'a> _BM1W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: BM1W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Byte Unmasked"]
    #[inline]
    pub fn bm1_0(self) -> &'a mut W {
        self.variant(BM1W::BM1_0)
    }
    #[doc = "Byte Masked"]
    #[inline]
    pub fn bm1_1(self) -> &'a mut W {
        self.variant(BM1W::BM1_1)
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
        const OFFSET: u8 = 1;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `BM2`"]
pub enum BM2W {
    #[doc = "Byte Unmasked"]
    BM2_0,
    #[doc = "Byte Masked"]
    BM2_1,
}
impl BM2W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            BM2W::BM2_0 => false,
            BM2W::BM2_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _BM2W<'a> {
    w: &'a mut W,
}
impl<'a> _BM2W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: BM2W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Byte Unmasked"]
    #[inline]
    pub fn bm2_0(self) -> &'a mut W {
        self.variant(BM2W::BM2_0)
    }
    #[doc = "Byte Masked"]
    #[inline]
    pub fn bm2_1(self) -> &'a mut W {
        self.variant(BM2W::BM2_1)
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
        const OFFSET: u8 = 2;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `BM3`"]
pub enum BM3W {
    #[doc = "Byte Unmasked"]
    BM3_0,
    #[doc = "Byte Masked"]
    BM3_1,
}
impl BM3W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            BM3W::BM3_0 => false,
            BM3W::BM3_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _BM3W<'a> {
    w: &'a mut W,
}
impl<'a> _BM3W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: BM3W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Byte Unmasked"]
    #[inline]
    pub fn bm3_0(self) -> &'a mut W {
        self.variant(BM3W::BM3_0)
    }
    #[doc = "Byte Masked"]
    #[inline]
    pub fn bm3_1(self) -> &'a mut W {
        self.variant(BM3W::BM3_1)
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
        const OFFSET: u8 = 3;
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
    #[doc = "Bit 0 - Byte Mask for Byte 0 (IPTXD bit 7:0)"]
    #[inline]
    pub fn bm0(&self) -> BM0R {
        BM0R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - Byte Mask for Byte 1 (IPTXD bit 15:8)"]
    #[inline]
    pub fn bm1(&self) -> BM1R {
        BM1R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - Byte Mask for Byte 2 (IPTXD bit 23:16)"]
    #[inline]
    pub fn bm2(&self) -> BM2R {
        BM2R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - Byte Mask for Byte 3 (IPTXD bit 31:24)"]
    #[inline]
    pub fn bm3(&self) -> BM3R {
        BM3R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
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
    #[doc = "Bit 0 - Byte Mask for Byte 0 (IPTXD bit 7:0)"]
    #[inline]
    pub fn bm0(&mut self) -> _BM0W {
        _BM0W { w: self }
    }
    #[doc = "Bit 1 - Byte Mask for Byte 1 (IPTXD bit 15:8)"]
    #[inline]
    pub fn bm1(&mut self) -> _BM1W {
        _BM1W { w: self }
    }
    #[doc = "Bit 2 - Byte Mask for Byte 2 (IPTXD bit 23:16)"]
    #[inline]
    pub fn bm2(&mut self) -> _BM2W {
        _BM2W { w: self }
    }
    #[doc = "Bit 3 - Byte Mask for Byte 3 (IPTXD bit 31:24)"]
    #[inline]
    pub fn bm3(&mut self) -> _BM3W {
        _BM3W { w: self }
    }
}
