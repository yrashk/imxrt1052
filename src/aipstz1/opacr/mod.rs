#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::OPACR {
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
#[doc = "Possible values of the field `OPAC7`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OPAC7R {
    #[doc = "Accesses from an untrusted master are allowed."]
    TP0,
    #[doc = "Accesses from an untrusted master are not allowed. If an access is attempted by an untrusted master, the access is terminated with an error response and no peripheral access is initiated on the IPS bus."]
    TP1,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl OPAC7R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            OPAC7R::TP0 => 0,
            OPAC7R::TP1 => 1,
            OPAC7R::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> OPAC7R {
        match value {
            0 => OPAC7R::TP0,
            1 => OPAC7R::TP1,
            i => OPAC7R::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `TP0`"]
    #[inline]
    pub fn is_tp0(&self) -> bool {
        *self == OPAC7R::TP0
    }
    #[doc = "Checks if the value of the field is `TP1`"]
    #[inline]
    pub fn is_tp1(&self) -> bool {
        *self == OPAC7R::TP1
    }
}
#[doc = "Possible values of the field `OPAC6`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OPAC6R {
    #[doc = "Accesses from an untrusted master are allowed."]
    TP0,
    #[doc = "Accesses from an untrusted master are not allowed. If an access is attempted by an untrusted master, the access is terminated with an error response and no peripheral access is initiated on the IPS bus."]
    TP1,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl OPAC6R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            OPAC6R::TP0 => 0,
            OPAC6R::TP1 => 1,
            OPAC6R::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> OPAC6R {
        match value {
            0 => OPAC6R::TP0,
            1 => OPAC6R::TP1,
            i => OPAC6R::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `TP0`"]
    #[inline]
    pub fn is_tp0(&self) -> bool {
        *self == OPAC6R::TP0
    }
    #[doc = "Checks if the value of the field is `TP1`"]
    #[inline]
    pub fn is_tp1(&self) -> bool {
        *self == OPAC6R::TP1
    }
}
#[doc = "Possible values of the field `OPAC5`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OPAC5R {
    #[doc = "Accesses from an untrusted master are allowed."]
    TP0,
    #[doc = "Accesses from an untrusted master are not allowed. If an access is attempted by an untrusted master, the access is terminated with an error response and no peripheral access is initiated on the IPS bus."]
    TP1,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl OPAC5R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            OPAC5R::TP0 => 0,
            OPAC5R::TP1 => 1,
            OPAC5R::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> OPAC5R {
        match value {
            0 => OPAC5R::TP0,
            1 => OPAC5R::TP1,
            i => OPAC5R::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `TP0`"]
    #[inline]
    pub fn is_tp0(&self) -> bool {
        *self == OPAC5R::TP0
    }
    #[doc = "Checks if the value of the field is `TP1`"]
    #[inline]
    pub fn is_tp1(&self) -> bool {
        *self == OPAC5R::TP1
    }
}
#[doc = "Possible values of the field `OPAC4`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OPAC4R {
    #[doc = "Accesses from an untrusted master are allowed."]
    TP0,
    #[doc = "Accesses from an untrusted master are not allowed. If an access is attempted by an untrusted master, the access is terminated with an error response and no peripheral access is initiated on the IPS bus."]
    TP1,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl OPAC4R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            OPAC4R::TP0 => 0,
            OPAC4R::TP1 => 1,
            OPAC4R::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> OPAC4R {
        match value {
            0 => OPAC4R::TP0,
            1 => OPAC4R::TP1,
            i => OPAC4R::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `TP0`"]
    #[inline]
    pub fn is_tp0(&self) -> bool {
        *self == OPAC4R::TP0
    }
    #[doc = "Checks if the value of the field is `TP1`"]
    #[inline]
    pub fn is_tp1(&self) -> bool {
        *self == OPAC4R::TP1
    }
}
#[doc = "Possible values of the field `OPAC3`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OPAC3R {
    #[doc = "Accesses from an untrusted master are allowed."]
    TP0,
    #[doc = "Accesses from an untrusted master are not allowed. If an access is attempted by an untrusted master, the access is terminated with an error response and no peripheral access is initiated on the IPS bus."]
    TP1,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl OPAC3R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            OPAC3R::TP0 => 0,
            OPAC3R::TP1 => 1,
            OPAC3R::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> OPAC3R {
        match value {
            0 => OPAC3R::TP0,
            1 => OPAC3R::TP1,
            i => OPAC3R::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `TP0`"]
    #[inline]
    pub fn is_tp0(&self) -> bool {
        *self == OPAC3R::TP0
    }
    #[doc = "Checks if the value of the field is `TP1`"]
    #[inline]
    pub fn is_tp1(&self) -> bool {
        *self == OPAC3R::TP1
    }
}
#[doc = "Possible values of the field `OPAC2`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OPAC2R {
    #[doc = "Accesses from an untrusted master are allowed."]
    TP0,
    #[doc = "Accesses from an untrusted master are not allowed. If an access is attempted by an untrusted master, the access is terminated with an error response and no peripheral access is initiated on the IPS bus."]
    TP1,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl OPAC2R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            OPAC2R::TP0 => 0,
            OPAC2R::TP1 => 1,
            OPAC2R::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> OPAC2R {
        match value {
            0 => OPAC2R::TP0,
            1 => OPAC2R::TP1,
            i => OPAC2R::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `TP0`"]
    #[inline]
    pub fn is_tp0(&self) -> bool {
        *self == OPAC2R::TP0
    }
    #[doc = "Checks if the value of the field is `TP1`"]
    #[inline]
    pub fn is_tp1(&self) -> bool {
        *self == OPAC2R::TP1
    }
}
#[doc = "Possible values of the field `OPAC1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OPAC1R {
    #[doc = "Accesses from an untrusted master are allowed."]
    TP0,
    #[doc = "Accesses from an untrusted master are not allowed. If an access is attempted by an untrusted master, the access is terminated with an error response and no peripheral access is initiated on the IPS bus."]
    TP1,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl OPAC1R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            OPAC1R::TP0 => 0,
            OPAC1R::TP1 => 1,
            OPAC1R::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> OPAC1R {
        match value {
            0 => OPAC1R::TP0,
            1 => OPAC1R::TP1,
            i => OPAC1R::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `TP0`"]
    #[inline]
    pub fn is_tp0(&self) -> bool {
        *self == OPAC1R::TP0
    }
    #[doc = "Checks if the value of the field is `TP1`"]
    #[inline]
    pub fn is_tp1(&self) -> bool {
        *self == OPAC1R::TP1
    }
}
#[doc = "Possible values of the field `OPAC0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OPAC0R {
    #[doc = "Accesses from an untrusted master are allowed."]
    TP0,
    #[doc = "Accesses from an untrusted master are not allowed. If an access is attempted by an untrusted master, the access is terminated with an error response and no peripheral access is initiated on the IPS bus."]
    TP1,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl OPAC0R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            OPAC0R::TP0 => 0,
            OPAC0R::TP1 => 1,
            OPAC0R::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> OPAC0R {
        match value {
            0 => OPAC0R::TP0,
            1 => OPAC0R::TP1,
            i => OPAC0R::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `TP0`"]
    #[inline]
    pub fn is_tp0(&self) -> bool {
        *self == OPAC0R::TP0
    }
    #[doc = "Checks if the value of the field is `TP1`"]
    #[inline]
    pub fn is_tp1(&self) -> bool {
        *self == OPAC0R::TP1
    }
}
#[doc = "Values that can be written to the field `OPAC7`"]
pub enum OPAC7W {
    #[doc = "Accesses from an untrusted master are allowed."]
    TP0,
    #[doc = "Accesses from an untrusted master are not allowed. If an access is attempted by an untrusted master, the access is terminated with an error response and no peripheral access is initiated on the IPS bus."]
    TP1,
}
impl OPAC7W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            OPAC7W::TP0 => 0,
            OPAC7W::TP1 => 1,
        }
    }
}
#[doc = r" Proxy"]
pub struct _OPAC7W<'a> {
    w: &'a mut W,
}
impl<'a> _OPAC7W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: OPAC7W) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Accesses from an untrusted master are allowed."]
    #[inline]
    pub fn tp0(self) -> &'a mut W {
        self.variant(OPAC7W::TP0)
    }
    #[doc = "Accesses from an untrusted master are not allowed. If an access is attempted by an untrusted master, the access is terminated with an error response and no peripheral access is initiated on the IPS bus."]
    #[inline]
    pub fn tp1(self) -> &'a mut W {
        self.variant(OPAC7W::TP1)
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
#[doc = "Values that can be written to the field `OPAC6`"]
pub enum OPAC6W {
    #[doc = "Accesses from an untrusted master are allowed."]
    TP0,
    #[doc = "Accesses from an untrusted master are not allowed. If an access is attempted by an untrusted master, the access is terminated with an error response and no peripheral access is initiated on the IPS bus."]
    TP1,
}
impl OPAC6W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            OPAC6W::TP0 => 0,
            OPAC6W::TP1 => 1,
        }
    }
}
#[doc = r" Proxy"]
pub struct _OPAC6W<'a> {
    w: &'a mut W,
}
impl<'a> _OPAC6W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: OPAC6W) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Accesses from an untrusted master are allowed."]
    #[inline]
    pub fn tp0(self) -> &'a mut W {
        self.variant(OPAC6W::TP0)
    }
    #[doc = "Accesses from an untrusted master are not allowed. If an access is attempted by an untrusted master, the access is terminated with an error response and no peripheral access is initiated on the IPS bus."]
    #[inline]
    pub fn tp1(self) -> &'a mut W {
        self.variant(OPAC6W::TP1)
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
#[doc = "Values that can be written to the field `OPAC5`"]
pub enum OPAC5W {
    #[doc = "Accesses from an untrusted master are allowed."]
    TP0,
    #[doc = "Accesses from an untrusted master are not allowed. If an access is attempted by an untrusted master, the access is terminated with an error response and no peripheral access is initiated on the IPS bus."]
    TP1,
}
impl OPAC5W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            OPAC5W::TP0 => 0,
            OPAC5W::TP1 => 1,
        }
    }
}
#[doc = r" Proxy"]
pub struct _OPAC5W<'a> {
    w: &'a mut W,
}
impl<'a> _OPAC5W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: OPAC5W) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Accesses from an untrusted master are allowed."]
    #[inline]
    pub fn tp0(self) -> &'a mut W {
        self.variant(OPAC5W::TP0)
    }
    #[doc = "Accesses from an untrusted master are not allowed. If an access is attempted by an untrusted master, the access is terminated with an error response and no peripheral access is initiated on the IPS bus."]
    #[inline]
    pub fn tp1(self) -> &'a mut W {
        self.variant(OPAC5W::TP1)
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
#[doc = "Values that can be written to the field `OPAC4`"]
pub enum OPAC4W {
    #[doc = "Accesses from an untrusted master are allowed."]
    TP0,
    #[doc = "Accesses from an untrusted master are not allowed. If an access is attempted by an untrusted master, the access is terminated with an error response and no peripheral access is initiated on the IPS bus."]
    TP1,
}
impl OPAC4W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            OPAC4W::TP0 => 0,
            OPAC4W::TP1 => 1,
        }
    }
}
#[doc = r" Proxy"]
pub struct _OPAC4W<'a> {
    w: &'a mut W,
}
impl<'a> _OPAC4W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: OPAC4W) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Accesses from an untrusted master are allowed."]
    #[inline]
    pub fn tp0(self) -> &'a mut W {
        self.variant(OPAC4W::TP0)
    }
    #[doc = "Accesses from an untrusted master are not allowed. If an access is attempted by an untrusted master, the access is terminated with an error response and no peripheral access is initiated on the IPS bus."]
    #[inline]
    pub fn tp1(self) -> &'a mut W {
        self.variant(OPAC4W::TP1)
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
#[doc = "Values that can be written to the field `OPAC3`"]
pub enum OPAC3W {
    #[doc = "Accesses from an untrusted master are allowed."]
    TP0,
    #[doc = "Accesses from an untrusted master are not allowed. If an access is attempted by an untrusted master, the access is terminated with an error response and no peripheral access is initiated on the IPS bus."]
    TP1,
}
impl OPAC3W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            OPAC3W::TP0 => 0,
            OPAC3W::TP1 => 1,
        }
    }
}
#[doc = r" Proxy"]
pub struct _OPAC3W<'a> {
    w: &'a mut W,
}
impl<'a> _OPAC3W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: OPAC3W) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Accesses from an untrusted master are allowed."]
    #[inline]
    pub fn tp0(self) -> &'a mut W {
        self.variant(OPAC3W::TP0)
    }
    #[doc = "Accesses from an untrusted master are not allowed. If an access is attempted by an untrusted master, the access is terminated with an error response and no peripheral access is initiated on the IPS bus."]
    #[inline]
    pub fn tp1(self) -> &'a mut W {
        self.variant(OPAC3W::TP1)
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
#[doc = "Values that can be written to the field `OPAC2`"]
pub enum OPAC2W {
    #[doc = "Accesses from an untrusted master are allowed."]
    TP0,
    #[doc = "Accesses from an untrusted master are not allowed. If an access is attempted by an untrusted master, the access is terminated with an error response and no peripheral access is initiated on the IPS bus."]
    TP1,
}
impl OPAC2W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            OPAC2W::TP0 => 0,
            OPAC2W::TP1 => 1,
        }
    }
}
#[doc = r" Proxy"]
pub struct _OPAC2W<'a> {
    w: &'a mut W,
}
impl<'a> _OPAC2W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: OPAC2W) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Accesses from an untrusted master are allowed."]
    #[inline]
    pub fn tp0(self) -> &'a mut W {
        self.variant(OPAC2W::TP0)
    }
    #[doc = "Accesses from an untrusted master are not allowed. If an access is attempted by an untrusted master, the access is terminated with an error response and no peripheral access is initiated on the IPS bus."]
    #[inline]
    pub fn tp1(self) -> &'a mut W {
        self.variant(OPAC2W::TP1)
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
#[doc = "Values that can be written to the field `OPAC1`"]
pub enum OPAC1W {
    #[doc = "Accesses from an untrusted master are allowed."]
    TP0,
    #[doc = "Accesses from an untrusted master are not allowed. If an access is attempted by an untrusted master, the access is terminated with an error response and no peripheral access is initiated on the IPS bus."]
    TP1,
}
impl OPAC1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            OPAC1W::TP0 => 0,
            OPAC1W::TP1 => 1,
        }
    }
}
#[doc = r" Proxy"]
pub struct _OPAC1W<'a> {
    w: &'a mut W,
}
impl<'a> _OPAC1W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: OPAC1W) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Accesses from an untrusted master are allowed."]
    #[inline]
    pub fn tp0(self) -> &'a mut W {
        self.variant(OPAC1W::TP0)
    }
    #[doc = "Accesses from an untrusted master are not allowed. If an access is attempted by an untrusted master, the access is terminated with an error response and no peripheral access is initiated on the IPS bus."]
    #[inline]
    pub fn tp1(self) -> &'a mut W {
        self.variant(OPAC1W::TP1)
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
#[doc = "Values that can be written to the field `OPAC0`"]
pub enum OPAC0W {
    #[doc = "Accesses from an untrusted master are allowed."]
    TP0,
    #[doc = "Accesses from an untrusted master are not allowed. If an access is attempted by an untrusted master, the access is terminated with an error response and no peripheral access is initiated on the IPS bus."]
    TP1,
}
impl OPAC0W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            OPAC0W::TP0 => 0,
            OPAC0W::TP1 => 1,
        }
    }
}
#[doc = r" Proxy"]
pub struct _OPAC0W<'a> {
    w: &'a mut W,
}
impl<'a> _OPAC0W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: OPAC0W) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Accesses from an untrusted master are allowed."]
    #[inline]
    pub fn tp0(self) -> &'a mut W {
        self.variant(OPAC0W::TP0)
    }
    #[doc = "Accesses from an untrusted master are not allowed. If an access is attempted by an untrusted master, the access is terminated with an error response and no peripheral access is initiated on the IPS bus."]
    #[inline]
    pub fn tp1(self) -> &'a mut W {
        self.variant(OPAC0W::TP1)
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
    #[doc = "Bits 0:3 - Off-platform Peripheral Access Control 7"]
    #[inline]
    pub fn opac7(&self) -> OPAC7R {
        OPAC7R::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 4:7 - Off-platform Peripheral Access Control 6"]
    #[inline]
    pub fn opac6(&self) -> OPAC6R {
        OPAC6R::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 8:11 - Off-platform Peripheral Access Control 5"]
    #[inline]
    pub fn opac5(&self) -> OPAC5R {
        OPAC5R::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 12:15 - Off-platform Peripheral Access Control 4"]
    #[inline]
    pub fn opac4(&self) -> OPAC4R {
        OPAC4R::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 16:19 - Off-platform Peripheral Access Control 3"]
    #[inline]
    pub fn opac3(&self) -> OPAC3R {
        OPAC3R::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 20:23 - Off-platform Peripheral Access Control 2"]
    #[inline]
    pub fn opac2(&self) -> OPAC2R {
        OPAC2R::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 24:27 - Off-platform Peripheral Access Control 1"]
    #[inline]
    pub fn opac1(&self) -> OPAC1R {
        OPAC1R::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 28:31 - Off-platform Peripheral Access Control 0"]
    #[inline]
    pub fn opac0(&self) -> OPAC0R {
        OPAC0R::_from({
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
    #[doc = "Bits 0:3 - Off-platform Peripheral Access Control 7"]
    #[inline]
    pub fn opac7(&mut self) -> _OPAC7W {
        _OPAC7W { w: self }
    }
    #[doc = "Bits 4:7 - Off-platform Peripheral Access Control 6"]
    #[inline]
    pub fn opac6(&mut self) -> _OPAC6W {
        _OPAC6W { w: self }
    }
    #[doc = "Bits 8:11 - Off-platform Peripheral Access Control 5"]
    #[inline]
    pub fn opac5(&mut self) -> _OPAC5W {
        _OPAC5W { w: self }
    }
    #[doc = "Bits 12:15 - Off-platform Peripheral Access Control 4"]
    #[inline]
    pub fn opac4(&mut self) -> _OPAC4W {
        _OPAC4W { w: self }
    }
    #[doc = "Bits 16:19 - Off-platform Peripheral Access Control 3"]
    #[inline]
    pub fn opac3(&mut self) -> _OPAC3W {
        _OPAC3W { w: self }
    }
    #[doc = "Bits 20:23 - Off-platform Peripheral Access Control 2"]
    #[inline]
    pub fn opac2(&mut self) -> _OPAC2W {
        _OPAC2W { w: self }
    }
    #[doc = "Bits 24:27 - Off-platform Peripheral Access Control 1"]
    #[inline]
    pub fn opac1(&mut self) -> _OPAC1W {
        _OPAC1W { w: self }
    }
    #[doc = "Bits 28:31 - Off-platform Peripheral Access Control 0"]
    #[inline]
    pub fn opac0(&mut self) -> _OPAC0W {
        _OPAC0W { w: self }
    }
}
