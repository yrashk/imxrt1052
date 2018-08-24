#[doc = r" Value read from the register"]
pub struct R {
    bits: u16,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u16,
}
impl super::SM1INTEN {
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
#[doc = "Possible values of the field `CMPIE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CMPIER {
    #[doc = "The corresponding STS[CMPF] bit will not cause an interrupt request."]
    CMPIE_0,
    #[doc = "The corresponding STS[CMPF] bit will cause an interrupt request."]
    CMPIE_1,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl CMPIER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            CMPIER::CMPIE_0 => 0,
            CMPIER::CMPIE_1 => 1,
            CMPIER::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> CMPIER {
        match value {
            0 => CMPIER::CMPIE_0,
            1 => CMPIER::CMPIE_1,
            i => CMPIER::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `CMPIE_0`"]
    #[inline]
    pub fn is_cmpie_0(&self) -> bool {
        *self == CMPIER::CMPIE_0
    }
    #[doc = "Checks if the value of the field is `CMPIE_1`"]
    #[inline]
    pub fn is_cmpie_1(&self) -> bool {
        *self == CMPIER::CMPIE_1
    }
}
#[doc = "Possible values of the field `CX0IE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CX0IER {
    #[doc = "Interrupt request disabled for STS[CFX0]."]
    CX0IE_0,
    #[doc = "Interrupt request enabled for STS[CFX0]."]
    CX0IE_1,
}
impl CX0IER {
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
            CX0IER::CX0IE_0 => false,
            CX0IER::CX0IE_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CX0IER {
        match value {
            false => CX0IER::CX0IE_0,
            true => CX0IER::CX0IE_1,
        }
    }
    #[doc = "Checks if the value of the field is `CX0IE_0`"]
    #[inline]
    pub fn is_cx0ie_0(&self) -> bool {
        *self == CX0IER::CX0IE_0
    }
    #[doc = "Checks if the value of the field is `CX0IE_1`"]
    #[inline]
    pub fn is_cx0ie_1(&self) -> bool {
        *self == CX0IER::CX0IE_1
    }
}
#[doc = "Possible values of the field `CX1IE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CX1IER {
    #[doc = "Interrupt request disabled for STS[CFX1]."]
    CX1IE_0,
    #[doc = "Interrupt request enabled for STS[CFX1]."]
    CX1IE_1,
}
impl CX1IER {
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
            CX1IER::CX1IE_0 => false,
            CX1IER::CX1IE_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CX1IER {
        match value {
            false => CX1IER::CX1IE_0,
            true => CX1IER::CX1IE_1,
        }
    }
    #[doc = "Checks if the value of the field is `CX1IE_0`"]
    #[inline]
    pub fn is_cx1ie_0(&self) -> bool {
        *self == CX1IER::CX1IE_0
    }
    #[doc = "Checks if the value of the field is `CX1IE_1`"]
    #[inline]
    pub fn is_cx1ie_1(&self) -> bool {
        *self == CX1IER::CX1IE_1
    }
}
#[doc = "Possible values of the field `CB0IE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CB0IER {
    #[doc = "Interrupt request disabled for STS[CFB0]."]
    CB0IE_0,
    #[doc = "Interrupt request enabled for STS[CFB0]."]
    CB0IE_1,
}
impl CB0IER {
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
            CB0IER::CB0IE_0 => false,
            CB0IER::CB0IE_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CB0IER {
        match value {
            false => CB0IER::CB0IE_0,
            true => CB0IER::CB0IE_1,
        }
    }
    #[doc = "Checks if the value of the field is `CB0IE_0`"]
    #[inline]
    pub fn is_cb0ie_0(&self) -> bool {
        *self == CB0IER::CB0IE_0
    }
    #[doc = "Checks if the value of the field is `CB0IE_1`"]
    #[inline]
    pub fn is_cb0ie_1(&self) -> bool {
        *self == CB0IER::CB0IE_1
    }
}
#[doc = "Possible values of the field `CB1IE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CB1IER {
    #[doc = "Interrupt request disabled for STS[CFB1]."]
    CB1IE_0,
    #[doc = "Interrupt request enabled for STS[CFB1]."]
    CB1IE_1,
}
impl CB1IER {
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
            CB1IER::CB1IE_0 => false,
            CB1IER::CB1IE_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CB1IER {
        match value {
            false => CB1IER::CB1IE_0,
            true => CB1IER::CB1IE_1,
        }
    }
    #[doc = "Checks if the value of the field is `CB1IE_0`"]
    #[inline]
    pub fn is_cb1ie_0(&self) -> bool {
        *self == CB1IER::CB1IE_0
    }
    #[doc = "Checks if the value of the field is `CB1IE_1`"]
    #[inline]
    pub fn is_cb1ie_1(&self) -> bool {
        *self == CB1IER::CB1IE_1
    }
}
#[doc = "Possible values of the field `CA0IE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CA0IER {
    #[doc = "Interrupt request disabled for STS[CFA0]."]
    CA0IE_0,
    #[doc = "Interrupt request enabled for STS[CFA0]."]
    CA0IE_1,
}
impl CA0IER {
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
            CA0IER::CA0IE_0 => false,
            CA0IER::CA0IE_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CA0IER {
        match value {
            false => CA0IER::CA0IE_0,
            true => CA0IER::CA0IE_1,
        }
    }
    #[doc = "Checks if the value of the field is `CA0IE_0`"]
    #[inline]
    pub fn is_ca0ie_0(&self) -> bool {
        *self == CA0IER::CA0IE_0
    }
    #[doc = "Checks if the value of the field is `CA0IE_1`"]
    #[inline]
    pub fn is_ca0ie_1(&self) -> bool {
        *self == CA0IER::CA0IE_1
    }
}
#[doc = "Possible values of the field `CA1IE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CA1IER {
    #[doc = "Interrupt request disabled for STS[CFA1]."]
    CA1IE_0,
    #[doc = "Interrupt request enabled for STS[CFA1]."]
    CA1IE_1,
}
impl CA1IER {
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
            CA1IER::CA1IE_0 => false,
            CA1IER::CA1IE_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CA1IER {
        match value {
            false => CA1IER::CA1IE_0,
            true => CA1IER::CA1IE_1,
        }
    }
    #[doc = "Checks if the value of the field is `CA1IE_0`"]
    #[inline]
    pub fn is_ca1ie_0(&self) -> bool {
        *self == CA1IER::CA1IE_0
    }
    #[doc = "Checks if the value of the field is `CA1IE_1`"]
    #[inline]
    pub fn is_ca1ie_1(&self) -> bool {
        *self == CA1IER::CA1IE_1
    }
}
#[doc = "Possible values of the field `RIE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RIER {
    #[doc = "STS[RF] CPU interrupt requests disabled"]
    RIE_0,
    #[doc = "STS[RF] CPU interrupt requests enabled"]
    RIE_1,
}
impl RIER {
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
            RIER::RIE_0 => false,
            RIER::RIE_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RIER {
        match value {
            false => RIER::RIE_0,
            true => RIER::RIE_1,
        }
    }
    #[doc = "Checks if the value of the field is `RIE_0`"]
    #[inline]
    pub fn is_rie_0(&self) -> bool {
        *self == RIER::RIE_0
    }
    #[doc = "Checks if the value of the field is `RIE_1`"]
    #[inline]
    pub fn is_rie_1(&self) -> bool {
        *self == RIER::RIE_1
    }
}
#[doc = "Possible values of the field `REIE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REIER {
    #[doc = "STS[REF] CPU interrupt requests disabled"]
    REIE_0,
    #[doc = "STS[REF] CPU interrupt requests enabled"]
    REIE_1,
}
impl REIER {
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
            REIER::REIE_0 => false,
            REIER::REIE_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> REIER {
        match value {
            false => REIER::REIE_0,
            true => REIER::REIE_1,
        }
    }
    #[doc = "Checks if the value of the field is `REIE_0`"]
    #[inline]
    pub fn is_reie_0(&self) -> bool {
        *self == REIER::REIE_0
    }
    #[doc = "Checks if the value of the field is `REIE_1`"]
    #[inline]
    pub fn is_reie_1(&self) -> bool {
        *self == REIER::REIE_1
    }
}
#[doc = "Values that can be written to the field `CMPIE`"]
pub enum CMPIEW {
    #[doc = "The corresponding STS[CMPF] bit will not cause an interrupt request."]
    CMPIE_0,
    #[doc = "The corresponding STS[CMPF] bit will cause an interrupt request."]
    CMPIE_1,
}
impl CMPIEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            CMPIEW::CMPIE_0 => 0,
            CMPIEW::CMPIE_1 => 1,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CMPIEW<'a> {
    w: &'a mut W,
}
impl<'a> _CMPIEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CMPIEW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "The corresponding STS[CMPF] bit will not cause an interrupt request."]
    #[inline]
    pub fn cmpie_0(self) -> &'a mut W {
        self.variant(CMPIEW::CMPIE_0)
    }
    #[doc = "The corresponding STS[CMPF] bit will cause an interrupt request."]
    #[inline]
    pub fn cmpie_1(self) -> &'a mut W {
        self.variant(CMPIEW::CMPIE_1)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 63;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u16) << OFFSET);
        self.w.bits |= ((value & MASK) as u16) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `CX0IE`"]
