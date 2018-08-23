#[doc = r" Value read from the register"]
pub struct R {
    bits: u16,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u16,
}
impl super::BFCRT01 {
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
#[doc = "Possible values of the field `PT1_DC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PT1_DCR {
    #[doc = "Force the D input in this product term to a logical zero"]
    PT1_DC_0,
    #[doc = "Pass the D input in this product term"]
    PT1_DC_1,
    #[doc = "Complement the D input in this product term"]
    PT1_DC_2,
    #[doc = "Force the D input in this product term to a logical one"]
    PT1_DC_3,
}
impl PT1_DCR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            PT1_DCR::PT1_DC_0 => 0,
            PT1_DCR::PT1_DC_1 => 1,
            PT1_DCR::PT1_DC_2 => 2,
            PT1_DCR::PT1_DC_3 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> PT1_DCR {
        match value {
            0 => PT1_DCR::PT1_DC_0,
            1 => PT1_DCR::PT1_DC_1,
            2 => PT1_DCR::PT1_DC_2,
            3 => PT1_DCR::PT1_DC_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `PT1_DC_0`"]
    #[inline]
    pub fn is_pt1_dc_0(&self) -> bool {
        *self == PT1_DCR::PT1_DC_0
    }
    #[doc = "Checks if the value of the field is `PT1_DC_1`"]
    #[inline]
    pub fn is_pt1_dc_1(&self) -> bool {
        *self == PT1_DCR::PT1_DC_1
    }
    #[doc = "Checks if the value of the field is `PT1_DC_2`"]
    #[inline]
    pub fn is_pt1_dc_2(&self) -> bool {
        *self == PT1_DCR::PT1_DC_2
    }
    #[doc = "Checks if the value of the field is `PT1_DC_3`"]
    #[inline]
    pub fn is_pt1_dc_3(&self) -> bool {
        *self == PT1_DCR::PT1_DC_3
    }
}
#[doc = "Possible values of the field `PT1_CC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PT1_CCR {
    #[doc = "Force the C input in this product term to a logical zero"]
    PT1_CC_0,
    #[doc = "Pass the C input in this product term"]
    PT1_CC_1,
    #[doc = "Complement the C input in this product term"]
    PT1_CC_2,
    #[doc = "Force the C input in this product term to a logical one"]
    PT1_CC_3,
}
impl PT1_CCR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            PT1_CCR::PT1_CC_0 => 0,
            PT1_CCR::PT1_CC_1 => 1,
            PT1_CCR::PT1_CC_2 => 2,
            PT1_CCR::PT1_CC_3 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> PT1_CCR {
        match value {
            0 => PT1_CCR::PT1_CC_0,
            1 => PT1_CCR::PT1_CC_1,
            2 => PT1_CCR::PT1_CC_2,
            3 => PT1_CCR::PT1_CC_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `PT1_CC_0`"]
    #[inline]
    pub fn is_pt1_cc_0(&self) -> bool {
        *self == PT1_CCR::PT1_CC_0
    }
    #[doc = "Checks if the value of the field is `PT1_CC_1`"]
    #[inline]
    pub fn is_pt1_cc_1(&self) -> bool {
        *self == PT1_CCR::PT1_CC_1
    }
    #[doc = "Checks if the value of the field is `PT1_CC_2`"]
    #[inline]
    pub fn is_pt1_cc_2(&self) -> bool {
        *self == PT1_CCR::PT1_CC_2
    }
    #[doc = "Checks if the value of the field is `PT1_CC_3`"]
    #[inline]
    pub fn is_pt1_cc_3(&self) -> bool {
        *self == PT1_CCR::PT1_CC_3
    }
}
#[doc = "Possible values of the field `PT1_BC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PT1_BCR {
    #[doc = "Force the B input in this product term to a logical zero"]
    PT1_BC_0,
    #[doc = "Pass the B input in this product term"]
    PT1_BC_1,
    #[doc = "Complement the B input in this product term"]
    PT1_BC_2,
    #[doc = "Force the B input in this product term to a logical one"]
    PT1_BC_3,
}
impl PT1_BCR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            PT1_BCR::PT1_BC_0 => 0,
            PT1_BCR::PT1_BC_1 => 1,
            PT1_BCR::PT1_BC_2 => 2,
            PT1_BCR::PT1_BC_3 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> PT1_BCR {
        match value {
            0 => PT1_BCR::PT1_BC_0,
            1 => PT1_BCR::PT1_BC_1,
            2 => PT1_BCR::PT1_BC_2,
            3 => PT1_BCR::PT1_BC_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `PT1_BC_0`"]
    #[inline]
    pub fn is_pt1_bc_0(&self) -> bool {
        *self == PT1_BCR::PT1_BC_0
    }
    #[doc = "Checks if the value of the field is `PT1_BC_1`"]
    #[inline]
    pub fn is_pt1_bc_1(&self) -> bool {
        *self == PT1_BCR::PT1_BC_1
    }
    #[doc = "Checks if the value of the field is `PT1_BC_2`"]
    #[inline]
    pub fn is_pt1_bc_2(&self) -> bool {
        *self == PT1_BCR::PT1_BC_2
    }
    #[doc = "Checks if the value of the field is `PT1_BC_3`"]
    #[inline]
    pub fn is_pt1_bc_3(&self) -> bool {
        *self == PT1_BCR::PT1_BC_3
    }
}
#[doc = "Possible values of the field `PT1_AC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PT1_ACR {
    #[doc = "Force the A input in this product term to a logical zero"]
    PT1_AC_0,
    #[doc = "Pass the A input in this product term"]
    PT1_AC_1,
    #[doc = "Complement the A input in this product term"]
    PT1_AC_2,
    #[doc = "Force the A input in this product term to a logical one"]
    PT1_AC_3,
}
impl PT1_ACR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            PT1_ACR::PT1_AC_0 => 0,
            PT1_ACR::PT1_AC_1 => 1,
            PT1_ACR::PT1_AC_2 => 2,
            PT1_ACR::PT1_AC_3 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> PT1_ACR {
        match value {
            0 => PT1_ACR::PT1_AC_0,
            1 => PT1_ACR::PT1_AC_1,
            2 => PT1_ACR::PT1_AC_2,
            3 => PT1_ACR::PT1_AC_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `PT1_AC_0`"]
    #[inline]
    pub fn is_pt1_ac_0(&self) -> bool {
        *self == PT1_ACR::PT1_AC_0
    }
    #[doc = "Checks if the value of the field is `PT1_AC_1`"]
    #[inline]
    pub fn is_pt1_ac_1(&self) -> bool {
        *self == PT1_ACR::PT1_AC_1
    }
    #[doc = "Checks if the value of the field is `PT1_AC_2`"]
    #[inline]
    pub fn is_pt1_ac_2(&self) -> bool {
        *self == PT1_ACR::PT1_AC_2
    }
    #[doc = "Checks if the value of the field is `PT1_AC_3`"]
    #[inline]
    pub fn is_pt1_ac_3(&self) -> bool {
        *self == PT1_ACR::PT1_AC_3
    }
}
#[doc = "Possible values of the field `PT0_DC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PT0_DCR {
    #[doc = "Force the D input in this product term to a logical zero"]
    PT0_DC_0,
    #[doc = "Pass the D input in this product term"]
    PT0_DC_1,
    #[doc = "Complement the D input in this product term"]
    PT0_DC_2,
    #[doc = "Force the D input in this product term to a logical one"]
    PT0_DC_3,
}
impl PT0_DCR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            PT0_DCR::PT0_DC_0 => 0,
            PT0_DCR::PT0_DC_1 => 1,
            PT0_DCR::PT0_DC_2 => 2,
            PT0_DCR::PT0_DC_3 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> PT0_DCR {
        match value {
            0 => PT0_DCR::PT0_DC_0,
            1 => PT0_DCR::PT0_DC_1,
            2 => PT0_DCR::PT0_DC_2,
            3 => PT0_DCR::PT0_DC_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `PT0_DC_0`"]
    #[inline]
    pub fn is_pt0_dc_0(&self) -> bool {
        *self == PT0_DCR::PT0_DC_0
    }
    #[doc = "Checks if the value of the field is `PT0_DC_1`"]
    #[inline]
    pub fn is_pt0_dc_1(&self) -> bool {
        *self == PT0_DCR::PT0_DC_1
    }
    #[doc = "Checks if the value of the field is `PT0_DC_2`"]
    #[inline]
    pub fn is_pt0_dc_2(&self) -> bool {
        *self == PT0_DCR::PT0_DC_2
    }
    #[doc = "Checks if the value of the field is `PT0_DC_3`"]
    #[inline]
    pub fn is_pt0_dc_3(&self) -> bool {
        *self == PT0_DCR::PT0_DC_3
    }
}
#[doc = "Possible values of the field `PT0_CC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PT0_CCR {
    #[doc = "Force the C input in this product term to a logical zero"]
    PT0_CC_0,
    #[doc = "Pass the C input in this product term"]
    PT0_CC_1,
    #[doc = "Complement the C input in this product term"]
    PT0_CC_2,
    #[doc = "Force the C input in this product term to a logical one"]
    PT0_CC_3,
}
impl PT0_CCR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            PT0_CCR::PT0_CC_0 => 0,
            PT0_CCR::PT0_CC_1 => 1,
            PT0_CCR::PT0_CC_2 => 2,
            PT0_CCR::PT0_CC_3 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> PT0_CCR {
        match value {
            0 => PT0_CCR::PT0_CC_0,
            1 => PT0_CCR::PT0_CC_1,
            2 => PT0_CCR::PT0_CC_2,
            3 => PT0_CCR::PT0_CC_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `PT0_CC_0`"]
    #[inline]
    pub fn is_pt0_cc_0(&self) -> bool {
        *self == PT0_CCR::PT0_CC_0
    }
    #[doc = "Checks if the value of the field is `PT0_CC_1`"]
    #[inline]
    pub fn is_pt0_cc_1(&self) -> bool {
        *self == PT0_CCR::PT0_CC_1
    }
    #[doc = "Checks if the value of the field is `PT0_CC_2`"]
    #[inline]
    pub fn is_pt0_cc_2(&self) -> bool {
        *self == PT0_CCR::PT0_CC_2
    }
    #[doc = "Checks if the value of the field is `PT0_CC_3`"]
    #[inline]
    pub fn is_pt0_cc_3(&self) -> bool {
        *self == PT0_CCR::PT0_CC_3
    }
}
#[doc = "Possible values of the field `PT0_BC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PT0_BCR {
    #[doc = "Force the B input in this product term to a logical zero"]
    PT0_BC_0,
    #[doc = "Pass the B input in this product term"]
    PT0_BC_1,
    #[doc = "Complement the B input in this product term"]
    PT0_BC_2,
    #[doc = "Force the B input in this product term to a logical one"]
    PT0_BC_3,
}
impl PT0_BCR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            PT0_BCR::PT0_BC_0 => 0,
            PT0_BCR::PT0_BC_1 => 1,
            PT0_BCR::PT0_BC_2 => 2,
            PT0_BCR::PT0_BC_3 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> PT0_BCR {
        match value {
            0 => PT0_BCR::PT0_BC_0,
            1 => PT0_BCR::PT0_BC_1,
            2 => PT0_BCR::PT0_BC_2,
            3 => PT0_BCR::PT0_BC_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `PT0_BC_0`"]
    #[inline]
    pub fn is_pt0_bc_0(&self) -> bool {
        *self == PT0_BCR::PT0_BC_0
    }
    #[doc = "Checks if the value of the field is `PT0_BC_1`"]
    #[inline]
    pub fn is_pt0_bc_1(&self) -> bool {
        *self == PT0_BCR::PT0_BC_1
    }
    #[doc = "Checks if the value of the field is `PT0_BC_2`"]
    #[inline]
    pub fn is_pt0_bc_2(&self) -> bool {
        *self == PT0_BCR::PT0_BC_2
    }
    #[doc = "Checks if the value of the field is `PT0_BC_3`"]
    #[inline]
    pub fn is_pt0_bc_3(&self) -> bool {
        *self == PT0_BCR::PT0_BC_3
    }
}
#[doc = "Possible values of the field `PT0_AC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PT0_ACR {
    #[doc = "Force the A input in this product term to a logical zero"]
    PT0_AC_0,
    #[doc = "Pass the A input in this product term"]
    PT0_AC_1,
    #[doc = "Complement the A input in this product term"]
    PT0_AC_2,
    #[doc = "Force the A input in this product term to a logical one"]
    PT0_AC_3,
}
impl PT0_ACR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            PT0_ACR::PT0_AC_0 => 0,
            PT0_ACR::PT0_AC_1 => 1,
            PT0_ACR::PT0_AC_2 => 2,
            PT0_ACR::PT0_AC_3 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> PT0_ACR {
        match value {
            0 => PT0_ACR::PT0_AC_0,
            1 => PT0_ACR::PT0_AC_1,
            2 => PT0_ACR::PT0_AC_2,
            3 => PT0_ACR::PT0_AC_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `PT0_AC_0`"]
    #[inline]
    pub fn is_pt0_ac_0(&self) -> bool {
        *self == PT0_ACR::PT0_AC_0
    }
    #[doc = "Checks if the value of the field is `PT0_AC_1`"]
    #[inline]
    pub fn is_pt0_ac_1(&self) -> bool {
        *self == PT0_ACR::PT0_AC_1
    }
    #[doc = "Checks if the value of the field is `PT0_AC_2`"]
    #[inline]
    pub fn is_pt0_ac_2(&self) -> bool {
        *self == PT0_ACR::PT0_AC_2
    }
    #[doc = "Checks if the value of the field is `PT0_AC_3`"]
    #[inline]
    pub fn is_pt0_ac_3(&self) -> bool {
        *self == PT0_ACR::PT0_AC_3
    }
}
#[doc = "Values that can be written to the field `PT1_DC`"]
pub enum PT1_DCW {
    #[doc = "Force the D input in this product term to a logical zero"]
    PT1_DC_0,
    #[doc = "Pass the D input in this product term"]
    PT1_DC_1,
    #[doc = "Complement the D input in this product term"]
    PT1_DC_2,
    #[doc = "Force the D input in this product term to a logical one"]
    PT1_DC_3,
}
impl PT1_DCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            PT1_DCW::PT1_DC_0 => 0,
            PT1_DCW::PT1_DC_1 => 1,
            PT1_DCW::PT1_DC_2 => 2,
            PT1_DCW::PT1_DC_3 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PT1_DCW<'a> {
    w: &'a mut W,
}
impl<'a> _PT1_DCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PT1_DCW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Force the D input in this product term to a logical zero"]
    #[inline]
    pub fn pt1_dc_0(self) -> &'a mut W {
        self.variant(PT1_DCW::PT1_DC_0)
    }
    #[doc = "Pass the D input in this product term"]
    #[inline]
    pub fn pt1_dc_1(self) -> &'a mut W {
        self.variant(PT1_DCW::PT1_DC_1)
    }
    #[doc = "Complement the D input in this product term"]
    #[inline]
    pub fn pt1_dc_2(self) -> &'a mut W {
        self.variant(PT1_DCW::PT1_DC_2)
    }
    #[doc = "Force the D input in this product term to a logical one"]
    #[inline]
    pub fn pt1_dc_3(self) -> &'a mut W {
        self.variant(PT1_DCW::PT1_DC_3)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u16) << OFFSET);
        self.w.bits |= ((value & MASK) as u16) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `PT1_CC`"]
