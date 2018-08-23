#[doc = r" Value read from the register"]
pub struct R {
    bits: u16,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u16,
}
impl super::CTRL0 {
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
#[doc = "Possible values of the field `DEN0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DEN0R {
    #[doc = "DMA disabled"]
    DEN0_0,
    #[doc = "DMA enabled"]
    DEN0_1,
}
impl DEN0R {
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
            DEN0R::DEN0_0 => false,
            DEN0R::DEN0_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DEN0R {
        match value {
            false => DEN0R::DEN0_0,
            true => DEN0R::DEN0_1,
        }
    }
    #[doc = "Checks if the value of the field is `DEN0_0`"]
    #[inline]
    pub fn is_den0_0(&self) -> bool {
        *self == DEN0R::DEN0_0
    }
    #[doc = "Checks if the value of the field is `DEN0_1`"]
    #[inline]
    pub fn is_den0_1(&self) -> bool {
        *self == DEN0R::DEN0_1
    }
}
#[doc = "Possible values of the field `IEN0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IEN0R {
    #[doc = "Interrupt disabled"]
    IEN0_0,
    #[doc = "Interrupt enabled"]
    IEN0_1,
}
impl IEN0R {
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
            IEN0R::IEN0_0 => false,
            IEN0R::IEN0_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> IEN0R {
        match value {
            false => IEN0R::IEN0_0,
            true => IEN0R::IEN0_1,
        }
    }
    #[doc = "Checks if the value of the field is `IEN0_0`"]
    #[inline]
    pub fn is_ien0_0(&self) -> bool {
        *self == IEN0R::IEN0_0
    }
    #[doc = "Checks if the value of the field is `IEN0_1`"]
    #[inline]
    pub fn is_ien0_1(&self) -> bool {
        *self == IEN0R::IEN0_1
    }
}
#[doc = "Possible values of the field `EDGE0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EDGE0R {
    #[doc = "STS0 never asserts"]
    EDGE0_0,
    #[doc = "STS0 asserts on rising edges of XBAR_OUT0"]
    EDGE0_1,
    #[doc = "STS0 asserts on falling edges of XBAR_OUT0"]
    EDGE0_2,
    #[doc = "STS0 asserts on rising and falling edges of XBAR_OUT0"]
    EDGE0_3,
}
impl EDGE0R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            EDGE0R::EDGE0_0 => 0,
            EDGE0R::EDGE0_1 => 1,
            EDGE0R::EDGE0_2 => 2,
            EDGE0R::EDGE0_3 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> EDGE0R {
        match value {
            0 => EDGE0R::EDGE0_0,
            1 => EDGE0R::EDGE0_1,
            2 => EDGE0R::EDGE0_2,
            3 => EDGE0R::EDGE0_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `EDGE0_0`"]
    #[inline]
    pub fn is_edge0_0(&self) -> bool {
        *self == EDGE0R::EDGE0_0
    }
    #[doc = "Checks if the value of the field is `EDGE0_1`"]
    #[inline]
    pub fn is_edge0_1(&self) -> bool {
        *self == EDGE0R::EDGE0_1
    }
    #[doc = "Checks if the value of the field is `EDGE0_2`"]
    #[inline]
    pub fn is_edge0_2(&self) -> bool {
        *self == EDGE0R::EDGE0_2
    }
    #[doc = "Checks if the value of the field is `EDGE0_3`"]
    #[inline]
    pub fn is_edge0_3(&self) -> bool {
        *self == EDGE0R::EDGE0_3
    }
}
#[doc = "Possible values of the field `STS0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum STS0R {
    #[doc = "Active edge not yet detected on XBAR_OUT0"]
    STS0_0,
    #[doc = "Active edge detected on XBAR_OUT0"]
    STS0_1,
}
impl STS0R {
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
            STS0R::STS0_0 => false,
            STS0R::STS0_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> STS0R {
        match value {
            false => STS0R::STS0_0,
            true => STS0R::STS0_1,
        }
    }
    #[doc = "Checks if the value of the field is `STS0_0`"]
    #[inline]
    pub fn is_sts0_0(&self) -> bool {
        *self == STS0R::STS0_0
    }
    #[doc = "Checks if the value of the field is `STS0_1`"]
    #[inline]
    pub fn is_sts0_1(&self) -> bool {
        *self == STS0R::STS0_1
    }
}
#[doc = "Possible values of the field `DEN1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DEN1R {
    #[doc = "DMA disabled"]
    DEN1_0,
    #[doc = "DMA enabled"]
    DEN1_1,
}
impl DEN1R {
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
            DEN1R::DEN1_0 => false,
            DEN1R::DEN1_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DEN1R {
        match value {
            false => DEN1R::DEN1_0,
            true => DEN1R::DEN1_1,
        }
    }
    #[doc = "Checks if the value of the field is `DEN1_0`"]
    #[inline]
    pub fn is_den1_0(&self) -> bool {
        *self == DEN1R::DEN1_0
    }
    #[doc = "Checks if the value of the field is `DEN1_1`"]
    #[inline]
    pub fn is_den1_1(&self) -> bool {
        *self == DEN1R::DEN1_1
    }
}
#[doc = "Possible values of the field `IEN1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IEN1R {
    #[doc = "Interrupt disabled"]
    IEN1_0,
    #[doc = "Interrupt enabled"]
    IEN1_1,
}
impl IEN1R {
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
            IEN1R::IEN1_0 => false,
            IEN1R::IEN1_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> IEN1R {
        match value {
            false => IEN1R::IEN1_0,
            true => IEN1R::IEN1_1,
        }
    }
    #[doc = "Checks if the value of the field is `IEN1_0`"]
    #[inline]
    pub fn is_ien1_0(&self) -> bool {
        *self == IEN1R::IEN1_0
    }
    #[doc = "Checks if the value of the field is `IEN1_1`"]
    #[inline]
    pub fn is_ien1_1(&self) -> bool {
        *self == IEN1R::IEN1_1
    }
}
#[doc = "Possible values of the field `EDGE1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EDGE1R {
    #[doc = "STS1 never asserts"]
    EDGE1_0,
    #[doc = "STS1 asserts on rising edges of XBAR_OUT1"]
    EDGE1_1,
    #[doc = "STS1 asserts on falling edges of XBAR_OUT1"]
    EDGE1_2,
    #[doc = "STS1 asserts on rising and falling edges of XBAR_OUT1"]
    EDGE1_3,
}
impl EDGE1R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            EDGE1R::EDGE1_0 => 0,
            EDGE1R::EDGE1_1 => 1,
            EDGE1R::EDGE1_2 => 2,
            EDGE1R::EDGE1_3 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> EDGE1R {
        match value {
            0 => EDGE1R::EDGE1_0,
            1 => EDGE1R::EDGE1_1,
            2 => EDGE1R::EDGE1_2,
            3 => EDGE1R::EDGE1_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `EDGE1_0`"]
    #[inline]
    pub fn is_edge1_0(&self) -> bool {
        *self == EDGE1R::EDGE1_0
    }
    #[doc = "Checks if the value of the field is `EDGE1_1`"]
    #[inline]
    pub fn is_edge1_1(&self) -> bool {
        *self == EDGE1R::EDGE1_1
    }
    #[doc = "Checks if the value of the field is `EDGE1_2`"]
    #[inline]
    pub fn is_edge1_2(&self) -> bool {
        *self == EDGE1R::EDGE1_2
    }
    #[doc = "Checks if the value of the field is `EDGE1_3`"]
    #[inline]
    pub fn is_edge1_3(&self) -> bool {
        *self == EDGE1R::EDGE1_3
    }
}
#[doc = "Possible values of the field `STS1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum STS1R {
    #[doc = "Active edge not yet detected on XBAR_OUT1"]
    STS1_0,
    #[doc = "Active edge detected on XBAR_OUT1"]
    STS1_1,
}
impl STS1R {
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
            STS1R::STS1_0 => false,
            STS1R::STS1_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> STS1R {
        match value {
            false => STS1R::STS1_0,
            true => STS1R::STS1_1,
        }
    }
    #[doc = "Checks if the value of the field is `STS1_0`"]
    #[inline]
    pub fn is_sts1_0(&self) -> bool {
        *self == STS1R::STS1_0
    }
    #[doc = "Checks if the value of the field is `STS1_1`"]
    #[inline]
    pub fn is_sts1_1(&self) -> bool {
        *self == STS1R::STS1_1
    }
}
#[doc = "Values that can be written to the field `DEN0`"]
pub enum DEN0W {
    #[doc = "DMA disabled"]
    DEN0_0,
    #[doc = "DMA enabled"]
    DEN0_1,
}
impl DEN0W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DEN0W::DEN0_0 => false,
            DEN0W::DEN0_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DEN0W<'a> {
    w: &'a mut W,
}
impl<'a> _DEN0W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DEN0W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "DMA disabled"]
    #[inline]
    pub fn den0_0(self) -> &'a mut W {
        self.variant(DEN0W::DEN0_0)
    }
    #[doc = "DMA enabled"]
    #[inline]
    pub fn den0_1(self) -> &'a mut W {
        self.variant(DEN0W::DEN0_1)
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
#[doc = "Values that can be written to the field `IEN0`"]
pub enum IEN0W {
    #[doc = "Interrupt disabled"]
    IEN0_0,
    #[doc = "Interrupt enabled"]
    IEN0_1,
}
impl IEN0W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            IEN0W::IEN0_0 => false,
            IEN0W::IEN0_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _IEN0W<'a> {
    w: &'a mut W,
}
impl<'a> _IEN0W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: IEN0W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Interrupt disabled"]
    #[inline]
    pub fn ien0_0(self) -> &'a mut W {
        self.variant(IEN0W::IEN0_0)
    }
    #[doc = "Interrupt enabled"]
    #[inline]
    pub fn ien0_1(self) -> &'a mut W {
        self.variant(IEN0W::IEN0_1)
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
#[doc = "Values that can be written to the field `EDGE0`"]
pub enum EDGE0W {
    #[doc = "STS0 never asserts"]
    EDGE0_0,
    #[doc = "STS0 asserts on rising edges of XBAR_OUT0"]
    EDGE0_1,
    #[doc = "STS0 asserts on falling edges of XBAR_OUT0"]
    EDGE0_2,
    #[doc = "STS0 asserts on rising and falling edges of XBAR_OUT0"]
    EDGE0_3,
}
impl EDGE0W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            EDGE0W::EDGE0_0 => 0,
            EDGE0W::EDGE0_1 => 1,
            EDGE0W::EDGE0_2 => 2,
            EDGE0W::EDGE0_3 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _EDGE0W<'a> {
    w: &'a mut W,
}
impl<'a> _EDGE0W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: EDGE0W) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "STS0 never asserts"]
    #[inline]
    pub fn edge0_0(self) -> &'a mut W {
        self.variant(EDGE0W::EDGE0_0)
    }
    #[doc = "STS0 asserts on rising edges of XBAR_OUT0"]
    #[inline]
    pub fn edge0_1(self) -> &'a mut W {
        self.variant(EDGE0W::EDGE0_1)
    }
    #[doc = "STS0 asserts on falling edges of XBAR_OUT0"]
    #[inline]
    pub fn edge0_2(self) -> &'a mut W {
        self.variant(EDGE0W::EDGE0_2)
    }
    #[doc = "STS0 asserts on rising and falling edges of XBAR_OUT0"]
    #[inline]
    pub fn edge0_3(self) -> &'a mut W {
        self.variant(EDGE0W::EDGE0_3)
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
#[doc = "Values that can be written to the field `STS0`"]
pub enum STS0W {
    #[doc = "Active edge not yet detected on XBAR_OUT0"]
    STS0_0,
    #[doc = "Active edge detected on XBAR_OUT0"]
    STS0_1,
}
impl STS0W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            STS0W::STS0_0 => false,
            STS0W::STS0_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _STS0W<'a> {
    w: &'a mut W,
}
impl<'a> _STS0W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: STS0W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Active edge not yet detected on XBAR_OUT0"]
    #[inline]
    pub fn sts0_0(self) -> &'a mut W {
        self.variant(STS0W::STS0_0)
    }
    #[doc = "Active edge detected on XBAR_OUT0"]
    #[inline]
    pub fn sts0_1(self) -> &'a mut W {
        self.variant(STS0W::STS0_1)
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
#[doc = "Values that can be written to the field `DEN1`"]
pub enum DEN1W {
    #[doc = "DMA disabled"]
    DEN1_0,
    #[doc = "DMA enabled"]
    DEN1_1,
}
impl DEN1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DEN1W::DEN1_0 => false,
            DEN1W::DEN1_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DEN1W<'a> {
    w: &'a mut W,
}
impl<'a> _DEN1W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DEN1W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "DMA disabled"]
    #[inline]
    pub fn den1_0(self) -> &'a mut W {
        self.variant(DEN1W::DEN1_0)
    }
    #[doc = "DMA enabled"]
    #[inline]
    pub fn den1_1(self) -> &'a mut W {
        self.variant(DEN1W::DEN1_1)
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
#[doc = "Values that can be written to the field `IEN1`"]
pub enum IEN1W {
    #[doc = "Interrupt disabled"]
    IEN1_0,
    #[doc = "Interrupt enabled"]
    IEN1_1,
}
impl IEN1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            IEN1W::IEN1_0 => false,
            IEN1W::IEN1_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _IEN1W<'a> {
    w: &'a mut W,
}
impl<'a> _IEN1W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: IEN1W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Interrupt disabled"]
    #[inline]
    pub fn ien1_0(self) -> &'a mut W {
        self.variant(IEN1W::IEN1_0)
    }
    #[doc = "Interrupt enabled"]
    #[inline]
    pub fn ien1_1(self) -> &'a mut W {
        self.variant(IEN1W::IEN1_1)
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
#[doc = "Values that can be written to the field `EDGE1`"]
pub enum EDGE1W {
    #[doc = "STS1 never asserts"]
    EDGE1_0,
    #[doc = "STS1 asserts on rising edges of XBAR_OUT1"]
    EDGE1_1,
    #[doc = "STS1 asserts on falling edges of XBAR_OUT1"]
    EDGE1_2,
    #[doc = "STS1 asserts on rising and falling edges of XBAR_OUT1"]
    EDGE1_3,
}
impl EDGE1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            EDGE1W::EDGE1_0 => 0,
            EDGE1W::EDGE1_1 => 1,
            EDGE1W::EDGE1_2 => 2,
            EDGE1W::EDGE1_3 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _EDGE1W<'a> {
    w: &'a mut W,
}
impl<'a> _EDGE1W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: EDGE1W) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "STS1 never asserts"]
    #[inline]
    pub fn edge1_0(self) -> &'a mut W {
        self.variant(EDGE1W::EDGE1_0)
    }
    #[doc = "STS1 asserts on rising edges of XBAR_OUT1"]
    #[inline]
    pub fn edge1_1(self) -> &'a mut W {
        self.variant(EDGE1W::EDGE1_1)
    }
    #[doc = "STS1 asserts on falling edges of XBAR_OUT1"]
    #[inline]
    pub fn edge1_2(self) -> &'a mut W {
        self.variant(EDGE1W::EDGE1_2)
    }
    #[doc = "STS1 asserts on rising and falling edges of XBAR_OUT1"]
    #[inline]
    pub fn edge1_3(self) -> &'a mut W {
        self.variant(EDGE1W::EDGE1_3)
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
#[doc = "Values that can be written to the field `STS1`"]
pub enum STS1W {
    #[doc = "Active edge not yet detected on XBAR_OUT1"]
    STS1_0,
    #[doc = "Active edge detected on XBAR_OUT1"]
    STS1_1,
}
impl STS1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            STS1W::STS1_0 => false,
            STS1W::STS1_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _STS1W<'a> {
    w: &'a mut W,
}
impl<'a> _STS1W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: STS1W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Active edge not yet detected on XBAR_OUT1"]
    #[inline]
    pub fn sts1_0(self) -> &'a mut W {
        self.variant(STS1W::STS1_0)
    }
    #[doc = "Active edge detected on XBAR_OUT1"]
    #[inline]
    pub fn sts1_1(self) -> &'a mut W {
        self.variant(STS1W::STS1_1)
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
    #[doc = "Bit 0 - DMA Enable for XBAR_OUT0"]
    #[inline]
    pub fn den0(&self) -> DEN0R {
        DEN0R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        })
    }
    #[doc = "Bit 1 - Interrupt Enable for XBAR_OUT0"]
    #[inline]
    pub fn ien0(&self) -> IEN0R {
        IEN0R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        })
    }
    #[doc = "Bits 2:3 - Active edge for edge detection on XBAR_OUT0"]
    #[inline]
    pub fn edge0(&self) -> EDGE0R {
        EDGE0R::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u16) as u8
        })
    }
    #[doc = "Bit 4 - Edge detection status for XBAR_OUT0"]
    #[inline]
    pub fn sts0(&self) -> STS0R {
        STS0R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        })
    }
    #[doc = "Bit 8 - DMA Enable for XBAR_OUT1"]
    #[inline]
    pub fn den1(&self) -> DEN1R {
        DEN1R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        })
    }
    #[doc = "Bit 9 - Interrupt Enable for XBAR_OUT1"]
    #[inline]
    pub fn ien1(&self) -> IEN1R {
        IEN1R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        })
    }
    #[doc = "Bits 10:11 - Active edge for edge detection on XBAR_OUT1"]
    #[inline]
    pub fn edge1(&self) -> EDGE1R {
        EDGE1R::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u16) as u8
        })
    }
    #[doc = "Bit 12 - Edge detection status for XBAR_OUT1"]
    #[inline]
    pub fn sts1(&self) -> STS1R {
        STS1R::_from({
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
    #[doc = "Bit 0 - DMA Enable for XBAR_OUT0"]
    #[inline]
    pub fn den0(&mut self) -> _DEN0W {
        _DEN0W { w: self }
    }
    #[doc = "Bit 1 - Interrupt Enable for XBAR_OUT0"]
    #[inline]
    pub fn ien0(&mut self) -> _IEN0W {
        _IEN0W { w: self }
    }
    #[doc = "Bits 2:3 - Active edge for edge detection on XBAR_OUT0"]
    #[inline]
    pub fn edge0(&mut self) -> _EDGE0W {
        _EDGE0W { w: self }
    }
    #[doc = "Bit 4 - Edge detection status for XBAR_OUT0"]
    #[inline]
    pub fn sts0(&mut self) -> _STS0W {
        _STS0W { w: self }
    }
    #[doc = "Bit 8 - DMA Enable for XBAR_OUT1"]
    #[inline]
    pub fn den1(&mut self) -> _DEN1W {
        _DEN1W { w: self }
    }
    #[doc = "Bit 9 - Interrupt Enable for XBAR_OUT1"]
    #[inline]
    pub fn ien1(&mut self) -> _IEN1W {
        _IEN1W { w: self }
    }
    #[doc = "Bits 10:11 - Active edge for edge detection on XBAR_OUT1"]
    #[inline]
    pub fn edge1(&mut self) -> _EDGE1W {
        _EDGE1W { w: self }
    }
    #[doc = "Bit 12 - Edge detection status for XBAR_OUT1"]
    #[inline]
    pub fn sts1(&mut self) -> _STS1W {
        _STS1W { w: self }
    }
}
