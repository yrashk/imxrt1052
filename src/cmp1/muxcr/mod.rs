#[doc = r" Value read from the register"]
pub struct R {
    bits: u8,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u8,
}
impl super::MUXCR {
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
#[doc = "Possible values of the field `MSEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MSELR {
    #[doc = "IN0"]
    MSEL_0,
    #[doc = "IN1"]
    MSEL_1,
    #[doc = "IN2"]
    MSEL_2,
    #[doc = "IN3"]
    MSEL_3,
    #[doc = "IN4"]
    MSEL_4,
    #[doc = "IN5"]
    MSEL_5,
    #[doc = "IN6"]
    MSEL_6,
    #[doc = "IN7"]
    MSEL_7,
}
impl MSELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            MSELR::MSEL_0 => 0,
            MSELR::MSEL_1 => 1,
            MSELR::MSEL_2 => 2,
            MSELR::MSEL_3 => 3,
            MSELR::MSEL_4 => 4,
            MSELR::MSEL_5 => 5,
            MSELR::MSEL_6 => 6,
            MSELR::MSEL_7 => 7,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> MSELR {
        match value {
            0 => MSELR::MSEL_0,
            1 => MSELR::MSEL_1,
            2 => MSELR::MSEL_2,
            3 => MSELR::MSEL_3,
            4 => MSELR::MSEL_4,
            5 => MSELR::MSEL_5,
            6 => MSELR::MSEL_6,
            7 => MSELR::MSEL_7,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `MSEL_0`"]
    #[inline]
    pub fn is_msel_0(&self) -> bool {
        *self == MSELR::MSEL_0
    }
    #[doc = "Checks if the value of the field is `MSEL_1`"]
    #[inline]
    pub fn is_msel_1(&self) -> bool {
        *self == MSELR::MSEL_1
    }
    #[doc = "Checks if the value of the field is `MSEL_2`"]
    #[inline]
    pub fn is_msel_2(&self) -> bool {
        *self == MSELR::MSEL_2
    }
    #[doc = "Checks if the value of the field is `MSEL_3`"]
    #[inline]
    pub fn is_msel_3(&self) -> bool {
        *self == MSELR::MSEL_3
    }
    #[doc = "Checks if the value of the field is `MSEL_4`"]
    #[inline]
    pub fn is_msel_4(&self) -> bool {
        *self == MSELR::MSEL_4
    }
    #[doc = "Checks if the value of the field is `MSEL_5`"]
    #[inline]
    pub fn is_msel_5(&self) -> bool {
        *self == MSELR::MSEL_5
    }
    #[doc = "Checks if the value of the field is `MSEL_6`"]
    #[inline]
    pub fn is_msel_6(&self) -> bool {
        *self == MSELR::MSEL_6
    }
    #[doc = "Checks if the value of the field is `MSEL_7`"]
    #[inline]
    pub fn is_msel_7(&self) -> bool {
        *self == MSELR::MSEL_7
    }
}
#[doc = "Possible values of the field `PSEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PSELR {
    #[doc = "IN0"]
    PSEL_0,
    #[doc = "IN1"]
    PSEL_1,
    #[doc = "IN2"]
    PSEL_2,
    #[doc = "IN3"]
    PSEL_3,
    #[doc = "IN4"]
    PSEL_4,
    #[doc = "IN5"]
    PSEL_5,
    #[doc = "IN6"]
    PSEL_6,
    #[doc = "IN7"]
    PSEL_7,
}
impl PSELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            PSELR::PSEL_0 => 0,
            PSELR::PSEL_1 => 1,
            PSELR::PSEL_2 => 2,
            PSELR::PSEL_3 => 3,
            PSELR::PSEL_4 => 4,
            PSELR::PSEL_5 => 5,
            PSELR::PSEL_6 => 6,
            PSELR::PSEL_7 => 7,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> PSELR {
        match value {
            0 => PSELR::PSEL_0,
            1 => PSELR::PSEL_1,
            2 => PSELR::PSEL_2,
            3 => PSELR::PSEL_3,
            4 => PSELR::PSEL_4,
            5 => PSELR::PSEL_5,
            6 => PSELR::PSEL_6,
            7 => PSELR::PSEL_7,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `PSEL_0`"]
    #[inline]
    pub fn is_psel_0(&self) -> bool {
        *self == PSELR::PSEL_0
    }
    #[doc = "Checks if the value of the field is `PSEL_1`"]
    #[inline]
    pub fn is_psel_1(&self) -> bool {
        *self == PSELR::PSEL_1
    }
    #[doc = "Checks if the value of the field is `PSEL_2`"]
    #[inline]
    pub fn is_psel_2(&self) -> bool {
        *self == PSELR::PSEL_2
    }
    #[doc = "Checks if the value of the field is `PSEL_3`"]
    #[inline]
    pub fn is_psel_3(&self) -> bool {
        *self == PSELR::PSEL_3
    }
    #[doc = "Checks if the value of the field is `PSEL_4`"]
    #[inline]
    pub fn is_psel_4(&self) -> bool {
        *self == PSELR::PSEL_4
    }
    #[doc = "Checks if the value of the field is `PSEL_5`"]
    #[inline]
    pub fn is_psel_5(&self) -> bool {
        *self == PSELR::PSEL_5
    }
    #[doc = "Checks if the value of the field is `PSEL_6`"]
    #[inline]
    pub fn is_psel_6(&self) -> bool {
        *self == PSELR::PSEL_6
    }
    #[doc = "Checks if the value of the field is `PSEL_7`"]
    #[inline]
    pub fn is_psel_7(&self) -> bool {
        *self == PSELR::PSEL_7
    }
}
#[doc = "Values that can be written to the field `MSEL`"]
pub enum MSELW {
    #[doc = "IN0"]
    MSEL_0,
    #[doc = "IN1"]
    MSEL_1,
    #[doc = "IN2"]
    MSEL_2,
    #[doc = "IN3"]
    MSEL_3,
    #[doc = "IN4"]
    MSEL_4,
    #[doc = "IN5"]
    MSEL_5,
    #[doc = "IN6"]
    MSEL_6,
    #[doc = "IN7"]
    MSEL_7,
}
impl MSELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            MSELW::MSEL_0 => 0,
            MSELW::MSEL_1 => 1,
            MSELW::MSEL_2 => 2,
            MSELW::MSEL_3 => 3,
            MSELW::MSEL_4 => 4,
            MSELW::MSEL_5 => 5,
            MSELW::MSEL_6 => 6,
            MSELW::MSEL_7 => 7,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MSELW<'a> {
    w: &'a mut W,
}
impl<'a> _MSELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MSELW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "IN0"]
    #[inline]
    pub fn msel_0(self) -> &'a mut W {
        self.variant(MSELW::MSEL_0)
    }
    #[doc = "IN1"]
    #[inline]
    pub fn msel_1(self) -> &'a mut W {
        self.variant(MSELW::MSEL_1)
    }
    #[doc = "IN2"]
    #[inline]
    pub fn msel_2(self) -> &'a mut W {
        self.variant(MSELW::MSEL_2)
    }
    #[doc = "IN3"]
    #[inline]
    pub fn msel_3(self) -> &'a mut W {
        self.variant(MSELW::MSEL_3)
    }
    #[doc = "IN4"]
    #[inline]
    pub fn msel_4(self) -> &'a mut W {
        self.variant(MSELW::MSEL_4)
    }
    #[doc = "IN5"]
    #[inline]
    pub fn msel_5(self) -> &'a mut W {
        self.variant(MSELW::MSEL_5)
    }
    #[doc = "IN6"]
    #[inline]
    pub fn msel_6(self) -> &'a mut W {
        self.variant(MSELW::MSEL_6)
    }
    #[doc = "IN7"]
    #[inline]
    pub fn msel_7(self) -> &'a mut W {
        self.variant(MSELW::MSEL_7)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u8) << OFFSET);
        self.w.bits |= ((value & MASK) as u8) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `PSEL`"]
