#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::RXIMR {
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
#[doc = "Possible values of the field `MI`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MIR {
    #[doc = "the corresponding bit in the filter is \"don't care\""]
    MI_0,
    #[doc = "The corresponding bit in the filter is checked"]
    MI_1,
    #[doc = r" Reserved"]
    _Reserved(u32),
}
impl MIR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        match *self {
            MIR::MI_0 => 0,
            MIR::MI_1 => 1,
            MIR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u32) -> MIR {
        match value {
            0 => MIR::MI_0,
            1 => MIR::MI_1,
            i => MIR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `MI_0`"]
    #[inline]
    pub fn is_mi_0(&self) -> bool {
        *self == MIR::MI_0
    }
    #[doc = "Checks if the value of the field is `MI_1`"]
    #[inline]
    pub fn is_mi_1(&self) -> bool {
        *self == MIR::MI_1
    }
}
#[doc = "Values that can be written to the field `MI`"]
pub enum MIW {
    #[doc = "the corresponding bit in the filter is \"don't care\""]
    MI_0,
    #[doc = "The corresponding bit in the filter is checked"]
    MI_1,
}
impl MIW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u32 {
        match *self {
            MIW::MI_0 => 0,
            MIW::MI_1 => 1,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MIW<'a> {
    w: &'a mut W,
}
impl<'a> _MIW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MIW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "the corresponding bit in the filter is \"don't care\""]
    #[inline]
    pub fn mi_0(self) -> &'a mut W {
        self.variant(MIW::MI_0)
    }
    #[doc = "The corresponding bit in the filter is checked"]
    #[inline]
    pub fn mi_1(self) -> &'a mut W {
        self.variant(MIW::MI_1)
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
    #[doc = "Bits 0:31 - These bits mask both Mailbox filter and Rx FIFO ID Filter Table element in distinct ways"]
    #[inline]
    pub fn mi(&self) -> MIR {
        MIR::_from({
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
    #[doc = "Bits 0:31 - These bits mask both Mailbox filter and Rx FIFO ID Filter Table element in distinct ways"]
    #[inline]
    pub fn mi(&mut self) -> _MIW {
        _MIW { w: self }
    }
}
