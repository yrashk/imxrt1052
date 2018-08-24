#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::MCR {
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
pub struct SWRSTR {
    bits: bool,
}
impl SWRSTR {
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
#[doc = "Possible values of the field `MDIS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MDISR {
    #[doc = "Module enabled"]
    MDIS_0,
    #[doc = "Master disabled."]
    MDIS_1,
}
impl MDISR {
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
            MDISR::MDIS_0 => false,
            MDISR::MDIS_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> MDISR {
        match value {
            false => MDISR::MDIS_0,
            true => MDISR::MDIS_1,
        }
    }
    #[doc = "Checks if the value of the field is `MDIS_0`"]
    #[inline]
    pub fn is_mdis_0(&self) -> bool {
        *self == MDISR::MDIS_0
    }
    #[doc = "Checks if the value of the field is `MDIS_1`"]
    #[inline]
    pub fn is_mdis_1(&self) -> bool {
        *self == MDISR::MDIS_1
    }
}
#[doc = "Possible values of the field `DQSMD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DQSMDR {
    #[doc = "Dummy read strobe loopbacked internally"]
    DQSMD_0,
    #[doc = "Dummy read strobe loopbacked from DQS pad"]
    DQSMD_1,
}
impl DQSMDR {
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
            DQSMDR::DQSMD_0 => false,
            DQSMDR::DQSMD_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DQSMDR {
        match value {
            false => DQSMDR::DQSMD_0,
            true => DQSMDR::DQSMD_1,
        }
    }
    #[doc = "Checks if the value of the field is `DQSMD_0`"]
    #[inline]
    pub fn is_dqsmd_0(&self) -> bool {
        *self == DQSMDR::DQSMD_0
    }
    #[doc = "Checks if the value of the field is `DQSMD_1`"]
    #[inline]
    pub fn is_dqsmd_1(&self) -> bool {
        *self == DQSMDR::DQSMD_1
    }
}
#[doc = "Possible values of the field `WPOL0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WPOL0R {
    #[doc = "Low active"]
    WPOL0_0,
    #[doc = "High active"]
    WPOL0_1,
}
impl WPOL0R {
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
            WPOL0R::WPOL0_0 => false,
            WPOL0R::WPOL0_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> WPOL0R {
        match value {
            false => WPOL0R::WPOL0_0,
            true => WPOL0R::WPOL0_1,
        }
    }
    #[doc = "Checks if the value of the field is `WPOL0_0`"]
    #[inline]
    pub fn is_wpol0_0(&self) -> bool {
        *self == WPOL0R::WPOL0_0
    }
    #[doc = "Checks if the value of the field is `WPOL0_1`"]
    #[inline]
    pub fn is_wpol0_1(&self) -> bool {
        *self == WPOL0R::WPOL0_1
    }
}
#[doc = "Possible values of the field `WPOL1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WPOL1R {
    #[doc = "Low active"]
    WPOL1_0,
    #[doc = "High active"]
    WPOL1_1,
}
impl WPOL1R {
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
            WPOL1R::WPOL1_0 => false,
            WPOL1R::WPOL1_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> WPOL1R {
        match value {
            false => WPOL1R::WPOL1_0,
            true => WPOL1R::WPOL1_1,
        }
    }
    #[doc = "Checks if the value of the field is `WPOL1_0`"]
    #[inline]
    pub fn is_wpol1_0(&self) -> bool {
        *self == WPOL1R::WPOL1_0
    }
    #[doc = "Checks if the value of the field is `WPOL1_1`"]
    #[inline]
    pub fn is_wpol1_1(&self) -> bool {
        *self == WPOL1R::WPOL1_1
    }
}
#[doc = r" Value of the field"]
pub struct CTOR {
    bits: u8,
}
impl CTOR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Possible values of the field `BTO`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BTOR {
    #[doc = "255*1"]
    BTO_0,
    #[doc = "255*2 - 255*2^30"]
    BTO_1,
    #[doc = "255*2 - 255*2^30"]
    BTO_2,
    #[doc = "255*2 - 255*2^30"]
    BTO_3,
    #[doc = "255*2 - 255*2^30"]
    BTO_4,
    #[doc = "255*2 - 255*2^30"]
    BTO_5,
    #[doc = "255*2 - 255*2^30"]
    BTO_6,
    #[doc = "255*2 - 255*2^30"]
    BTO_7,
    #[doc = "255*2 - 255*2^30"]
    BTO_8,
    #[doc = "255*2 - 255*2^30"]
    BTO_9,
    #[doc = "255*2^31"]
    BTO_31,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl BTOR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            BTOR::BTO_0 => 0,
            BTOR::BTO_1 => 1,
            BTOR::BTO_2 => 2,
            BTOR::BTO_3 => 3,
            BTOR::BTO_4 => 4,
            BTOR::BTO_5 => 5,
            BTOR::BTO_6 => 6,
            BTOR::BTO_7 => 7,
            BTOR::BTO_8 => 8,
            BTOR::BTO_9 => 9,
            BTOR::BTO_31 => 31,
            BTOR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> BTOR {
        match value {
            0 => BTOR::BTO_0,
            1 => BTOR::BTO_1,
            2 => BTOR::BTO_2,
            3 => BTOR::BTO_3,
            4 => BTOR::BTO_4,
            5 => BTOR::BTO_5,
            6 => BTOR::BTO_6,
            7 => BTOR::BTO_7,
            8 => BTOR::BTO_8,
            9 => BTOR::BTO_9,
            31 => BTOR::BTO_31,
            i => BTOR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `BTO_0`"]
    #[inline]
    pub fn is_bto_0(&self) -> bool {
        *self == BTOR::BTO_0
    }
    #[doc = "Checks if the value of the field is `BTO_1`"]
    #[inline]
    pub fn is_bto_1(&self) -> bool {
        *self == BTOR::BTO_1
    }
    #[doc = "Checks if the value of the field is `BTO_2`"]
    #[inline]
    pub fn is_bto_2(&self) -> bool {
        *self == BTOR::BTO_2
    }
    #[doc = "Checks if the value of the field is `BTO_3`"]
    #[inline]
    pub fn is_bto_3(&self) -> bool {
        *self == BTOR::BTO_3
    }
    #[doc = "Checks if the value of the field is `BTO_4`"]
    #[inline]
    pub fn is_bto_4(&self) -> bool {
        *self == BTOR::BTO_4
    }
    #[doc = "Checks if the value of the field is `BTO_5`"]
    #[inline]
    pub fn is_bto_5(&self) -> bool {
        *self == BTOR::BTO_5
    }
    #[doc = "Checks if the value of the field is `BTO_6`"]
    #[inline]
    pub fn is_bto_6(&self) -> bool {
        *self == BTOR::BTO_6
    }
    #[doc = "Checks if the value of the field is `BTO_7`"]
    #[inline]
    pub fn is_bto_7(&self) -> bool {
        *self == BTOR::BTO_7
    }
    #[doc = "Checks if the value of the field is `BTO_8`"]
    #[inline]
    pub fn is_bto_8(&self) -> bool {
        *self == BTOR::BTO_8
    }
    #[doc = "Checks if the value of the field is `BTO_9`"]
    #[inline]
    pub fn is_bto_9(&self) -> bool {
        *self == BTOR::BTO_9
    }
    #[doc = "Checks if the value of the field is `BTO_31`"]
    #[inline]
    pub fn is_bto_31(&self) -> bool {
        *self == BTOR::BTO_31
    }
}
#[doc = r" Proxy"]
pub struct _SWRSTW<'a> {
    w: &'a mut W,
}
impl<'a> _SWRSTW<'a> {
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
#[doc = "Values that can be written to the field `MDIS`"]
pub enum MDISW {
    #[doc = "Module enabled"]
    MDIS_0,
    #[doc = "Master disabled."]
    MDIS_1,
}
impl MDISW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            MDISW::MDIS_0 => false,
            MDISW::MDIS_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MDISW<'a> {
    w: &'a mut W,
}
impl<'a> _MDISW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MDISW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Module enabled"]
    #[inline]
    pub fn mdis_0(self) -> &'a mut W {
        self.variant(MDISW::MDIS_0)
    }
    #[doc = "Master disabled."]
    #[inline]
    pub fn mdis_1(self) -> &'a mut W {
        self.variant(MDISW::MDIS_1)
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
#[doc = "Values that can be written to the field `DQSMD`"]
pub enum DQSMDW {
    #[doc = "Dummy read strobe loopbacked internally"]
    DQSMD_0,
    #[doc = "Dummy read strobe loopbacked from DQS pad"]
    DQSMD_1,
}
impl DQSMDW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DQSMDW::DQSMD_0 => false,
            DQSMDW::DQSMD_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DQSMDW<'a> {
    w: &'a mut W,
}
impl<'a> _DQSMDW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DQSMDW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Dummy read strobe loopbacked internally"]
    #[inline]
    pub fn dqsmd_0(self) -> &'a mut W {
        self.variant(DQSMDW::DQSMD_0)
    }
    #[doc = "Dummy read strobe loopbacked from DQS pad"]
    #[inline]
    pub fn dqsmd_1(self) -> &'a mut W {
        self.variant(DQSMDW::DQSMD_1)
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
#[doc = "Values that can be written to the field `WPOL0`"]
pub enum WPOL0W {
    #[doc = "Low active"]
    WPOL0_0,
    #[doc = "High active"]
    WPOL0_1,
}
impl WPOL0W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            WPOL0W::WPOL0_0 => false,
            WPOL0W::WPOL0_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _WPOL0W<'a> {
    w: &'a mut W,
}
impl<'a> _WPOL0W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: WPOL0W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Low active"]
    #[inline]
    pub fn wpol0_0(self) -> &'a mut W {
        self.variant(WPOL0W::WPOL0_0)
    }
    #[doc = "High active"]
    #[inline]
    pub fn wpol0_1(self) -> &'a mut W {
        self.variant(WPOL0W::WPOL0_1)
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
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `WPOL1`"]
pub enum WPOL1W {
    #[doc = "Low active"]
    WPOL1_0,
    #[doc = "High active"]
    WPOL1_1,
}
impl WPOL1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            WPOL1W::WPOL1_0 => false,
            WPOL1W::WPOL1_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _WPOL1W<'a> {
    w: &'a mut W,
}
impl<'a> _WPOL1W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: WPOL1W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Low active"]
    #[inline]
    pub fn wpol1_0(self) -> &'a mut W {
        self.variant(WPOL1W::WPOL1_0)
    }
    #[doc = "High active"]
    #[inline]
    pub fn wpol1_1(self) -> &'a mut W {
        self.variant(WPOL1W::WPOL1_1)
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
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _CTOW<'a> {
    w: &'a mut W,
}
impl<'a> _CTOW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 255;
        const OFFSET: u8 = 16;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `BTO`"]
