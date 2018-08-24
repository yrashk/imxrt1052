#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::RX15MASK {
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
#[doc = "Possible values of the field `RX15M`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RX15MR {
    #[doc = "the corresponding bit in the filter is \"don't care\""]
    RX15M_0,
    #[doc = "The corresponding bit in the filter is checked"]
    RX15M_1,
    #[doc = r" Reserved"]
    _Reserved(u32),
}
impl RX15MR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        match *self {
            RX15MR::RX15M_0 => 0,
            RX15MR::RX15M_1 => 1,
            RX15MR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u32) -> RX15MR {
        match value {
            0 => RX15MR::RX15M_0,
            1 => RX15MR::RX15M_1,
            i => RX15MR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `RX15M_0`"]
    #[inline]
    pub fn is_rx15m_0(&self) -> bool {
        *self == RX15MR::RX15M_0
    }
    #[doc = "Checks if the value of the field is `RX15M_1`"]
    #[inline]
    pub fn is_rx15m_1(&self) -> bool {
        *self == RX15MR::RX15M_1
    }
}
#[doc = "Values that can be written to the field `RX15M`"]
pub enum RX15MW {
    #[doc = "the corresponding bit in the filter is \"don't care\""]
    RX15M_0,
    #[doc = "The corresponding bit in the filter is checked"]
    RX15M_1,
}
impl RX15MW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u32 {
        match *self {
            RX15MW::RX15M_0 => 0,
            RX15MW::RX15M_1 => 1,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RX15MW<'a> {
    w: &'a mut W,
}
impl<'a> _RX15MW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RX15MW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "the corresponding bit in the filter is \"don't care\""]
    #[inline]
    pub fn rx15m_0(self) -> &'a mut W {
        self.variant(RX15MW::RX15M_0)
    }
    #[doc = "The corresponding bit in the filter is checked"]
    #[inline]
    pub fn rx15m_1(self) -> &'a mut W {
        self.variant(RX15MW::RX15M_1)
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
    #[doc = "Bits 0:31 - These bits mask Mailbox 15 filter bits in the same fashion as RXMGMASK masks other Mailboxes filters (see RXMGMASKRx Mailboxes Global Mask Register )"]
    #[inline]
    pub fn rx15m(&self) -> RX15MR {
        RX15MR::_from({
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
        W { bits: 4294967295 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:31 - These bits mask Mailbox 15 filter bits in the same fashion as RXMGMASK masks other Mailboxes filters (see RXMGMASKRx Mailboxes Global Mask Register )"]
    #[inline]
    pub fn rx15m(&mut self) -> _RX15MW {
        _RX15MW { w: self }
    }
}
