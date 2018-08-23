#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::CONFIGFLAG {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = "Possible values of the field `CF`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CFR {
    #[doc = "Port routing control logic default-routes each port to an implementation dependent classic host controller."]
    CF_0,
    #[doc = "Port routing control logic default-routes all ports to this host controller."]
    CF_1,
}
impl CFR {
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
            CFR::CF_0 => false,
            CFR::CF_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CFR {
        match value {
            false => CFR::CF_0,
            true => CFR::CF_1,
        }
    }
    #[doc = "Checks if the value of the field is `CF_0`"]
    #[inline]
    pub fn is_cf_0(&self) -> bool {
        *self == CFR::CF_0
    }
    #[doc = "Checks if the value of the field is `CF_1`"]
    #[inline]
    pub fn is_cf_1(&self) -> bool {
        *self == CFR::CF_1
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - Configure Flag Host software sets this bit as the last action in its process of configuring the Host Controller"]
    #[inline]
    pub fn cf(&self) -> CFR {
        CFR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
}
