#[doc = r" Value read from the register"]
pub struct R {
    bits: u16,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u16,
}
impl super::TCD22_ATTR {
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
pub struct DSIZER {
    bits: u8,
}
impl DSIZER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct DMODR {
    bits: u8,
}
impl DMODR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Possible values of the field `SSIZE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SSIZER {
    #[doc = "8-bit"]
    SSIZE_0,
    #[doc = "16-bit"]
    SSIZE_1,
    #[doc = "32-bit"]
    SSIZE_2,
    #[doc = "no description available"]
    SSIZE_3,
    #[doc = "no description available"]
    SSIZE_5,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl SSIZER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            SSIZER::SSIZE_0 => 0,
            SSIZER::SSIZE_1 => 1,
            SSIZER::SSIZE_2 => 2,
            SSIZER::SSIZE_3 => 3,
            SSIZER::SSIZE_5 => 5,
            SSIZER::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> SSIZER {
        match value {
            0 => SSIZER::SSIZE_0,
            1 => SSIZER::SSIZE_1,
            2 => SSIZER::SSIZE_2,
            3 => SSIZER::SSIZE_3,
            5 => SSIZER::SSIZE_5,
            i => SSIZER::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `SSIZE_0`"]
    #[inline]
    pub fn is_ssize_0(&self) -> bool {
        *self == SSIZER::SSIZE_0
    }
    #[doc = "Checks if the value of the field is `SSIZE_1`"]
    #[inline]
    pub fn is_ssize_1(&self) -> bool {
        *self == SSIZER::SSIZE_1
    }
    #[doc = "Checks if the value of the field is `SSIZE_2`"]
    #[inline]
    pub fn is_ssize_2(&self) -> bool {
        *self == SSIZER::SSIZE_2
    }
    #[doc = "Checks if the value of the field is `SSIZE_3`"]
    #[inline]
    pub fn is_ssize_3(&self) -> bool {
        *self == SSIZER::SSIZE_3
    }
    #[doc = "Checks if the value of the field is `SSIZE_5`"]
    #[inline]
    pub fn is_ssize_5(&self) -> bool {
        *self == SSIZER::SSIZE_5
    }
}
#[doc = "Possible values of the field `SMOD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SMODR {
    #[doc = "Source address modulo feature is disabled"]
    SMOD_0,
    #[doc = "This value defines a specific address range specified to be the value after SADDR + SOFF calculation is performed on the original register value. Setting this field provides the ability to implement a circular data queue easily. For data queues requiring power-of-2 size bytes, the queue should start at a 0-modulo-size address and the SMOD field should be set to the appropriate value for the queue, freezing the desired number of upper address bits. The value programmed into this field specifies the number of lower address bits allowed to change. For a circular queue application, the SOFF is typically set to the transfer size to implement post-increment addressing with the SMOD function constraining the addresses to a 0-modulo-size range."]
    SMOD_1,
    #[doc = "This value defines a specific address range specified to be the value after SADDR + SOFF calculation is performed on the original register value. Setting this field provides the ability to implement a circular data queue easily. For data queues requiring power-of-2 size bytes, the queue should start at a 0-modulo-size address and the SMOD field should be set to the appropriate value for the queue, freezing the desired number of upper address bits. The value programmed into this field specifies the number of lower address bits allowed to change. For a circular queue application, the SOFF is typically set to the transfer size to implement post-increment addressing with the SMOD function constraining the addresses to a 0-modulo-size range."]
    SMOD_2,
    #[doc = "This value defines a specific address range specified to be the value after SADDR + SOFF calculation is performed on the original register value. Setting this field provides the ability to implement a circular data queue easily. For data queues requiring power-of-2 size bytes, the queue should start at a 0-modulo-size address and the SMOD field should be set to the appropriate value for the queue, freezing the desired number of upper address bits. The value programmed into this field specifies the number of lower address bits allowed to change. For a circular queue application, the SOFF is typically set to the transfer size to implement post-increment addressing with the SMOD function constraining the addresses to a 0-modulo-size range."]
    SMOD_3,
    #[doc = "This value defines a specific address range specified to be the value after SADDR + SOFF calculation is performed on the original register value. Setting this field provides the ability to implement a circular data queue easily. For data queues requiring power-of-2 size bytes, the queue should start at a 0-modulo-size address and the SMOD field should be set to the appropriate value for the queue, freezing the desired number of upper address bits. The value programmed into this field specifies the number of lower address bits allowed to change. For a circular queue application, the SOFF is typically set to the transfer size to implement post-increment addressing with the SMOD function constraining the addresses to a 0-modulo-size range."]
    SMOD_4,
    #[doc = "This value defines a specific address range specified to be the value after SADDR + SOFF calculation is performed on the original register value. Setting this field provides the ability to implement a circular data queue easily. For data queues requiring power-of-2 size bytes, the queue should start at a 0-modulo-size address and the SMOD field should be set to the appropriate value for the queue, freezing the desired number of upper address bits. The value programmed into this field specifies the number of lower address bits allowed to change. For a circular queue application, the SOFF is typically set to the transfer size to implement post-increment addressing with the SMOD function constraining the addresses to a 0-modulo-size range."]
    SMOD_5,
    #[doc = "This value defines a specific address range specified to be the value after SADDR + SOFF calculation is performed on the original register value. Setting this field provides the ability to implement a circular data queue easily. For data queues requiring power-of-2 size bytes, the queue should start at a 0-modulo-size address and the SMOD field should be set to the appropriate value for the queue, freezing the desired number of upper address bits. The value programmed into this field specifies the number of lower address bits allowed to change. For a circular queue application, the SOFF is typically set to the transfer size to implement post-increment addressing with the SMOD function constraining the addresses to a 0-modulo-size range."]
    SMOD_6,
    #[doc = "This value defines a specific address range specified to be the value after SADDR + SOFF calculation is performed on the original register value. Setting this field provides the ability to implement a circular data queue easily. For data queues requiring power-of-2 size bytes, the queue should start at a 0-modulo-size address and the SMOD field should be set to the appropriate value for the queue, freezing the desired number of upper address bits. The value programmed into this field specifies the number of lower address bits allowed to change. For a circular queue application, the SOFF is typically set to the transfer size to implement post-increment addressing with the SMOD function constraining the addresses to a 0-modulo-size range."]
    SMOD_7,
    #[doc = "This value defines a specific address range specified to be the value after SADDR + SOFF calculation is performed on the original register value. Setting this field provides the ability to implement a circular data queue easily. For data queues requiring power-of-2 size bytes, the queue should start at a 0-modulo-size address and the SMOD field should be set to the appropriate value for the queue, freezing the desired number of upper address bits. The value programmed into this field specifies the number of lower address bits allowed to change. For a circular queue application, the SOFF is typically set to the transfer size to implement post-increment addressing with the SMOD function constraining the addresses to a 0-modulo-size range."]
    SMOD_8,
    #[doc = "This value defines a specific address range specified to be the value after SADDR + SOFF calculation is performed on the original register value. Setting this field provides the ability to implement a circular data queue easily. For data queues requiring power-of-2 size bytes, the queue should start at a 0-modulo-size address and the SMOD field should be set to the appropriate value for the queue, freezing the desired number of upper address bits. The value programmed into this field specifies the number of lower address bits allowed to change. For a circular queue application, the SOFF is typically set to the transfer size to implement post-increment addressing with the SMOD function constraining the addresses to a 0-modulo-size range."]
    SMOD_9,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl SMODR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            SMODR::SMOD_0 => 0,
            SMODR::SMOD_1 => 1,
            SMODR::SMOD_2 => 2,
            SMODR::SMOD_3 => 3,
            SMODR::SMOD_4 => 4,
            SMODR::SMOD_5 => 5,
            SMODR::SMOD_6 => 6,
            SMODR::SMOD_7 => 7,
            SMODR::SMOD_8 => 8,
            SMODR::SMOD_9 => 9,
            SMODR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> SMODR {
        match value {
            0 => SMODR::SMOD_0,
            1 => SMODR::SMOD_1,
            2 => SMODR::SMOD_2,
            3 => SMODR::SMOD_3,
            4 => SMODR::SMOD_4,
            5 => SMODR::SMOD_5,
            6 => SMODR::SMOD_6,
            7 => SMODR::SMOD_7,
            8 => SMODR::SMOD_8,
            9 => SMODR::SMOD_9,
            i => SMODR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `SMOD_0`"]
    #[inline]
    pub fn is_smod_0(&self) -> bool {
        *self == SMODR::SMOD_0
    }
    #[doc = "Checks if the value of the field is `SMOD_1`"]
    #[inline]
    pub fn is_smod_1(&self) -> bool {
        *self == SMODR::SMOD_1
    }
    #[doc = "Checks if the value of the field is `SMOD_2`"]
    #[inline]
    pub fn is_smod_2(&self) -> bool {
        *self == SMODR::SMOD_2
    }
    #[doc = "Checks if the value of the field is `SMOD_3`"]
    #[inline]
    pub fn is_smod_3(&self) -> bool {
        *self == SMODR::SMOD_3
    }
    #[doc = "Checks if the value of the field is `SMOD_4`"]
    #[inline]
    pub fn is_smod_4(&self) -> bool {
        *self == SMODR::SMOD_4
    }
    #[doc = "Checks if the value of the field is `SMOD_5`"]
    #[inline]
    pub fn is_smod_5(&self) -> bool {
        *self == SMODR::SMOD_5
    }
    #[doc = "Checks if the value of the field is `SMOD_6`"]
    #[inline]
    pub fn is_smod_6(&self) -> bool {
        *self == SMODR::SMOD_6
    }
    #[doc = "Checks if the value of the field is `SMOD_7`"]
    #[inline]
    pub fn is_smod_7(&self) -> bool {
        *self == SMODR::SMOD_7
    }
    #[doc = "Checks if the value of the field is `SMOD_8`"]
    #[inline]
    pub fn is_smod_8(&self) -> bool {
        *self == SMODR::SMOD_8
    }
    #[doc = "Checks if the value of the field is `SMOD_9`"]
    #[inline]
    pub fn is_smod_9(&self) -> bool {
        *self == SMODR::SMOD_9
    }
}
#[doc = r" Proxy"]
pub struct _DSIZEW<'a> {
    w: &'a mut W,
}
impl<'a> _DSIZEW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u16) << OFFSET);
        self.w.bits |= ((value & MASK) as u16) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _DMODW<'a> {
    w: &'a mut W,
}
impl<'a> _DMODW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 31;
        const OFFSET: u8 = 3;
        self.w.bits &= !((MASK as u16) << OFFSET);
        self.w.bits |= ((value & MASK) as u16) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `SSIZE`"]
pub enum SSIZEW {
    #[doc = "8-bit"]
    SSIZE_0,
    #[doc = "16-bit"]
    SSIZE_1,
    #[doc = "32-bit"]
    SSIZE_2,
    #[doc = "no description available"]
    SSIZE_3,
    #[doc = "no description available"]
    SSIZE_5,
}
impl SSIZEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            SSIZEW::SSIZE_0 => 0,
            SSIZEW::SSIZE_1 => 1,
            SSIZEW::SSIZE_2 => 2,
            SSIZEW::SSIZE_3 => 3,
            SSIZEW::SSIZE_5 => 5,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SSIZEW<'a> {
    w: &'a mut W,
}
impl<'a> _SSIZEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SSIZEW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "8-bit"]
    #[inline]
    pub fn ssize_0(self) -> &'a mut W {
        self.variant(SSIZEW::SSIZE_0)
    }
    #[doc = "16-bit"]
    #[inline]
    pub fn ssize_1(self) -> &'a mut W {
        self.variant(SSIZEW::SSIZE_1)
    }
    #[doc = "32-bit"]
    #[inline]
    pub fn ssize_2(self) -> &'a mut W {
        self.variant(SSIZEW::SSIZE_2)
    }
    #[doc = "no description available"]
    #[inline]
    pub fn ssize_3(self) -> &'a mut W {
        self.variant(SSIZEW::SSIZE_3)
    }
    #[doc = "no description available"]
    #[inline]
    pub fn ssize_5(self) -> &'a mut W {
        self.variant(SSIZEW::SSIZE_5)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u16) << OFFSET);
        self.w.bits |= ((value & MASK) as u16) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `SMOD`"]
