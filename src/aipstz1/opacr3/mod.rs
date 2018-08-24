#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::OPACR3 {
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
#[doc = "Possible values of the field `OPAC31`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OPAC31R {
    #[doc = "Accesses from an untrusted master are allowed."]
    TP0,
    #[doc = "Accesses from an untrusted master are not allowed. If an access is attempted by an untrusted master, the access is terminated with an error response and no peripheral access is initiated on the IPS bus."]
    TP1,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl OPAC31R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            OPAC31R::TP0 => 0,
            OPAC31R::TP1 => 1,
            OPAC31R::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> OPAC31R {
        match value {
            0 => OPAC31R::TP0,
            1 => OPAC31R::TP1,
            i => OPAC31R::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `TP0`"]
    #[inline]
    pub fn is_tp0(&self) -> bool {
        *self == OPAC31R::TP0
    }
    #[doc = "Checks if the value of the field is `TP1`"]
    #[inline]
    pub fn is_tp1(&self) -> bool {
        *self == OPAC31R::TP1
    }
}
#[doc = "Possible values of the field `OPAC30`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OPAC30R {
    #[doc = "Accesses from an untrusted master are allowed."]
    TP0,
    #[doc = "Accesses from an untrusted master are not allowed. If an access is attempted by an untrusted master, the access is terminated with an error response and no peripheral access is initiated on the IPS bus."]
    TP1,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl OPAC30R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            OPAC30R::TP0 => 0,
            OPAC30R::TP1 => 1,
            OPAC30R::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> OPAC30R {
        match value {
            0 => OPAC30R::TP0,
            1 => OPAC30R::TP1,
            i => OPAC30R::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `TP0`"]
    #[inline]
    pub fn is_tp0(&self) -> bool {
        *self == OPAC30R::TP0
    }
    #[doc = "Checks if the value of the field is `TP1`"]
    #[inline]
    pub fn is_tp1(&self) -> bool {
        *self == OPAC30R::TP1
    }
}
#[doc = "Possible values of the field `OPAC29`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OPAC29R {
    #[doc = "Accesses from an untrusted master are allowed."]
    TP0,
    #[doc = "Accesses from an untrusted master are not allowed. If an access is attempted by an untrusted master, the access is terminated with an error response and no peripheral access is initiated on the IPS bus."]
    TP1,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl OPAC29R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            OPAC29R::TP0 => 0,
            OPAC29R::TP1 => 1,
            OPAC29R::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> OPAC29R {
        match value {
            0 => OPAC29R::TP0,
            1 => OPAC29R::TP1,
            i => OPAC29R::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `TP0`"]
    #[inline]
    pub fn is_tp0(&self) -> bool {
        *self == OPAC29R::TP0
    }
    #[doc = "Checks if the value of the field is `TP1`"]
    #[inline]
    pub fn is_tp1(&self) -> bool {
        *self == OPAC29R::TP1
    }
}
#[doc = "Possible values of the field `OPAC28`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OPAC28R {
    #[doc = "Accesses from an untrusted master are allowed."]
    TP0,
    #[doc = "Accesses from an untrusted master are not allowed. If an access is attempted by an untrusted master, the access is terminated with an error response and no peripheral access is initiated on the IPS bus."]
    TP1,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl OPAC28R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            OPAC28R::TP0 => 0,
            OPAC28R::TP1 => 1,
            OPAC28R::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> OPAC28R {
        match value {
            0 => OPAC28R::TP0,
            1 => OPAC28R::TP1,
            i => OPAC28R::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `TP0`"]
    #[inline]
    pub fn is_tp0(&self) -> bool {
        *self == OPAC28R::TP0
    }
    #[doc = "Checks if the value of the field is `TP1`"]
    #[inline]
    pub fn is_tp1(&self) -> bool {
        *self == OPAC28R::TP1
    }
}
#[doc = "Possible values of the field `OPAC27`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OPAC27R {
    #[doc = "Accesses from an untrusted master are allowed."]
    TP0,
    #[doc = "Accesses from an untrusted master are not allowed. If an access is attempted by an untrusted master, the access is terminated with an error response and no peripheral access is initiated on the IPS bus."]
    TP1,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl OPAC27R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            OPAC27R::TP0 => 0,
            OPAC27R::TP1 => 1,
            OPAC27R::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> OPAC27R {
        match value {
            0 => OPAC27R::TP0,
            1 => OPAC27R::TP1,
            i => OPAC27R::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `TP0`"]
    #[inline]
    pub fn is_tp0(&self) -> bool {
        *self == OPAC27R::TP0
    }
    #[doc = "Checks if the value of the field is `TP1`"]
    #[inline]
    pub fn is_tp1(&self) -> bool {
        *self == OPAC27R::TP1
    }
}
#[doc = "Possible values of the field `OPAC26`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OPAC26R {
    #[doc = "Accesses from an untrusted master are allowed."]
    TP0,
    #[doc = "Accesses from an untrusted master are not allowed. If an access is attempted by an untrusted master, the access is terminated with an error response and no peripheral access is initiated on the IPS bus."]
    TP1,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl OPAC26R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            OPAC26R::TP0 => 0,
            OPAC26R::TP1 => 1,
            OPAC26R::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> OPAC26R {
        match value {
            0 => OPAC26R::TP0,
            1 => OPAC26R::TP1,
            i => OPAC26R::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `TP0`"]
    #[inline]
    pub fn is_tp0(&self) -> bool {
        *self == OPAC26R::TP0
    }
    #[doc = "Checks if the value of the field is `TP1`"]
    #[inline]
    pub fn is_tp1(&self) -> bool {
        *self == OPAC26R::TP1
    }
}
#[doc = "Possible values of the field `OPAC25`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OPAC25R {
    #[doc = "Accesses from an untrusted master are allowed."]
    TP0,
    #[doc = "Accesses from an untrusted master are not allowed. If an access is attempted by an untrusted master, the access is terminated with an error response and no peripheral access is initiated on the IPS bus."]
    TP1,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl OPAC25R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            OPAC25R::TP0 => 0,
            OPAC25R::TP1 => 1,
            OPAC25R::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> OPAC25R {
        match value {
            0 => OPAC25R::TP0,
            1 => OPAC25R::TP1,
            i => OPAC25R::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `TP0`"]
    #[inline]
    pub fn is_tp0(&self) -> bool {
        *self == OPAC25R::TP0
    }
    #[doc = "Checks if the value of the field is `TP1`"]
    #[inline]
    pub fn is_tp1(&self) -> bool {
        *self == OPAC25R::TP1
    }
}
#[doc = "Possible values of the field `OPAC24`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OPAC24R {
    #[doc = "Accesses from an untrusted master are allowed."]
    TP0,
    #[doc = "Accesses from an untrusted master are not allowed. If an access is attempted by an untrusted master, the access is terminated with an error response and no peripheral access is initiated on the IPS bus."]
    TP1,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl OPAC24R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            OPAC24R::TP0 => 0,
            OPAC24R::TP1 => 1,
            OPAC24R::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> OPAC24R {
        match value {
            0 => OPAC24R::TP0,
            1 => OPAC24R::TP1,
            i => OPAC24R::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `TP0`"]
    #[inline]
    pub fn is_tp0(&self) -> bool {
        *self == OPAC24R::TP0
    }
    #[doc = "Checks if the value of the field is `TP1`"]
    #[inline]
    pub fn is_tp1(&self) -> bool {
        *self == OPAC24R::TP1
    }
}
#[doc = "Values that can be written to the field `OPAC31`"]
pub enum OPAC31W {
    #[doc = "Accesses from an untrusted master are allowed."]
    TP0,
    #[doc = "Accesses from an untrusted master are not allowed. If an access is attempted by an untrusted master, the access is terminated with an error response and no peripheral access is initiated on the IPS bus."]
    TP1,
}
impl OPAC31W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            OPAC31W::TP0 => 0,
            OPAC31W::TP1 => 1,
        }
    }
}
#[doc = r" Proxy"]
pub struct _OPAC31W<'a> {
    w: &'a mut W,
}
impl<'a> _OPAC31W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: OPAC31W) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Accesses from an untrusted master are allowed."]
    #[inline]
    pub fn tp0(self) -> &'a mut W {
        self.variant(OPAC31W::TP0)
    }
    #[doc = "Accesses from an untrusted master are not allowed. If an access is attempted by an untrusted master, the access is terminated with an error response and no peripheral access is initiated on the IPS bus."]
    #[inline]
    pub fn tp1(self) -> &'a mut W {
        self.variant(OPAC31W::TP1)
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
#[doc = "Values that can be written to the field `OPAC30`"]
pub enum OPAC30W {
    #[doc = "Accesses from an untrusted master are allowed."]
    TP0,
    #[doc = "Accesses from an untrusted master are not allowed. If an access is attempted by an untrusted master, the access is terminated with an error response and no peripheral access is initiated on the IPS bus."]
    TP1,
}
impl OPAC30W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            OPAC30W::TP0 => 0,
            OPAC30W::TP1 => 1,
        }
    }
}
#[doc = r" Proxy"]
pub struct _OPAC30W<'a> {
    w: &'a mut W,
}
impl<'a> _OPAC30W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: OPAC30W) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Accesses from an untrusted master are allowed."]
    #[inline]
    pub fn tp0(self) -> &'a mut W {
        self.variant(OPAC30W::TP0)
    }
    #[doc = "Accesses from an untrusted master are not allowed. If an access is attempted by an untrusted master, the access is terminated with an error response and no peripheral access is initiated on the IPS bus."]
    #[inline]
    pub fn tp1(self) -> &'a mut W {
        self.variant(OPAC30W::TP1)
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
#[doc = "Values that can be written to the field `OPAC29`"]
pub enum OPAC29W {
    #[doc = "Accesses from an untrusted master are allowed."]
    TP0,
    #[doc = "Accesses from an untrusted master are not allowed. If an access is attempted by an untrusted master, the access is terminated with an error response and no peripheral access is initiated on the IPS bus."]
    TP1,
}
impl OPAC29W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            OPAC29W::TP0 => 0,
            OPAC29W::TP1 => 1,
        }
    }
}
#[doc = r" Proxy"]
pub struct _OPAC29W<'a> {
    w: &'a mut W,
}
impl<'a> _OPAC29W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: OPAC29W) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Accesses from an untrusted master are allowed."]
    #[inline]
    pub fn tp0(self) -> &'a mut W {
        self.variant(OPAC29W::TP0)
    }
    #[doc = "Accesses from an untrusted master are not allowed. If an access is attempted by an untrusted master, the access is terminated with an error response and no peripheral access is initiated on the IPS bus."]
    #[inline]
    pub fn tp1(self) -> &'a mut W {
        self.variant(OPAC29W::TP1)
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
#[doc = "Values that can be written to the field `OPAC28`"]
pub enum OPAC28W {
    #[doc = "Accesses from an untrusted master are allowed."]
    TP0,
    #[doc = "Accesses from an untrusted master are not allowed. If an access is attempted by an untrusted master, the access is terminated with an error response and no peripheral access is initiated on the IPS bus."]
    TP1,
}
impl OPAC28W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            OPAC28W::TP0 => 0,
            OPAC28W::TP1 => 1,
        }
    }
}
#[doc = r" Proxy"]
pub struct _OPAC28W<'a> {
    w: &'a mut W,
}
impl<'a> _OPAC28W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: OPAC28W) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Accesses from an untrusted master are allowed."]
    #[inline]
    pub fn tp0(self) -> &'a mut W {
        self.variant(OPAC28W::TP0)
    }
    #[doc = "Accesses from an untrusted master are not allowed. If an access is attempted by an untrusted master, the access is terminated with an error response and no peripheral access is initiated on the IPS bus."]
    #[inline]
    pub fn tp1(self) -> &'a mut W {
        self.variant(OPAC28W::TP1)
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
#[doc = "Values that can be written to the field `OPAC27`"]
pub enum OPAC27W {
    #[doc = "Accesses from an untrusted master are allowed."]
    TP0,
    #[doc = "Accesses from an untrusted master are not allowed. If an access is attempted by an untrusted master, the access is terminated with an error response and no peripheral access is initiated on the IPS bus."]
    TP1,
}
impl OPAC27W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            OPAC27W::TP0 => 0,
            OPAC27W::TP1 => 1,
        }
    }
}
#[doc = r" Proxy"]
pub struct _OPAC27W<'a> {
    w: &'a mut W,
}
impl<'a> _OPAC27W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: OPAC27W) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Accesses from an untrusted master are allowed."]
    #[inline]
    pub fn tp0(self) -> &'a mut W {
        self.variant(OPAC27W::TP0)
    }
    #[doc = "Accesses from an untrusted master are not allowed. If an access is attempted by an untrusted master, the access is terminated with an error response and no peripheral access is initiated on the IPS bus."]
    #[inline]
    pub fn tp1(self) -> &'a mut W {
        self.variant(OPAC27W::TP1)
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
#[doc = "Values that can be written to the field `OPAC26`"]
pub enum OPAC26W {
    #[doc = "Accesses from an untrusted master are allowed."]
    TP0,
    #[doc = "Accesses from an untrusted master are not allowed. If an access is attempted by an untrusted master, the access is terminated with an error response and no peripheral access is initiated on the IPS bus."]
    TP1,
}
impl OPAC26W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            OPAC26W::TP0 => 0,
            OPAC26W::TP1 => 1,
        }
    }
}
#[doc = r" Proxy"]
pub struct _OPAC26W<'a> {
    w: &'a mut W,
}
impl<'a> _OPAC26W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: OPAC26W) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Accesses from an untrusted master are allowed."]
    #[inline]
    pub fn tp0(self) -> &'a mut W {
        self.variant(OPAC26W::TP0)
    }
    #[doc = "Accesses from an untrusted master are not allowed. If an access is attempted by an untrusted master, the access is terminated with an error response and no peripheral access is initiated on the IPS bus."]
    #[inline]
    pub fn tp1(self) -> &'a mut W {
        self.variant(OPAC26W::TP1)
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
#[doc = "Values that can be written to the field `OPAC25`"]
pub enum OPAC25W {
    #[doc = "Accesses from an untrusted master are allowed."]
    TP0,
    #[doc = "Accesses from an untrusted master are not allowed. If an access is attempted by an untrusted master, the access is terminated with an error response and no peripheral access is initiated on the IPS bus."]
    TP1,
}
impl OPAC25W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            OPAC25W::TP0 => 0,
            OPAC25W::TP1 => 1,
        }
    }
}
#[doc = r" Proxy"]
pub struct _OPAC25W<'a> {
    w: &'a mut W,
}
impl<'a> _OPAC25W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: OPAC25W) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Accesses from an untrusted master are allowed."]
    #[inline]
    pub fn tp0(self) -> &'a mut W {
        self.variant(OPAC25W::TP0)
    }
    #[doc = "Accesses from an untrusted master are not allowed. If an access is attempted by an untrusted master, the access is terminated with an error response and no peripheral access is initiated on the IPS bus."]
    #[inline]
    pub fn tp1(self) -> &'a mut W {
        self.variant(OPAC25W::TP1)
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
#[doc = "Values that can be written to the field `OPAC24`"]
pub enum OPAC24W {
    #[doc = "Accesses from an untrusted master are allowed."]
    TP0,
    #[doc = "Accesses from an untrusted master are not allowed. If an access is attempted by an untrusted master, the access is terminated with an error response and no peripheral access is initiated on the IPS bus."]
    TP1,
}
impl OPAC24W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            OPAC24W::TP0 => 0,
            OPAC24W::TP1 => 1,
        }
    }
}
#[doc = r" Proxy"]
pub struct _OPAC24W<'a> {
    w: &'a mut W,
}
impl<'a> _OPAC24W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: OPAC24W) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Accesses from an untrusted master are allowed."]
    #[inline]
    pub fn tp0(self) -> &'a mut W {
        self.variant(OPAC24W::TP0)
    }
    #[doc = "Accesses from an untrusted master are not allowed. If an access is attempted by an untrusted master, the access is terminated with an error response and no peripheral access is initiated on the IPS bus."]
    #[inline]
    pub fn tp1(self) -> &'a mut W {
        self.variant(OPAC24W::TP1)
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
    #[doc = "Bits 0:3 - Off-platform Peripheral Access Control 31"]
    #[inline]
    pub fn opac31(&self) -> OPAC31R {
        OPAC31R::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 4:7 - Off-platform Peripheral Access Control 30"]
    #[inline]
    pub fn opac30(&self) -> OPAC30R {
        OPAC30R::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 8:11 - Off-platform Peripheral Access Control 29"]
    #[inline]
    pub fn opac29(&self) -> OPAC29R {
        OPAC29R::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 12:15 - Off-platform Peripheral Access Control 28"]
    #[inline]
    pub fn opac28(&self) -> OPAC28R {
        OPAC28R::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 16:19 - Off-platform Peripheral Access Control 27"]
    #[inline]
    pub fn opac27(&self) -> OPAC27R {
        OPAC27R::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 20:23 - Off-platform Peripheral Access Control 26"]
    #[inline]
    pub fn opac26(&self) -> OPAC26R {
        OPAC26R::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 24:27 - Off-platform Peripheral Access Control 25"]
    #[inline]
    pub fn opac25(&self) -> OPAC25R {
        OPAC25R::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 28:31 - Off-platform Peripheral Access Control 24"]
    #[inline]
    pub fn opac24(&self) -> OPAC24R {
        OPAC24R::_from({
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
    #[doc = "Bits 0:3 - Off-platform Peripheral Access Control 31"]
    #[inline]
    pub fn opac31(&mut self) -> _OPAC31W {
        _OPAC31W { w: self }
    }
    #[doc = "Bits 4:7 - Off-platform Peripheral Access Control 30"]
    #[inline]
    pub fn opac30(&mut self) -> _OPAC30W {
        _OPAC30W { w: self }
    }
    #[doc = "Bits 8:11 - Off-platform Peripheral Access Control 29"]
    #[inline]
    pub fn opac29(&mut self) -> _OPAC29W {
        _OPAC29W { w: self }
    }
    #[doc = "Bits 12:15 - Off-platform Peripheral Access Control 28"]
    #[inline]
    pub fn opac28(&mut self) -> _OPAC28W {
        _OPAC28W { w: self }
    }
    #[doc = "Bits 16:19 - Off-platform Peripheral Access Control 27"]
    #[inline]
    pub fn opac27(&mut self) -> _OPAC27W {
        _OPAC27W { w: self }
    }
    #[doc = "Bits 20:23 - Off-platform Peripheral Access Control 26"]
    #[inline]
    pub fn opac26(&mut self) -> _OPAC26W {
        _OPAC26W { w: self }
    }
    #[doc = "Bits 24:27 - Off-platform Peripheral Access Control 25"]
    #[inline]
    pub fn opac25(&mut self) -> _OPAC25W {
        _OPAC25W { w: self }
    }
    #[doc = "Bits 28:31 - Off-platform Peripheral Access Control 24"]
    #[inline]
    pub fn opac24(&mut self) -> _OPAC24W {
        _OPAC24W { w: self }
    }
}