pub enum PSELW {
    #[doc = "IN0"]
    PSEL_0,
    #[doc = "IN1"]
    PSEL_1,
    #[doc = "IN2"]
    PSEL_2,
    #[doc = "IN3"]
    PSEL_3,
    #[doc = "IN4"]
    PSEL_4,
    #[doc = "IN5"]
    PSEL_5,
    #[doc = "IN6"]
    PSEL_6,
    #[doc = "IN7"]
    PSEL_7,
}
impl PSELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            PSELW::PSEL_0 => 0,
            PSELW::PSEL_1 => 1,
            PSELW::PSEL_2 => 2,
            PSELW::PSEL_3 => 3,
            PSELW::PSEL_4 => 4,
            PSELW::PSEL_5 => 5,
            PSELW::PSEL_6 => 6,
            PSELW::PSEL_7 => 7,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PSELW<'a> {
    w: &'a mut W,
}
impl<'a> _PSELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PSELW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "IN0"]
    #[inline]
    pub fn psel_0(self) -> &'a mut W {
        self.variant(PSELW::PSEL_0)
    }
    #[doc = "IN1"]
    #[inline]
    pub fn psel_1(self) -> &'a mut W {
        self.variant(PSELW::PSEL_1)
    }
    #[doc = "IN2"]
    #[inline]
    pub fn psel_2(self) -> &'a mut W {
        self.variant(PSELW::PSEL_2)
    }
    #[doc = "IN3"]
    #[inline]
    pub fn psel_3(self) -> &'a mut W {
        self.variant(PSELW::PSEL_3)
    }
    #[doc = "IN4"]
    #[inline]
    pub fn psel_4(self) -> &'a mut W {
        self.variant(PSELW::PSEL_4)
    }
    #[doc = "IN5"]
    #[inline]
    pub fn psel_5(self) -> &'a mut W {
        self.variant(PSELW::PSEL_5)
    }
    #[doc = "IN6"]
    #[inline]
    pub fn psel_6(self) -> &'a mut W {
        self.variant(PSELW::PSEL_6)
    }
    #[doc = "IN7"]
    #[inline]
    pub fn psel_7(self) -> &'a mut W {
        self.variant(PSELW::PSEL_7)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 3;
        self.w.bits &= !((MASK as u8) << OFFSET);
        self.w.bits |= ((value & MASK) as u8) << OFFSET;
        self.w
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
    #[doc = "Bits 0:2 - Minus Input Mux Control"]
    #[inline]
    pub fn msel(&self) -> MSELR {
        MSELR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u8) as u8
        })
    }
    #[doc = "Bits 3:5 - Plus Input Mux Control"]
    #[inline]
    pub fn psel(&self) -> PSELR {
        PSELR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u8) as u8
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
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:2 - Minus Input Mux Control"]
    #[inline]
    pub fn msel(&mut self) -> _MSELW {
        _MSELW { w: self }
    }
    #[doc = "Bits 3:5 - Plus Input Mux Control"]
    #[inline]
    pub fn psel(&mut self) -> _PSELW {
        _PSELW { w: self }
    }
}
