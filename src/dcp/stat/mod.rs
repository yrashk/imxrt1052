#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::STAT {
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
pub struct IRQR {
    bits: u8,
}
impl IRQR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Possible values of the field `READY_CHANNELS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum READY_CHANNELSR {
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
impl READY_CHANNELSR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            READY_CHANNELSR::CH0 => 1,
            READY_CHANNELSR::CH1 => 2,
            READY_CHANNELSR::CH2 => 4,
            READY_CHANNELSR::CH3 => 8,
            READY_CHANNELSR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> READY_CHANNELSR {
        match value {
            1 => READY_CHANNELSR::CH0,
            2 => READY_CHANNELSR::CH1,
            4 => READY_CHANNELSR::CH2,
            8 => READY_CHANNELSR::CH3,
            i => READY_CHANNELSR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `CH0`"]
    #[inline]
    pub fn is_ch0(&self) -> bool {
        *self == READY_CHANNELSR::CH0
    }
    #[doc = "Checks if the value of the field is `CH1`"]
    #[inline]
    pub fn is_ch1(&self) -> bool {
        *self == READY_CHANNELSR::CH1
    }
    #[doc = "Checks if the value of the field is `CH2`"]
    #[inline]
    pub fn is_ch2(&self) -> bool {
        *self == READY_CHANNELSR::CH2
    }
    #[doc = "Checks if the value of the field is `CH3`"]
    #[inline]
    pub fn is_ch3(&self) -> bool {
        *self == READY_CHANNELSR::CH3
    }
}
#[doc = "Possible values of the field `CUR_CHANNEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CUR_CHANNELR {
    #[doc = "None"]
    NONE,
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
impl CUR_CHANNELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            CUR_CHANNELR::NONE => 0,
            CUR_CHANNELR::CH0 => 1,
            CUR_CHANNELR::CH1 => 2,
            CUR_CHANNELR::CH2 => 3,
            CUR_CHANNELR::CH3 => 4,
            CUR_CHANNELR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> CUR_CHANNELR {
        match value {
            0 => CUR_CHANNELR::NONE,
            1 => CUR_CHANNELR::CH0,
            2 => CUR_CHANNELR::CH1,
            3 => CUR_CHANNELR::CH2,
            4 => CUR_CHANNELR::CH3,
            i => CUR_CHANNELR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline]
    pub fn is_none(&self) -> bool {
        *self == CUR_CHANNELR::NONE
    }
    #[doc = "Checks if the value of the field is `CH0`"]
    #[inline]
    pub fn is_ch0(&self) -> bool {
        *self == CUR_CHANNELR::CH0
    }
    #[doc = "Checks if the value of the field is `CH1`"]
    #[inline]
    pub fn is_ch1(&self) -> bool {
        *self == CUR_CHANNELR::CH1
    }
    #[doc = "Checks if the value of the field is `CH2`"]
    #[inline]
    pub fn is_ch2(&self) -> bool {
        *self == CUR_CHANNELR::CH2
    }
    #[doc = "Checks if the value of the field is `CH3`"]
    #[inline]
    pub fn is_ch3(&self) -> bool {
        *self == CUR_CHANNELR::CH3
    }
}
#[doc = r" Value of the field"]
pub struct OTP_KEY_READYR {
    bits: bool,
}
impl OTP_KEY_READYR {
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
#[doc = r" Proxy"]
pub struct _IRQW<'a> {
    w: &'a mut W,
}
impl<'a> _IRQW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 0;
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
    #[doc = "Bits 0:3 - Indicates which channels have pending interrupt requests"]
    #[inline]
    pub fn irq(&self) -> IRQR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        IRQR { bits }
    }
    #[doc = "Bits 16:23 - Indicates which channels are ready to proceed with a transfer (the active channel is also included)"]
    #[inline]
    pub fn ready_channels(&self) -> READY_CHANNELSR {
        READY_CHANNELSR::_from({
            const MASK: u8 = 255;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 24:27 - Current (active) channel (encoded)"]
    #[inline]
    pub fn cur_channel(&self) -> CUR_CHANNELR {
        CUR_CHANNELR::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 28 - When set, it indicates that the OTP key is shifted from the fuse block and is ready for use."]
    #[inline]
    pub fn otp_key_ready(&self) -> OTP_KEY_READYR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 28;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        OTP_KEY_READYR { bits }
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 268435456 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:3 - Indicates which channels have pending interrupt requests"]
    #[inline]
    pub fn irq(&mut self) -> _IRQW {
        _IRQW { w: self }
    }
}
