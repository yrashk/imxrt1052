#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::SHIFTCFG {
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
#[doc = "Possible values of the field `SSTART`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SSTARTR {
    #[doc = "Start bit disabled for transmitter/receiver/match store, transmitter loads data on enable"]
    SSTART_0,
    #[doc = "Start bit disabled for transmitter/receiver/match store, transmitter loads data on first shift"]
    SSTART_1,
    #[doc = "Transmitter outputs start bit value 0 before loading data on first shift, receiver/match store sets error flag if start bit is not 0"]
    SSTART_2,
    #[doc = "Transmitter outputs start bit value 1 before loading data on first shift, receiver/match store sets error flag if start bit is not 1"]
    SSTART_3,
}
impl SSTARTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            SSTARTR::SSTART_0 => 0,
            SSTARTR::SSTART_1 => 1,
            SSTARTR::SSTART_2 => 2,
            SSTARTR::SSTART_3 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> SSTARTR {
        match value {
            0 => SSTARTR::SSTART_0,
            1 => SSTARTR::SSTART_1,
            2 => SSTARTR::SSTART_2,
            3 => SSTARTR::SSTART_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `SSTART_0`"]
    #[inline]
    pub fn is_sstart_0(&self) -> bool {
        *self == SSTARTR::SSTART_0
    }
    #[doc = "Checks if the value of the field is `SSTART_1`"]
    #[inline]
    pub fn is_sstart_1(&self) -> bool {
        *self == SSTARTR::SSTART_1
    }
    #[doc = "Checks if the value of the field is `SSTART_2`"]
    #[inline]
    pub fn is_sstart_2(&self) -> bool {
        *self == SSTARTR::SSTART_2
    }
    #[doc = "Checks if the value of the field is `SSTART_3`"]
    #[inline]
    pub fn is_sstart_3(&self) -> bool {
        *self == SSTARTR::SSTART_3
    }
}
#[doc = "Possible values of the field `SSTOP`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SSTOPR {
    #[doc = "Stop bit disabled for transmitter/receiver/match store"]
    SSTOP_0,
    #[doc = "Transmitter outputs stop bit value 0 on store, receiver/match store sets error flag if stop bit is not 0"]
    SSTOP_2,
    #[doc = "Transmitter outputs stop bit value 1 on store, receiver/match store sets error flag if stop bit is not 1"]
    SSTOP_3,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl SSTOPR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            SSTOPR::SSTOP_0 => 0,
            SSTOPR::SSTOP_2 => 2,
            SSTOPR::SSTOP_3 => 3,
            SSTOPR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> SSTOPR {
        match value {
            0 => SSTOPR::SSTOP_0,
            2 => SSTOPR::SSTOP_2,
            3 => SSTOPR::SSTOP_3,
            i => SSTOPR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `SSTOP_0`"]
    #[inline]
    pub fn is_sstop_0(&self) -> bool {
        *self == SSTOPR::SSTOP_0
    }
    #[doc = "Checks if the value of the field is `SSTOP_2`"]
    #[inline]
    pub fn is_sstop_2(&self) -> bool {
        *self == SSTOPR::SSTOP_2
    }
    #[doc = "Checks if the value of the field is `SSTOP_3`"]
    #[inline]
    pub fn is_sstop_3(&self) -> bool {
        *self == SSTOPR::SSTOP_3
    }
}
#[doc = "Possible values of the field `INSRC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INSRCR {
    #[doc = "Pin"]
    INSRC_0,
    #[doc = "Shifter N+1 Output"]
    INSRC_1,
}
impl INSRCR {
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
            INSRCR::INSRC_0 => false,
            INSRCR::INSRC_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> INSRCR {
        match value {
            false => INSRCR::INSRC_0,
            true => INSRCR::INSRC_1,
        }
    }
    #[doc = "Checks if the value of the field is `INSRC_0`"]
    #[inline]
    pub fn is_insrc_0(&self) -> bool {
        *self == INSRCR::INSRC_0
    }
    #[doc = "Checks if the value of the field is `INSRC_1`"]
    #[inline]
    pub fn is_insrc_1(&self) -> bool {
        *self == INSRCR::INSRC_1
    }
}
#[doc = r" Value of the field"]
pub struct PWIDTHR {
    bits: u8,
}
impl PWIDTHR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Values that can be written to the field `SSTART`"]
pub enum SSTARTW {
    #[doc = "Start bit disabled for transmitter/receiver/match store, transmitter loads data on enable"]
    SSTART_0,
    #[doc = "Start bit disabled for transmitter/receiver/match store, transmitter loads data on first shift"]
    SSTART_1,
    #[doc = "Transmitter outputs start bit value 0 before loading data on first shift, receiver/match store sets error flag if start bit is not 0"]
    SSTART_2,
    #[doc = "Transmitter outputs start bit value 1 before loading data on first shift, receiver/match store sets error flag if start bit is not 1"]
    SSTART_3,
}
impl SSTARTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            SSTARTW::SSTART_0 => 0,
            SSTARTW::SSTART_1 => 1,
            SSTARTW::SSTART_2 => 2,
            SSTARTW::SSTART_3 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SSTARTW<'a> {
    w: &'a mut W,
}
impl<'a> _SSTARTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SSTARTW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Start bit disabled for transmitter/receiver/match store, transmitter loads data on enable"]
    #[inline]
    pub fn sstart_0(self) -> &'a mut W {
        self.variant(SSTARTW::SSTART_0)
    }
    #[doc = "Start bit disabled for transmitter/receiver/match store, transmitter loads data on first shift"]
    #[inline]
    pub fn sstart_1(self) -> &'a mut W {
        self.variant(SSTARTW::SSTART_1)
    }
    #[doc = "Transmitter outputs start bit value 0 before loading data on first shift, receiver/match store sets error flag if start bit is not 0"]
    #[inline]
    pub fn sstart_2(self) -> &'a mut W {
        self.variant(SSTARTW::SSTART_2)
    }
    #[doc = "Transmitter outputs start bit value 1 before loading data on first shift, receiver/match store sets error flag if start bit is not 1"]
    #[inline]
    pub fn sstart_3(self) -> &'a mut W {
        self.variant(SSTARTW::SSTART_3)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `SSTOP`"]
