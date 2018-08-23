#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::OPACR2 {
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
#[doc = "Possible values of the field `OPAC23`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OPAC23R {
    #[doc = "Accesses from an untrusted master are allowed."]
    TP0,
    #[doc = "Accesses from an untrusted master are not allowed. If an access is attempted by an untrusted master, the access is terminated with an error response and no peripheral access is initiated on the IPS bus."]
    TP1,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl OPAC23R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            OPAC23R::TP0 => 0,
            OPAC23R::TP1 => 1,
            OPAC23R::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> OPAC23R {
        match value {
            0 => OPAC23R::TP0,
            1 => OPAC23R::TP1,
            i => OPAC23R::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `TP0`"]
    #[inline]
    pub fn is_tp0(&self) -> bool {
        *self == OPAC23R::TP0
    }
    #[doc = "Checks if the value of the field is `TP1`"]
    #[inline]
    pub fn is_tp1(&self) -> bool {
        *self == OPAC23R::TP1
    }
}
#[doc = "Possible values of the field `OPAC22`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OPAC22R {
    #[doc = "Accesses from an untrusted master are allowed."]
    TP0,
    #[doc = "Accesses from an untrusted master are not allowed. If an access is attempted by an untrusted master, the access is terminated with an error response and no peripheral access is initiated on the IPS bus."]
    TP1,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl OPAC22R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            OPAC22R::TP0 => 0,
            OPAC22R::TP1 => 1,
            OPAC22R::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> OPAC22R {
        match value {
            0 => OPAC22R::TP0,
            1 => OPAC22R::TP1,
            i => OPAC22R::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `TP0`"]
    #[inline]
    pub fn is_tp0(&self) -> bool {
        *self == OPAC22R::TP0
    }
    #[doc = "Checks if the value of the field is `TP1`"]
    #[inline]
    pub fn is_tp1(&self) -> bool {
        *self == OPAC22R::TP1
    }
}
#[doc = "Possible values of the field `OPAC21`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OPAC21R {
    #[doc = "Accesses from an untrusted master are allowed."]
    TP0,
    #[doc = "Accesses from an untrusted master are not allowed. If an access is attempted by an untrusted master, the access is terminated with an error response and no peripheral access is initiated on the IPS bus."]
    TP1,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl OPAC21R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            OPAC21R::TP0 => 0,
            OPAC21R::TP1 => 1,
            OPAC21R::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> OPAC21R {
        match value {
            0 => OPAC21R::TP0,
            1 => OPAC21R::TP1,
            i => OPAC21R::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `TP0`"]
    #[inline]
    pub fn is_tp0(&self) -> bool {
        *self == OPAC21R::TP0
    }
    #[doc = "Checks if the value of the field is `TP1`"]
    #[inline]
    pub fn is_tp1(&self) -> bool {
        *self == OPAC21R::TP1
    }
}
#[doc = "Possible values of the field `OPAC20`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OPAC20R {
    #[doc = "Accesses from an untrusted master are allowed."]
    TP0,
    #[doc = "Accesses from an untrusted master are not allowed. If an access is attempted by an untrusted master, the access is terminated with an error response and no peripheral access is initiated on the IPS bus."]
    TP1,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl OPAC20R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            OPAC20R::TP0 => 0,
            OPAC20R::TP1 => 1,
            OPAC20R::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> OPAC20R {
        match value {
            0 => OPAC20R::TP0,
            1 => OPAC20R::TP1,
            i => OPAC20R::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `TP0`"]
    #[inline]
    pub fn is_tp0(&self) -> bool {
        *self == OPAC20R::TP0
    }
    #[doc = "Checks if the value of the field is `TP1`"]
    #[inline]
    pub fn is_tp1(&self) -> bool {
        *self == OPAC20R::TP1
    }
}
#[doc = "Possible values of the field `OPAC19`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OPAC19R {
    #[doc = "Accesses from an untrusted master are allowed."]
    TP0,
    #[doc = "Accesses from an untrusted master are not allowed. If an access is attempted by an untrusted master, the access is terminated with an error response and no peripheral access is initiated on the IPS bus."]
    TP1,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl OPAC19R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            OPAC19R::TP0 => 0,
            OPAC19R::TP1 => 1,
            OPAC19R::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> OPAC19R {
        match value {
            0 => OPAC19R::TP0,
            1 => OPAC19R::TP1,
            i => OPAC19R::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `TP0`"]
    #[inline]
    pub fn is_tp0(&self) -> bool {
        *self == OPAC19R::TP0
    }
    #[doc = "Checks if the value of the field is `TP1`"]
    #[inline]
    pub fn is_tp1(&self) -> bool {
        *self == OPAC19R::TP1
    }
}
#[doc = "Possible values of the field `OPAC18`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OPAC18R {
    #[doc = "Accesses from an untrusted master are allowed."]
    TP0,
    #[doc = "Accesses from an untrusted master are not allowed. If an access is attempted by an untrusted master, the access is terminated with an error response and no peripheral access is initiated on the IPS bus."]
    TP1,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl OPAC18R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            OPAC18R::TP0 => 0,
            OPAC18R::TP1 => 1,
            OPAC18R::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> OPAC18R {
        match value {
            0 => OPAC18R::TP0,
            1 => OPAC18R::TP1,
            i => OPAC18R::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `TP0`"]
    #[inline]
    pub fn is_tp0(&self) -> bool {
        *self == OPAC18R::TP0
    }
    #[doc = "Checks if the value of the field is `TP1`"]
    #[inline]
    pub fn is_tp1(&self) -> bool {
        *self == OPAC18R::TP1
    }
}
#[doc = "Possible values of the field `OPAC17`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OPAC17R {
    #[doc = "Accesses from an untrusted master are allowed."]
    TP0,
    #[doc = "Accesses from an untrusted master are not allowed. If an access is attempted by an untrusted master, the access is terminated with an error response and no peripheral access is initiated on the IPS bus."]
    TP1,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl OPAC17R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            OPAC17R::TP0 => 0,
            OPAC17R::TP1 => 1,
            OPAC17R::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> OPAC17R {
        match value {
            0 => OPAC17R::TP0,
            1 => OPAC17R::TP1,
            i => OPAC17R::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `TP0`"]
    #[inline]
    pub fn is_tp0(&self) -> bool {
        *self == OPAC17R::TP0
    }
    #[doc = "Checks if the value of the field is `TP1`"]
    #[inline]
    pub fn is_tp1(&self) -> bool {
        *self == OPAC17R::TP1
    }
}
#[doc = "Possible values of the field `OPAC16`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OPAC16R {
    #[doc = "Accesses from an untrusted master are allowed."]
    TP0,
    #[doc = "Accesses from an untrusted master are not allowed. If an access is attempted by an untrusted master, the access is terminated with an error response and no peripheral access is initiated on the IPS bus."]
    TP1,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl OPAC16R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            OPAC16R::TP0 => 0,
            OPAC16R::TP1 => 1,
            OPAC16R::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> OPAC16R {
        match value {
            0 => OPAC16R::TP0,
            1 => OPAC16R::TP1,
            i => OPAC16R::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `TP0`"]
    #[inline]
    pub fn is_tp0(&self) -> bool {
        *self == OPAC16R::TP0
    }
    #[doc = "Checks if the value of the field is `TP1`"]
    #[inline]
    pub fn is_tp1(&self) -> bool {
        *self == OPAC16R::TP1
    }
}
#[doc = "Values that can be written to the field `OPAC23`"]
pub enum OPAC23W {
    #[doc = "Accesses from an untrusted master are allowed."]
    TP0,
    #[doc = "Accesses from an untrusted master are not allowed. If an access is attempted by an untrusted master, the access is terminated with an error response and no peripheral access is initiated on the IPS bus."]
    TP1,
}
impl OPAC23W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            OPAC23W::TP0 => 0,
            OPAC23W::TP1 => 1,
        }
    }
}
#[doc = r" Proxy"]
pub struct _OPAC23W<'a> {
    w: &'a mut W,
}
impl<'a> _OPAC23W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: OPAC23W) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Accesses from an untrusted master are allowed."]
    #[inline]
    pub fn tp0(self) -> &'a mut W {
        self.variant(OPAC23W::TP0)
    }
    #[doc = "Accesses from an untrusted master are not allowed. If an access is attempted by an untrusted master, the access is terminated with an error response and no peripheral access is initiated on the IPS bus."]
    #[inline]
    pub fn tp1(self) -> &'a mut W {
        self.variant(OPAC23W::TP1)
    }
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
#[doc = "Values that can be written to the field `OPAC22`"]
pub enum OPAC22W {
    #[doc = "Accesses from an untrusted master are allowed."]
    TP0,
    #[doc = "Accesses from an untrusted master are not allowed. If an access is attempted by an untrusted master, the access is terminated with an error response and no peripheral access is initiated on the IPS bus."]
    TP1,
}
impl OPAC22W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            OPAC22W::TP0 => 0,
            OPAC22W::TP1 => 1,
        }
    }
}
#[doc = r" Proxy"]
pub struct _OPAC22W<'a> {
    w: &'a mut W,
}
impl<'a> _OPAC22W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: OPAC22W) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Accesses from an untrusted master are allowed."]
    #[inline]
    pub fn tp0(self) -> &'a mut W {
        self.variant(OPAC22W::TP0)
    }
    #[doc = "Accesses from an untrusted master are not allowed. If an access is attempted by an untrusted master, the access is terminated with an error response and no peripheral access is initiated on the IPS bus."]
    #[inline]
    pub fn tp1(self) -> &'a mut W {
        self.variant(OPAC22W::TP1)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 4;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `OPAC21`"]
