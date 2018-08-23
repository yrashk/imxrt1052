#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CTRL {
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
#[doc = "Possible values of the field `CHANNEL_INTERRUPT_ENABLE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CHANNEL_INTERRUPT_ENABLER {
    #[doc = "CH0"]
    CH0,
    #[doc = "CH1"]
    CH1,
    #[doc = "CH2"]
    CH2,
    #[doc = "CH3"]
    CH3,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl CHANNEL_INTERRUPT_ENABLER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            CHANNEL_INTERRUPT_ENABLER::CH0 => 1,
            CHANNEL_INTERRUPT_ENABLER::CH1 => 2,
            CHANNEL_INTERRUPT_ENABLER::CH2 => 4,
            CHANNEL_INTERRUPT_ENABLER::CH3 => 8,
            CHANNEL_INTERRUPT_ENABLER::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> CHANNEL_INTERRUPT_ENABLER {
        match value {
            1 => CHANNEL_INTERRUPT_ENABLER::CH0,
            2 => CHANNEL_INTERRUPT_ENABLER::CH1,
            4 => CHANNEL_INTERRUPT_ENABLER::CH2,
            8 => CHANNEL_INTERRUPT_ENABLER::CH3,
            i => CHANNEL_INTERRUPT_ENABLER::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `CH0`"]
    #[inline]
    pub fn is_ch0(&self) -> bool {
        *self == CHANNEL_INTERRUPT_ENABLER::CH0
    }
    #[doc = "Checks if the value of the field is `CH1`"]
    #[inline]
    pub fn is_ch1(&self) -> bool {
        *self == CHANNEL_INTERRUPT_ENABLER::CH1
    }
    #[doc = "Checks if the value of the field is `CH2`"]
    #[inline]
    pub fn is_ch2(&self) -> bool {
        *self == CHANNEL_INTERRUPT_ENABLER::CH2
    }
    #[doc = "Checks if the value of the field is `CH3`"]
    #[inline]
    pub fn is_ch3(&self) -> bool {
        *self == CHANNEL_INTERRUPT_ENABLER::CH3
    }
}
#[doc = r" Value of the field"]
pub struct ENABLE_CONTEXT_SWITCHINGR {
    bits: bool,
}
impl ENABLE_CONTEXT_SWITCHINGR {
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
#[doc = r" Value of the field"]
pub struct ENABLE_CONTEXT_CACHINGR {
    bits: bool,
}
impl ENABLE_CONTEXT_CACHINGR {
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
#[doc = r" Value of the field"]
pub struct GATHER_RESIDUAL_WRITESR {
    bits: bool,
}
impl GATHER_RESIDUAL_WRITESR {
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
#[doc = "Possible values of the field `PRESENT_SHA`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PRESENT_SHAR {
    #[doc = "Absent"]
    ABSENT,
    #[doc = "Present"]
    PRESENT,
}
impl PRESENT_SHAR {
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
            PRESENT_SHAR::ABSENT => false,
            PRESENT_SHAR::PRESENT => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PRESENT_SHAR {
        match value {
            false => PRESENT_SHAR::ABSENT,
            true => PRESENT_SHAR::PRESENT,
        }
    }
    #[doc = "Checks if the value of the field is `ABSENT`"]
    #[inline]
    pub fn is_absent(&self) -> bool {
        *self == PRESENT_SHAR::ABSENT
    }
    #[doc = "Checks if the value of the field is `PRESENT`"]
    #[inline]
    pub fn is_present(&self) -> bool {
        *self == PRESENT_SHAR::PRESENT
    }
}
#[doc = "Possible values of the field `PRESENT_CRYPTO`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PRESENT_CRYPTOR {
    #[doc = "Absent"]
    ABSENT,
    #[doc = "Present"]
    PRESENT,
}
impl PRESENT_CRYPTOR {
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
            PRESENT_CRYPTOR::ABSENT => false,
            PRESENT_CRYPTOR::PRESENT => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PRESENT_CRYPTOR {
        match value {
            false => PRESENT_CRYPTOR::ABSENT,
            true => PRESENT_CRYPTOR::PRESENT,
        }
    }
    #[doc = "Checks if the value of the field is `ABSENT`"]
    #[inline]
    pub fn is_absent(&self) -> bool {
        *self == PRESENT_CRYPTOR::ABSENT
    }
    #[doc = "Checks if the value of the field is `PRESENT`"]
    #[inline]
    pub fn is_present(&self) -> bool {
        *self == PRESENT_CRYPTOR::PRESENT
    }
}
#[doc = r" Value of the field"]
pub struct CLKGATER {
    bits: bool,
}
impl CLKGATER {
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
#[doc = r" Value of the field"]
pub struct SFTRSTR {
    bits: bool,
}
impl SFTRSTR {
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
#[doc = "Values that can be written to the field `CHANNEL_INTERRUPT_ENABLE`"]
pub enum CHANNEL_INTERRUPT_ENABLEW {
    #[doc = "CH0"]
    CH0,
    #[doc = "CH1"]
    CH1,
    #[doc = "CH2"]
    CH2,
    #[doc = "CH3"]
    CH3,
}
impl CHANNEL_INTERRUPT_ENABLEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            CHANNEL_INTERRUPT_ENABLEW::CH0 => 1,
            CHANNEL_INTERRUPT_ENABLEW::CH1 => 2,
            CHANNEL_INTERRUPT_ENABLEW::CH2 => 4,
            CHANNEL_INTERRUPT_ENABLEW::CH3 => 8,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CHANNEL_INTERRUPT_ENABLEW<'a> {
    w: &'a mut W,
}
impl<'a> _CHANNEL_INTERRUPT_ENABLEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CHANNEL_INTERRUPT_ENABLEW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "CH0"]
    #[inline]
    pub fn ch0(self) -> &'a mut W {
        self.variant(CHANNEL_INTERRUPT_ENABLEW::CH0)
    }
    #[doc = "CH1"]
    #[inline]
    pub fn ch1(self) -> &'a mut W {
        self.variant(CHANNEL_INTERRUPT_ENABLEW::CH1)
    }
    #[doc = "CH2"]
    #[inline]
    pub fn ch2(self) -> &'a mut W {
        self.variant(CHANNEL_INTERRUPT_ENABLEW::CH2)
    }
    #[doc = "CH3"]
    #[inline]
    pub fn ch3(self) -> &'a mut W {
        self.variant(CHANNEL_INTERRUPT_ENABLEW::CH3)
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
#[doc = r" Proxy"]
pub struct _ENABLE_CONTEXT_SWITCHINGW<'a> {
    w: &'a mut W,
}
impl<'a> _ENABLE_CONTEXT_SWITCHINGW<'a> {
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
        const OFFSET: u8 = 21;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _ENABLE_CONTEXT_CACHINGW<'a> {
    w: &'a mut W,
}
impl<'a> _ENABLE_CONTEXT_CACHINGW<'a> {
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
        const OFFSET: u8 = 22;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _GATHER_RESIDUAL_WRITESW<'a> {
    w: &'a mut W,
}
impl<'a> _GATHER_RESIDUAL_WRITESW<'a> {
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
        const OFFSET: u8 = 23;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _CLKGATEW<'a> {
    w: &'a mut W,
}
impl<'a> _CLKGATEW<'a> {
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
        const OFFSET: u8 = 30;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _SFTRSTW<'a> {
    w: &'a mut W,
}
impl<'a> _SFTRSTW<'a> {
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
        const OFFSET: u8 = 31;
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
    #[doc = "Bits 0:7 - Per-channel interrupt enable bit"]
    #[inline]
    pub fn channel_interrupt_enable(&self) -> CHANNEL_INTERRUPT_ENABLER {
        CHANNEL_INTERRUPT_ENABLER::_from({
            const MASK: u8 = 255;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 21 - Enable automatic context switching for the channels"]
    #[inline]
    pub fn enable_context_switching(&self) -> ENABLE_CONTEXT_SWITCHINGR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 21;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        ENABLE_CONTEXT_SWITCHINGR { bits }
    }
    #[doc = "Bit 22 - The software must set this bit to enable the caching of contexts between the operations"]
    #[inline]
    pub fn enable_context_caching(&self) -> ENABLE_CONTEXT_CACHINGR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 22;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        ENABLE_CONTEXT_CACHINGR { bits }
    }
    #[doc = "Bit 23 - The software must set this bit to enable the ragged writes to the unaligned buffers to be gathered between multiple write operations"]
    #[inline]
    pub fn gather_residual_writes(&self) -> GATHER_RESIDUAL_WRITESR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 23;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        GATHER_RESIDUAL_WRITESR { bits }
    }
    #[doc = "Bit 28 - Indicates whether the SHA1/SHA2 functions are present."]
    #[inline]
    pub fn present_sha(&self) -> PRESENT_SHAR {
        PRESENT_SHAR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 28;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 29 - Indicates whether the crypto (cipher/hash) functions are present."]
    #[inline]
    pub fn present_crypto(&self) -> PRESENT_CRYPTOR {
        PRESENT_CRYPTOR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 29;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 30 - This bit must be set to zero for a normal operation"]
    #[inline]
    pub fn clkgate(&self) -> CLKGATER {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 30;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        CLKGATER { bits }
    }
    #[doc = "Bit 31 - Set this bit to zero to enable a normal DCP operation"]
    #[inline]
    pub fn sftrst(&self) -> SFTRSTR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 31;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        SFTRSTR { bits }
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 4034920448 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:7 - Per-channel interrupt enable bit"]
    #[inline]
    pub fn channel_interrupt_enable(&mut self) -> _CHANNEL_INTERRUPT_ENABLEW {
        _CHANNEL_INTERRUPT_ENABLEW { w: self }
    }
    #[doc = "Bit 21 - Enable automatic context switching for the channels"]
    #[inline]
    pub fn enable_context_switching(&mut self) -> _ENABLE_CONTEXT_SWITCHINGW {
        _ENABLE_CONTEXT_SWITCHINGW { w: self }
    }
    #[doc = "Bit 22 - The software must set this bit to enable the caching of contexts between the operations"]
    #[inline]
    pub fn enable_context_caching(&mut self) -> _ENABLE_CONTEXT_CACHINGW {
        _ENABLE_CONTEXT_CACHINGW { w: self }
    }
    #[doc = "Bit 23 - The software must set this bit to enable the ragged writes to the unaligned buffers to be gathered between multiple write operations"]
    #[inline]
    pub fn gather_residual_writes(&mut self) -> _GATHER_RESIDUAL_WRITESW {
        _GATHER_RESIDUAL_WRITESW { w: self }
    }
    #[doc = "Bit 30 - This bit must be set to zero for a normal operation"]
    #[inline]
    pub fn clkgate(&mut self) -> _CLKGATEW {
        _CLKGATEW { w: self }
    }
    #[doc = "Bit 31 - Set this bit to zero to enable a normal DCP operation"]
    #[inline]
    pub fn sftrst(&mut self) -> _SFTRSTW {
        _SFTRSTW { w: self }
    }
}
