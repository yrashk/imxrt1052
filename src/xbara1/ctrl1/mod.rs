#[doc = r" Value read from the register"]
pub struct R {
    bits: u16,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u16,
}
impl super::CTRL1 {
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
#[doc = "Possible values of the field `DEN2`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DEN2R {
    #[doc = "DMA disabled"]
    DEN2_0,
    #[doc = "DMA enabled"]
    DEN2_1,
}
impl DEN2R {
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
            DEN2R::DEN2_0 => false,
            DEN2R::DEN2_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DEN2R {
        match value {
            false => DEN2R::DEN2_0,
            true => DEN2R::DEN2_1,
        }
    }
    #[doc = "Checks if the value of the field is `DEN2_0`"]
    #[inline]
    pub fn is_den2_0(&self) -> bool {
        *self == DEN2R::DEN2_0
    }
    #[doc = "Checks if the value of the field is `DEN2_1`"]
    #[inline]
    pub fn is_den2_1(&self) -> bool {
        *self == DEN2R::DEN2_1
    }
}
#[doc = "Possible values of the field `IEN2`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IEN2R {
    #[doc = "Interrupt disabled"]
    IEN2_0,
    #[doc = "Interrupt enabled"]
    IEN2_1,
}
impl IEN2R {
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
            IEN2R::IEN2_0 => false,
            IEN2R::IEN2_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> IEN2R {
        match value {
            false => IEN2R::IEN2_0,
            true => IEN2R::IEN2_1,
        }
    }
    #[doc = "Checks if the value of the field is `IEN2_0`"]
    #[inline]
    pub fn is_ien2_0(&self) -> bool {
        *self == IEN2R::IEN2_0
    }
    #[doc = "Checks if the value of the field is `IEN2_1`"]
    #[inline]
    pub fn is_ien2_1(&self) -> bool {
        *self == IEN2R::IEN2_1
    }
}
#[doc = "Possible values of the field `EDGE2`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EDGE2R {
    #[doc = "STS2 never asserts"]
    EDGE2_0,
    #[doc = "STS2 asserts on rising edges of XBAR_OUT2"]
    EDGE2_1,
    #[doc = "STS2 asserts on falling edges of XBAR_OUT2"]
    EDGE2_2,
    #[doc = "STS2 asserts on rising and falling edges of XBAR_OUT2"]
    EDGE2_3,
}
impl EDGE2R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            EDGE2R::EDGE2_0 => 0,
            EDGE2R::EDGE2_1 => 1,
            EDGE2R::EDGE2_2 => 2,
            EDGE2R::EDGE2_3 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> EDGE2R {
        match value {
            0 => EDGE2R::EDGE2_0,
            1 => EDGE2R::EDGE2_1,
            2 => EDGE2R::EDGE2_2,
            3 => EDGE2R::EDGE2_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `EDGE2_0`"]
    #[inline]
    pub fn is_edge2_0(&self) -> bool {
        *self == EDGE2R::EDGE2_0
    }
    #[doc = "Checks if the value of the field is `EDGE2_1`"]
    #[inline]
    pub fn is_edge2_1(&self) -> bool {
        *self == EDGE2R::EDGE2_1
    }
    #[doc = "Checks if the value of the field is `EDGE2_2`"]
    #[inline]
    pub fn is_edge2_2(&self) -> bool {
        *self == EDGE2R::EDGE2_2
    }
    #[doc = "Checks if the value of the field is `EDGE2_3`"]
    #[inline]
    pub fn is_edge2_3(&self) -> bool {
        *self == EDGE2R::EDGE2_3
    }
}
#[doc = "Possible values of the field `STS2`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum STS2R {
    #[doc = "Active edge not yet detected on XBAR_OUT2"]
    STS2_0,
    #[doc = "Active edge detected on XBAR_OUT2"]
    STS2_1,
}
impl STS2R {
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
            STS2R::STS2_0 => false,
            STS2R::STS2_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> STS2R {
        match value {
            false => STS2R::STS2_0,
            true => STS2R::STS2_1,
        }
    }
    #[doc = "Checks if the value of the field is `STS2_0`"]
    #[inline]
    pub fn is_sts2_0(&self) -> bool {
        *self == STS2R::STS2_0
    }
    #[doc = "Checks if the value of the field is `STS2_1`"]
    #[inline]
    pub fn is_sts2_1(&self) -> bool {
        *self == STS2R::STS2_1
    }
}
#[doc = "Possible values of the field `DEN3`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DEN3R {
    #[doc = "DMA disabled"]
    DEN3_0,
    #[doc = "DMA enabled"]
    DEN3_1,
}
impl DEN3R {
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
            DEN3R::DEN3_0 => false,
            DEN3R::DEN3_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DEN3R {
        match value {
            false => DEN3R::DEN3_0,
            true => DEN3R::DEN3_1,
        }
    }
    #[doc = "Checks if the value of the field is `DEN3_0`"]
    #[inline]
    pub fn is_den3_0(&self) -> bool {
        *self == DEN3R::DEN3_0
    }
    #[doc = "Checks if the value of the field is `DEN3_1`"]
    #[inline]
    pub fn is_den3_1(&self) -> bool {
        *self == DEN3R::DEN3_1
    }
}
#[doc = "Possible values of the field `IEN3`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IEN3R {
    #[doc = "Interrupt disabled"]
    IEN3_0,
    #[doc = "Interrupt enabled"]
    IEN3_1,
}
impl IEN3R {
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
            IEN3R::IEN3_0 => false,
            IEN3R::IEN3_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> IEN3R {
        match value {
            false => IEN3R::IEN3_0,
            true => IEN3R::IEN3_1,
        }
    }
    #[doc = "Checks if the value of the field is `IEN3_0`"]
    #[inline]
    pub fn is_ien3_0(&self) -> bool {
        *self == IEN3R::IEN3_0
    }
    #[doc = "Checks if the value of the field is `IEN3_1`"]
    #[inline]
    pub fn is_ien3_1(&self) -> bool {
        *self == IEN3R::IEN3_1
    }
}
#[doc = "Possible values of the field `EDGE3`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EDGE3R {
    #[doc = "STS3 never asserts"]
    EDGE3_0,
    #[doc = "STS3 asserts on rising edges of XBAR_OUT3"]
    EDGE3_1,
    #[doc = "STS3 asserts on falling edges of XBAR_OUT3"]
    EDGE3_2,
    #[doc = "STS3 asserts on rising and falling edges of XBAR_OUT3"]
    EDGE3_3,
}
impl EDGE3R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            EDGE3R::EDGE3_0 => 0,
            EDGE3R::EDGE3_1 => 1,
            EDGE3R::EDGE3_2 => 2,
            EDGE3R::EDGE3_3 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> EDGE3R {
        match value {
            0 => EDGE3R::EDGE3_0,
            1 => EDGE3R::EDGE3_1,
            2 => EDGE3R::EDGE3_2,
            3 => EDGE3R::EDGE3_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `EDGE3_0`"]
    #[inline]
    pub fn is_edge3_0(&self) -> bool {
        *self == EDGE3R::EDGE3_0
    }
    #[doc = "Checks if the value of the field is `EDGE3_1`"]
    #[inline]
    pub fn is_edge3_1(&self) -> bool {
        *self == EDGE3R::EDGE3_1
    }
    #[doc = "Checks if the value of the field is `EDGE3_2`"]
    #[inline]
    pub fn is_edge3_2(&self) -> bool {
        *self == EDGE3R::EDGE3_2
    }
    #[doc = "Checks if the value of the field is `EDGE3_3`"]
    #[inline]
    pub fn is_edge3_3(&self) -> bool {
        *self == EDGE3R::EDGE3_3
    }
}
#[doc = "Possible values of the field `STS3`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum STS3R {
    #[doc = "Active edge not yet detected on XBAR_OUT3"]
    STS3_0,
    #[doc = "Active edge detected on XBAR_OUT3"]
    STS3_1,
}
impl STS3R {
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
            STS3R::STS3_0 => false,
            STS3R::STS3_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> STS3R {
        match value {
            false => STS3R::STS3_0,
            true => STS3R::STS3_1,
        }
    }
    #[doc = "Checks if the value of the field is `STS3_0`"]
    #[inline]
    pub fn is_sts3_0(&self) -> bool {
        *self == STS3R::STS3_0
    }
    #[doc = "Checks if the value of the field is `STS3_1`"]
    #[inline]
    pub fn is_sts3_1(&self) -> bool {
        *self == STS3R::STS3_1
    }
}
#[doc = "Values that can be written to the field `DEN2`"]
pub enum DEN2W {
    #[doc = "DMA disabled"]
    DEN2_0,
    #[doc = "DMA enabled"]
    DEN2_1,
}
impl DEN2W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DEN2W::DEN2_0 => false,
            DEN2W::DEN2_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DEN2W<'a> {
    w: &'a mut W,
}
impl<'a> _DEN2W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DEN2W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "DMA disabled"]
    #[inline]
    pub fn den2_0(self) -> &'a mut W {
        self.variant(DEN2W::DEN2_0)
    }
    #[doc = "DMA enabled"]
    #[inline]
    pub fn den2_1(self) -> &'a mut W {
        self.variant(DEN2W::DEN2_1)
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
        self.w.bits &= !((MASK as u16) << OFFSET);
        self.w.bits |= ((value & MASK) as u16) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `IEN2`"]