pub enum PT1_CCW {
    #[doc = "Force the C input in this product term to a logical zero"]
    PT1_CC_0,
    #[doc = "Pass the C input in this product term"]
    PT1_CC_1,
    #[doc = "Complement the C input in this product term"]
    PT1_CC_2,
    #[doc = "Force the C input in this product term to a logical one"]
    PT1_CC_3,
}
impl PT1_CCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            PT1_CCW::PT1_CC_0 => 0,
            PT1_CCW::PT1_CC_1 => 1,
            PT1_CCW::PT1_CC_2 => 2,
            PT1_CCW::PT1_CC_3 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PT1_CCW<'a> {
    w: &'a mut W,
}
impl<'a> _PT1_CCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PT1_CCW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Force the C input in this product term to a logical zero"]
    #[inline]
    pub fn pt1_cc_0(self) -> &'a mut W {
        self.variant(PT1_CCW::PT1_CC_0)
    }
    #[doc = "Pass the C input in this product term"]
    #[inline]
    pub fn pt1_cc_1(self) -> &'a mut W {
        self.variant(PT1_CCW::PT1_CC_1)
    }
    #[doc = "Complement the C input in this product term"]
    #[inline]
    pub fn pt1_cc_2(self) -> &'a mut W {
        self.variant(PT1_CCW::PT1_CC_2)
    }
    #[doc = "Force the C input in this product term to a logical one"]
    #[inline]
    pub fn pt1_cc_3(self) -> &'a mut W {
        self.variant(PT1_CCW::PT1_CC_3)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 2;
        self.w.bits &= !((MASK as u16) << OFFSET);
        self.w.bits |= ((value & MASK) as u16) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `PT1_BC`"]
