#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::ROMPATCHCNTL {
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
#[doc = "Possible values of the field `DATAFIX`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DATAFIXR {
    #[doc = "Address comparator triggers a opcode patch"]
    DATAFIX_0,
    #[doc = "Address comparator triggers a data fix"]
    DATAFIX_1,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl DATAFIXR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            DATAFIXR::DATAFIX_0 => 0,
            DATAFIXR::DATAFIX_1 => 1,
            DATAFIXR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> DATAFIXR {
        match value {
            0 => DATAFIXR::DATAFIX_0,
            1 => DATAFIXR::DATAFIX_1,
            i => DATAFIXR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `DATAFIX_0`"]
    #[inline]
    pub fn is_datafix_0(&self) -> bool {
        *self == DATAFIXR::DATAFIX_0
    }
    #[doc = "Checks if the value of the field is `DATAFIX_1`"]
    #[inline]
    pub fn is_datafix_1(&self) -> bool {
        *self == DATAFIXR::DATAFIX_1
    }
}
#[doc = "Possible values of the field `DIS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DISR {
    #[doc = "Does not affect any ROMC functions (default)"]
    DIS_0,
    #[doc = "Disable all ROMC functions: data fixing, and opcode patching"]
    DIS_1,
}
impl DISR {
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
            DISR::DIS_0 => false,
            DISR::DIS_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DISR {
        match value {
            false => DISR::DIS_0,
            true => DISR::DIS_1,
        }
    }
    #[doc = "Checks if the value of the field is `DIS_0`"]
    #[inline]
    pub fn is_dis_0(&self) -> bool {
        *self == DISR::DIS_0
    }
    #[doc = "Checks if the value of the field is `DIS_1`"]
    #[inline]
    pub fn is_dis_1(&self) -> bool {
        *self == DISR::DIS_1
    }
}
#[doc = "Values that can be written to the field `DATAFIX`"]
pub enum DATAFIXW {
    #[doc = "Address comparator triggers a opcode patch"]
    DATAFIX_0,
    #[doc = "Address comparator triggers a data fix"]
    DATAFIX_1,
}
impl DATAFIXW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            DATAFIXW::DATAFIX_0 => 0,
            DATAFIXW::DATAFIX_1 => 1,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DATAFIXW<'a> {
    w: &'a mut W,
}
impl<'a> _DATAFIXW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DATAFIXW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Address comparator triggers a opcode patch"]
    #[inline]
    pub fn datafix_0(self) -> &'a mut W {
        self.variant(DATAFIXW::DATAFIX_0)
    }
    #[doc = "Address comparator triggers a data fix"]
    #[inline]
    pub fn datafix_1(self) -> &'a mut W {
        self.variant(DATAFIXW::DATAFIX_1)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 255;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `DIS`"]
pub enum DISW {
    #[doc = "Does not affect any ROMC functions (default)"]
    DIS_0,
    #[doc = "Disable all ROMC functions: data fixing, and opcode patching"]
    DIS_1,
}
impl DISW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DISW::DIS_0 => false,
            DISW::DIS_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DISW<'a> {
    w: &'a mut W,
}
impl<'a> _DISW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DISW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Does not affect any ROMC functions (default)"]
    #[inline]
    pub fn dis_0(self) -> &'a mut W {
        self.variant(DISW::DIS_0)
    }
    #[doc = "Disable all ROMC functions: data fixing, and opcode patching"]
    #[inline]
    pub fn dis_1(self) -> &'a mut W {
        self.variant(DISW::DIS_1)
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
        const OFFSET: u8 = 29;
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
    #[doc = "Bits 0:7 - Data Fix Enable - Controls the use of the first 8 address comparators for 1-word data fix or for code patch routine"]
    #[inline]
    pub fn datafix(&self) -> DATAFIXR {
        DATAFIXR::_from({
            const MASK: u8 = 255;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 29 - ROMC Disable -- This bit, when set, disables all ROMC operations"]
    #[inline]
    pub fn dis(&self) -> DISR {
        DISR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 29;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 138412032 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:7 - Data Fix Enable - Controls the use of the first 8 address comparators for 1-word data fix or for code patch routine"]
    #[inline]
    pub fn datafix(&mut self) -> _DATAFIXW {
        _DATAFIXW { w: self }
    }
    #[doc = "Bit 29 - ROMC Disable -- This bit, when set, disables all ROMC operations"]
    #[inline]
    pub fn dis(&mut self) -> _DISW {
        _DISW { w: self }
    }
}
