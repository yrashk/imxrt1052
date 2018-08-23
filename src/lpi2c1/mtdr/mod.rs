#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::MTDR {
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
#[doc = r" Proxy"]
pub struct _DATAW<'a> {
    w: &'a mut W,
}
impl<'a> _DATAW<'a> {
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
#[doc = "Values that can be written to the field `CMD`"]
pub enum CMDW {
    #[doc = "Transmit DATA[7:0]"]
    CMD_0,
    #[doc = "Receive (DATA[7:0] + 1) bytes"]
    CMD_1,
    #[doc = "Generate STOP condition"]
    CMD_2,
    #[doc = "Receive and discard (DATA[7:0] + 1) bytes"]
    CMD_3,
    #[doc = "Generate (repeated) START and transmit address in DATA[7:0]"]
    CMD_4,
    #[doc = "Generate (repeated) START and transmit address in DATA[7:0]. This transfer expects a NACK to be returned."]
    CMD_5,
    #[doc = "Generate (repeated) START and transmit address in DATA[7:0] using high speed mode"]
    CMD_6,
    #[doc = "Generate (repeated) START and transmit address in DATA[7:0] using high speed mode. This transfer expects a NACK to be returned."]
    CMD_7,
}
impl CMDW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            CMDW::CMD_0 => 0,
            CMDW::CMD_1 => 1,
            CMDW::CMD_2 => 2,
            CMDW::CMD_3 => 3,
            CMDW::CMD_4 => 4,
            CMDW::CMD_5 => 5,
            CMDW::CMD_6 => 6,
            CMDW::CMD_7 => 7,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CMDW<'a> {
    w: &'a mut W,
}
impl<'a> _CMDW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CMDW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Transmit DATA[7:0]"]
    #[inline]
    pub fn cmd_0(self) -> &'a mut W {
        self.variant(CMDW::CMD_0)
    }
    #[doc = "Receive (DATA[7:0] + 1) bytes"]
    #[inline]
    pub fn cmd_1(self) -> &'a mut W {
        self.variant(CMDW::CMD_1)
    }
    #[doc = "Generate STOP condition"]
    #[inline]
    pub fn cmd_2(self) -> &'a mut W {
        self.variant(CMDW::CMD_2)
    }
    #[doc = "Receive and discard (DATA[7:0] + 1) bytes"]
    #[inline]
    pub fn cmd_3(self) -> &'a mut W {
        self.variant(CMDW::CMD_3)
    }
    #[doc = "Generate (repeated) START and transmit address in DATA[7:0]"]
    #[inline]
    pub fn cmd_4(self) -> &'a mut W {
        self.variant(CMDW::CMD_4)
    }
    #[doc = "Generate (repeated) START and transmit address in DATA[7:0]. This transfer expects a NACK to be returned."]
    #[inline]
    pub fn cmd_5(self) -> &'a mut W {
        self.variant(CMDW::CMD_5)
    }
    #[doc = "Generate (repeated) START and transmit address in DATA[7:0] using high speed mode"]
    #[inline]
    pub fn cmd_6(self) -> &'a mut W {
        self.variant(CMDW::CMD_6)
    }
    #[doc = "Generate (repeated) START and transmit address in DATA[7:0] using high speed mode. This transfer expects a NACK to be returned."]
    #[inline]
    pub fn cmd_7(self) -> &'a mut W {
        self.variant(CMDW::CMD_7)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 8;
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
    #[doc = "Bits 0:7 - Transmit Data"]
    #[inline]
    pub fn data(&mut self) -> _DATAW {
        _DATAW { w: self }
    }
    #[doc = "Bits 8:10 - Command Data"]
    #[inline]
    pub fn cmd(&mut self) -> _CMDW {
        _CMDW { w: self }
    }
}
