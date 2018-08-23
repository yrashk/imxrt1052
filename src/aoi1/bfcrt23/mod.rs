#[doc = r" Value read from the register"]
pub struct R {
    bits: u16,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u16,
}
impl super::BFCRT23 {
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
#[doc = "Possible values of the field `PT3_DC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PT3_DCR {
    #[doc = "Force the D input in this product term to a logical zero"]
    PT3_DC_0,
    #[doc = "Pass the D input in this product term"]
    PT3_DC_1,
    #[doc = "Complement the D input in this product term"]
    PT3_DC_2,
    #[doc = "Force the D input in this product term to a logical one"]
    PT3_DC_3,
}
impl PT3_DCR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            PT3_DCR::PT3_DC_0 => 0,
            PT3_DCR::PT3_DC_1 => 1,
            PT3_DCR::PT3_DC_2 => 2,
            PT3_DCR::PT3_DC_3 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> PT3_DCR {
        match value {
            0 => PT3_DCR::PT3_DC_0,
            1 => PT3_DCR::PT3_DC_1,
            2 => PT3_DCR::PT3_DC_2,
            3 => PT3_DCR::PT3_DC_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `PT3_DC_0`"]
    #[inline]
    pub fn is_pt3_dc_0(&self) -> bool {
        *self == PT3_DCR::PT3_DC_0
    }
    #[doc = "Checks if the value of the field is `PT3_DC_1`"]
    #[inline]
    pub fn is_pt3_dc_1(&self) -> bool {
        *self == PT3_DCR::PT3_DC_1
    }
    #[doc = "Checks if the value of the field is `PT3_DC_2`"]
    #[inline]
    pub fn is_pt3_dc_2(&self) -> bool {
        *self == PT3_DCR::PT3_DC_2
    }
    #[doc = "Checks if the value of the field is `PT3_DC_3`"]
    #[inline]
    pub fn is_pt3_dc_3(&self) -> bool {
        *self == PT3_DCR::PT3_DC_3
    }
}
#[doc = "Possible values of the field `PT3_CC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PT3_CCR {
    #[doc = "Force the C input in this product term to a logical zero"]
    PT3_CC_0,
    #[doc = "Pass the C input in this product term"]
    PT3_CC_1,
    #[doc = "Complement the C input in this product term"]
    PT3_CC_2,
    #[doc = "Force the C input in this product term to a logical one"]
    PT3_CC_3,
}
impl PT3_CCR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            PT3_CCR::PT3_CC_0 => 0,
            PT3_CCR::PT3_CC_1 => 1,
            PT3_CCR::PT3_CC_2 => 2,
            PT3_CCR::PT3_CC_3 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> PT3_CCR {
        match value {
            0 => PT3_CCR::PT3_CC_0,
            1 => PT3_CCR::PT3_CC_1,
            2 => PT3_CCR::PT3_CC_2,
            3 => PT3_CCR::PT3_CC_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `PT3_CC_0`"]
    #[inline]
    pub fn is_pt3_cc_0(&self) -> bool {
        *self == PT3_CCR::PT3_CC_0
    }
    #[doc = "Checks if the value of the field is `PT3_CC_1`"]
    #[inline]
    pub fn is_pt3_cc_1(&self) -> bool {
        *self == PT3_CCR::PT3_CC_1
    }
    #[doc = "Checks if the value of the field is `PT3_CC_2`"]
    #[inline]
    pub fn is_pt3_cc_2(&self) -> bool {
        *self == PT3_CCR::PT3_CC_2
    }
    #[doc = "Checks if the value of the field is `PT3_CC_3`"]
    #[inline]
    pub fn is_pt3_cc_3(&self) -> bool {
        *self == PT3_CCR::PT3_CC_3
    }
}
#[doc = "Possible values of the field `PT3_BC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PT3_BCR {
    #[doc = "Force the B input in this product term to a logical zero"]
    PT3_BC_0,
    #[doc = "Pass the B input in this product term"]
    PT3_BC_1,
    #[doc = "Complement the B input in this product term"]
    PT3_BC_2,
    #[doc = "Force the B input in this product term to a logical one"]
    PT3_BC_3,
}
impl PT3_BCR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            PT3_BCR::PT3_BC_0 => 0,
            PT3_BCR::PT3_BC_1 => 1,
            PT3_BCR::PT3_BC_2 => 2,
            PT3_BCR::PT3_BC_3 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> PT3_BCR {
        match value {
            0 => PT3_BCR::PT3_BC_0,
            1 => PT3_BCR::PT3_BC_1,
            2 => PT3_BCR::PT3_BC_2,
            3 => PT3_BCR::PT3_BC_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `PT3_BC_0`"]
    #[inline]
    pub fn is_pt3_bc_0(&self) -> bool {
        *self == PT3_BCR::PT3_BC_0
    }
    #[doc = "Checks if the value of the field is `PT3_BC_1`"]
    #[inline]
    pub fn is_pt3_bc_1(&self) -> bool {
        *self == PT3_BCR::PT3_BC_1
    }
    #[doc = "Checks if the value of the field is `PT3_BC_2`"]
    #[inline]
    pub fn is_pt3_bc_2(&self) -> bool {
        *self == PT3_BCR::PT3_BC_2
    }
    #[doc = "Checks if the value of the field is `PT3_BC_3`"]
    #[inline]
    pub fn is_pt3_bc_3(&self) -> bool {
        *self == PT3_BCR::PT3_BC_3
    }
}
#[doc = "Possible values of the field `PT3_AC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PT3_ACR {
    #[doc = "Force the A input in this product term to a logical zero"]
    PT3_AC_0,
    #[doc = "Pass the A input in this product term"]
    PT3_AC_1,
    #[doc = "Complement the A input in this product term"]
    PT3_AC_2,
    #[doc = "Force the A input in this product term to a logical one"]
    PT3_AC_3,
}
impl PT3_ACR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            PT3_ACR::PT3_AC_0 => 0,
            PT3_ACR::PT3_AC_1 => 1,
            PT3_ACR::PT3_AC_2 => 2,
            PT3_ACR::PT3_AC_3 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> PT3_ACR {
        match value {
            0 => PT3_ACR::PT3_AC_0,
            1 => PT3_ACR::PT3_AC_1,
            2 => PT3_ACR::PT3_AC_2,
            3 => PT3_ACR::PT3_AC_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `PT3_AC_0`"]
    #[inline]
    pub fn is_pt3_ac_0(&self) -> bool {
        *self == PT3_ACR::PT3_AC_0
    }
    #[doc = "Checks if the value of the field is `PT3_AC_1`"]
    #[inline]
    pub fn is_pt3_ac_1(&self) -> bool {
        *self == PT3_ACR::PT3_AC_1
    }
    #[doc = "Checks if the value of the field is `PT3_AC_2`"]
    #[inline]
    pub fn is_pt3_ac_2(&self) -> bool {
        *self == PT3_ACR::PT3_AC_2
    }
    #[doc = "Checks if the value of the field is `PT3_AC_3`"]
    #[inline]
    pub fn is_pt3_ac_3(&self) -> bool {
        *self == PT3_ACR::PT3_AC_3
    }
}
#[doc = "Possible values of the field `PT2_DC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PT2_DCR {
    #[doc = "Force the D input in this product term to a logical zero"]
    PT2_DC_0,
    #[doc = "Pass the D input in this product term"]
    PT2_DC_1,
    #[doc = "Complement the D input in this product term"]
    PT2_DC_2,
    #[doc = "Force the D input in this product term to a logical one"]
    PT2_DC_3,
}
impl PT2_DCR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            PT2_DCR::PT2_DC_0 => 0,
            PT2_DCR::PT2_DC_1 => 1,
            PT2_DCR::PT2_DC_2 => 2,
            PT2_DCR::PT2_DC_3 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> PT2_DCR {
        match value {
            0 => PT2_DCR::PT2_DC_0,
            1 => PT2_DCR::PT2_DC_1,
            2 => PT2_DCR::PT2_DC_2,
            3 => PT2_DCR::PT2_DC_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `PT2_DC_0`"]
    #[inline]
    pub fn is_pt2_dc_0(&self) -> bool {
        *self == PT2_DCR::PT2_DC_0
    }
    #[doc = "Checks if the value of the field is `PT2_DC_1`"]
    #[inline]
    pub fn is_pt2_dc_1(&self) -> bool {
        *self == PT2_DCR::PT2_DC_1
    }
    #[doc = "Checks if the value of the field is `PT2_DC_2`"]
    #[inline]
    pub fn is_pt2_dc_2(&self) -> bool {
        *self == PT2_DCR::PT2_DC_2
    }
    #[doc = "Checks if the value of the field is `PT2_DC_3`"]
    #[inline]
    pub fn is_pt2_dc_3(&self) -> bool {
        *self == PT2_DCR::PT2_DC_3
    }
}
#[doc = "Possible values of the field `PT2_CC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PT2_CCR {
    #[doc = "Force the C input in this product term to a logical zero"]
    PT2_CC_0,
    #[doc = "Pass the C input in this product term"]
    PT2_CC_1,
    #[doc = "Complement the C input in this product term"]
    PT2_CC_2,
    #[doc = "Force the C input in this product term to a logical one"]
    PT2_CC_3,
}
impl PT2_CCR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            PT2_CCR::PT2_CC_0 => 0,
            PT2_CCR::PT2_CC_1 => 1,
            PT2_CCR::PT2_CC_2 => 2,
            PT2_CCR::PT2_CC_3 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> PT2_CCR {
        match value {
            0 => PT2_CCR::PT2_CC_0,
            1 => PT2_CCR::PT2_CC_1,
            2 => PT2_CCR::PT2_CC_2,
            3 => PT2_CCR::PT2_CC_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `PT2_CC_0`"]
    #[inline]
    pub fn is_pt2_cc_0(&self) -> bool {
        *self == PT2_CCR::PT2_CC_0
    }
    #[doc = "Checks if the value of the field is `PT2_CC_1`"]
    #[inline]
    pub fn is_pt2_cc_1(&self) -> bool {
        *self == PT2_CCR::PT2_CC_1
    }
    #[doc = "Checks if the value of the field is `PT2_CC_2`"]
    #[inline]
    pub fn is_pt2_cc_2(&self) -> bool {
        *self == PT2_CCR::PT2_CC_2
    }
    #[doc = "Checks if the value of the field is `PT2_CC_3`"]
    #[inline]
    pub fn is_pt2_cc_3(&self) -> bool {
        *self == PT2_CCR::PT2_CC_3
    }
}
#[doc = "Possible values of the field `PT2_BC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PT2_BCR {
    #[doc = "Force the B input in this product term to a logical zero"]
    PT2_BC_0,
    #[doc = "Pass the B input in this product term"]
    PT2_BC_1,
    #[doc = "Complement the B input in this product term"]
    PT2_BC_2,
    #[doc = "Force the B input in this product term to a logical one"]
    PT2_BC_3,
}
impl PT2_BCR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            PT2_BCR::PT2_BC_0 => 0,
            PT2_BCR::PT2_BC_1 => 1,
            PT2_BCR::PT2_BC_2 => 2,
            PT2_BCR::PT2_BC_3 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> PT2_BCR {
        match value {
            0 => PT2_BCR::PT2_BC_0,
            1 => PT2_BCR::PT2_BC_1,
            2 => PT2_BCR::PT2_BC_2,
            3 => PT2_BCR::PT2_BC_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `PT2_BC_0`"]
    #[inline]
    pub fn is_pt2_bc_0(&self) -> bool {
        *self == PT2_BCR::PT2_BC_0
    }
    #[doc = "Checks if the value of the field is `PT2_BC_1`"]
    #[inline]
    pub fn is_pt2_bc_1(&self) -> bool {
        *self == PT2_BCR::PT2_BC_1
    }
    #[doc = "Checks if the value of the field is `PT2_BC_2`"]
    #[inline]
    pub fn is_pt2_bc_2(&self) -> bool {
        *self == PT2_BCR::PT2_BC_2
    }
    #[doc = "Checks if the value of the field is `PT2_BC_3`"]
    #[inline]
    pub fn is_pt2_bc_3(&self) -> bool {
        *self == PT2_BCR::PT2_BC_3
    }
}
#[doc = "Possible values of the field `PT2_AC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PT2_ACR {
    #[doc = "Force the A input in this product term to a logical zero"]
    PT2_AC_0,
    #[doc = "Pass the A input in this product term"]
    PT2_AC_1,
    #[doc = "Complement the A input in this product term"]
    PT2_AC_2,
    #[doc = "Force the A input in this product term to a logical one"]
    PT2_AC_3,
}
impl PT2_ACR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            PT2_ACR::PT2_AC_0 => 0,
            PT2_ACR::PT2_AC_1 => 1,
            PT2_ACR::PT2_AC_2 => 2,
            PT2_ACR::PT2_AC_3 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> PT2_ACR {
        match value {
            0 => PT2_ACR::PT2_AC_0,
            1 => PT2_ACR::PT2_AC_1,
            2 => PT2_ACR::PT2_AC_2,
            3 => PT2_ACR::PT2_AC_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `PT2_AC_0`"]
    #[inline]
    pub fn is_pt2_ac_0(&self) -> bool {
        *self == PT2_ACR::PT2_AC_0
    }
    #[doc = "Checks if the value of the field is `PT2_AC_1`"]
    #[inline]
    pub fn is_pt2_ac_1(&self) -> bool {
        *self == PT2_ACR::PT2_AC_1
    }
    #[doc = "Checks if the value of the field is `PT2_AC_2`"]
    #[inline]
    pub fn is_pt2_ac_2(&self) -> bool {
        *self == PT2_ACR::PT2_AC_2
    }
    #[doc = "Checks if the value of the field is `PT2_AC_3`"]
    #[inline]
    pub fn is_pt2_ac_3(&self) -> bool {
        *self == PT2_ACR::PT2_AC_3
    }
}
#[doc = "Values that can be written to the field `PT3_DC`"]
pub enum PT3_DCW {
    #[doc = "Force the D input in this product term to a logical zero"]
    PT3_DC_0,
    #[doc = "Pass the D input in this product term"]
    PT3_DC_1,
    #[doc = "Complement the D input in this product term"]
    PT3_DC_2,
    #[doc = "Force the D input in this product term to a logical one"]
    PT3_DC_3,
}
impl PT3_DCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            PT3_DCW::PT3_DC_0 => 0,
            PT3_DCW::PT3_DC_1 => 1,
            PT3_DCW::PT3_DC_2 => 2,
            PT3_DCW::PT3_DC_3 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PT3_DCW<'a> {
    w: &'a mut W,
}
impl<'a> _PT3_DCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PT3_DCW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Force the D input in this product term to a logical zero"]
    #[inline]
    pub fn pt3_dc_0(self) -> &'a mut W {
        self.variant(PT3_DCW::PT3_DC_0)
    }
    #[doc = "Pass the D input in this product term"]
    #[inline]
    pub fn pt3_dc_1(self) -> &'a mut W {
        self.variant(PT3_DCW::PT3_DC_1)
    }
    #[doc = "Complement the D input in this product term"]
    #[inline]
    pub fn pt3_dc_2(self) -> &'a mut W {
        self.variant(PT3_DCW::PT3_DC_2)
    }
    #[doc = "Force the D input in this product term to a logical one"]
    #[inline]
    pub fn pt3_dc_3(self) -> &'a mut W {
        self.variant(PT3_DCW::PT3_DC_3)
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
#[doc = "Values that can be written to the field `PT3_CC`"]
pub enum PT3_CCW {
    #[doc = "Force the C input in this product term to a logical zero"]
    PT3_CC_0,
    #[doc = "Pass the C input in this product term"]
    PT3_CC_1,
    #[doc = "Complement the C input in this product term"]
    PT3_CC_2,
    #[doc = "Force the C input in this product term to a logical one"]
    PT3_CC_3,
}
impl PT3_CCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            PT3_CCW::PT3_CC_0 => 0,
            PT3_CCW::PT3_CC_1 => 1,
            PT3_CCW::PT3_CC_2 => 2,
            PT3_CCW::PT3_CC_3 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PT3_CCW<'a> {
    w: &'a mut W,
}
impl<'a> _PT3_CCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PT3_CCW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Force the C input in this product term to a logical zero"]
    #[inline]
    pub fn pt3_cc_0(self) -> &'a mut W {
        self.variant(PT3_CCW::PT3_CC_0)
    }
    #[doc = "Pass the C input in this product term"]
    #[inline]
    pub fn pt3_cc_1(self) -> &'a mut W {
        self.variant(PT3_CCW::PT3_CC_1)
    }
    #[doc = "Complement the C input in this product term"]
    #[inline]
    pub fn pt3_cc_2(self) -> &'a mut W {
        self.variant(PT3_CCW::PT3_CC_2)
    }
    #[doc = "Force the C input in this product term to a logical one"]
    #[inline]
    pub fn pt3_cc_3(self) -> &'a mut W {
        self.variant(PT3_CCW::PT3_CC_3)
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
#[doc = "Values that can be written to the field `PT3_BC`"]
pub enum PT3_BCW {
    #[doc = "Force the B input in this product term to a logical zero"]
    PT3_BC_0,
    #[doc = "Pass the B input in this product term"]
    PT3_BC_1,
    #[doc = "Complement the B input in this product term"]
    PT3_BC_2,
    #[doc = "Force the B input in this product term to a logical one"]
    PT3_BC_3,
}
impl PT3_BCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            PT3_BCW::PT3_BC_0 => 0,
            PT3_BCW::PT3_BC_1 => 1,
            PT3_BCW::PT3_BC_2 => 2,
            PT3_BCW::PT3_BC_3 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PT3_BCW<'a> {
    w: &'a mut W,
}
impl<'a> _PT3_BCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PT3_BCW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Force the B input in this product term to a logical zero"]
    #[inline]
    pub fn pt3_bc_0(self) -> &'a mut W {
        self.variant(PT3_BCW::PT3_BC_0)
    }
    #[doc = "Pass the B input in this product term"]
    #[inline]
    pub fn pt3_bc_1(self) -> &'a mut W {
        self.variant(PT3_BCW::PT3_BC_1)
    }
    #[doc = "Complement the B input in this product term"]
    #[inline]
    pub fn pt3_bc_2(self) -> &'a mut W {
        self.variant(PT3_BCW::PT3_BC_2)
    }
    #[doc = "Force the B input in this product term to a logical one"]
    #[inline]
    pub fn pt3_bc_3(self) -> &'a mut W {
        self.variant(PT3_BCW::PT3_BC_3)
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
#[doc = "Values that can be written to the field `PT3_AC`"]
pub enum PT3_ACW {
    #[doc = "Force the A input in this product term to a logical zero"]
    PT3_AC_0,
    #[doc = "Pass the A input in this product term"]
    PT3_AC_1,
    #[doc = "Complement the A input in this product term"]
    PT3_AC_2,
    #[doc = "Force the A input in this product term to a logical one"]
    PT3_AC_3,
}
impl PT3_ACW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            PT3_ACW::PT3_AC_0 => 0,
            PT3_ACW::PT3_AC_1 => 1,
            PT3_ACW::PT3_AC_2 => 2,
            PT3_ACW::PT3_AC_3 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PT3_ACW<'a> {
    w: &'a mut W,
}
impl<'a> _PT3_ACW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PT3_ACW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Force the A input in this product term to a logical zero"]
    #[inline]
    pub fn pt3_ac_0(self) -> &'a mut W {
        self.variant(PT3_ACW::PT3_AC_0)
    }
    #[doc = "Pass the A input in this product term"]
    #[inline]
    pub fn pt3_ac_1(self) -> &'a mut W {
        self.variant(PT3_ACW::PT3_AC_1)
    }
    #[doc = "Complement the A input in this product term"]
    #[inline]
    pub fn pt3_ac_2(self) -> &'a mut W {
        self.variant(PT3_ACW::PT3_AC_2)
    }
    #[doc = "Force the A input in this product term to a logical one"]
    #[inline]
    pub fn pt3_ac_3(self) -> &'a mut W {
        self.variant(PT3_ACW::PT3_AC_3)
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
#[doc = "Values that can be written to the field `PT2_DC`"]
pub enum PT2_DCW {
    #[doc = "Force the D input in this product term to a logical zero"]
    PT2_DC_0,
    #[doc = "Pass the D input in this product term"]
    PT2_DC_1,
    #[doc = "Complement the D input in this product term"]
    PT2_DC_2,
    #[doc = "Force the D input in this product term to a logical one"]
    PT2_DC_3,
}
impl PT2_DCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            PT2_DCW::PT2_DC_0 => 0,
            PT2_DCW::PT2_DC_1 => 1,
            PT2_DCW::PT2_DC_2 => 2,
            PT2_DCW::PT2_DC_3 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PT2_DCW<'a> {
    w: &'a mut W,
}
impl<'a> _PT2_DCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PT2_DCW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Force the D input in this product term to a logical zero"]
    #[inline]
    pub fn pt2_dc_0(self) -> &'a mut W {
        self.variant(PT2_DCW::PT2_DC_0)
    }
    #[doc = "Pass the D input in this product term"]
    #[inline]
    pub fn pt2_dc_1(self) -> &'a mut W {
        self.variant(PT2_DCW::PT2_DC_1)
    }
    #[doc = "Complement the D input in this product term"]
    #[inline]
    pub fn pt2_dc_2(self) -> &'a mut W {
        self.variant(PT2_DCW::PT2_DC_2)
    }
    #[doc = "Force the D input in this product term to a logical one"]
    #[inline]
    pub fn pt2_dc_3(self) -> &'a mut W {
        self.variant(PT2_DCW::PT2_DC_3)
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
#[doc = "Values that can be written to the field `PT2_CC`"]
pub enum PT2_CCW {
    #[doc = "Force the C input in this product term to a logical zero"]
    PT2_CC_0,
    #[doc = "Pass the C input in this product term"]
    PT2_CC_1,
    #[doc = "Complement the C input in this product term"]
    PT2_CC_2,
    #[doc = "Force the C input in this product term to a logical one"]
    PT2_CC_3,
}
impl PT2_CCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            PT2_CCW::PT2_CC_0 => 0,
            PT2_CCW::PT2_CC_1 => 1,
            PT2_CCW::PT2_CC_2 => 2,
            PT2_CCW::PT2_CC_3 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PT2_CCW<'a> {
    w: &'a mut W,
}
impl<'a> _PT2_CCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PT2_CCW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Force the C input in this product term to a logical zero"]
    #[inline]
    pub fn pt2_cc_0(self) -> &'a mut W {
        self.variant(PT2_CCW::PT2_CC_0)
    }
    #[doc = "Pass the C input in this product term"]
    #[inline]
    pub fn pt2_cc_1(self) -> &'a mut W {
        self.variant(PT2_CCW::PT2_CC_1)
    }
    #[doc = "Complement the C input in this product term"]
    #[inline]
    pub fn pt2_cc_2(self) -> &'a mut W {
        self.variant(PT2_CCW::PT2_CC_2)
    }
    #[doc = "Force the C input in this product term to a logical one"]
    #[inline]
    pub fn pt2_cc_3(self) -> &'a mut W {
        self.variant(PT2_CCW::PT2_CC_3)
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
#[doc = "Values that can be written to the field `PT2_BC`"]
pub enum PT2_BCW {
    #[doc = "Force the B input in this product term to a logical zero"]
    PT2_BC_0,
    #[doc = "Pass the B input in this product term"]
    PT2_BC_1,
    #[doc = "Complement the B input in this product term"]
    PT2_BC_2,
    #[doc = "Force the B input in this product term to a logical one"]
    PT2_BC_3,
}
impl PT2_BCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            PT2_BCW::PT2_BC_0 => 0,
            PT2_BCW::PT2_BC_1 => 1,
            PT2_BCW::PT2_BC_2 => 2,
            PT2_BCW::PT2_BC_3 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PT2_BCW<'a> {
    w: &'a mut W,
}
impl<'a> _PT2_BCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PT2_BCW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Force the B input in this product term to a logical zero"]
    #[inline]
    pub fn pt2_bc_0(self) -> &'a mut W {
        self.variant(PT2_BCW::PT2_BC_0)
    }
    #[doc = "Pass the B input in this product term"]
    #[inline]
    pub fn pt2_bc_1(self) -> &'a mut W {
        self.variant(PT2_BCW::PT2_BC_1)
    }
    #[doc = "Complement the B input in this product term"]
    #[inline]
    pub fn pt2_bc_2(self) -> &'a mut W {
        self.variant(PT2_BCW::PT2_BC_2)
    }
    #[doc = "Force the B input in this product term to a logical one"]
    #[inline]
    pub fn pt2_bc_3(self) -> &'a mut W {
        self.variant(PT2_BCW::PT2_BC_3)
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
#[doc = "Values that can be written to the field `PT2_AC`"]
pub enum PT2_ACW {
    #[doc = "Force the A input in this product term to a logical zero"]
    PT2_AC_0,
    #[doc = "Pass the A input in this product term"]
    PT2_AC_1,
    #[doc = "Complement the A input in this product term"]
    PT2_AC_2,
    #[doc = "Force the A input in this product term to a logical one"]
    PT2_AC_3,
}
impl PT2_ACW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            PT2_ACW::PT2_AC_0 => 0,
            PT2_ACW::PT2_AC_1 => 1,
            PT2_ACW::PT2_AC_2 => 2,
            PT2_ACW::PT2_AC_3 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PT2_ACW<'a> {
    w: &'a mut W,
}
impl<'a> _PT2_ACW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PT2_ACW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Force the A input in this product term to a logical zero"]
    #[inline]
    pub fn pt2_ac_0(self) -> &'a mut W {
        self.variant(PT2_ACW::PT2_AC_0)
    }
    #[doc = "Pass the A input in this product term"]
    #[inline]
    pub fn pt2_ac_1(self) -> &'a mut W {
        self.variant(PT2_ACW::PT2_AC_1)
    }
    #[doc = "Complement the A input in this product term"]
    #[inline]
    pub fn pt2_ac_2(self) -> &'a mut W {
        self.variant(PT2_ACW::PT2_AC_2)
    }
    #[doc = "Force the A input in this product term to a logical one"]
    #[inline]
    pub fn pt2_ac_3(self) -> &'a mut W {
        self.variant(PT2_ACW::PT2_AC_3)
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
    #[doc = "Bits 0:1 - Product term 3, D input configuration"]
    #[inline]
    pub fn pt3_dc(&self) -> PT3_DCR {
        PT3_DCR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u16) as u8
        })
    }
    #[doc = "Bits 2:3 - Product term 3, C input configuration"]
    #[inline]
    pub fn pt3_cc(&self) -> PT3_CCR {
        PT3_CCR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u16) as u8
        })
    }
    #[doc = "Bits 4:5 - Product term 3, B input configuration"]
    #[inline]
    pub fn pt3_bc(&self) -> PT3_BCR {
        PT3_BCR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u16) as u8
        })
    }
    #[doc = "Bits 6:7 - Product term 3, A input configuration"]
    #[inline]
    pub fn pt3_ac(&self) -> PT3_ACR {
        PT3_ACR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u16) as u8
        })
    }
    #[doc = "Bits 8:9 - Product term 2, D input configuration"]
    #[inline]
    pub fn pt2_dc(&self) -> PT2_DCR {
        PT2_DCR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u16) as u8
        })
    }
    #[doc = "Bits 10:11 - Product term 2, C input configuration"]
    #[inline]
    pub fn pt2_cc(&self) -> PT2_CCR {
        PT2_CCR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u16) as u8
        })
    }
    #[doc = "Bits 12:13 - Product term 2, B input configuration"]
    #[inline]
    pub fn pt2_bc(&self) -> PT2_BCR {
        PT2_BCR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u16) as u8
        })
    }
    #[doc = "Bits 14:15 - Product term 2, A input configuration"]
    #[inline]
    pub fn pt2_ac(&self) -> PT2_ACR {
        PT2_ACR::_from({
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
    #[doc = "Bits 0:1 - Product term 3, D input configuration"]
    #[inline]
    pub fn pt3_dc(&mut self) -> _PT3_DCW {
        _PT3_DCW { w: self }
    }
    #[doc = "Bits 2:3 - Product term 3, C input configuration"]
    #[inline]
    pub fn pt3_cc(&mut self) -> _PT3_CCW {
        _PT3_CCW { w: self }
    }
    #[doc = "Bits 4:5 - Product term 3, B input configuration"]
    #[inline]
    pub fn pt3_bc(&mut self) -> _PT3_BCW {
        _PT3_BCW { w: self }
    }
    #[doc = "Bits 6:7 - Product term 3, A input configuration"]
    #[inline]
    pub fn pt3_ac(&mut self) -> _PT3_ACW {
        _PT3_ACW { w: self }
    }
    #[doc = "Bits 8:9 - Product term 2, D input configuration"]
    #[inline]
    pub fn pt2_dc(&mut self) -> _PT2_DCW {
        _PT2_DCW { w: self }
    }
    #[doc = "Bits 10:11 - Product term 2, C input configuration"]
    #[inline]
    pub fn pt2_cc(&mut self) -> _PT2_CCW {
        _PT2_CCW { w: self }
    }
    #[doc = "Bits 12:13 - Product term 2, B input configuration"]
    #[inline]
    pub fn pt2_bc(&mut self) -> _PT2_BCW {
        _PT2_BCW { w: self }
    }
    #[doc = "Bits 14:15 - Product term 2, A input configuration"]
    #[inline]
    pub fn pt2_ac(&mut self) -> _PT2_ACW {
        _PT2_ACW { w: self }
    }
}
