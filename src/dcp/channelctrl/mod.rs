#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CHANNELCTRL {
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
#[doc = "Possible values of the field `ENABLE_CHANNEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ENABLE_CHANNELR {
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
impl ENABLE_CHANNELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            ENABLE_CHANNELR::CH0 => 1,
            ENABLE_CHANNELR::CH1 => 2,
            ENABLE_CHANNELR::CH2 => 4,
            ENABLE_CHANNELR::CH3 => 8,
            ENABLE_CHANNELR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> ENABLE_CHANNELR {
        match value {
            1 => ENABLE_CHANNELR::CH0,
            2 => ENABLE_CHANNELR::CH1,
            4 => ENABLE_CHANNELR::CH2,
            8 => ENABLE_CHANNELR::CH3,
            i => ENABLE_CHANNELR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `CH0`"]
    #[inline]
    pub fn is_ch0(&self) -> bool {
        *self == ENABLE_CHANNELR::CH0
    }
    #[doc = "Checks if the value of the field is `CH1`"]
    #[inline]
    pub fn is_ch1(&self) -> bool {
        *self == ENABLE_CHANNELR::CH1
    }
    #[doc = "Checks if the value of the field is `CH2`"]
    #[inline]
    pub fn is_ch2(&self) -> bool {
        *self == ENABLE_CHANNELR::CH2
    }
    #[doc = "Checks if the value of the field is `CH3`"]
    #[inline]
    pub fn is_ch3(&self) -> bool {
        *self == ENABLE_CHANNELR::CH3
    }
}
#[doc = "Possible values of the field `HIGH_PRIORITY_CHANNEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HIGH_PRIORITY_CHANNELR {
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
impl HIGH_PRIORITY_CHANNELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            HIGH_PRIORITY_CHANNELR::CH0 => 1,
            HIGH_PRIORITY_CHANNELR::CH1 => 2,
            HIGH_PRIORITY_CHANNELR::CH2 => 4,
            HIGH_PRIORITY_CHANNELR::CH3 => 8,
            HIGH_PRIORITY_CHANNELR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> HIGH_PRIORITY_CHANNELR {
        match value {
            1 => HIGH_PRIORITY_CHANNELR::CH0,
            2 => HIGH_PRIORITY_CHANNELR::CH1,
            4 => HIGH_PRIORITY_CHANNELR::CH2,
            8 => HIGH_PRIORITY_CHANNELR::CH3,
            i => HIGH_PRIORITY_CHANNELR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `CH0`"]
    #[inline]
    pub fn is_ch0(&self) -> bool {
        *self == HIGH_PRIORITY_CHANNELR::CH0
    }
    #[doc = "Checks if the value of the field is `CH1`"]
    #[inline]
    pub fn is_ch1(&self) -> bool {
        *self == HIGH_PRIORITY_CHANNELR::CH1
    }
    #[doc = "Checks if the value of the field is `CH2`"]
    #[inline]
    pub fn is_ch2(&self) -> bool {
        *self == HIGH_PRIORITY_CHANNELR::CH2
    }
    #[doc = "Checks if the value of the field is `CH3`"]
    #[inline]
    pub fn is_ch3(&self) -> bool {
        *self == HIGH_PRIORITY_CHANNELR::CH3
    }
}
#[doc = r" Value of the field"]
pub struct CH0_IRQ_MERGEDR {
    bits: bool,
}
impl CH0_IRQ_MERGEDR {
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
#[doc = "Values that can be written to the field `ENABLE_CHANNEL`"]
pub enum ENABLE_CHANNELW {
    #[doc = "CH0"]
    CH0,
    #[doc = "CH1"]
    CH1,
    #[doc = "CH2"]
    CH2,
    #[doc = "CH3"]
    CH3,
}
impl ENABLE_CHANNELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            ENABLE_CHANNELW::CH0 => 1,
            ENABLE_CHANNELW::CH1 => 2,
            ENABLE_CHANNELW::CH2 => 4,
            ENABLE_CHANNELW::CH3 => 8,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ENABLE_CHANNELW<'a> {
    w: &'a mut W,
}
impl<'a> _ENABLE_CHANNELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ENABLE_CHANNELW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "CH0"]
    #[inline]
    pub fn ch0(self) -> &'a mut W {
        self.variant(ENABLE_CHANNELW::CH0)
    }
    #[doc = "CH1"]
    #[inline]
    pub fn ch1(self) -> &'a mut W {
        self.variant(ENABLE_CHANNELW::CH1)
    }
    #[doc = "CH2"]
    #[inline]
    pub fn ch2(self) -> &'a mut W {
        self.variant(ENABLE_CHANNELW::CH2)
    }
    #[doc = "CH3"]
    #[inline]
    pub fn ch3(self) -> &'a mut W {
        self.variant(ENABLE_CHANNELW::CH3)
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
#[doc = "Values that can be written to the field `HIGH_PRIORITY_CHANNEL`"]
pub enum HIGH_PRIORITY_CHANNELW {
    #[doc = "CH0"]
    CH0,
    #[doc = "CH1"]
    CH1,
    #[doc = "CH2"]
    CH2,
    #[doc = "CH3"]
    CH3,
}
impl HIGH_PRIORITY_CHANNELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            HIGH_PRIORITY_CHANNELW::CH0 => 1,
            HIGH_PRIORITY_CHANNELW::CH1 => 2,
            HIGH_PRIORITY_CHANNELW::CH2 => 4,
            HIGH_PRIORITY_CHANNELW::CH3 => 8,
        }
    }
}
#[doc = r" Proxy"]
pub struct _HIGH_PRIORITY_CHANNELW<'a> {
    w: &'a mut W,
}
impl<'a> _HIGH_PRIORITY_CHANNELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: HIGH_PRIORITY_CHANNELW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "CH0"]
    #[inline]
    pub fn ch0(self) -> &'a mut W {
        self.variant(HIGH_PRIORITY_CHANNELW::CH0)
    }
    #[doc = "CH1"]
    #[inline]
    pub fn ch1(self) -> &'a mut W {
        self.variant(HIGH_PRIORITY_CHANNELW::CH1)
    }
    #[doc = "CH2"]
    #[inline]
    pub fn ch2(self) -> &'a mut W {
        self.variant(HIGH_PRIORITY_CHANNELW::CH2)
    }
    #[doc = "CH3"]
    #[inline]
    pub fn ch3(self) -> &'a mut W {
        self.variant(HIGH_PRIORITY_CHANNELW::CH3)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 255;
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _CH0_IRQ_MERGEDW<'a> {
    w: &'a mut W,
}
impl<'a> _CH0_IRQ_MERGEDW<'a> {
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
    #[doc = "Bits 0:7 - Setting a bit in this field enables the DMA channel associated with it"]
    #[inline]
    pub fn enable_channel(&self) -> ENABLE_CHANNELR {
        ENABLE_CHANNELR::_from({
            const MASK: u8 = 255;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 8:15 - Setting a bit in this field causes the corresponding channel to have high-priority arbitration"]
    #[inline]
    pub fn high_priority_channel(&self) -> HIGH_PRIORITY_CHANNELR {
        HIGH_PRIORITY_CHANNELR::_from({
            const MASK: u8 = 255;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 16 - Indicates that the interrupt for channel 0 must be merged with the other interrupts on the shared dcp_irq interrupt"]
    #[inline]
    pub fn ch0_irq_merged(&self) -> CH0_IRQ_MERGEDR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        CH0_IRQ_MERGEDR { bits }
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
    #[doc = "Bits 0:7 - Setting a bit in this field enables the DMA channel associated with it"]
    #[inline]
    pub fn enable_channel(&mut self) -> _ENABLE_CHANNELW {
        _ENABLE_CHANNELW { w: self }
    }
    #[doc = "Bits 8:15 - Setting a bit in this field causes the corresponding channel to have high-priority arbitration"]
    #[inline]
    pub fn high_priority_channel(&mut self) -> _HIGH_PRIORITY_CHANNELW {
        _HIGH_PRIORITY_CHANNELW { w: self }
    }
    #[doc = "Bit 16 - Indicates that the interrupt for channel 0 must be merged with the other interrupts on the shared dcp_irq interrupt"]
    #[inline]
    pub fn ch0_irq_merged(&mut self) -> _CH0_IRQ_MERGEDW {
        _CH0_IRQ_MERGEDW { w: self }
    }
}