pub enum PT1_BCW {
    #[doc = "Force the B input in this product term to a logical zero"]
    PT1_BC_0,
    #[doc = "Pass the B input in this product term"]
    PT1_BC_1,
    #[doc = "Complement the B input in this product term"]
    PT1_BC_2,
    #[doc = "Force the B input in this product term to a logical one"]
    PT1_BC_3,
}
impl PT1_BCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            PT1_BCW::PT1_BC_0 => 0,
            PT1_BCW::PT1_BC_1 => 1,
            PT1_BCW::PT1_BC_2 => 2,
            PT1_BCW::PT1_BC_3 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PT1_BCW<'a> {
    w: &'a mut W,
}
impl<'a> _PT1_BCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PT1_BCW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Force the B input in this product term to a logical zero"]
    #[inline]
    pub fn pt1_bc_0(self) -> &'a mut W {
        self.variant(PT1_BCW::PT1_BC_0)
    }
    #[doc = "Pass the B input in this product term"]
    #[inline]
    pub fn pt1_bc_1(self) -> &'a mut W {
        self.variant(PT1_BCW::PT1_BC_1)
    }
    #[doc = "Complement the B input in this product term"]
    #[inline]
    pub fn pt1_bc_2(self) -> &'a mut W {
        self.variant(PT1_BCW::PT1_BC_2)
    }
    #[doc = "Force the B input in this product term to a logical one"]
    #[inline]
    pub fn pt1_bc_3(self) -> &'a mut W {
        self.variant(PT1_BCW::PT1_BC_3)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 4;
        self.w.bits &= !((MASK as u16) << OFFSET);
        self.w.bits |= ((value & MASK) as u16) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `PT1_AC`"]