pub enum SSTOPW {
    #[doc = "Stop bit disabled for transmitter/receiver/match store"]
    SSTOP_0,
    #[doc = "Transmitter outputs stop bit value 0 on store, receiver/match store sets error flag if stop bit is not 0"]
    SSTOP_2,
    #[doc = "Transmitter outputs stop bit value 1 on store, receiver/match store sets error flag if stop bit is not 1"]
    SSTOP_3,
}
impl SSTOPW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            SSTOPW::SSTOP_0 => 0,
            SSTOPW::SSTOP_2 => 2,
            SSTOPW::SSTOP_3 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SSTOPW<'a> {
    w: &'a mut W,
}
impl<'a> _SSTOPW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SSTOPW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Stop bit disabled for transmitter/receiver/match store"]
    #[inline]
    pub fn sstop_0(self) -> &'a mut W {
        self.variant(SSTOPW::SSTOP_0)
    }
    #[doc = "Transmitter outputs stop bit value 0 on store, receiver/match store sets error flag if stop bit is not 0"]
    #[inline]
    pub fn sstop_2(self) -> &'a mut W {
        self.variant(SSTOPW::SSTOP_2)
    }
    #[doc = "Transmitter outputs stop bit value 1 on store, receiver/match store sets error flag if stop bit is not 1"]
    #[inline]
    pub fn sstop_3(self) -> &'a mut W {
        self.variant(SSTOPW::SSTOP_3)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 4;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `INSRC`"]
pub enum INSRCW {
    #[doc = "Pin"]
    INSRC_0,
    #[doc = "Shifter N+1 Output"]
    INSRC_1,
}
impl INSRCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            INSRCW::INSRC_0 => false,
            INSRCW::INSRC_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _INSRCW<'a> {
    w: &'a mut W,
}
impl<'a> _INSRCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: INSRCW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Pin"]
    #[inline]
    pub fn insrc_0(self) -> &'a mut W {
        self.variant(INSRCW::INSRC_0)
    }
    #[doc = "Shifter N+1 Output"]
    #[inline]
    pub fn insrc_1(self) -> &'a mut W {
        self.variant(INSRCW::INSRC_1)
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
#[doc = r" Proxy"]
pub struct _PWIDTHW<'a> {
    w: &'a mut W,
}
impl<'a> _PWIDTHW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 31;
        const OFFSET: u8 = 16;
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
    #[doc = "Bits 0:1 - Shifter Start bit"]
    #[inline]
    pub fn sstart(&self) -> SSTARTR {
        SSTARTR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 4:5 - Shifter Stop bit"]
    #[inline]
    pub fn sstop(&self) -> SSTOPR {
        SSTOPR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 8 - Input Source"]
    #[inline]
    pub fn insrc(&self) -> INSRCR {
        INSRCR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 16:20 - Parallel Width"]
    #[inline]
    pub fn pwidth(&self) -> PWIDTHR {
        let bits = {
            const MASK: u8 = 31;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        PWIDTHR { bits }
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
    #[doc = "Bits 0:1 - Shifter Start bit"]
    #[inline]
    pub fn sstart(&mut self) -> _SSTARTW {
        _SSTARTW { w: self }
    }
    #[doc = "Bits 4:5 - Shifter Stop bit"]
    #[inline]
    pub fn sstop(&mut self) -> _SSTOPW {
        _SSTOPW { w: self }
    }
    #[doc = "Bit 8 - Input Source"]
    #[inline]
    pub fn insrc(&mut self) -> _INSRCW {
        _INSRCW { w: self }
    }
    #[doc = "Bits 16:20 - Parallel Width"]
    #[inline]
    pub fn pwidth(&mut self) -> _PWIDTHW {
        _PWIDTHW { w: self }
    }
}