pub enum IEN2W {
    #[doc = "Interrupt disabled"]
    IEN2_0,
    #[doc = "Interrupt enabled"]
    IEN2_1,
}
impl IEN2W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            IEN2W::IEN2_0 => false,
            IEN2W::IEN2_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _IEN2W<'a> {
    w: &'a mut W,
}
impl<'a> _IEN2W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: IEN2W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Interrupt disabled"]
    #[inline]
    pub fn ien2_0(self) -> &'a mut W {
        self.variant(IEN2W::IEN2_0)
    }
    #[doc = "Interrupt enabled"]
    #[inline]
    pub fn ien2_1(self) -> &'a mut W {
        self.variant(IEN2W::IEN2_1)
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
        self.w.bits &= !((MASK as u16) << OFFSET);
        self.w.bits |= ((value & MASK) as u16) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `EDGE2`"]
pub enum EDGE2W {
    #[doc = "STS2 never asserts"]
    EDGE2_0,
    #[doc = "STS2 asserts on rising edges of XBAR_OUT2"]
    EDGE2_1,
    #[doc = "STS2 asserts on falling edges of XBAR_OUT2"]
    EDGE2_2,
    #[doc = "STS2 asserts on rising and falling edges of XBAR_OUT2"]
    EDGE2_3,
}
impl EDGE2W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            EDGE2W::EDGE2_0 => 0,
            EDGE2W::EDGE2_1 => 1,
            EDGE2W::EDGE2_2 => 2,
            EDGE2W::EDGE2_3 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _EDGE2W<'a> {
    w: &'a mut W,
}
impl<'a> _EDGE2W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: EDGE2W) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "STS2 never asserts"]
    #[inline]
    pub fn edge2_0(self) -> &'a mut W {
        self.variant(EDGE2W::EDGE2_0)
    }
    #[doc = "STS2 asserts on rising edges of XBAR_OUT2"]
    #[inline]
    pub fn edge2_1(self) -> &'a mut W {
        self.variant(EDGE2W::EDGE2_1)
    }
    #[doc = "STS2 asserts on falling edges of XBAR_OUT2"]
    #[inline]
    pub fn edge2_2(self) -> &'a mut W {
        self.variant(EDGE2W::EDGE2_2)
    }
    #[doc = "STS2 asserts on rising and falling edges of XBAR_OUT2"]
    #[inline]
    pub fn edge2_3(self) -> &'a mut W {
        self.variant(EDGE2W::EDGE2_3)
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
#[doc = "Values that can be written to the field `STS2`"]
pub enum STS2W {
    #[doc = "Active edge not yet detected on XBAR_OUT2"]
    STS2_0,
    #[doc = "Active edge detected on XBAR_OUT2"]
    STS2_1,
}
impl STS2W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            STS2W::STS2_0 => false,
            STS2W::STS2_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _STS2W<'a> {
    w: &'a mut W,
}
impl<'a> _STS2W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: STS2W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Active edge not yet detected on XBAR_OUT2"]
    #[inline]
    pub fn sts2_0(self) -> &'a mut W {
        self.variant(STS2W::STS2_0)
    }
    #[doc = "Active edge detected on XBAR_OUT2"]
    #[inline]
    pub fn sts2_1(self) -> &'a mut W {
        self.variant(STS2W::STS2_1)
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
        const OFFSET: u8 = 4;
        self.w.bits &= !((MASK as u16) << OFFSET);
        self.w.bits |= ((value & MASK) as u16) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `DEN3`"]