pub enum PT1_ACW {
    #[doc = "Force the A input in this product term to a logical zero"]
    PT1_AC_0,
    #[doc = "Pass the A input in this product term"]
    PT1_AC_1,
    #[doc = "Complement the A input in this product term"]
    PT1_AC_2,
    #[doc = "Force the A input in this product term to a logical one"]
    PT1_AC_3,
}
impl PT1_ACW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            PT1_ACW::PT1_AC_0 => 0,
            PT1_ACW::PT1_AC_1 => 1,
            PT1_ACW::PT1_AC_2 => 2,
            PT1_ACW::PT1_AC_3 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PT1_ACW<'a> {
    w: &'a mut W,
}
impl<'a> _PT1_ACW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PT1_ACW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Force the A input in this product term to a logical zero"]
    #[inline]
    pub fn pt1_ac_0(self) -> &'a mut W {
        self.variant(PT1_ACW::PT1_AC_0)
    }
    #[doc = "Pass the A input in this product term"]
    #[inline]
    pub fn pt1_ac_1(self) -> &'a mut W {
        self.variant(PT1_ACW::PT1_AC_1)
    }
    #[doc = "Complement the A input in this product term"]
    #[inline]
    pub fn pt1_ac_2(self) -> &'a mut W {
        self.variant(PT1_ACW::PT1_AC_2)
    }
    #[doc = "Force the A input in this product term to a logical one"]
    #[inline]
    pub fn pt1_ac_3(self) -> &'a mut W {
        self.variant(PT1_ACW::PT1_AC_3)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 6;
        self.w.bits &= !((MASK as u16) << OFFSET);
        self.w.bits |= ((value & MASK) as u16) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `PT0_DC`"]