pub enum CX0IEW {
    #[doc = "Interrupt request disabled for STS[CFX0]."]
    CX0IE_0,
    #[doc = "Interrupt request enabled for STS[CFX0]."]
    CX0IE_1,
}
impl CX0IEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CX0IEW::CX0IE_0 => false,
            CX0IEW::CX0IE_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CX0IEW<'a> {
    w: &'a mut W,
}
impl<'a> _CX0IEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CX0IEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Interrupt request disabled for STS[CFX0]."]
    #[inline]
    pub fn cx0ie_0(self) -> &'a mut W {
        self.variant(CX0IEW::CX0IE_0)
    }
    #[doc = "Interrupt request enabled for STS[CFX0]."]
    #[inline]
    pub fn cx0ie_1(self) -> &'a mut W {
        self.variant(CX0IEW::CX0IE_1)
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
        const OFFSET: u8 = 6;
        self.w.bits &= !((MASK as u16) << OFFSET);
        self.w.bits |= ((value & MASK) as u16) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `CX1IE`"]
pub enum CX1IEW {
    #[doc = "Interrupt request disabled for STS[CFX1]."]
    CX1IE_0,
    #[doc = "Interrupt request enabled for STS[CFX1]."]
    CX1IE_1,
}
impl CX1IEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CX1IEW::CX1IE_0 => false,
            CX1IEW::CX1IE_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CX1IEW<'a> {
    w: &'a mut W,
}
impl<'a> _CX1IEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CX1IEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Interrupt request disabled for STS[CFX1]."]
    #[inline]
    pub fn cx1ie_0(self) -> &'a mut W {
        self.variant(CX1IEW::CX1IE_0)
    }
    #[doc = "Interrupt request enabled for STS[CFX1]."]
    #[inline]
    pub fn cx1ie_1(self) -> &'a mut W {
        self.variant(CX1IEW::CX1IE_1)
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
        const OFFSET: u8 = 7;
        self.w.bits &= !((MASK as u16) << OFFSET);
        self.w.bits |= ((value & MASK) as u16) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `CB0IE`"]
