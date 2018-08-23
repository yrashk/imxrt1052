#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::ROMPATCHSR {
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
#[doc = "Possible values of the field `SOURCE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SOURCER {
    #[doc = "Address Comparator 0 matched"]
    SOURCE_0,
    #[doc = "Address Comparator 1 matched"]
    SOURCE_1,
    #[doc = "Address Comparator 15 matched"]
    SOURCE_15,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl SOURCER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            SOURCER::SOURCE_0 => 0,
            SOURCER::SOURCE_1 => 1,
            SOURCER::SOURCE_15 => 15,
            SOURCER::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> SOURCER {
        match value {
            0 => SOURCER::SOURCE_0,
            1 => SOURCER::SOURCE_1,
            15 => SOURCER::SOURCE_15,
            i => SOURCER::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `SOURCE_0`"]
    #[inline]
    pub fn is_source_0(&self) -> bool {
        *self == SOURCER::SOURCE_0
    }
    #[doc = "Checks if the value of the field is `SOURCE_1`"]
    #[inline]
    pub fn is_source_1(&self) -> bool {
        *self == SOURCER::SOURCE_1
    }
    #[doc = "Checks if the value of the field is `SOURCE_15`"]
    #[inline]
    pub fn is_source_15(&self) -> bool {
        *self == SOURCER::SOURCE_15
    }
}
#[doc = "Possible values of the field `SW`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SWR {
    #[doc = "no event or comparator collisions"]
    SW_0,
    #[doc = "a collision has occurred"]
    SW_1,
}
impl SWR {
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
            SWR::SW_0 => false,
            SWR::SW_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SWR {
        match value {
            false => SWR::SW_0,
            true => SWR::SW_1,
        }
    }
    #[doc = "Checks if the value of the field is `SW_0`"]
    #[inline]
    pub fn is_sw_0(&self) -> bool {
        *self == SWR::SW_0
    }
    #[doc = "Checks if the value of the field is `SW_1`"]
    #[inline]
    pub fn is_sw_1(&self) -> bool {
        *self == SWR::SW_1
    }
}
#[doc = "Values that can be written to the field `SW`"]
pub enum SWW {
    #[doc = "no event or comparator collisions"]
    SW_0,
    #[doc = "a collision has occurred"]
    SW_1,
}
impl SWW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SWW::SW_0 => false,
            SWW::SW_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SWW<'a> {
    w: &'a mut W,
}
impl<'a> _SWW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SWW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "no event or comparator collisions"]
    #[inline]
    pub fn sw_0(self) -> &'a mut W {
        self.variant(SWW::SW_0)
    }
    #[doc = "a collision has occurred"]
    #[inline]
    pub fn sw_1(self) -> &'a mut W {
        self.variant(SWW::SW_1)
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
        const OFFSET: u8 = 17;
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
    #[doc = "Bits 0:5 - ROMC Source Number - Binary encoding of the number of the address comparator which has an address match in the most recent patch event on ROMC AHB"]
    #[inline]
    pub fn source(&self) -> SOURCER {
        SOURCER::_from({
            const MASK: u8 = 63;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 17 - ROMC AHB Multiple Address Comparator matches Indicator - Indicates that multiple address comparator matches occurred"]
    #[inline]
    pub fn sw(&self) -> SWR {
        SWR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 17;
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
    #[doc = "Bit 17 - ROMC AHB Multiple Address Comparator matches Indicator - Indicates that multiple address comparator matches occurred"]
    #[inline]
    pub fn sw(&mut self) -> _SWW {
        _SWW { w: self }
    }
}
