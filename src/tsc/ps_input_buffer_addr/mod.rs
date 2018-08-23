#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::PS_INPUT_BUFFER_ADDR {
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
#[doc = "Possible values of the field `PRE_CHARGE_TIME`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PRE_CHARGE_TIMER {
    #[doc = "Disable Auto Measure"]
    PRE_CHARGE_TIME_0,
    #[doc = "Auto Measure"]
    PRE_CHARGE_TIME_1,
    #[doc = r" Reserved"]
    _Reserved(u32),
}
impl PRE_CHARGE_TIMER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        match *self {
            PRE_CHARGE_TIMER::PRE_CHARGE_TIME_0 => 0,
            PRE_CHARGE_TIMER::PRE_CHARGE_TIME_1 => 1,
            PRE_CHARGE_TIMER::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u32) -> PRE_CHARGE_TIMER {
        match value {
            0 => PRE_CHARGE_TIMER::PRE_CHARGE_TIME_0,
            1 => PRE_CHARGE_TIMER::PRE_CHARGE_TIME_1,
            i => PRE_CHARGE_TIMER::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `PRE_CHARGE_TIME_0`"]
    #[inline]
    pub fn is_pre_charge_time_0(&self) -> bool {
        *self == PRE_CHARGE_TIMER::PRE_CHARGE_TIME_0
    }
    #[doc = "Checks if the value of the field is `PRE_CHARGE_TIME_1`"]
    #[inline]
    pub fn is_pre_charge_time_1(&self) -> bool {
        *self == PRE_CHARGE_TIMER::PRE_CHARGE_TIME_1
    }
}
#[doc = "Values that can be written to the field `PRE_CHARGE_TIME`"]
pub enum PRE_CHARGE_TIMEW {
    #[doc = "Disable Auto Measure"]
    PRE_CHARGE_TIME_0,
    #[doc = "Auto Measure"]
    PRE_CHARGE_TIME_1,
}
impl PRE_CHARGE_TIMEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u32 {
        match *self {
            PRE_CHARGE_TIMEW::PRE_CHARGE_TIME_0 => 0,
            PRE_CHARGE_TIMEW::PRE_CHARGE_TIME_1 => 1,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PRE_CHARGE_TIMEW<'a> {
    w: &'a mut W,
}
impl<'a> _PRE_CHARGE_TIMEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PRE_CHARGE_TIMEW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Disable Auto Measure"]
    #[inline]
    pub fn pre_charge_time_0(self) -> &'a mut W {
        self.variant(PRE_CHARGE_TIMEW::PRE_CHARGE_TIME_0)
    }
    #[doc = "Auto Measure"]
    #[inline]
    pub fn pre_charge_time_1(self) -> &'a mut W {
        self.variant(PRE_CHARGE_TIMEW::PRE_CHARGE_TIME_1)
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
    #[doc = "Bits 0:31 - Auto Measure"]
    #[inline]
    pub fn pre_charge_time(&self) -> PRE_CHARGE_TIMER {
        PRE_CHARGE_TIMER::_from({
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
    #[doc = "Bits 0:31 - Auto Measure"]
    #[inline]
    pub fn pre_charge_time(&mut self) -> _PRE_CHARGE_TIMEW {
        _PRE_CHARGE_TIMEW { w: self }
    }
}