pub enum DEN3W {
    #[doc = "DMA disabled"]
    DEN3_0,
    #[doc = "DMA enabled"]
    DEN3_1,
}
impl DEN3W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DEN3W::DEN3_0 => false,
            DEN3W::DEN3_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DEN3W<'a> {
    w: &'a mut W,
}
impl<'a> _DEN3W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DEN3W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "DMA disabled"]
    #[inline]
    pub fn den3_0(self) -> &'a mut W {
        self.variant(DEN3W::DEN3_0)
    }
    #[doc = "DMA enabled"]
    #[inline]
    pub fn den3_1(self) -> &'a mut W {
        self.variant(DEN3W::DEN3_1)
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
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u16) << OFFSET);
        self.w.bits |= ((value & MASK) as u16) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `IEN3`"]
pub enum IEN3W {
    #[doc = "Interrupt disabled"]
    IEN3_0,
    #[doc = "Interrupt enabled"]
    IEN3_1,
}
impl IEN3W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            IEN3W::IEN3_0 => false,
            IEN3W::IEN3_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _IEN3W<'a> {
    w: &'a mut W,
}
impl<'a> _IEN3W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: IEN3W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Interrupt disabled"]
    #[inline]
    pub fn ien3_0(self) -> &'a mut W {
        self.variant(IEN3W::IEN3_0)
    }
    #[doc = "Interrupt enabled"]
    #[inline]
    pub fn ien3_1(self) -> &'a mut W {
        self.variant(IEN3W::IEN3_1)
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
        const OFFSET: u8 = 9;
        self.w.bits &= !((MASK as u16) << OFFSET);
        self.w.bits |= ((value & MASK) as u16) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `EDGE3`"]
