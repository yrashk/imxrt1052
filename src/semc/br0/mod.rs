#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::BR0 {
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
pub struct VLDR {
    bits: bool,
}
impl VLDR {
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
#[doc = "Possible values of the field `MS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MSR {
    #[doc = "4KB"]
    MS_0,
    #[doc = "8KB"]
    MS_1,
    #[doc = "16KB"]
    MS_2,
    #[doc = "32KB"]
    MS_3,
    #[doc = "64KB"]
    MS_4,
    #[doc = "128KB"]
    MS_5,
    #[doc = "256KB"]
    MS_6,
    #[doc = "512KB"]
    MS_7,
    #[doc = "1MB"]
    MS_8,
    #[doc = "2MB"]
    MS_9,
    #[doc = "4MB"]
    MS_10,
    #[doc = "8MB"]
    MS_11,
    #[doc = "16MB"]
    MS_12,
    #[doc = "32MB"]
    MS_13,
    #[doc = "64MB"]
    MS_14,
    #[doc = "128MB"]
    MS_15,
    #[doc = "256MB"]
    MS_16,
    #[doc = "512MB"]
    MS_17,
    #[doc = "1GB"]
    MS_18,
    #[doc = "2GB"]
    MS_19,
    #[doc = "4GB"]
    MS_20,
    #[doc = "4GB"]
    MS_21,
    #[doc = "4GB"]
    MS_22,
    #[doc = "4GB"]
    MS_23,
    #[doc = "4GB"]
    MS_24,
    #[doc = "4GB"]
    MS_25,
    #[doc = "4GB"]
    MS_26,
    #[doc = "4GB"]
    MS_27,
    #[doc = "4GB"]
    MS_28,
    #[doc = "4GB"]
    MS_29,
    #[doc = "4GB"]
    MS_30,
    #[doc = "4GB"]
    MS_31,
}
impl MSR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            MSR::MS_0 => 0,
            MSR::MS_1 => 1,
            MSR::MS_2 => 2,
            MSR::MS_3 => 3,
            MSR::MS_4 => 4,
            MSR::MS_5 => 5,
            MSR::MS_6 => 6,
            MSR::MS_7 => 7,
            MSR::MS_8 => 8,
            MSR::MS_9 => 9,
            MSR::MS_10 => 10,
            MSR::MS_11 => 11,
            MSR::MS_12 => 12,
            MSR::MS_13 => 13,
            MSR::MS_14 => 14,
            MSR::MS_15 => 15,
            MSR::MS_16 => 16,
            MSR::MS_17 => 17,
            MSR::MS_18 => 18,
            MSR::MS_19 => 19,
            MSR::MS_20 => 20,
            MSR::MS_21 => 21,
            MSR::MS_22 => 22,
            MSR::MS_23 => 23,
            MSR::MS_24 => 24,
            MSR::MS_25 => 25,
            MSR::MS_26 => 26,
            MSR::MS_27 => 27,
            MSR::MS_28 => 28,
            MSR::MS_29 => 29,
            MSR::MS_30 => 30,
            MSR::MS_31 => 31,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> MSR {
        match value {
            0 => MSR::MS_0,
            1 => MSR::MS_1,
            2 => MSR::MS_2,
            3 => MSR::MS_3,
            4 => MSR::MS_4,
            5 => MSR::MS_5,
            6 => MSR::MS_6,
            7 => MSR::MS_7,
            8 => MSR::MS_8,
            9 => MSR::MS_9,
            10 => MSR::MS_10,
            11 => MSR::MS_11,
            12 => MSR::MS_12,
            13 => MSR::MS_13,
            14 => MSR::MS_14,
            15 => MSR::MS_15,
            16 => MSR::MS_16,
            17 => MSR::MS_17,
            18 => MSR::MS_18,
            19 => MSR::MS_19,
            20 => MSR::MS_20,
            21 => MSR::MS_21,
            22 => MSR::MS_22,
            23 => MSR::MS_23,
            24 => MSR::MS_24,
            25 => MSR::MS_25,
            26 => MSR::MS_26,
            27 => MSR::MS_27,
            28 => MSR::MS_28,
            29 => MSR::MS_29,
            30 => MSR::MS_30,
            31 => MSR::MS_31,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `MS_0`"]
    #[inline]
    pub fn is_ms_0(&self) -> bool {
        *self == MSR::MS_0
    }
    #[doc = "Checks if the value of the field is `MS_1`"]
    #[inline]
    pub fn is_ms_1(&self) -> bool {
        *self == MSR::MS_1
    }
    #[doc = "Checks if the value of the field is `MS_2`"]
    #[inline]
    pub fn is_ms_2(&self) -> bool {
        *self == MSR::MS_2
    }
    #[doc = "Checks if the value of the field is `MS_3`"]
    #[inline]
    pub fn is_ms_3(&self) -> bool {
        *self == MSR::MS_3
    }
    #[doc = "Checks if the value of the field is `MS_4`"]
    #[inline]
    pub fn is_ms_4(&self) -> bool {
        *self == MSR::MS_4
    }
    #[doc = "Checks if the value of the field is `MS_5`"]
    #[inline]
    pub fn is_ms_5(&self) -> bool {
        *self == MSR::MS_5
    }
    #[doc = "Checks if the value of the field is `MS_6`"]
    #[inline]
    pub fn is_ms_6(&self) -> bool {
        *self == MSR::MS_6
    }
    #[doc = "Checks if the value of the field is `MS_7`"]
    #[inline]
    pub fn is_ms_7(&self) -> bool {
        *self == MSR::MS_7
    }
    #[doc = "Checks if the value of the field is `MS_8`"]
    #[inline]
    pub fn is_ms_8(&self) -> bool {
        *self == MSR::MS_8
    }
    #[doc = "Checks if the value of the field is `MS_9`"]
    #[inline]
    pub fn is_ms_9(&self) -> bool {
        *self == MSR::MS_9
    }
    #[doc = "Checks if the value of the field is `MS_10`"]
    #[inline]
    pub fn is_ms_10(&self) -> bool {
        *self == MSR::MS_10
    }
    #[doc = "Checks if the value of the field is `MS_11`"]
    #[inline]
    pub fn is_ms_11(&self) -> bool {
        *self == MSR::MS_11
    }
    #[doc = "Checks if the value of the field is `MS_12`"]
    #[inline]
    pub fn is_ms_12(&self) -> bool {
        *self == MSR::MS_12
    }
    #[doc = "Checks if the value of the field is `MS_13`"]
    #[inline]
    pub fn is_ms_13(&self) -> bool {
        *self == MSR::MS_13
    }
    #[doc = "Checks if the value of the field is `MS_14`"]
    #[inline]
    pub fn is_ms_14(&self) -> bool {
        *self == MSR::MS_14
    }
    #[doc = "Checks if the value of the field is `MS_15`"]
    #[inline]
    pub fn is_ms_15(&self) -> bool {
        *self == MSR::MS_15
    }
    #[doc = "Checks if the value of the field is `MS_16`"]
    #[inline]
    pub fn is_ms_16(&self) -> bool {
        *self == MSR::MS_16
    }
    #[doc = "Checks if the value of the field is `MS_17`"]
    #[inline]
    pub fn is_ms_17(&self) -> bool {
        *self == MSR::MS_17
    }
    #[doc = "Checks if the value of the field is `MS_18`"]
    #[inline]
    pub fn is_ms_18(&self) -> bool {
        *self == MSR::MS_18
    }
    #[doc = "Checks if the value of the field is `MS_19`"]
    #[inline]
    pub fn is_ms_19(&self) -> bool {
        *self == MSR::MS_19
    }
    #[doc = "Checks if the value of the field is `MS_20`"]
    #[inline]
    pub fn is_ms_20(&self) -> bool {
        *self == MSR::MS_20
    }
    #[doc = "Checks if the value of the field is `MS_21`"]
    #[inline]
    pub fn is_ms_21(&self) -> bool {
        *self == MSR::MS_21
    }
    #[doc = "Checks if the value of the field is `MS_22`"]
    #[inline]
    pub fn is_ms_22(&self) -> bool {
        *self == MSR::MS_22
    }
    #[doc = "Checks if the value of the field is `MS_23`"]
    #[inline]
    pub fn is_ms_23(&self) -> bool {
        *self == MSR::MS_23
    }
    #[doc = "Checks if the value of the field is `MS_24`"]
    #[inline]
    pub fn is_ms_24(&self) -> bool {
        *self == MSR::MS_24
    }
    #[doc = "Checks if the value of the field is `MS_25`"]
    #[inline]
    pub fn is_ms_25(&self) -> bool {
        *self == MSR::MS_25
    }
    #[doc = "Checks if the value of the field is `MS_26`"]
    #[inline]
    pub fn is_ms_26(&self) -> bool {
        *self == MSR::MS_26
    }
    #[doc = "Checks if the value of the field is `MS_27`"]
    #[inline]
    pub fn is_ms_27(&self) -> bool {
        *self == MSR::MS_27
    }
    #[doc = "Checks if the value of the field is `MS_28`"]
    #[inline]
    pub fn is_ms_28(&self) -> bool {
        *self == MSR::MS_28
    }
    #[doc = "Checks if the value of the field is `MS_29`"]
    #[inline]
    pub fn is_ms_29(&self) -> bool {
        *self == MSR::MS_29
    }
    #[doc = "Checks if the value of the field is `MS_30`"]
    #[inline]
    pub fn is_ms_30(&self) -> bool {
        *self == MSR::MS_30
    }
    #[doc = "Checks if the value of the field is `MS_31`"]
    #[inline]
    pub fn is_ms_31(&self) -> bool {
        *self == MSR::MS_31
    }
}
#[doc = r" Value of the field"]
pub struct BAR {
    bits: u32,
}
impl BAR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
}
#[doc = r" Proxy"]
pub struct _VLDW<'a> {
    w: &'a mut W,
}
impl<'a> _VLDW<'a> {
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
#[doc = "Values that can be written to the field `MS`"]
pub enum MSW {
    #[doc = "4KB"]
    MS_0,
    #[doc = "8KB"]
    MS_1,
    #[doc = "16KB"]
    MS_2,
    #[doc = "32KB"]
    MS_3,
    #[doc = "64KB"]
    MS_4,
    #[doc = "128KB"]
    MS_5,
    #[doc = "256KB"]
    MS_6,
    #[doc = "512KB"]
    MS_7,
    #[doc = "1MB"]
    MS_8,
    #[doc = "2MB"]
    MS_9,
    #[doc = "4MB"]
    MS_10,
    #[doc = "8MB"]
    MS_11,
    #[doc = "16MB"]
    MS_12,
    #[doc = "32MB"]
    MS_13,
    #[doc = "64MB"]
    MS_14,
    #[doc = "128MB"]
    MS_15,
    #[doc = "256MB"]
    MS_16,
    #[doc = "512MB"]
    MS_17,
    #[doc = "1GB"]
    MS_18,
    #[doc = "2GB"]
    MS_19,
    #[doc = "4GB"]
    MS_20,
    #[doc = "4GB"]
    MS_21,
    #[doc = "4GB"]
    MS_22,
    #[doc = "4GB"]
    MS_23,
    #[doc = "4GB"]
    MS_24,
    #[doc = "4GB"]
    MS_25,
    #[doc = "4GB"]
    MS_26,
    #[doc = "4GB"]
    MS_27,
    #[doc = "4GB"]
    MS_28,
    #[doc = "4GB"]
    MS_29,
    #[doc = "4GB"]
    MS_30,
    #[doc = "4GB"]
    MS_31,
}
impl MSW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            MSW::MS_0 => 0,
            MSW::MS_1 => 1,
            MSW::MS_2 => 2,
            MSW::MS_3 => 3,
            MSW::MS_4 => 4,
            MSW::MS_5 => 5,
            MSW::MS_6 => 6,
            MSW::MS_7 => 7,
            MSW::MS_8 => 8,
            MSW::MS_9 => 9,
            MSW::MS_10 => 10,
            MSW::MS_11 => 11,
            MSW::MS_12 => 12,
            MSW::MS_13 => 13,
            MSW::MS_14 => 14,
            MSW::MS_15 => 15,
            MSW::MS_16 => 16,
            MSW::MS_17 => 17,
            MSW::MS_18 => 18,
            MSW::MS_19 => 19,
            MSW::MS_20 => 20,
            MSW::MS_21 => 21,
            MSW::MS_22 => 22,
            MSW::MS_23 => 23,
            MSW::MS_24 => 24,
            MSW::MS_25 => 25,
            MSW::MS_26 => 26,
            MSW::MS_27 => 27,
            MSW::MS_28 => 28,
            MSW::MS_29 => 29,
            MSW::MS_30 => 30,
            MSW::MS_31 => 31,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MSW<'a> {
    w: &'a mut W,
}
impl<'a> _MSW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MSW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "4KB"]
    #[inline]
    pub fn ms_0(self) -> &'a mut W {
        self.variant(MSW::MS_0)
    }
    #[doc = "8KB"]
    #[inline]
    pub fn ms_1(self) -> &'a mut W {
        self.variant(MSW::MS_1)
    }
    #[doc = "16KB"]
    #[inline]
    pub fn ms_2(self) -> &'a mut W {
        self.variant(MSW::MS_2)
    }
    #[doc = "32KB"]
    #[inline]
    pub fn ms_3(self) -> &'a mut W {
        self.variant(MSW::MS_3)
    }
    #[doc = "64KB"]
    #[inline]
    pub fn ms_4(self) -> &'a mut W {
        self.variant(MSW::MS_4)
    }
    #[doc = "128KB"]
    #[inline]
    pub fn ms_5(self) -> &'a mut W {
        self.variant(MSW::MS_5)
    }
    #[doc = "256KB"]
    #[inline]
    pub fn ms_6(self) -> &'a mut W {
        self.variant(MSW::MS_6)
    }
    #[doc = "512KB"]
    #[inline]
    pub fn ms_7(self) -> &'a mut W {
        self.variant(MSW::MS_7)
    }
    #[doc = "1MB"]
    #[inline]
    pub fn ms_8(self) -> &'a mut W {
        self.variant(MSW::MS_8)
    }
    #[doc = "2MB"]
    #[inline]
    pub fn ms_9(self) -> &'a mut W {
        self.variant(MSW::MS_9)
    }
    #[doc = "4MB"]
    #[inline]
    pub fn ms_10(self) -> &'a mut W {
        self.variant(MSW::MS_10)
    }
    #[doc = "8MB"]
    #[inline]
    pub fn ms_11(self) -> &'a mut W {
        self.variant(MSW::MS_11)
    }
    #[doc = "16MB"]
    #[inline]
    pub fn ms_12(self) -> &'a mut W {
        self.variant(MSW::MS_12)
    }
    #[doc = "32MB"]
    #[inline]
    pub fn ms_13(self) -> &'a mut W {
        self.variant(MSW::MS_13)
    }
    #[doc = "64MB"]
    #[inline]
    pub fn ms_14(self) -> &'a mut W {
        self.variant(MSW::MS_14)
    }
    #[doc = "128MB"]
    #[inline]
    pub fn ms_15(self) -> &'a mut W {
        self.variant(MSW::MS_15)
    }
    #[doc = "256MB"]
    #[inline]
    pub fn ms_16(self) -> &'a mut W {
        self.variant(MSW::MS_16)
    }
    #[doc = "512MB"]
    #[inline]
    pub fn ms_17(self) -> &'a mut W {
        self.variant(MSW::MS_17)
    }
    #[doc = "1GB"]
    #[inline]
    pub fn ms_18(self) -> &'a mut W {
        self.variant(MSW::MS_18)
    }
    #[doc = "2GB"]
    #[inline]
    pub fn ms_19(self) -> &'a mut W {
        self.variant(MSW::MS_19)
    }
    #[doc = "4GB"]
    #[inline]
    pub fn ms_20(self) -> &'a mut W {
        self.variant(MSW::MS_20)
    }
    #[doc = "4GB"]
    #[inline]
    pub fn ms_21(self) -> &'a mut W {
        self.variant(MSW::MS_21)
    }
    #[doc = "4GB"]
    #[inline]
    pub fn ms_22(self) -> &'a mut W {
        self.variant(MSW::MS_22)
    }
    #[doc = "4GB"]
    #[inline]
    pub fn ms_23(self) -> &'a mut W {
        self.variant(MSW::MS_23)
    }
    #[doc = "4GB"]
    #[inline]
    pub fn ms_24(self) -> &'a mut W {
        self.variant(MSW::MS_24)
    }
    #[doc = "4GB"]
    #[inline]
    pub fn ms_25(self) -> &'a mut W {
        self.variant(MSW::MS_25)
    }
    #[doc = "4GB"]
    #[inline]
    pub fn ms_26(self) -> &'a mut W {
        self.variant(MSW::MS_26)
    }
    #[doc = "4GB"]
    #[inline]
    pub fn ms_27(self) -> &'a mut W {
        self.variant(MSW::MS_27)
    }
    #[doc = "4GB"]
    #[inline]
    pub fn ms_28(self) -> &'a mut W {
        self.variant(MSW::MS_28)
    }
    #[doc = "4GB"]
    #[inline]
    pub fn ms_29(self) -> &'a mut W {
        self.variant(MSW::MS_29)
    }
    #[doc = "4GB"]
    #[inline]
    pub fn ms_30(self) -> &'a mut W {
        self.variant(MSW::MS_30)
    }
    #[doc = "4GB"]
    #[inline]
    pub fn ms_31(self) -> &'a mut W {
        self.variant(MSW::MS_31)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 31;
        const OFFSET: u8 = 1;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _BAW<'a> {
    w: &'a mut W,
}
impl<'a> _BAW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        const MASK: u32 = 1048575;
        const OFFSET: u8 = 12;
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
    #[doc = "Bit 0 - Valid"]
    #[inline]
    pub fn vld(&self) -> VLDR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        VLDR { bits }
    }
    #[doc = "Bits 1:5 - Memory size"]
    #[inline]
    pub fn ms(&self) -> MSR {
        MSR::_from({
            const MASK: u8 = 31;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 12:31 - Base Address"]
    #[inline]
    pub fn ba(&self) -> BAR {
        let bits = {
            const MASK: u32 = 1048575;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) as u32
        };
        BAR { bits }
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 2147483677 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - Valid"]
    #[inline]
    pub fn vld(&mut self) -> _VLDW {
        _VLDW { w: self }
    }
    #[doc = "Bits 1:5 - Memory size"]
    #[inline]
    pub fn ms(&mut self) -> _MSW {
        _MSW { w: self }
    }
    #[doc = "Bits 12:31 - Base Address"]
    #[inline]
    pub fn ba(&mut self) -> _BAW {
        _BAW { w: self }
    }
}