pub enum PT0_DCW {
    #[doc = "Force the D input in this product term to a logical zero"]
    PT0_DC_0,
    #[doc = "Pass the D input in this product term"]
    PT0_DC_1,
    #[doc = "Complement the D input in this product term"]
    PT0_DC_2,
    #[doc = "Force the D input in this product term to a logical one"]
    PT0_DC_3,
}
impl PT0_DCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            PT0_DCW::PT0_DC_0 => 0,
            PT0_DCW::PT0_DC_1 => 1,
            PT0_DCW::PT0_DC_2 => 2,
            PT0_DCW::PT0_DC_3 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PT0_DCW<'a> {
    w: &'a mut W,
}
impl<'a> _PT0_DCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PT0_DCW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Force the D input in this product term to a logical zero"]
    #[inline]
    pub fn pt0_dc_0(self) -> &'a mut W {
        self.variant(PT0_DCW::PT0_DC_0)
    }
    #[doc = "Pass the D input in this product term"]
    #[inline]
    pub fn pt0_dc_1(self) -> &'a mut W {
        self.variant(PT0_DCW::PT0_DC_1)
    }
    #[doc = "Complement the D input in this product term"]
    #[inline]
    pub fn pt0_dc_2(self) -> &'a mut W {
        self.variant(PT0_DCW::PT0_DC_2)
    }
    #[doc = "Force the D input in this product term to a logical one"]
    #[inline]
    pub fn pt0_dc_3(self) -> &'a mut W {
        self.variant(PT0_DCW::PT0_DC_3)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u16) << OFFSET);
        self.w.bits |= ((value & MASK) as u16) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `PT0_CC`"]