pub enum OPAC21W {
    #[doc = "Accesses from an untrusted master are allowed."]
    TP0,
    #[doc = "Accesses from an untrusted master are not allowed. If an access is attempted by an untrusted master, the access is terminated with an error response and no peripheral access is initiated on the IPS bus."]
    TP1,
}
impl OPAC21W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            OPAC21W::TP0 => 0,
            OPAC21W::TP1 => 1,
        }
    }
}
#[doc = r" Proxy"]
pub struct _OPAC21W<'a> {
    w: &'a mut W,
}
impl<'a> _OPAC21W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: OPAC21W) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Accesses from an untrusted master are allowed."]
    #[inline]
    pub fn tp0(self) -> &'a mut W {
        self.variant(OPAC21W::TP0)
    }
    #[doc = "Accesses from an untrusted master are not allowed. If an access is attempted by an untrusted master, the access is terminated with an error response and no peripheral access is initiated on the IPS bus."]
    #[inline]
    pub fn tp1(self) -> &'a mut W {
        self.variant(OPAC21W::TP1)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `OPAC20`"]
pub enum OPAC20W {
    #[doc = "Accesses from an untrusted master are allowed."]
    TP0,
    #[doc = "Accesses from an untrusted master are not allowed. If an access is attempted by an untrusted master, the access is terminated with an error response and no peripheral access is initiated on the IPS bus."]
    TP1,
}
impl OPAC20W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            OPAC20W::TP0 => 0,
            OPAC20W::TP1 => 1,
        }
    }
}
#[doc = r" Proxy"]
pub struct _OPAC20W<'a> {
    w: &'a mut W,
}
impl<'a> _OPAC20W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: OPAC20W) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Accesses from an untrusted master are allowed."]
    #[inline]
    pub fn tp0(self) -> &'a mut W {
        self.variant(OPAC20W::TP0)
    }
    #[doc = "Accesses from an untrusted master are not allowed. If an access is attempted by an untrusted master, the access is terminated with an error response and no peripheral access is initiated on the IPS bus."]
    #[inline]
    pub fn tp1(self) -> &'a mut W {
        self.variant(OPAC20W::TP1)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 12;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `OPAC19`"]