pub enum CB0IEW {
    #[doc = "Interrupt request disabled for STS[CFB0]."]
    CB0IE_0,
    #[doc = "Interrupt request enabled for STS[CFB0]."]
    CB0IE_1,
}
impl CB0IEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CB0IEW::CB0IE_0 => false,
            CB0IEW::CB0IE_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CB0IEW<'a> {
    w: &'a mut W,
}
impl<'a> _CB0IEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CB0IEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Interrupt request disabled for STS[CFB0]."]
    #[inline]
    pub fn cb0ie_0(self) -> &'a mut W {
        self.variant(CB0IEW::CB0IE_0)
    }
    #[doc = "Interrupt request enabled for STS[CFB0]."]
    #[inline]
    pub fn cb0ie_1(self) -> &'a mut W {
        self.variant(CB0IEW::CB0IE_1)
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
#[doc = "Values that can be written to the field `CB1IE`"]
pub enum CB1IEW {
    #[doc = "Interrupt request disabled for STS[CFB1]."]
    CB1IE_0,
    #[doc = "Interrupt request enabled for STS[CFB1]."]
    CB1IE_1,
}
impl CB1IEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CB1IEW::CB1IE_0 => false,
            CB1IEW::CB1IE_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CB1IEW<'a> {
    w: &'a mut W,
}
impl<'a> _CB1IEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CB1IEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Interrupt request disabled for STS[CFB1]."]
    #[inline]
    pub fn cb1ie_0(self) -> &'a mut W {
        self.variant(CB1IEW::CB1IE_0)
    }
    #[doc = "Interrupt request enabled for STS[CFB1]."]
    #[inline]
    pub fn cb1ie_1(self) -> &'a mut W {
        self.variant(CB1IEW::CB1IE_1)
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
#[doc = "Values that can be written to the field `CA0IE`"]
pub enum CA0IEW {
    #[doc = "Interrupt request disabled for STS[CFA0]."]
    CA0IE_0,
    #[doc = "Interrupt request enabled for STS[CFA0]."]
    CA0IE_1,
}
impl CA0IEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CA0IEW::CA0IE_0 => false,
            CA0IEW::CA0IE_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CA0IEW<'a> {
    w: &'a mut W,
}
impl<'a> _CA0IEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CA0IEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Interrupt request disabled for STS[CFA0]."]
    #[inline]
    pub fn ca0ie_0(self) -> &'a mut W {
        self.variant(CA0IEW::CA0IE_0)
    }
    #[doc = "Interrupt request enabled for STS[CFA0]."]
    #[inline]
    pub fn ca0ie_1(self) -> &'a mut W {
        self.variant(CA0IEW::CA0IE_1)
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
        self.w.bits &= !((MASK as u16) << OFFSET);
        self.w.bits |= ((value & MASK) as u16) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `CA1IE`"]