pub enum SMODW {
    #[doc = "Source address modulo feature is disabled"]
    SMOD_0,
    #[doc = "This value defines a specific address range specified to be the value after SADDR + SOFF calculation is performed on the original register value. Setting this field provides the ability to implement a circular data queue easily. For data queues requiring power-of-2 size bytes, the queue should start at a 0-modulo-size address and the SMOD field should be set to the appropriate value for the queue, freezing the desired number of upper address bits. The value programmed into this field specifies the number of lower address bits allowed to change. For a circular queue application, the SOFF is typically set to the transfer size to implement post-increment addressing with the SMOD function constraining the addresses to a 0-modulo-size range."]
    SMOD_1,
    #[doc = "This value defines a specific address range specified to be the value after SADDR + SOFF calculation is performed on the original register value. Setting this field provides the ability to implement a circular data queue easily. For data queues requiring power-of-2 size bytes, the queue should start at a 0-modulo-size address and the SMOD field should be set to the appropriate value for the queue, freezing the desired number of upper address bits. The value programmed into this field specifies the number of lower address bits allowed to change. For a circular queue application, the SOFF is typically set to the transfer size to implement post-increment addressing with the SMOD function constraining the addresses to a 0-modulo-size range."]
    SMOD_2,
    #[doc = "This value defines a specific address range specified to be the value after SADDR + SOFF calculation is performed on the original register value. Setting this field provides the ability to implement a circular data queue easily. For data queues requiring power-of-2 size bytes, the queue should start at a 0-modulo-size address and the SMOD field should be set to the appropriate value for the queue, freezing the desired number of upper address bits. The value programmed into this field specifies the number of lower address bits allowed to change. For a circular queue application, the SOFF is typically set to the transfer size to implement post-increment addressing with the SMOD function constraining the addresses to a 0-modulo-size range."]
    SMOD_3,
    #[doc = "This value defines a specific address range specified to be the value after SADDR + SOFF calculation is performed on the original register value. Setting this field provides the ability to implement a circular data queue easily. For data queues requiring power-of-2 size bytes, the queue should start at a 0-modulo-size address and the SMOD field should be set to the appropriate value for the queue, freezing the desired number of upper address bits. The value programmed into this field specifies the number of lower address bits allowed to change. For a circular queue application, the SOFF is typically set to the transfer size to implement post-increment addressing with the SMOD function constraining the addresses to a 0-modulo-size range."]
    SMOD_4,
    #[doc = "This value defines a specific address range specified to be the value after SADDR + SOFF calculation is performed on the original register value. Setting this field provides the ability to implement a circular data queue easily. For data queues requiring power-of-2 size bytes, the queue should start at a 0-modulo-size address and the SMOD field should be set to the appropriate value for the queue, freezing the desired number of upper address bits. The value programmed into this field specifies the number of lower address bits allowed to change. For a circular queue application, the SOFF is typically set to the transfer size to implement post-increment addressing with the SMOD function constraining the addresses to a 0-modulo-size range."]
    SMOD_5,
    #[doc = "This value defines a specific address range specified to be the value after SADDR + SOFF calculation is performed on the original register value. Setting this field provides the ability to implement a circular data queue easily. For data queues requiring power-of-2 size bytes, the queue should start at a 0-modulo-size address and the SMOD field should be set to the appropriate value for the queue, freezing the desired number of upper address bits. The value programmed into this field specifies the number of lower address bits allowed to change. For a circular queue application, the SOFF is typically set to the transfer size to implement post-increment addressing with the SMOD function constraining the addresses to a 0-modulo-size range."]
    SMOD_6,
    #[doc = "This value defines a specific address range specified to be the value after SADDR + SOFF calculation is performed on the original register value. Setting this field provides the ability to implement a circular data queue easily. For data queues requiring power-of-2 size bytes, the queue should start at a 0-modulo-size address and the SMOD field should be set to the appropriate value for the queue, freezing the desired number of upper address bits. The value programmed into this field specifies the number of lower address bits allowed to change. For a circular queue application, the SOFF is typically set to the transfer size to implement post-increment addressing with the SMOD function constraining the addresses to a 0-modulo-size range."]
    SMOD_7,
    #[doc = "This value defines a specific address range specified to be the value after SADDR + SOFF calculation is performed on the original register value. Setting this field provides the ability to implement a circular data queue easily. For data queues requiring power-of-2 size bytes, the queue should start at a 0-modulo-size address and the SMOD field should be set to the appropriate value for the queue, freezing the desired number of upper address bits. The value programmed into this field specifies the number of lower address bits allowed to change. For a circular queue application, the SOFF is typically set to the transfer size to implement post-increment addressing with the SMOD function constraining the addresses to a 0-modulo-size range."]
    SMOD_8,
    #[doc = "This value defines a specific address range specified to be the value after SADDR + SOFF calculation is performed on the original register value. Setting this field provides the ability to implement a circular data queue easily. For data queues requiring power-of-2 size bytes, the queue should start at a 0-modulo-size address and the SMOD field should be set to the appropriate value for the queue, freezing the desired number of upper address bits. The value programmed into this field specifies the number of lower address bits allowed to change. For a circular queue application, the SOFF is typically set to the transfer size to implement post-increment addressing with the SMOD function constraining the addresses to a 0-modulo-size range."]
    SMOD_9,
}
impl SMODW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            SMODW::SMOD_0 => 0,
            SMODW::SMOD_1 => 1,
            SMODW::SMOD_2 => 2,
            SMODW::SMOD_3 => 3,
            SMODW::SMOD_4 => 4,
            SMODW::SMOD_5 => 5,
            SMODW::SMOD_6 => 6,
            SMODW::SMOD_7 => 7,
            SMODW::SMOD_8 => 8,
            SMODW::SMOD_9 => 9,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SMODW<'a> {
    w: &'a mut W,
}
impl<'a> _SMODW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SMODW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Source address modulo feature is disabled"]
    #[inline]
    pub fn smod_0(self) -> &'a mut W {
        self.variant(SMODW::SMOD_0)
    }
    #[doc = "This value defines a specific address range specified to be the value after SADDR + SOFF calculation is performed on the original register value. Setting this field provides the ability to implement a circular data queue easily. For data queues requiring power-of-2 size bytes, the queue should start at a 0-modulo-size address and the SMOD field should be set to the appropriate value for the queue, freezing the desired number of upper address bits. The value programmed into this field specifies the number of lower address bits allowed to change. For a circular queue application, the SOFF is typically set to the transfer size to implement post-increment addressing with the SMOD function constraining the addresses to a 0-modulo-size range."]
    #[inline]
    pub fn smod_1(self) -> &'a mut W {
        self.variant(SMODW::SMOD_1)
    }
    #[doc = "This value defines a specific address range specified to be the value after SADDR + SOFF calculation is performed on the original register value. Setting this field provides the ability to implement a circular data queue easily. For data queues requiring power-of-2 size bytes, the queue should start at a 0-modulo-size address and the SMOD field should be set to the appropriate value for the queue, freezing the desired number of upper address bits. The value programmed into this field specifies the number of lower address bits allowed to change. For a circular queue application, the SOFF is typically set to the transfer size to implement post-increment addressing with the SMOD function constraining the addresses to a 0-modulo-size range."]
    #[inline]
    pub fn smod_2(self) -> &'a mut W {
        self.variant(SMODW::SMOD_2)
    }
    #[doc = "This value defines a specific address range specified to be the value after SADDR + SOFF calculation is performed on the original register value. Setting this field provides the ability to implement a circular data queue easily. For data queues requiring power-of-2 size bytes, the queue should start at a 0-modulo-size address and the SMOD field should be set to the appropriate value for the queue, freezing the desired number of upper address bits. The value programmed into this field specifies the number of lower address bits allowed to change. For a circular queue application, the SOFF is typically set to the transfer size to implement post-increment addressing with the SMOD function constraining the addresses to a 0-modulo-size range."]
    #[inline]
    pub fn smod_3(self) -> &'a mut W {
        self.variant(SMODW::SMOD_3)
    }
    #[doc = "This value defines a specific address range specified to be the value after SADDR + SOFF calculation is performed on the original register value. Setting this field provides the ability to implement a circular data queue easily. For data queues requiring power-of-2 size bytes, the queue should start at a 0-modulo-size address and the SMOD field should be set to the appropriate value for the queue, freezing the desired number of upper address bits. The value programmed into this field specifies the number of lower address bits allowed to change. For a circular queue application, the SOFF is typically set to the transfer size to implement post-increment addressing with the SMOD function constraining the addresses to a 0-modulo-size range."]
    #[inline]
    pub fn smod_4(self) -> &'a mut W {
        self.variant(SMODW::SMOD_4)
    }
    #[doc = "This value defines a specific address range specified to be the value after SADDR + SOFF calculation is performed on the original register value. Setting this field provides the ability to implement a circular data queue easily. For data queues requiring power-of-2 size bytes, the queue should start at a 0-modulo-size address and the SMOD field should be set to the appropriate value for the queue, freezing the desired number of upper address bits. The value programmed into this field specifies the number of lower address bits allowed to change. For a circular queue application, the SOFF is typically set to the transfer size to implement post-increment addressing with the SMOD function constraining the addresses to a 0-modulo-size range."]
    #[inline]
    pub fn smod_5(self) -> &'a mut W {
        self.variant(SMODW::SMOD_5)
    }
    #[doc = "This value defines a specific address range specified to be the value after SADDR + SOFF calculation is performed on the original register value. Setting this field provides the ability to implement a circular data queue easily. For data queues requiring power-of-2 size bytes, the queue should start at a 0-modulo-size address and the SMOD field should be set to the appropriate value for the queue, freezing the desired number of upper address bits. The value programmed into this field specifies the number of lower address bits allowed to change. For a circular queue application, the SOFF is typically set to the transfer size to implement post-increment addressing with the SMOD function constraining the addresses to a 0-modulo-size range."]
    #[inline]
    pub fn smod_6(self) -> &'a mut W {
        self.variant(SMODW::SMOD_6)
    }
    #[doc = "This value defines a specific address range specified to be the value after SADDR + SOFF calculation is performed on the original register value. Setting this field provides the ability to implement a circular data queue easily. For data queues requiring power-of-2 size bytes, the queue should start at a 0-modulo-size address and the SMOD field should be set to the appropriate value for the queue, freezing the desired number of upper address bits. The value programmed into this field specifies the number of lower address bits allowed to change. For a circular queue application, the SOFF is typically set to the transfer size to implement post-increment addressing with the SMOD function constraining the addresses to a 0-modulo-size range."]
    #[inline]
    pub fn smod_7(self) -> &'a mut W {
        self.variant(SMODW::SMOD_7)
    }
    #[doc = "This value defines a specific address range specified to be the value after SADDR + SOFF calculation is performed on the original register value. Setting this field provides the ability to implement a circular data queue easily. For data queues requiring power-of-2 size bytes, the queue should start at a 0-modulo-size address and the SMOD field should be set to the appropriate value for the queue, freezing the desired number of upper address bits. The value programmed into this field specifies the number of lower address bits allowed to change. For a circular queue application, the SOFF is typically set to the transfer size to implement post-increment addressing with the SMOD function constraining the addresses to a 0-modulo-size range."]
    #[inline]
    pub fn smod_8(self) -> &'a mut W {
        self.variant(SMODW::SMOD_8)
    }
    #[doc = "This value defines a specific address range specified to be the value after SADDR + SOFF calculation is performed on the original register value. Setting this field provides the ability to implement a circular data queue easily. For data queues requiring power-of-2 size bytes, the queue should start at a 0-modulo-size address and the SMOD field should be set to the appropriate value for the queue, freezing the desired number of upper address bits. The value programmed into this field specifies the number of lower address bits allowed to change. For a circular queue application, the SOFF is typically set to the transfer size to implement post-increment addressing with the SMOD function constraining the addresses to a 0-modulo-size range."]
    #[inline]
    pub fn smod_9(self) -> &'a mut W {
        self.variant(SMODW::SMOD_9)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 31;
        const OFFSET: u8 = 11;
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
    #[doc = "Bits 0:2 - Destination data transfer size"]
    #[inline]
    pub fn dsize(&self) -> DSIZER {
        let bits = {
            const MASK: u8 = 7;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u16) as u8
        };
        DSIZER { bits }
    }
    #[doc = "Bits 3:7 - Destination Address Modulo"]
    #[inline]
    pub fn dmod(&self) -> DMODR {
        let bits = {
            const MASK: u8 = 31;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u16) as u8
        };
        DMODR { bits }
    }
    #[doc = "Bits 8:10 - Source data transfer size"]
    #[inline]
    pub fn ssize(&self) -> SSIZER {
        SSIZER::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u16) as u8
        })
    }
    #[doc = "Bits 11:15 - Source Address Modulo"]
    #[inline]
    pub fn smod(&self) -> SMODR {
        SMODR::_from({
            const MASK: u8 = 31;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u16) as u8
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
    #[doc = "Bits 0:2 - Destination data transfer size"]
    #[inline]
    pub fn dsize(&mut self) -> _DSIZEW {
        _DSIZEW { w: self }
    }
    #[doc = "Bits 3:7 - Destination Address Modulo"]
    #[inline]
    pub fn dmod(&mut self) -> _DMODW {
        _DMODW { w: self }
    }
    #[doc = "Bits 8:10 - Source data transfer size"]
    #[inline]
    pub fn ssize(&mut self) -> _SSIZEW {
        _SSIZEW { w: self }
    }
    #[doc = "Bits 11:15 - Source Address Modulo"]
    #[inline]
    pub fn smod(&mut self) -> _SMODW {
        _SMODW { w: self }
    }
}