pub enum PT0_CCW {
    #[doc = "Force the C input in this product term to a logical zero"]
    PT0_CC_0,
    #[doc = "Pass the C input in this product term"]
    PT0_CC_1,
    #[doc = "Complement the C input in this product term"]
    PT0_CC_2,
    #[doc = "Force the C input in this product term to a logical one"]
    PT0_CC_3,
}
impl PT0_CCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            PT0_CCW::PT0_CC_0 => 0,
            PT0_CCW::PT0_CC_1 => 1,
            PT0_CCW::PT0_CC_2 => 2,
            PT0_CCW::PT0_CC_3 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PT0_CCW<'a> {
    w: &'a mut W,
}
impl<'a> _PT0_CCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PT0_CCW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Force the C input in this product term to a logical zero"]
    #[inline]
    pub fn pt0_cc_0(self) -> &'a mut W {
        self.variant(PT0_CCW::PT0_CC_0)
    }
    #[doc = "Pass the C input in this product term"]
    #[inline]
    pub fn pt0_cc_1(self) -> &'a mut W {
        self.variant(PT0_CCW::PT0_CC_1)
    }
    #[doc = "Complement the C input in this product term"]
    #[inline]
    pub fn pt0_cc_2(self) -> &'a mut W {
        self.variant(PT0_CCW::PT0_CC_2)
    }
    #[doc = "Force the C input in this product term to a logical one"]
    #[inline]
    pub fn pt0_cc_3(self) -> &'a mut W {
        self.variant(PT0_CCW::PT0_CC_3)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 10;
        self.w.bits &= !((MASK as u16) << OFFSET);
        self.w.bits |= ((value & MASK) as u16) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `PT0_BC`"]