pub enum EDGE3W {
    #[doc = "STS3 never asserts"]
    EDGE3_0,
    #[doc = "STS3 asserts on rising edges of XBAR_OUT3"]
    EDGE3_1,
    #[doc = "STS3 asserts on falling edges of XBAR_OUT3"]
    EDGE3_2,
    #[doc = "STS3 asserts on rising and falling edges of XBAR_OUT3"]
    EDGE3_3,
}
impl EDGE3W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            EDGE3W::EDGE3_0 => 0,
            EDGE3W::EDGE3_1 => 1,
            EDGE3W::EDGE3_2 => 2,
            EDGE3W::EDGE3_3 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _EDGE3W<'a> {
    w: &'a mut W,
}
impl<'a> _EDGE3W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: EDGE3W) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "STS3 never asserts"]
    #[inline]
    pub fn edge3_0(self) -> &'a mut W {
        self.variant(EDGE3W::EDGE3_0)
    }
    #[doc = "STS3 asserts on rising edges of XBAR_OUT3"]
    #[inline]
    pub fn edge3_1(self) -> &'a mut W {
        self.variant(EDGE3W::EDGE3_1)
    }
    #[doc = "STS3 asserts on falling edges of XBAR_OUT3"]
    #[inline]
    pub fn edge3_2(self) -> &'a mut W {
        self.variant(EDGE3W::EDGE3_2)
    }
    #[doc = "STS3 asserts on rising and falling edges of XBAR_OUT3"]
    #[inline]
    pub fn edge3_3(self) -> &'a mut W {
        self.variant(EDGE3W::EDGE3_3)
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
#[doc = "Values that can be written to the field `STS3`"]
pub enum STS3W {
    #[doc = "Active edge not yet detected on XBAR_OUT3"]
    STS3_0,
    #[doc = "Active edge detected on XBAR_OUT3"]
    STS3_1,
}
impl STS3W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            STS3W::STS3_0 => false,
            STS3W::STS3_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _STS3W<'a> {
    w: &'a mut W,
}
impl<'a> _STS3W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: STS3W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Active edge not yet detected on XBAR_OUT3"]
    #[inline]
    pub fn sts3_0(self) -> &'a mut W {
        self.variant(STS3W::STS3_0)
    }
    #[doc = "Active edge detected on XBAR_OUT3"]
    #[inline]
    pub fn sts3_1(self) -> &'a mut W {
        self.variant(STS3W::STS3_1)
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
        const OFFSET: u8 = 12;
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
    #[doc = "Bit 0 - DMA Enable for XBAR_OUT2"]
    #[inline]
    pub fn den2(&self) -> DEN2R {
        DEN2R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        })
    }
    #[doc = "Bit 1 - Interrupt Enable for XBAR_OUT2"]
    #[inline]
    pub fn ien2(&self) -> IEN2R {
        IEN2R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        })
    }
    #[doc = "Bits 2:3 - Active edge for edge detection on XBAR_OUT2"]
    #[inline]
    pub fn edge2(&self) -> EDGE2R {
        EDGE2R::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u16) as u8
        })
    }
    #[doc = "Bit 4 - Edge detection status for XBAR_OUT2"]
    #[inline]
    pub fn sts2(&self) -> STS2R {
        STS2R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        })
    }
    #[doc = "Bit 8 - DMA Enable for XBAR_OUT3"]
    #[inline]
    pub fn den3(&self) -> DEN3R {
        DEN3R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        })
    }
    #[doc = "Bit 9 - Interrupt Enable for XBAR_OUT3"]
    #[inline]
    pub fn ien3(&self) -> IEN3R {
        IEN3R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        })
    }
    #[doc = "Bits 10:11 - Active edge for edge detection on XBAR_OUT3"]
    #[inline]
    pub fn edge3(&self) -> EDGE3R {
        EDGE3R::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u16) as u8
        })
    }
    #[doc = "Bit 12 - Edge detection status for XBAR_OUT3"]
    #[inline]
    pub fn sts3(&self) -> STS3R {
        STS3R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u16) != 0
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
    #[doc = "Bit 0 - DMA Enable for XBAR_OUT2"]
    #[inline]
    pub fn den2(&mut self) -> _DEN2W {
        _DEN2W { w: self }
    }
    #[doc = "Bit 1 - Interrupt Enable for XBAR_OUT2"]
    #[inline]
    pub fn ien2(&mut self) -> _IEN2W {
        _IEN2W { w: self }
    }
    #[doc = "Bits 2:3 - Active edge for edge detection on XBAR_OUT2"]
    #[inline]
    pub fn edge2(&mut self) -> _EDGE2W {
        _EDGE2W { w: self }
    }
    #[doc = "Bit 4 - Edge detection status for XBAR_OUT2"]
    #[inline]
    pub fn sts2(&mut self) -> _STS2W {
        _STS2W { w: self }
    }
    #[doc = "Bit 8 - DMA Enable for XBAR_OUT3"]
    #[inline]
    pub fn den3(&mut self) -> _DEN3W {
        _DEN3W { w: self }
    }
    #[doc = "Bit 9 - Interrupt Enable for XBAR_OUT3"]
    #[inline]
    pub fn ien3(&mut self) -> _IEN3W {
        _IEN3W { w: self }
    }
    #[doc = "Bits 10:11 - Active edge for edge detection on XBAR_OUT3"]
    #[inline]
    pub fn edge3(&mut self) -> _EDGE3W {
        _EDGE3W { w: self }
    }
    #[doc = "Bit 12 - Edge detection status for XBAR_OUT3"]
    #[inline]
    pub fn sts3(&mut self) -> _STS3W {
        _STS3W { w: self }
    }
}
