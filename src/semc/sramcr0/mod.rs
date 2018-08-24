#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::SRAMCR0 {
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
#[doc = "Possible values of the field `PS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PSR {
    #[doc = "8bit"]
    PS_0,
    #[doc = "16bit"]
    PS_1,
}
impl PSR {
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
            PSR::PS_0 => false,
            PSR::PS_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PSR {
        match value {
            false => PSR::PS_0,
            true => PSR::PS_1,
        }
    }
    #[doc = "Checks if the value of the field is `PS_0`"]
    #[inline]
    pub fn is_ps_0(&self) -> bool {
        *self == PSR::PS_0
    }
    #[doc = "Checks if the value of the field is `PS_1`"]
    #[inline]
    pub fn is_ps_1(&self) -> bool {
        *self == PSR::PS_1
    }
}
#[doc = "Possible values of the field `BL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BLR {
    #[doc = "1"]
    BL_0,
    #[doc = "2"]
    BL_1,
    #[doc = "4"]
    BL_2,
    #[doc = "8"]
    BL_3,
    #[doc = "16"]
    BL_4,
    #[doc = "32"]
    BL_5,
    #[doc = "64"]
    BL_6,
    #[doc = "64"]
    BL_7,
}
impl BLR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            BLR::BL_0 => 0,
            BLR::BL_1 => 1,
            BLR::BL_2 => 2,
            BLR::BL_3 => 3,
            BLR::BL_4 => 4,
            BLR::BL_5 => 5,
            BLR::BL_6 => 6,
            BLR::BL_7 => 7,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> BLR {
        match value {
            0 => BLR::BL_0,
            1 => BLR::BL_1,
            2 => BLR::BL_2,
            3 => BLR::BL_3,
            4 => BLR::BL_4,
            5 => BLR::BL_5,
            6 => BLR::BL_6,
            7 => BLR::BL_7,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `BL_0`"]
    #[inline]
    pub fn is_bl_0(&self) -> bool {
        *self == BLR::BL_0
    }
    #[doc = "Checks if the value of the field is `BL_1`"]
    #[inline]
    pub fn is_bl_1(&self) -> bool {
        *self == BLR::BL_1
    }
    #[doc = "Checks if the value of the field is `BL_2`"]
    #[inline]
    pub fn is_bl_2(&self) -> bool {
        *self == BLR::BL_2
    }
    #[doc = "Checks if the value of the field is `BL_3`"]
    #[inline]
    pub fn is_bl_3(&self) -> bool {
        *self == BLR::BL_3
    }
    #[doc = "Checks if the value of the field is `BL_4`"]
    #[inline]
    pub fn is_bl_4(&self) -> bool {
        *self == BLR::BL_4
    }
    #[doc = "Checks if the value of the field is `BL_5`"]
    #[inline]
    pub fn is_bl_5(&self) -> bool {
        *self == BLR::BL_5
    }
    #[doc = "Checks if the value of the field is `BL_6`"]
    #[inline]
    pub fn is_bl_6(&self) -> bool {
        *self == BLR::BL_6
    }
    #[doc = "Checks if the value of the field is `BL_7`"]
    #[inline]
    pub fn is_bl_7(&self) -> bool {
        *self == BLR::BL_7
    }
}
#[doc = "Possible values of the field `AM`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AMR {
    #[doc = "Address/Data MUX mode"]
    AM_0,
    #[doc = "Advanced Address/Data MUX mode"]
    AM_1,
    #[doc = "Address/Data non-MUX mode"]
    AM_2,
    #[doc = "Address/Data non-MUX mode"]
    AM_3,
}
impl AMR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            AMR::AM_0 => 0,
            AMR::AM_1 => 1,
            AMR::AM_2 => 2,
            AMR::AM_3 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> AMR {
        match value {
            0 => AMR::AM_0,
            1 => AMR::AM_1,
            2 => AMR::AM_2,
            3 => AMR::AM_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `AM_0`"]
    #[inline]
    pub fn is_am_0(&self) -> bool {
        *self == AMR::AM_0
    }
    #[doc = "Checks if the value of the field is `AM_1`"]
    #[inline]
    pub fn is_am_1(&self) -> bool {
        *self == AMR::AM_1
    }
    #[doc = "Checks if the value of the field is `AM_2`"]
    #[inline]
    pub fn is_am_2(&self) -> bool {
        *self == AMR::AM_2
    }
    #[doc = "Checks if the value of the field is `AM_3`"]
    #[inline]
    pub fn is_am_3(&self) -> bool {
        *self == AMR::AM_3
    }
}
#[doc = "Possible values of the field `ADVP`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADVPR {
    #[doc = "ADV# is Low Active. In ASYNC mode, device sample address with ADV# rise edge; In SYNC mode, device sample address when ADV# is LOW."]
    ADVP_0,
    #[doc = "ADV# is High Active. In ASYNC mode, device sample address with ADV# fall edge; In SYNC mode, device sample address when ADV# is HIGH."]
    ADVP_1,
}
impl ADVPR {
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
            ADVPR::ADVP_0 => false,
            ADVPR::ADVP_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ADVPR {
        match value {
            false => ADVPR::ADVP_0,
            true => ADVPR::ADVP_1,
        }
    }
    #[doc = "Checks if the value of the field is `ADVP_0`"]
    #[inline]
    pub fn is_advp_0(&self) -> bool {
        *self == ADVPR::ADVP_0
    }
    #[doc = "Checks if the value of the field is `ADVP_1`"]
    #[inline]
    pub fn is_advp_1(&self) -> bool {
        *self == ADVPR::ADVP_1
    }
}
#[doc = "Possible values of the field `COL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum COLR {
    #[doc = "12 Bits"]
    COL_0,
    #[doc = "11 Bits"]
    COL_1,
    #[doc = "10 Bits"]
    COL_2,
    #[doc = "9 Bits"]
    COL_3,
    #[doc = "8 Bits"]
    COL_4,
    #[doc = "7 Bits"]
    COL_5,
    #[doc = "6 Bits"]
    COL_6,
    #[doc = "5 Bits"]
    COL_7,
    #[doc = "4 Bits"]
    COL_8,
    #[doc = "3 Bits"]
    COL_9,
    #[doc = "2 Bits"]
    COL_10,
    #[doc = "12 Bits"]
    COL_11,
    #[doc = "12 Bits"]
    COL_12,
    #[doc = "12 Bits"]
    COL_13,
    #[doc = "12 Bits"]
    COL_14,
    #[doc = "12 Bits"]
    COL_15,
}
impl COLR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            COLR::COL_0 => 0,
            COLR::COL_1 => 1,
            COLR::COL_2 => 2,
            COLR::COL_3 => 3,
            COLR::COL_4 => 4,
            COLR::COL_5 => 5,
            COLR::COL_6 => 6,
            COLR::COL_7 => 7,
            COLR::COL_8 => 8,
            COLR::COL_9 => 9,
            COLR::COL_10 => 10,
            COLR::COL_11 => 11,
            COLR::COL_12 => 12,
            COLR::COL_13 => 13,
            COLR::COL_14 => 14,
            COLR::COL_15 => 15,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> COLR {
        match value {
            0 => COLR::COL_0,
            1 => COLR::COL_1,
            2 => COLR::COL_2,
            3 => COLR::COL_3,
            4 => COLR::COL_4,
            5 => COLR::COL_5,
            6 => COLR::COL_6,
            7 => COLR::COL_7,
            8 => COLR::COL_8,
            9 => COLR::COL_9,
            10 => COLR::COL_10,
            11 => COLR::COL_11,
            12 => COLR::COL_12,
            13 => COLR::COL_13,
            14 => COLR::COL_14,
            15 => COLR::COL_15,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `COL_0`"]
    #[inline]
    pub fn is_col_0(&self) -> bool {
        *self == COLR::COL_0
    }
    #[doc = "Checks if the value of the field is `COL_1`"]
    #[inline]
    pub fn is_col_1(&self) -> bool {
        *self == COLR::COL_1
    }
    #[doc = "Checks if the value of the field is `COL_2`"]
    #[inline]
    pub fn is_col_2(&self) -> bool {
        *self == COLR::COL_2
    }
    #[doc = "Checks if the value of the field is `COL_3`"]
    #[inline]
    pub fn is_col_3(&self) -> bool {
        *self == COLR::COL_3
    }
    #[doc = "Checks if the value of the field is `COL_4`"]
    #[inline]
    pub fn is_col_4(&self) -> bool {
        *self == COLR::COL_4
    }
    #[doc = "Checks if the value of the field is `COL_5`"]
    #[inline]
    pub fn is_col_5(&self) -> bool {
        *self == COLR::COL_5
    }
    #[doc = "Checks if the value of the field is `COL_6`"]
    #[inline]
    pub fn is_col_6(&self) -> bool {
        *self == COLR::COL_6
    }
    #[doc = "Checks if the value of the field is `COL_7`"]
    #[inline]
    pub fn is_col_7(&self) -> bool {
        *self == COLR::COL_7
    }
    #[doc = "Checks if the value of the field is `COL_8`"]
    #[inline]
    pub fn is_col_8(&self) -> bool {
        *self == COLR::COL_8
    }
    #[doc = "Checks if the value of the field is `COL_9`"]
    #[inline]
    pub fn is_col_9(&self) -> bool {
        *self == COLR::COL_9
    }
    #[doc = "Checks if the value of the field is `COL_10`"]
    #[inline]
    pub fn is_col_10(&self) -> bool {
        *self == COLR::COL_10
    }
    #[doc = "Checks if the value of the field is `COL_11`"]
    #[inline]
    pub fn is_col_11(&self) -> bool {
        *self == COLR::COL_11
    }
    #[doc = "Checks if the value of the field is `COL_12`"]
    #[inline]
    pub fn is_col_12(&self) -> bool {
        *self == COLR::COL_12
    }
    #[doc = "Checks if the value of the field is `COL_13`"]
    #[inline]
    pub fn is_col_13(&self) -> bool {
        *self == COLR::COL_13
    }
    #[doc = "Checks if the value of the field is `COL_14`"]
    #[inline]
    pub fn is_col_14(&self) -> bool {
        *self == COLR::COL_14
    }
    #[doc = "Checks if the value of the field is `COL_15`"]
    #[inline]
    pub fn is_col_15(&self) -> bool {
        *self == COLR::COL_15
    }
}
#[doc = "Values that can be written to the field `PS`"]
pub enum PSW {
    #[doc = "8bit"]
    PS_0,
    #[doc = "16bit"]
    PS_1,
}
impl PSW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PSW::PS_0 => false,
            PSW::PS_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PSW<'a> {
    w: &'a mut W,
}
impl<'a> _PSW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PSW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "8bit"]
    #[inline]
    pub fn ps_0(self) -> &'a mut W {
        self.variant(PSW::PS_0)
    }
    #[doc = "16bit"]
    #[inline]
    pub fn ps_1(self) -> &'a mut W {
        self.variant(PSW::PS_1)
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
#[doc = "Values that can be written to the field `BL`"]
pub enum BLW {
    #[doc = "1"]
    BL_0,
    #[doc = "2"]
    BL_1,
    #[doc = "4"]
    BL_2,
    #[doc = "8"]
    BL_3,
    #[doc = "16"]
    BL_4,
    #[doc = "32"]
    BL_5,
    #[doc = "64"]
    BL_6,
    #[doc = "64"]
    BL_7,
}
impl BLW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            BLW::BL_0 => 0,
            BLW::BL_1 => 1,
            BLW::BL_2 => 2,
            BLW::BL_3 => 3,
            BLW::BL_4 => 4,
            BLW::BL_5 => 5,
            BLW::BL_6 => 6,
            BLW::BL_7 => 7,
        }
    }
}
#[doc = r" Proxy"]
pub struct _BLW<'a> {
    w: &'a mut W,
}
impl<'a> _BLW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: BLW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "1"]
    #[inline]
    pub fn bl_0(self) -> &'a mut W {
        self.variant(BLW::BL_0)
    }
    #[doc = "2"]
    #[inline]
    pub fn bl_1(self) -> &'a mut W {
        self.variant(BLW::BL_1)
    }
    #[doc = "4"]
    #[inline]
    pub fn bl_2(self) -> &'a mut W {
        self.variant(BLW::BL_2)
    }
    #[doc = "8"]
    #[inline]
    pub fn bl_3(self) -> &'a mut W {
        self.variant(BLW::BL_3)
    }
    #[doc = "16"]
    #[inline]
    pub fn bl_4(self) -> &'a mut W {
        self.variant(BLW::BL_4)
    }
    #[doc = "32"]
    #[inline]
    pub fn bl_5(self) -> &'a mut W {
        self.variant(BLW::BL_5)
    }
    #[doc = "64"]
    #[inline]
    pub fn bl_6(self) -> &'a mut W {
        self.variant(BLW::BL_6)
    }
    #[doc = "64"]
    #[inline]
    pub fn bl_7(self) -> &'a mut W {
        self.variant(BLW::BL_7)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 4;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `AM`"]