pub enum PT0_BCW {
    #[doc = "Force the B input in this product term to a logical zero"]
    PT0_BC_0,
    #[doc = "Pass the B input in this product term"]
    PT0_BC_1,
    #[doc = "Complement the B input in this product term"]
    PT0_BC_2,
    #[doc = "Force the B input in this product term to a logical one"]
    PT0_BC_3,
}
impl PT0_BCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            PT0_BCW::PT0_BC_0 => 0,
            PT0_BCW::PT0_BC_1 => 1,
            PT0_BCW::PT0_BC_2 => 2,
            PT0_BCW::PT0_BC_3 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PT0_BCW<'a> {
    w: &'a mut W,
}
impl<'a> _PT0_BCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PT0_BCW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Force the B input in this product term to a logical zero"]
    #[inline]
    pub fn pt0_bc_0(self) -> &'a mut W {
        self.variant(PT0_BCW::PT0_BC_0)
    }
    #[doc = "Pass the B input in this product term"]
    #[inline]
    pub fn pt0_bc_1(self) -> &'a mut W {
        self.variant(PT0_BCW::PT0_BC_1)
    }
    #[doc = "Complement the B input in this product term"]
    #[inline]
    pub fn pt0_bc_2(self) -> &'a mut W {
        self.variant(PT0_BCW::PT0_BC_2)
    }
    #[doc = "Force the B input in this product term to a logical one"]
    #[inline]
    pub fn pt0_bc_3(self) -> &'a mut W {
        self.variant(PT0_BCW::PT0_BC_3)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 12;
        self.w.bits &= !((MASK as u16) << OFFSET);
        self.w.bits |= ((value & MASK) as u16) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `PT0_AC`"]