pub enum CA1IEW {
    #[doc = "Interrupt request disabled for STS[CFA1]."]
    CA1IE_0,
    #[doc = "Interrupt request enabled for STS[CFA1]."]
    CA1IE_1,
}
impl CA1IEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CA1IEW::CA1IE_0 => false,
            CA1IEW::CA1IE_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CA1IEW<'a> {
    w: &'a mut W,
}
impl<'a> _CA1IEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CA1IEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Interrupt request disabled for STS[CFA1]."]
    #[inline]
    pub fn ca1ie_0(self) -> &'a mut W {
        self.variant(CA1IEW::CA1IE_0)
    }
    #[doc = "Interrupt request enabled for STS[CFA1]."]
    #[inline]
    pub fn ca1ie_1(self) -> &'a mut W {
        self.variant(CA1IEW::CA1IE_1)
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
        const OFFSET: u8 = 11;
        self.w.bits &= !((MASK as u16) << OFFSET);
        self.w.bits |= ((value & MASK) as u16) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `RIE`"]
pub enum RIEW {
    #[doc = "STS[RF] CPU interrupt requests disabled"]
    RIE_0,
    #[doc = "STS[RF] CPU interrupt requests enabled"]
    RIE_1,
}
impl RIEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RIEW::RIE_0 => false,
            RIEW::RIE_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RIEW<'a> {
    w: &'a mut W,
}
impl<'a> _RIEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RIEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "STS[RF] CPU interrupt requests disabled"]
    #[inline]
    pub fn rie_0(self) -> &'a mut W {
        self.variant(RIEW::RIE_0)
    }
    #[doc = "STS[RF] CPU interrupt requests enabled"]
    #[inline]
    pub fn rie_1(self) -> &'a mut W {
        self.variant(RIEW::RIE_1)
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
#[doc = "Values that can be written to the field `REIE`"]
pub enum REIEW {
    #[doc = "STS[REF] CPU interrupt requests disabled"]
    REIE_0,
    #[doc = "STS[REF] CPU interrupt requests enabled"]
    REIE_1,
}
impl REIEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            REIEW::REIE_0 => false,
            REIEW::REIE_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _REIEW<'a> {
    w: &'a mut W,
}
impl<'a> _REIEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: REIEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "STS[REF] CPU interrupt requests disabled"]
    #[inline]
    pub fn reie_0(self) -> &'a mut W {
        self.variant(REIEW::REIE_0)
    }
    #[doc = "STS[REF] CPU interrupt requests enabled"]
    #[inline]
    pub fn reie_1(self) -> &'a mut W {
        self.variant(REIEW::REIE_1)
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
        const OFFSET: u8 = 13;
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
    #[doc = "Bits 0:5 - Compare Interrupt Enables"]
    #[inline]
    pub fn cmpie(&self) -> CMPIER {
        CMPIER::_from({
            const MASK: u8 = 63;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u16) as u8
        })
    }
    #[doc = "Bit 6 - Capture X 0 Interrupt Enable"]
    #[inline]
    pub fn cx0ie(&self) -> CX0IER {
        CX0IER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        })
    }
    #[doc = "Bit 7 - Capture X 1 Interrupt Enable"]
    #[inline]
    pub fn cx1ie(&self) -> CX1IER {
        CX1IER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        })
    }
    #[doc = "Bit 8 - Capture B 0 Interrupt Enable"]
    #[inline]
    pub fn cb0ie(&self) -> CB0IER {
        CB0IER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        })
    }
    #[doc = "Bit 9 - Capture B 1 Interrupt Enable"]
    #[inline]
    pub fn cb1ie(&self) -> CB1IER {
        CB1IER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        })
    }
    #[doc = "Bit 10 - Capture A 0 Interrupt Enable"]
    #[inline]
    pub fn ca0ie(&self) -> CA0IER {
        CA0IER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        })
    }
    #[doc = "Bit 11 - Capture A 1 Interrupt Enable"]
    #[inline]
    pub fn ca1ie(&self) -> CA1IER {
        CA1IER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        })
    }
    #[doc = "Bit 12 - Reload Interrupt Enable"]
    #[inline]
    pub fn rie(&self) -> RIER {
        RIER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        })
    }
    #[doc = "Bit 13 - Reload Error Interrupt Enable"]
    #[inline]
    pub fn reie(&self) -> REIER {
        REIER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 13;
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
    #[doc = "Bits 0:5 - Compare Interrupt Enables"]
    #[inline]
    pub fn cmpie(&mut self) -> _CMPIEW {
        _CMPIEW { w: self }
    }
    #[doc = "Bit 6 - Capture X 0 Interrupt Enable"]
    #[inline]
    pub fn cx0ie(&mut self) -> _CX0IEW {
        _CX0IEW { w: self }
    }
    #[doc = "Bit 7 - Capture X 1 Interrupt Enable"]
    #[inline]
    pub fn cx1ie(&mut self) -> _CX1IEW {
        _CX1IEW { w: self }
    }
    #[doc = "Bit 8 - Capture B 0 Interrupt Enable"]
    #[inline]
    pub fn cb0ie(&mut self) -> _CB0IEW {
        _CB0IEW { w: self }
    }
    #[doc = "Bit 9 - Capture B 1 Interrupt Enable"]
    #[inline]
    pub fn cb1ie(&mut self) -> _CB1IEW {
        _CB1IEW { w: self }
    }
    #[doc = "Bit 10 - Capture A 0 Interrupt Enable"]
    #[inline]
    pub fn ca0ie(&mut self) -> _CA0IEW {
        _CA0IEW { w: self }
    }
    #[doc = "Bit 11 - Capture A 1 Interrupt Enable"]
    #[inline]
    pub fn ca1ie(&mut self) -> _CA1IEW {
        _CA1IEW { w: self }
    }
    #[doc = "Bit 12 - Reload Interrupt Enable"]
    #[inline]
    pub fn rie(&mut self) -> _RIEW {
        _RIEW { w: self }
    }
    #[doc = "Bit 13 - Reload Error Interrupt Enable"]
    #[inline]
    pub fn reie(&mut self) -> _REIEW {
        _REIEW { w: self }
    }
}
