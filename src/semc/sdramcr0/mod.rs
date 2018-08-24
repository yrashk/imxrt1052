#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::SDRAMCR0 {
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
    #[doc = "8"]
    BL_4,
    #[doc = "8"]
    BL_5,
    #[doc = "8"]
    BL_6,
    #[doc = "8"]
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
#[doc = "Possible values of the field `COL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum COLR {
    #[doc = "12 bit"]
    COL_0,
    #[doc = "11 bit"]
    COL_1,
    #[doc = "10 bit"]
    COL_2,
    #[doc = "9 bit"]
    COL_3,
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
}
#[doc = "Possible values of the field `CL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CLR {
    #[doc = "1"]
    CL_0,
    #[doc = "1"]
    CL_1,
    #[doc = "2"]
    CL_2,
    #[doc = "3"]
    CL_3,
}
impl CLR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            CLR::CL_0 => 0,
            CLR::CL_1 => 1,
            CLR::CL_2 => 2,
            CLR::CL_3 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> CLR {
        match value {
            0 => CLR::CL_0,
            1 => CLR::CL_1,
            2 => CLR::CL_2,
            3 => CLR::CL_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `CL_0`"]
    #[inline]
    pub fn is_cl_0(&self) -> bool {
        *self == CLR::CL_0
    }
    #[doc = "Checks if the value of the field is `CL_1`"]
    #[inline]
    pub fn is_cl_1(&self) -> bool {
        *self == CLR::CL_1
    }
    #[doc = "Checks if the value of the field is `CL_2`"]
    #[inline]
    pub fn is_cl_2(&self) -> bool {
        *self == CLR::CL_2
    }
    #[doc = "Checks if the value of the field is `CL_3`"]
    #[inline]
    pub fn is_cl_3(&self) -> bool {
        *self == CLR::CL_3
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
    #[doc = "8"]
    BL_4,
    #[doc = "8"]
    BL_5,
    #[doc = "8"]
    BL_6,
    #[doc = "8"]
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
    #[doc = "8"]
    #[inline]
    pub fn bl_4(self) -> &'a mut W {
        self.variant(BLW::BL_4)
    }
    #[doc = "8"]
    #[inline]
    pub fn bl_5(self) -> &'a mut W {
        self.variant(BLW::BL_5)
    }
    #[doc = "8"]
    #[inline]
    pub fn bl_6(self) -> &'a mut W {
        self.variant(BLW::BL_6)
    }
    #[doc = "8"]
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
#[doc = "Values that can be written to the field `COL`"]
pub enum COLW {
    #[doc = "12 bit"]
    COL_0,
    #[doc = "11 bit"]
    COL_1,
    #[doc = "10 bit"]
    COL_2,
    #[doc = "9 bit"]
    COL_3,
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
    #[doc = "12 bit"]
    #[inline]
    pub fn col_0(self) -> &'a mut W {
        self.variant(COLW::COL_0)
    }
    #[doc = "11 bit"]
    #[inline]
    pub fn col_1(self) -> &'a mut W {
        self.variant(COLW::COL_1)
    }
    #[doc = "10 bit"]
    #[inline]
    pub fn col_2(self) -> &'a mut W {
        self.variant(COLW::COL_2)
    }
    #[doc = "9 bit"]
    #[inline]
    pub fn col_3(self) -> &'a mut W {
        self.variant(COLW::COL_3)
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
#[doc = "Values that can be written to the field `CL`"]
pub enum CLW {
    #[doc = "1"]
    CL_0,
    #[doc = "1"]
    CL_1,
    #[doc = "2"]
    CL_2,
    #[doc = "3"]
    CL_3,
}
impl CLW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            CLW::CL_0 => 0,
            CLW::CL_1 => 1,
            CLW::CL_2 => 2,
            CLW::CL_3 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CLW<'a> {
    w: &'a mut W,
}
impl<'a> _CLW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CLW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "1"]
    #[inline]
    pub fn cl_0(self) -> &'a mut W {
        self.variant(CLW::CL_0)
    }
    #[doc = "1"]
    #[inline]
    pub fn cl_1(self) -> &'a mut W {
        self.variant(CLW::CL_1)
    }
    #[doc = "2"]
    #[inline]
    pub fn cl_2(self) -> &'a mut W {
        self.variant(CLW::CL_2)
    }
    #[doc = "3"]
    #[inline]
    pub fn cl_3(self) -> &'a mut W {
        self.variant(CLW::CL_3)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 10;
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
    #[doc = "Bits 8:9 - Column address bit number"]
    #[inline]
    pub fn col(&self) -> COLR {
        COLR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 10:11 - CAS Latency"]
    #[inline]
    pub fn cl(&self) -> CLR {
        CLR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 3110 }
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
    #[doc = "Bits 8:9 - Column address bit number"]
    #[inline]
    pub fn col(&mut self) -> _COLW {
        _COLW { w: self }
    }
    #[doc = "Bits 10:11 - CAS Latency"]
    #[inline]
    pub fn cl(&mut self) -> _CLW {
        _CLW { w: self }
    }
}
