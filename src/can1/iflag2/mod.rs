#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::IFLAG2 {
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
#[doc = "Possible values of the field `BUFHI`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BUFHIR {
    #[doc = "No such occurrence"]
    BUFHI_0,
    #[doc = "The corresponding buffer has successfully completed transmission or reception"]
    BUFHI_1,
    #[doc = r" Reserved"]
    _Reserved(u32),
}
impl BUFHIR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        match *self {
            BUFHIR::BUFHI_0 => 0,
            BUFHIR::BUFHI_1 => 1,
            BUFHIR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u32) -> BUFHIR {
        match value {
            0 => BUFHIR::BUFHI_0,
            1 => BUFHIR::BUFHI_1,
            i => BUFHIR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `BUFHI_0`"]
    #[inline]
    pub fn is_bufhi_0(&self) -> bool {
        *self == BUFHIR::BUFHI_0
    }
    #[doc = "Checks if the value of the field is `BUFHI_1`"]
    #[inline]
    pub fn is_bufhi_1(&self) -> bool {
        *self == BUFHIR::BUFHI_1
    }
}
#[doc = "Values that can be written to the field `BUFHI`"]
pub enum BUFHIW {
    #[doc = "No such occurrence"]
    BUFHI_0,
    #[doc = "The corresponding buffer has successfully completed transmission or reception"]
    BUFHI_1,
}
impl BUFHIW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u32 {
        match *self {
            BUFHIW::BUFHI_0 => 0,
            BUFHIW::BUFHI_1 => 1,
        }
    }
}
#[doc = r" Proxy"]
pub struct _BUFHIW<'a> {
    w: &'a mut W,
}
impl<'a> _BUFHIW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: BUFHIW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "No such occurrence"]
    #[inline]
    pub fn bufhi_0(self) -> &'a mut W {
        self.variant(BUFHIW::BUFHI_0)
    }
    #[doc = "The corresponding buffer has successfully completed transmission or reception"]
    #[inline]
    pub fn bufhi_1(self) -> &'a mut W {
        self.variant(BUFHIW::BUFHI_1)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        const MASK: u32 = 4294967295;
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
    #[doc = "Bits 0:31 - Each bit flags the respective FLEXCAN Message Buffer (MB32 to MB63) interrupt."]
    #[inline]
    pub fn bufhi(&self) -> BUFHIR {
        BUFHIR::_from({
            const MASK: u32 = 4294967295;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u32
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
    #[doc = "Bits 0:31 - Each bit flags the respective FLEXCAN Message Buffer (MB32 to MB63) interrupt."]
    #[inline]
    pub fn bufhi(&mut self) -> _BUFHIW {
        _BUFHIW { w: self }
    }
}