pub enum OPAC19W {
    #[doc = "Accesses from an untrusted master are allowed."]
    TP0,
    #[doc = "Accesses from an untrusted master are not allowed. If an access is attempted by an untrusted master, the access is terminated with an error response and no peripheral access is initiated on the IPS bus."]
    TP1,
}
impl OPAC19W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            OPAC19W::TP0 => 0,
            OPAC19W::TP1 => 1,
        }
    }
}
#[doc = r" Proxy"]
pub struct _OPAC19W<'a> {
    w: &'a mut W,
}
impl<'a> _OPAC19W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: OPAC19W) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Accesses from an untrusted master are allowed."]
    #[inline]
    pub fn tp0(self) -> &'a mut W {
        self.variant(OPAC19W::TP0)
    }
    #[doc = "Accesses from an untrusted master are not allowed. If an access is attempted by an untrusted master, the access is terminated with an error response and no peripheral access is initiated on the IPS bus."]
    #[inline]
    pub fn tp1(self) -> &'a mut W {
        self.variant(OPAC19W::TP1)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 16;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `OPAC18`"]
pub enum OPAC18W {
    #[doc = "Accesses from an untrusted master are allowed."]
    TP0,
    #[doc = "Accesses from an untrusted master are not allowed. If an access is attempted by an untrusted master, the access is terminated with an error response and no peripheral access is initiated on the IPS bus."]
    TP1,
}
impl OPAC18W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            OPAC18W::TP0 => 0,
            OPAC18W::TP1 => 1,
        }
    }
}
#[doc = r" Proxy"]
pub struct _OPAC18W<'a> {
    w: &'a mut W,
}
impl<'a> _OPAC18W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: OPAC18W) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Accesses from an untrusted master are allowed."]
    #[inline]
    pub fn tp0(self) -> &'a mut W {
        self.variant(OPAC18W::TP0)
    }
    #[doc = "Accesses from an untrusted master are not allowed. If an access is attempted by an untrusted master, the access is terminated with an error response and no peripheral access is initiated on the IPS bus."]
    #[inline]
    pub fn tp1(self) -> &'a mut W {
        self.variant(OPAC18W::TP1)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 20;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `OPAC17`"]