pub enum BTOW {
    #[doc = "255*1"]
    BTO_0,
    #[doc = "255*2 - 255*2^30"]
    BTO_1,
    #[doc = "255*2 - 255*2^30"]
    BTO_2,
    #[doc = "255*2 - 255*2^30"]
    BTO_3,
    #[doc = "255*2 - 255*2^30"]
    BTO_4,
    #[doc = "255*2 - 255*2^30"]
    BTO_5,
    #[doc = "255*2 - 255*2^30"]
    BTO_6,
    #[doc = "255*2 - 255*2^30"]
    BTO_7,
    #[doc = "255*2 - 255*2^30"]
    BTO_8,
    #[doc = "255*2 - 255*2^30"]
    BTO_9,
    #[doc = "255*2^31"]
    BTO_31,
}
impl BTOW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            BTOW::BTO_0 => 0,
            BTOW::BTO_1 => 1,
            BTOW::BTO_2 => 2,
            BTOW::BTO_3 => 3,
            BTOW::BTO_4 => 4,
            BTOW::BTO_5 => 5,
            BTOW::BTO_6 => 6,
            BTOW::BTO_7 => 7,
            BTOW::BTO_8 => 8,
            BTOW::BTO_9 => 9,
            BTOW::BTO_31 => 31,
        }
    }
}
#[doc = r" Proxy"]
pub struct _BTOW<'a> {
    w: &'a mut W,
}
impl<'a> _BTOW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: BTOW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "255*1"]
    #[inline]
    pub fn bto_0(self) -> &'a mut W {
        self.variant(BTOW::BTO_0)
    }
    #[doc = "255*2 - 255*2^30"]
    #[inline]
    pub fn bto_1(self) -> &'a mut W {
        self.variant(BTOW::BTO_1)
    }
    #[doc = "255*2 - 255*2^30"]
    #[inline]
    pub fn bto_2(self) -> &'a mut W {
        self.variant(BTOW::BTO_2)
    }
    #[doc = "255*2 - 255*2^30"]
    #[inline]
    pub fn bto_3(self) -> &'a mut W {
        self.variant(BTOW::BTO_3)
    }
    #[doc = "255*2 - 255*2^30"]
    #[inline]
    pub fn bto_4(self) -> &'a mut W {
        self.variant(BTOW::BTO_4)
    }
    #[doc = "255*2 - 255*2^30"]
    #[inline]
    pub fn bto_5(self) -> &'a mut W {
        self.variant(BTOW::BTO_5)
    }
    #[doc = "255*2 - 255*2^30"]
    #[inline]
    pub fn bto_6(self) -> &'a mut W {
        self.variant(BTOW::BTO_6)
    }
    #[doc = "255*2 - 255*2^30"]
    #[inline]
    pub fn bto_7(self) -> &'a mut W {
        self.variant(BTOW::BTO_7)
    }
    #[doc = "255*2 - 255*2^30"]
    #[inline]
    pub fn bto_8(self) -> &'a mut W {
        self.variant(BTOW::BTO_8)
    }
    #[doc = "255*2 - 255*2^30"]
    #[inline]
    pub fn bto_9(self) -> &'a mut W {
        self.variant(BTOW::BTO_9)
    }
    #[doc = "255*2^31"]
    #[inline]
    pub fn bto_31(self) -> &'a mut W {
        self.variant(BTOW::BTO_31)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 31;
        const OFFSET: u8 = 24;
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
    #[doc = "Bit 0 - Software Reset"]
    #[inline]
    pub fn swrst(&self) -> SWRSTR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        SWRSTR { bits }
    }
    #[doc = "Bit 1 - Module Disable"]
    #[inline]
    pub fn mdis(&self) -> MDISR {
        MDISR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - DQS (read strobe) mode"]
    #[inline]
    pub fn dqsmd(&self) -> DQSMDR {
        DQSMDR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 6 - WAIT/RDY# polarity for NOR/PSRAM"]
    #[inline]
    pub fn wpol0(&self) -> WPOL0R {
        WPOL0R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 7 - WAIT/RDY# polarity for NAND"]
    #[inline]
    pub fn wpol1(&self) -> WPOL1R {
        WPOL1R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 16:23 - Command Execution timeout cycles"]
    #[inline]
    pub fn cto(&self) -> CTOR {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        CTOR { bits }
    }
    #[doc = "Bits 24:28 - Bus timeout cycles"]
    #[inline]
    pub fn bto(&self) -> BTOR {
        BTOR::_from({
            const MASK: u8 = 31;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 268435458 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - Software Reset"]
    #[inline]
    pub fn swrst(&mut self) -> _SWRSTW {
        _SWRSTW { w: self }
    }
    #[doc = "Bit 1 - Module Disable"]
    #[inline]
    pub fn mdis(&mut self) -> _MDISW {
        _MDISW { w: self }
    }
    #[doc = "Bit 2 - DQS (read strobe) mode"]
    #[inline]
    pub fn dqsmd(&mut self) -> _DQSMDW {
        _DQSMDW { w: self }
    }
    #[doc = "Bit 6 - WAIT/RDY# polarity for NOR/PSRAM"]
    #[inline]
    pub fn wpol0(&mut self) -> _WPOL0W {
        _WPOL0W { w: self }
    }
    #[doc = "Bit 7 - WAIT/RDY# polarity for NAND"]
    #[inline]
    pub fn wpol1(&mut self) -> _WPOL1W {
        _WPOL1W { w: self }
    }
    #[doc = "Bits 16:23 - Command Execution timeout cycles"]
    #[inline]
    pub fn cto(&mut self) -> _CTOW {
        _CTOW { w: self }
    }
    #[doc = "Bits 24:28 - Bus timeout cycles"]
    #[inline]
    pub fn bto(&mut self) -> _BTOW {
        _BTOW { w: self }
    }
}