pub enum AMW {
    #[doc = "Address/Data MUX mode"]
    AM_0,
    #[doc = "Advanced Address/Data MUX mode"]
    AM_1,
    #[doc = "Address/Data non-MUX mode"]
    AM_2,
    #[doc = "Address/Data non-MUX mode"]
    AM_3,
}
impl AMW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            AMW::AM_0 => 0,
            AMW::AM_1 => 1,
            AMW::AM_2 => 2,
            AMW::AM_3 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _AMW<'a> {
    w: &'a mut W,
}
impl<'a> _AMW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: AMW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Address/Data MUX mode"]
    #[inline]
    pub fn am_0(self) -> &'a mut W {
        self.variant(AMW::AM_0)
    }
    #[doc = "Advanced Address/Data MUX mode"]
    #[inline]
    pub fn am_1(self) -> &'a mut W {
        self.variant(AMW::AM_1)
    }
    #[doc = "Address/Data non-MUX mode"]
    #[inline]
    pub fn am_2(self) -> &'a mut W {
        self.variant(AMW::AM_2)
    }
    #[doc = "Address/Data non-MUX mode"]
    #[inline]
    pub fn am_3(self) -> &'a mut W {
        self.variant(AMW::AM_3)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `ADVP`"]
pub enum ADVPW {
    #[doc = "ADV# is Low Active. In ASYNC mode, device sample address with ADV# rise edge; In SYNC mode, device sample address when ADV# is LOW."]
    ADVP_0,
    #[doc = "ADV# is High Active. In ASYNC mode, device sample address with ADV# fall edge; In SYNC mode, device sample address when ADV# is HIGH."]
    ADVP_1,
}
impl ADVPW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ADVPW::ADVP_0 => false,
            ADVPW::ADVP_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ADVPW<'a> {
    w: &'a mut W,
}
impl<'a> _ADVPW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ADVPW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "ADV# is Low Active. In ASYNC mode, device sample address with ADV# rise edge; In SYNC mode, device sample address when ADV# is LOW."]
    #[inline]
    pub fn advp_0(self) -> &'a mut W {
        self.variant(ADVPW::ADVP_0)
    }
    #[doc = "ADV# is High Active. In ASYNC mode, device sample address with ADV# fall edge; In SYNC mode, device sample address when ADV# is HIGH."]
    #[inline]
    pub fn advp_1(self) -> &'a mut W {
        self.variant(ADVPW::ADVP_1)
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
        const OFFSET: u8 = 10;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `COL`"]