pub enum OPAC17W {
    #[doc = "Accesses from an untrusted master are allowed."]
    TP0,
    #[doc = "Accesses from an untrusted master are not allowed. If an access is attempted by an untrusted master, the access is terminated with an error response and no peripheral access is initiated on the IPS bus."]
    TP1,
}
impl OPAC17W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            OPAC17W::TP0 => 0,
            OPAC17W::TP1 => 1,
        }
    }
}
#[doc = r" Proxy"]
pub struct _OPAC17W<'a> {
    w: &'a mut W,
}
impl<'a> _OPAC17W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: OPAC17W) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Accesses from an untrusted master are allowed."]
    #[inline]
    pub fn tp0(self) -> &'a mut W {
        self.variant(OPAC17W::TP0)
    }
    #[doc = "Accesses from an untrusted master are not allowed. If an access is attempted by an untrusted master, the access is terminated with an error response and no peripheral access is initiated on the IPS bus."]
    #[inline]
    pub fn tp1(self) -> &'a mut W {
        self.variant(OPAC17W::TP1)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 24;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `OPAC16`"]
pub enum OPAC16W {
    #[doc = "Accesses from an untrusted master are allowed."]
    TP0,
    #[doc = "Accesses from an untrusted master are not allowed. If an access is attempted by an untrusted master, the access is terminated with an error response and no peripheral access is initiated on the IPS bus."]
    TP1,
}
impl OPAC16W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            OPAC16W::TP0 => 0,
            OPAC16W::TP1 => 1,
        }
    }
}
#[doc = r" Proxy"]
pub struct _OPAC16W<'a> {
    w: &'a mut W,
}
impl<'a> _OPAC16W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: OPAC16W) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Accesses from an untrusted master are allowed."]
    #[inline]
    pub fn tp0(self) -> &'a mut W {
        self.variant(OPAC16W::TP0)
    }
    #[doc = "Accesses from an untrusted master are not allowed. If an access is attempted by an untrusted master, the access is terminated with an error response and no peripheral access is initiated on the IPS bus."]
    #[inline]
    pub fn tp1(self) -> &'a mut W {
        self.variant(OPAC16W::TP1)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 28;
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
    #[doc = "Bits 0:3 - Off-platform Peripheral Access Control 23"]
    #[inline]
    pub fn opac23(&self) -> OPAC23R {
        OPAC23R::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 4:7 - Off-platform Peripheral Access Control 22"]
    #[inline]
    pub fn opac22(&self) -> OPAC22R {
        OPAC22R::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 8:11 - Off-platform Peripheral Access Control 21"]
    #[inline]
    pub fn opac21(&self) -> OPAC21R {
        OPAC21R::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 12:15 - Off-platform Peripheral Access Control 20"]
    #[inline]
    pub fn opac20(&self) -> OPAC20R {
        OPAC20R::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 16:19 - Off-platform Peripheral Access Control 19"]
    #[inline]
    pub fn opac19(&self) -> OPAC19R {
        OPAC19R::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 20:23 - Off-platform Peripheral Access Control 18"]
    #[inline]
    pub fn opac18(&self) -> OPAC18R {
        OPAC18R::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 24:27 - Off-platform Peripheral Access Control 17"]
    #[inline]
    pub fn opac17(&self) -> OPAC17R {
        OPAC17R::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 28:31 - Off-platform Peripheral Access Control 16"]
    #[inline]
    pub fn opac16(&self) -> OPAC16R {
        OPAC16R::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 28;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 1145324612 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:3 - Off-platform Peripheral Access Control 23"]
    #[inline]
    pub fn opac23(&mut self) -> _OPAC23W {
        _OPAC23W { w: self }
    }
    #[doc = "Bits 4:7 - Off-platform Peripheral Access Control 22"]
    #[inline]
    pub fn opac22(&mut self) -> _OPAC22W {
        _OPAC22W { w: self }
    }
    #[doc = "Bits 8:11 - Off-platform Peripheral Access Control 21"]
    #[inline]
    pub fn opac21(&mut self) -> _OPAC21W {
        _OPAC21W { w: self }
    }
    #[doc = "Bits 12:15 - Off-platform Peripheral Access Control 20"]
    #[inline]
    pub fn opac20(&mut self) -> _OPAC20W {
        _OPAC20W { w: self }
    }
    #[doc = "Bits 16:19 - Off-platform Peripheral Access Control 19"]
    #[inline]
    pub fn opac19(&mut self) -> _OPAC19W {
        _OPAC19W { w: self }
    }
    #[doc = "Bits 20:23 - Off-platform Peripheral Access Control 18"]
    #[inline]
    pub fn opac18(&mut self) -> _OPAC18W {
        _OPAC18W { w: self }
    }
    #[doc = "Bits 24:27 - Off-platform Peripheral Access Control 17"]
    #[inline]
    pub fn opac17(&mut self) -> _OPAC17W {
        _OPAC17W { w: self }
    }
    #[doc = "Bits 28:31 - Off-platform Peripheral Access Control 16"]
    #[inline]
    pub fn opac16(&mut self) -> _OPAC16W {
        _OPAC16W { w: self }
    }
}
