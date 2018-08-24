#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::OPACR1 {
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
#[doc = "Possible values of the field `OPAC15`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OPAC15R {
    #[doc = "Accesses from an untrusted master are allowed."]
    TP0,
    #[doc = "Accesses from an untrusted master are not allowed. If an access is attempted by an untrusted master, the access is terminated with an error response and no peripheral access is initiated on the IPS bus."]
    TP1,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl OPAC15R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            OPAC15R::TP0 => 0,
            OPAC15R::TP1 => 1,
            OPAC15R::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> OPAC15R {
        match value {
            0 => OPAC15R::TP0,
            1 => OPAC15R::TP1,
            i => OPAC15R::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `TP0`"]
    #[inline]
    pub fn is_tp0(&self) -> bool {
        *self == OPAC15R::TP0
    }
    #[doc = "Checks if the value of the field is `TP1`"]
    #[inline]
    pub fn is_tp1(&self) -> bool {
        *self == OPAC15R::TP1
    }
}
#[doc = "Possible values of the field `OPAC14`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OPAC14R {
    #[doc = "Accesses from an untrusted master are allowed."]
    TP0,
    #[doc = "Accesses from an untrusted master are not allowed. If an access is attempted by an untrusted master, the access is terminated with an error response and no peripheral access is initiated on the IPS bus."]
    TP1,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl OPAC14R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            OPAC14R::TP0 => 0,
            OPAC14R::TP1 => 1,
            OPAC14R::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> OPAC14R {
        match value {
            0 => OPAC14R::TP0,
            1 => OPAC14R::TP1,
            i => OPAC14R::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `TP0`"]
    #[inline]
    pub fn is_tp0(&self) -> bool {
        *self == OPAC14R::TP0
    }
    #[doc = "Checks if the value of the field is `TP1`"]
    #[inline]
    pub fn is_tp1(&self) -> bool {
        *self == OPAC14R::TP1
    }
}
#[doc = "Possible values of the field `OPAC13`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OPAC13R {
    #[doc = "Accesses from an untrusted master are allowed."]
    TP0,
    #[doc = "Accesses from an untrusted master are not allowed. If an access is attempted by an untrusted master, the access is terminated with an error response and no peripheral access is initiated on the IPS bus."]
    TP1,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl OPAC13R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            OPAC13R::TP0 => 0,
            OPAC13R::TP1 => 1,
            OPAC13R::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> OPAC13R {
        match value {
            0 => OPAC13R::TP0,
            1 => OPAC13R::TP1,
            i => OPAC13R::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `TP0`"]
    #[inline]
    pub fn is_tp0(&self) -> bool {
        *self == OPAC13R::TP0
    }
    #[doc = "Checks if the value of the field is `TP1`"]
    #[inline]
    pub fn is_tp1(&self) -> bool {
        *self == OPAC13R::TP1
    }
}
#[doc = "Possible values of the field `OPAC12`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OPAC12R {
    #[doc = "Accesses from an untrusted master are allowed."]
    TP0,
    #[doc = "Accesses from an untrusted master are not allowed. If an access is attempted by an untrusted master, the access is terminated with an error response and no peripheral access is initiated on the IPS bus."]
    TP1,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl OPAC12R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            OPAC12R::TP0 => 0,
            OPAC12R::TP1 => 1,
            OPAC12R::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> OPAC12R {
        match value {
            0 => OPAC12R::TP0,
            1 => OPAC12R::TP1,
            i => OPAC12R::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `TP0`"]
    #[inline]
    pub fn is_tp0(&self) -> bool {
        *self == OPAC12R::TP0
    }
    #[doc = "Checks if the value of the field is `TP1`"]
    #[inline]
    pub fn is_tp1(&self) -> bool {
        *self == OPAC12R::TP1
    }
}
#[doc = "Possible values of the field `OPAC11`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OPAC11R {
    #[doc = "Accesses from an untrusted master are allowed."]
    TP0,
    #[doc = "Accesses from an untrusted master are not allowed. If an access is attempted by an untrusted master, the access is terminated with an error response and no peripheral access is initiated on the IPS bus."]
    TP1,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl OPAC11R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            OPAC11R::TP0 => 0,
            OPAC11R::TP1 => 1,
            OPAC11R::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> OPAC11R {
        match value {
            0 => OPAC11R::TP0,
            1 => OPAC11R::TP1,
            i => OPAC11R::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `TP0`"]
    #[inline]
    pub fn is_tp0(&self) -> bool {
        *self == OPAC11R::TP0
    }
    #[doc = "Checks if the value of the field is `TP1`"]
    #[inline]
    pub fn is_tp1(&self) -> bool {
        *self == OPAC11R::TP1
    }
}
#[doc = "Possible values of the field `OPAC10`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OPAC10R {
    #[doc = "Accesses from an untrusted master are allowed."]
    TP0,
    #[doc = "Accesses from an untrusted master are not allowed. If an access is attempted by an untrusted master, the access is terminated with an error response and no peripheral access is initiated on the IPS bus."]
    TP1,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl OPAC10R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            OPAC10R::TP0 => 0,
            OPAC10R::TP1 => 1,
            OPAC10R::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> OPAC10R {
        match value {
            0 => OPAC10R::TP0,
            1 => OPAC10R::TP1,
            i => OPAC10R::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `TP0`"]
    #[inline]
    pub fn is_tp0(&self) -> bool {
        *self == OPAC10R::TP0
    }
    #[doc = "Checks if the value of the field is `TP1`"]
    #[inline]
    pub fn is_tp1(&self) -> bool {
        *self == OPAC10R::TP1
    }
}
#[doc = "Possible values of the field `OPAC9`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OPAC9R {
    #[doc = "Accesses from an untrusted master are allowed."]
    TP0,
    #[doc = "Accesses from an untrusted master are not allowed. If an access is attempted by an untrusted master, the access is terminated with an error response and no peripheral access is initiated on the IPS bus."]
    TP1,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl OPAC9R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            OPAC9R::TP0 => 0,
            OPAC9R::TP1 => 1,
            OPAC9R::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> OPAC9R {
        match value {
            0 => OPAC9R::TP0,
            1 => OPAC9R::TP1,
            i => OPAC9R::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `TP0`"]
    #[inline]
    pub fn is_tp0(&self) -> bool {
        *self == OPAC9R::TP0
    }
    #[doc = "Checks if the value of the field is `TP1`"]
    #[inline]
    pub fn is_tp1(&self) -> bool {
        *self == OPAC9R::TP1
    }
}
#[doc = "Possible values of the field `OPAC8`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OPAC8R {
    #[doc = "Accesses from an untrusted master are allowed."]
    TP0,
    #[doc = "Accesses from an untrusted master are not allowed. If an access is attempted by an untrusted master, the access is terminated with an error response and no peripheral access is initiated on the IPS bus."]
    TP1,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl OPAC8R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            OPAC8R::TP0 => 0,
            OPAC8R::TP1 => 1,
            OPAC8R::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> OPAC8R {
        match value {
            0 => OPAC8R::TP0,
            1 => OPAC8R::TP1,
            i => OPAC8R::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `TP0`"]
    #[inline]
    pub fn is_tp0(&self) -> bool {
        *self == OPAC8R::TP0
    }
    #[doc = "Checks if the value of the field is `TP1`"]
    #[inline]
    pub fn is_tp1(&self) -> bool {
        *self == OPAC8R::TP1
    }
}
#[doc = "Values that can be written to the field `OPAC15`"]
pub enum OPAC15W {
    #[doc = "Accesses from an untrusted master are allowed."]
    TP0,
    #[doc = "Accesses from an untrusted master are not allowed. If an access is attempted by an untrusted master, the access is terminated with an error response and no peripheral access is initiated on the IPS bus."]
    TP1,
}
impl OPAC15W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            OPAC15W::TP0 => 0,
            OPAC15W::TP1 => 1,
        }
    }
}
#[doc = r" Proxy"]
pub struct _OPAC15W<'a> {
    w: &'a mut W,
}
impl<'a> _OPAC15W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: OPAC15W) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Accesses from an untrusted master are allowed."]
    #[inline]
    pub fn tp0(self) -> &'a mut W {
        self.variant(OPAC15W::TP0)
    }
    #[doc = "Accesses from an untrusted master are not allowed. If an access is attempted by an untrusted master, the access is terminated with an error response and no peripheral access is initiated on the IPS bus."]
    #[inline]
    pub fn tp1(self) -> &'a mut W {
        self.variant(OPAC15W::TP1)
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
#[doc = "Values that can be written to the field `OPAC14`"]
pub enum OPAC14W {
    #[doc = "Accesses from an untrusted master are allowed."]
    TP0,
    #[doc = "Accesses from an untrusted master are not allowed. If an access is attempted by an untrusted master, the access is terminated with an error response and no peripheral access is initiated on the IPS bus."]
    TP1,
}
impl OPAC14W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            OPAC14W::TP0 => 0,
            OPAC14W::TP1 => 1,
        }
    }
}
#[doc = r" Proxy"]
pub struct _OPAC14W<'a> {
    w: &'a mut W,
}
impl<'a> _OPAC14W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: OPAC14W) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Accesses from an untrusted master are allowed."]
    #[inline]
    pub fn tp0(self) -> &'a mut W {
        self.variant(OPAC14W::TP0)
    }
    #[doc = "Accesses from an untrusted master are not allowed. If an access is attempted by an untrusted master, the access is terminated with an error response and no peripheral access is initiated on the IPS bus."]
    #[inline]
    pub fn tp1(self) -> &'a mut W {
        self.variant(OPAC14W::TP1)
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
#[doc = "Values that can be written to the field `OPAC13`"]
pub enum OPAC13W {
    #[doc = "Accesses from an untrusted master are allowed."]
    TP0,
    #[doc = "Accesses from an untrusted master are not allowed. If an access is attempted by an untrusted master, the access is terminated with an error response and no peripheral access is initiated on the IPS bus."]
    TP1,
}
impl OPAC13W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            OPAC13W::TP0 => 0,
            OPAC13W::TP1 => 1,
        }
    }
}
#[doc = r" Proxy"]
pub struct _OPAC13W<'a> {
    w: &'a mut W,
}
impl<'a> _OPAC13W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: OPAC13W) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Accesses from an untrusted master are allowed."]
    #[inline]
    pub fn tp0(self) -> &'a mut W {
        self.variant(OPAC13W::TP0)
    }
    #[doc = "Accesses from an untrusted master are not allowed. If an access is attempted by an untrusted master, the access is terminated with an error response and no peripheral access is initiated on the IPS bus."]
    #[inline]
    pub fn tp1(self) -> &'a mut W {
        self.variant(OPAC13W::TP1)
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
#[doc = "Values that can be written to the field `OPAC12`"]
pub enum OPAC12W {
    #[doc = "Accesses from an untrusted master are allowed."]
    TP0,
    #[doc = "Accesses from an untrusted master are not allowed. If an access is attempted by an untrusted master, the access is terminated with an error response and no peripheral access is initiated on the IPS bus."]
    TP1,
}
impl OPAC12W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            OPAC12W::TP0 => 0,
            OPAC12W::TP1 => 1,
        }
    }
}
#[doc = r" Proxy"]
pub struct _OPAC12W<'a> {
    w: &'a mut W,
}
impl<'a> _OPAC12W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: OPAC12W) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Accesses from an untrusted master are allowed."]
    #[inline]
    pub fn tp0(self) -> &'a mut W {
        self.variant(OPAC12W::TP0)
    }
    #[doc = "Accesses from an untrusted master are not allowed. If an access is attempted by an untrusted master, the access is terminated with an error response and no peripheral access is initiated on the IPS bus."]
    #[inline]
    pub fn tp1(self) -> &'a mut W {
        self.variant(OPAC12W::TP1)
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
#[doc = "Values that can be written to the field `OPAC11`"]
pub enum OPAC11W {
    #[doc = "Accesses from an untrusted master are allowed."]
    TP0,
    #[doc = "Accesses from an untrusted master are not allowed. If an access is attempted by an untrusted master, the access is terminated with an error response and no peripheral access is initiated on the IPS bus."]
    TP1,
}
impl OPAC11W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            OPAC11W::TP0 => 0,
            OPAC11W::TP1 => 1,
        }
    }
}
#[doc = r" Proxy"]
pub struct _OPAC11W<'a> {
    w: &'a mut W,
}
impl<'a> _OPAC11W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: OPAC11W) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Accesses from an untrusted master are allowed."]
    #[inline]
    pub fn tp0(self) -> &'a mut W {
        self.variant(OPAC11W::TP0)
    }
    #[doc = "Accesses from an untrusted master are not allowed. If an access is attempted by an untrusted master, the access is terminated with an error response and no peripheral access is initiated on the IPS bus."]
    #[inline]
    pub fn tp1(self) -> &'a mut W {
        self.variant(OPAC11W::TP1)
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
#[doc = "Values that can be written to the field `OPAC10`"]
pub enum OPAC10W {
    #[doc = "Accesses from an untrusted master are allowed."]
    TP0,
    #[doc = "Accesses from an untrusted master are not allowed. If an access is attempted by an untrusted master, the access is terminated with an error response and no peripheral access is initiated on the IPS bus."]
    TP1,
}
impl OPAC10W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            OPAC10W::TP0 => 0,
            OPAC10W::TP1 => 1,
        }
    }
}
#[doc = r" Proxy"]
pub struct _OPAC10W<'a> {
    w: &'a mut W,
}
impl<'a> _OPAC10W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: OPAC10W) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Accesses from an untrusted master are allowed."]
    #[inline]
    pub fn tp0(self) -> &'a mut W {
        self.variant(OPAC10W::TP0)
    }
    #[doc = "Accesses from an untrusted master are not allowed. If an access is attempted by an untrusted master, the access is terminated with an error response and no peripheral access is initiated on the IPS bus."]
    #[inline]
    pub fn tp1(self) -> &'a mut W {
        self.variant(OPAC10W::TP1)
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
#[doc = "Values that can be written to the field `OPAC9`"]
pub enum OPAC9W {
    #[doc = "Accesses from an untrusted master are allowed."]
    TP0,
    #[doc = "Accesses from an untrusted master are not allowed. If an access is attempted by an untrusted master, the access is terminated with an error response and no peripheral access is initiated on the IPS bus."]
    TP1,
}
impl OPAC9W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            OPAC9W::TP0 => 0,
            OPAC9W::TP1 => 1,
        }
    }
}
#[doc = r" Proxy"]
pub struct _OPAC9W<'a> {
    w: &'a mut W,
}
impl<'a> _OPAC9W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: OPAC9W) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Accesses from an untrusted master are allowed."]
    #[inline]
    pub fn tp0(self) -> &'a mut W {
        self.variant(OPAC9W::TP0)
    }
    #[doc = "Accesses from an untrusted master are not allowed. If an access is attempted by an untrusted master, the access is terminated with an error response and no peripheral access is initiated on the IPS bus."]
    #[inline]
    pub fn tp1(self) -> &'a mut W {
        self.variant(OPAC9W::TP1)
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
#[doc = "Values that can be written to the field `OPAC8`"]
pub enum OPAC8W {
    #[doc = "Accesses from an untrusted master are allowed."]
    TP0,
    #[doc = "Accesses from an untrusted master are not allowed. If an access is attempted by an untrusted master, the access is terminated with an error response and no peripheral access is initiated on the IPS bus."]
    TP1,
}
impl OPAC8W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            OPAC8W::TP0 => 0,
            OPAC8W::TP1 => 1,
        }
    }
}
#[doc = r" Proxy"]
pub struct _OPAC8W<'a> {
    w: &'a mut W,
}
impl<'a> _OPAC8W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: OPAC8W) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Accesses from an untrusted master are allowed."]
    #[inline]
    pub fn tp0(self) -> &'a mut W {
        self.variant(OPAC8W::TP0)
    }
    #[doc = "Accesses from an untrusted master are not allowed. If an access is attempted by an untrusted master, the access is terminated with an error response and no peripheral access is initiated on the IPS bus."]
    #[inline]
    pub fn tp1(self) -> &'a mut W {
        self.variant(OPAC8W::TP1)
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
    #[doc = "Bits 0:3 - Off-platform Peripheral Access Control 15"]
    #[inline]
    pub fn opac15(&self) -> OPAC15R {
        OPAC15R::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 4:7 - Off-platform Peripheral Access Control 14"]
    #[inline]
    pub fn opac14(&self) -> OPAC14R {
        OPAC14R::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 8:11 - Off-platform Peripheral Access Control 13"]
    #[inline]
    pub fn opac13(&self) -> OPAC13R {
        OPAC13R::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 12:15 - Off-platform Peripheral Access Control 12"]
    #[inline]
    pub fn opac12(&self) -> OPAC12R {
        OPAC12R::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 16:19 - Off-platform Peripheral Access Control 11"]
    #[inline]
    pub fn opac11(&self) -> OPAC11R {
        OPAC11R::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 20:23 - Off-platform Peripheral Access Control 10"]
    #[inline]
    pub fn opac10(&self) -> OPAC10R {
        OPAC10R::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 24:27 - Off-platform Peripheral Access Control 9"]
    #[inline]
    pub fn opac9(&self) -> OPAC9R {
        OPAC9R::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 28:31 - Off-platform Peripheral Access Control 8"]
    #[inline]
    pub fn opac8(&self) -> OPAC8R {
        OPAC8R::_from({
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
    #[doc = "Bits 0:3 - Off-platform Peripheral Access Control 15"]
    #[inline]
    pub fn opac15(&mut self) -> _OPAC15W {
        _OPAC15W { w: self }
    }
    #[doc = "Bits 4:7 - Off-platform Peripheral Access Control 14"]
    #[inline]
    pub fn opac14(&mut self) -> _OPAC14W {
        _OPAC14W { w: self }
    }
    #[doc = "Bits 8:11 - Off-platform Peripheral Access Control 13"]
    #[inline]
    pub fn opac13(&mut self) -> _OPAC13W {
        _OPAC13W { w: self }
    }
    #[doc = "Bits 12:15 - Off-platform Peripheral Access Control 12"]
    #[inline]
    pub fn opac12(&mut self) -> _OPAC12W {
        _OPAC12W { w: self }
    }
    #[doc = "Bits 16:19 - Off-platform Peripheral Access Control 11"]
    #[inline]
    pub fn opac11(&mut self) -> _OPAC11W {
        _OPAC11W { w: self }
    }
    #[doc = "Bits 20:23 - Off-platform Peripheral Access Control 10"]
    #[inline]
    pub fn opac10(&mut self) -> _OPAC10W {
        _OPAC10W { w: self }
    }
    #[doc = "Bits 24:27 - Off-platform Peripheral Access Control 9"]
    #[inline]
    pub fn opac9(&mut self) -> _OPAC9W {
        _OPAC9W { w: self }
    }
    #[doc = "Bits 28:31 - Off-platform Peripheral Access Control 8"]
    #[inline]
    pub fn opac8(&mut self) -> _OPAC8W {
        _OPAC8W { w: self }
    }
}