pub enum COLW {
    #[doc = "12 Bits"]
    COL_0,
    #[doc = "11 Bits"]
    COL_1,
    #[doc = "10 Bits"]
    COL_2,
    #[doc = "9 Bits"]
    COL_3,
    #[doc = "8 Bits"]
    COL_4,
    #[doc = "7 Bits"]
    COL_5,
    #[doc = "6 Bits"]
    COL_6,
    #[doc = "5 Bits"]
    COL_7,
    #[doc = "4 Bits"]
    COL_8,
    #[doc = "3 Bits"]
    COL_9,
    #[doc = "2 Bits"]
    COL_10,
    #[doc = "12 Bits"]
    COL_11,
    #[doc = "12 Bits"]
    COL_12,
    #[doc = "12 Bits"]
    COL_13,
    #[doc = "12 Bits"]
    COL_14,
    #[doc = "12 Bits"]
    COL_15,
}
impl COLW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            COLW::COL_0 => 0,
            COLW::COL_1 => 1,
            COLW::COL_2 => 2,
            COLW::COL_3 => 3,
            COLW::COL_4 => 4,
            COLW::COL_5 => 5,
            COLW::COL_6 => 6,
            COLW::COL_7 => 7,
            COLW::COL_8 => 8,
            COLW::COL_9 => 9,
            COLW::COL_10 => 10,
            COLW::COL_11 => 11,
            COLW::COL_12 => 12,
            COLW::COL_13 => 13,
            COLW::COL_14 => 14,
            COLW::COL_15 => 15,
        }
    }
}
#[doc = r" Proxy"]
pub struct _COLW<'a> {
    w: &'a mut W,
}
impl<'a> _COLW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: COLW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "12 Bits"]
    #[inline]
    pub fn col_0(self) -> &'a mut W {
        self.variant(COLW::COL_0)
    }
    #[doc = "11 Bits"]
    #[inline]
    pub fn col_1(self) -> &'a mut W {
        self.variant(COLW::COL_1)
    }
    #[doc = "10 Bits"]
    #[inline]
    pub fn col_2(self) -> &'a mut W {
        self.variant(COLW::COL_2)
    }
    #[doc = "9 Bits"]
    #[inline]
    pub fn col_3(self) -> &'a mut W {
        self.variant(COLW::COL_3)
    }
    #[doc = "8 Bits"]
    #[inline]
    pub fn col_4(self) -> &'a mut W {
        self.variant(COLW::COL_4)
    }
    #[doc = "7 Bits"]
    #[inline]
    pub fn col_5(self) -> &'a mut W {
        self.variant(COLW::COL_5)
    }
    #[doc = "6 Bits"]
    #[inline]
    pub fn col_6(self) -> &'a mut W {
        self.variant(COLW::COL_6)
    }
    #[doc = "5 Bits"]
    #[inline]
    pub fn col_7(self) -> &'a mut W {
        self.variant(COLW::COL_7)
    }
    #[doc = "4 Bits"]
    #[inline]
    pub fn col_8(self) -> &'a mut W {
        self.variant(COLW::COL_8)
    }
    #[doc = "3 Bits"]
    #[inline]
    pub fn col_9(self) -> &'a mut W {
        self.variant(COLW::COL_9)
    }
    #[doc = "2 Bits"]
    #[inline]
    pub fn col_10(self) -> &'a mut W {
        self.variant(COLW::COL_10)
    }
    #[doc = "12 Bits"]
    #[inline]
    pub fn col_11(self) -> &'a mut W {
        self.variant(COLW::COL_11)
    }
    #[doc = "12 Bits"]
    #[inline]
    pub fn col_12(self) -> &'a mut W {
        self.variant(COLW::COL_12)
    }
    #[doc = "12 Bits"]
    #[inline]
    pub fn col_13(self) -> &'a mut W {
        self.variant(COLW::COL_13)
    }
    #[doc = "12 Bits"]
    #[inline]
    pub fn col_14(self) -> &'a mut W {
        self.variant(COLW::COL_14)
    }
    #[doc = "12 Bits"]
    #[inline]
    pub fn col_15(self) -> &'a mut W {
        self.variant(COLW::COL_15)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
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
    #[doc = "Bit 0 - Port Size"]
    #[inline]
    pub fn ps(&self) -> PSR {
        PSR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 4:6 - Burst Length"]
    #[inline]
    pub fn bl(&self) -> BLR {
        BLR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 8:9 - Address Mode"]
    #[inline]
    pub fn am(&self) -> AMR {
        AMR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 10 - ADV# polarity"]
    #[inline]
    pub fn advp(&self) -> ADVPR {
        ADVPR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 12:15 - Column Address bit width"]
    #[inline]
    pub fn col(&self) -> COLR {
        COLR::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 12;
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
    #[doc = "Bit 0 - Port Size"]
    #[inline]
    pub fn ps(&mut self) -> _PSW {
        _PSW { w: self }
    }
    #[doc = "Bits 4:6 - Burst Length"]
    #[inline]
    pub fn bl(&mut self) -> _BLW {
        _BLW { w: self }
    }
    #[doc = "Bits 8:9 - Address Mode"]
    #[inline]
    pub fn am(&mut self) -> _AMW {
        _AMW { w: self }
    }
    #[doc = "Bit 10 - ADV# polarity"]
    #[inline]
    pub fn advp(&mut self) -> _ADVPW {
        _ADVPW { w: self }
    }
    #[doc = "Bits 12:15 - Column Address bit width"]
    #[inline]
    pub fn col(&mut self) -> _COLW {
        _COLW { w: self }
    }
}