pub enum PT0_ACW {
    #[doc = "Force the A input in this product term to a logical zero"]
    PT0_AC_0,
    #[doc = "Pass the A input in this product term"]
    PT0_AC_1,
    #[doc = "Complement the A input in this product term"]
    PT0_AC_2,
    #[doc = "Force the A input in this product term to a logical one"]
    PT0_AC_3,
}
impl PT0_ACW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            PT0_ACW::PT0_AC_0 => 0,
            PT0_ACW::PT0_AC_1 => 1,
            PT0_ACW::PT0_AC_2 => 2,
            PT0_ACW::PT0_AC_3 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PT0_ACW<'a> {
    w: &'a mut W,
}
impl<'a> _PT0_ACW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PT0_ACW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Force the A input in this product term to a logical zero"]
    #[inline]
    pub fn pt0_ac_0(self) -> &'a mut W {
        self.variant(PT0_ACW::PT0_AC_0)
    }
    #[doc = "Pass the A input in this product term"]
    #[inline]
    pub fn pt0_ac_1(self) -> &'a mut W {
        self.variant(PT0_ACW::PT0_AC_1)
    }
    #[doc = "Complement the A input in this product term"]
    #[inline]
    pub fn pt0_ac_2(self) -> &'a mut W {
        self.variant(PT0_ACW::PT0_AC_2)
    }
    #[doc = "Force the A input in this product term to a logical one"]
    #[inline]
    pub fn pt0_ac_3(self) -> &'a mut W {
        self.variant(PT0_ACW::PT0_AC_3)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 14;
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
    #[doc = "Bits 0:1 - Product term 1, D input configuration"]
    #[inline]
    pub fn pt1_dc(&self) -> PT1_DCR {
        PT1_DCR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u16) as u8
        })
    }
    #[doc = "Bits 2:3 - Product term 1, C input configuration"]
    #[inline]
    pub fn pt1_cc(&self) -> PT1_CCR {
        PT1_CCR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u16) as u8
        })
    }
    #[doc = "Bits 4:5 - Product term 1, B input configuration"]
    #[inline]
    pub fn pt1_bc(&self) -> PT1_BCR {
        PT1_BCR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u16) as u8
        })
    }
    #[doc = "Bits 6:7 - Product term 1, A input configuration"]
    #[inline]
    pub fn pt1_ac(&self) -> PT1_ACR {
        PT1_ACR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u16) as u8
        })
    }
    #[doc = "Bits 8:9 - Product term 0, D input configuration"]
    #[inline]
    pub fn pt0_dc(&self) -> PT0_DCR {
        PT0_DCR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u16) as u8
        })
    }
    #[doc = "Bits 10:11 - Product term 0, C input configuration"]
    #[inline]
    pub fn pt0_cc(&self) -> PT0_CCR {
        PT0_CCR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u16) as u8
        })
    }
    #[doc = "Bits 12:13 - Product term 0, B input configuration"]
    #[inline]
    pub fn pt0_bc(&self) -> PT0_BCR {
        PT0_BCR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u16) as u8
        })
    }
    #[doc = "Bits 14:15 - Product term 0, A input configuration"]
    #[inline]
    pub fn pt0_ac(&self) -> PT0_ACR {
        PT0_ACR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 14;
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
    #[doc = "Bits 0:1 - Product term 1, D input configuration"]
    #[inline]
    pub fn pt1_dc(&mut self) -> _PT1_DCW {
        _PT1_DCW { w: self }
    }
    #[doc = "Bits 2:3 - Product term 1, C input configuration"]
    #[inline]
    pub fn pt1_cc(&mut self) -> _PT1_CCW {
        _PT1_CCW { w: self }
    }
    #[doc = "Bits 4:5 - Product term 1, B input configuration"]
    #[inline]
    pub fn pt1_bc(&mut self) -> _PT1_BCW {
        _PT1_BCW { w: self }
    }
    #[doc = "Bits 6:7 - Product term 1, A input configuration"]
    #[inline]
    pub fn pt1_ac(&mut self) -> _PT1_ACW {
        _PT1_ACW { w: self }
    }
    #[doc = "Bits 8:9 - Product term 0, D input configuration"]
    #[inline]
    pub fn pt0_dc(&mut self) -> _PT0_DCW {
        _PT0_DCW { w: self }
    }
    #[doc = "Bits 10:11 - Product term 0, C input configuration"]
    #[inline]
    pub fn pt0_cc(&mut self) -> _PT0_CCW {
        _PT0_CCW { w: self }
    }
    #[doc = "Bits 12:13 - Product term 0, B input configuration"]
    #[inline]
    pub fn pt0_bc(&mut self) -> _PT0_BCW {
        _PT0_BCW { w: self }
    }
    #[doc = "Bits 14:15 - Product term 0, A input configuration"]
    #[inline]
    pub fn pt0_ac(&mut self) -> _PT0_ACW {
        _PT0_ACW { w: self }
    }
}
