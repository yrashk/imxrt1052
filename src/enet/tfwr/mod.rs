#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::TFWR {
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
#[doc = "Possible values of the field `TFWR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TFWRR {
    #[doc = "64 bytes written."]
    TFWR_0,
    #[doc = "64 bytes written."]
    TFWR_1,
    #[doc = "128 bytes written."]
    TFWR_2,
    #[doc = "192 bytes written."]
    TFWR_3,
    #[doc = "1984 bytes written."]
    TFWR_31,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl TFWRR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            TFWRR::TFWR_0 => 0,
            TFWRR::TFWR_1 => 1,
            TFWRR::TFWR_2 => 2,
            TFWRR::TFWR_3 => 3,
            TFWRR::TFWR_31 => 31,
            TFWRR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> TFWRR {
        match value {
            0 => TFWRR::TFWR_0,
            1 => TFWRR::TFWR_1,
            2 => TFWRR::TFWR_2,
            3 => TFWRR::TFWR_3,
            31 => TFWRR::TFWR_31,
            i => TFWRR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `TFWR_0`"]
    #[inline]
    pub fn is_tfwr_0(&self) -> bool {
        *self == TFWRR::TFWR_0
    }
    #[doc = "Checks if the value of the field is `TFWR_1`"]
    #[inline]
    pub fn is_tfwr_1(&self) -> bool {
        *self == TFWRR::TFWR_1
    }
    #[doc = "Checks if the value of the field is `TFWR_2`"]
    #[inline]
    pub fn is_tfwr_2(&self) -> bool {
        *self == TFWRR::TFWR_2
    }
    #[doc = "Checks if the value of the field is `TFWR_3`"]
    #[inline]
    pub fn is_tfwr_3(&self) -> bool {
        *self == TFWRR::TFWR_3
    }
    #[doc = "Checks if the value of the field is `TFWR_31`"]
    #[inline]
    pub fn is_tfwr_31(&self) -> bool {
        *self == TFWRR::TFWR_31
    }
}
#[doc = "Possible values of the field `STRFWD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum STRFWDR {
    #[doc = "Reset. The transmission start threshold is programmed in TFWR[TFWR]."]
    STRFWD_0,
    #[doc = "Enabled."]
    STRFWD_1,
}
impl STRFWDR {
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
            STRFWDR::STRFWD_0 => false,
            STRFWDR::STRFWD_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> STRFWDR {
        match value {
            false => STRFWDR::STRFWD_0,
            true => STRFWDR::STRFWD_1,
        }
    }
    #[doc = "Checks if the value of the field is `STRFWD_0`"]
    #[inline]
    pub fn is_strfwd_0(&self) -> bool {
        *self == STRFWDR::STRFWD_0
    }
    #[doc = "Checks if the value of the field is `STRFWD_1`"]
    #[inline]
    pub fn is_strfwd_1(&self) -> bool {
        *self == STRFWDR::STRFWD_1
    }
}
#[doc = "Values that can be written to the field `TFWR`"]
pub enum TFWRW {
    #[doc = "64 bytes written."]
    TFWR_0,
    #[doc = "64 bytes written."]
    TFWR_1,
    #[doc = "128 bytes written."]
    TFWR_2,
    #[doc = "192 bytes written."]
    TFWR_3,
    #[doc = "1984 bytes written."]
    TFWR_31,
}
impl TFWRW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            TFWRW::TFWR_0 => 0,
            TFWRW::TFWR_1 => 1,
            TFWRW::TFWR_2 => 2,
            TFWRW::TFWR_3 => 3,
            TFWRW::TFWR_31 => 31,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TFWRW<'a> {
    w: &'a mut W,
}
impl<'a> _TFWRW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TFWRW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "64 bytes written."]
    #[inline]
    pub fn tfwr_0(self) -> &'a mut W {
        self.variant(TFWRW::TFWR_0)
    }
    #[doc = "64 bytes written."]
    #[inline]
    pub fn tfwr_1(self) -> &'a mut W {
        self.variant(TFWRW::TFWR_1)
    }
    #[doc = "128 bytes written."]
    #[inline]
    pub fn tfwr_2(self) -> &'a mut W {
        self.variant(TFWRW::TFWR_2)
    }
    #[doc = "192 bytes written."]
    #[inline]
    pub fn tfwr_3(self) -> &'a mut W {
        self.variant(TFWRW::TFWR_3)
    }
    #[doc = "1984 bytes written."]
    #[inline]
    pub fn tfwr_31(self) -> &'a mut W {
        self.variant(TFWRW::TFWR_31)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 63;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `STRFWD`"]
pub enum STRFWDW {
    #[doc = "Reset. The transmission start threshold is programmed in TFWR[TFWR]."]
    STRFWD_0,
    #[doc = "Enabled."]
    STRFWD_1,
}
impl STRFWDW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            STRFWDW::STRFWD_0 => false,
            STRFWDW::STRFWD_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _STRFWDW<'a> {
    w: &'a mut W,
}
impl<'a> _STRFWDW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: STRFWDW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Reset. The transmission start threshold is programmed in TFWR[TFWR]."]
    #[inline]
    pub fn strfwd_0(self) -> &'a mut W {
        self.variant(STRFWDW::STRFWD_0)
    }
    #[doc = "Enabled."]
    #[inline]
    pub fn strfwd_1(self) -> &'a mut W {
        self.variant(STRFWDW::STRFWD_1)
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
    #[doc = "Bits 0:5 - Transmit FIFO Write"]
    #[inline]
    pub fn tfwr(&self) -> TFWRR {
        TFWRR::_from({
            const MASK: u8 = 63;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 8 - Store And Forward Enable"]
    #[inline]
    pub fn strfwd(&self) -> STRFWDR {
        STRFWDR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
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
    #[doc = "Bits 0:5 - Transmit FIFO Write"]
    #[inline]
    pub fn tfwr(&mut self) -> _TFWRW {
        _TFWRW { w: self }
    }
    #[doc = "Bit 8 - Store And Forward Enable"]
    #[inline]
    pub fn strfwd(&mut self) -> _STRFWDW {
        _STRFWDW { w: self }
    }
}
