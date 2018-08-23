#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::MPR {
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
#[doc = "Possible values of the field `MPROT5`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MPROT5R {
    #[doc = "Accesses from this master are forced to user-mode (ips_supervisor_access is forced to zero) regardless of the hprot[1] access attribute."]
    MPL0,
    #[doc = "Accesses from this master are not forced to user-mode. The hprot[1] access attribute is used directly to determine ips_supervisor_access."]
    MPL1,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl MPROT5R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            MPROT5R::MPL0 => 0,
            MPROT5R::MPL1 => 1,
            MPROT5R::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> MPROT5R {
        match value {
            0 => MPROT5R::MPL0,
            1 => MPROT5R::MPL1,
            i => MPROT5R::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `MPL0`"]
    #[inline]
    pub fn is_mpl0(&self) -> bool {
        *self == MPROT5R::MPL0
    }
    #[doc = "Checks if the value of the field is `MPL1`"]
    #[inline]
    pub fn is_mpl1(&self) -> bool {
        *self == MPROT5R::MPL1
    }
}
#[doc = "Possible values of the field `MPROT3`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MPROT3R {
    #[doc = "Accesses from this master are forced to user-mode (ips_supervisor_access is forced to zero) regardless of the hprot[1] access attribute."]
    MPL0,
    #[doc = "Accesses from this master are not forced to user-mode. The hprot[1] access attribute is used directly to determine ips_supervisor_access."]
    MPL1,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl MPROT3R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            MPROT3R::MPL0 => 0,
            MPROT3R::MPL1 => 1,
            MPROT3R::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> MPROT3R {
        match value {
            0 => MPROT3R::MPL0,
            1 => MPROT3R::MPL1,
            i => MPROT3R::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `MPL0`"]
    #[inline]
    pub fn is_mpl0(&self) -> bool {
        *self == MPROT3R::MPL0
    }
    #[doc = "Checks if the value of the field is `MPL1`"]
    #[inline]
    pub fn is_mpl1(&self) -> bool {
        *self == MPROT3R::MPL1
    }
}
#[doc = "Possible values of the field `MPROT2`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MPROT2R {
    #[doc = "Accesses from this master are forced to user-mode (ips_supervisor_access is forced to zero) regardless of the hprot[1] access attribute."]
    MPL0,
    #[doc = "Accesses from this master are not forced to user-mode. The hprot[1] access attribute is used directly to determine ips_supervisor_access."]
    MPL1,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl MPROT2R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            MPROT2R::MPL0 => 0,
            MPROT2R::MPL1 => 1,
            MPROT2R::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> MPROT2R {
        match value {
            0 => MPROT2R::MPL0,
            1 => MPROT2R::MPL1,
            i => MPROT2R::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `MPL0`"]
    #[inline]
    pub fn is_mpl0(&self) -> bool {
        *self == MPROT2R::MPL0
    }
    #[doc = "Checks if the value of the field is `MPL1`"]
    #[inline]
    pub fn is_mpl1(&self) -> bool {
        *self == MPROT2R::MPL1
    }
}
#[doc = "Possible values of the field `MPROT1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MPROT1R {
    #[doc = "Accesses from this master are forced to user-mode (ips_supervisor_access is forced to zero) regardless of the hprot[1] access attribute."]
    MPL0,
    #[doc = "Accesses from this master are not forced to user-mode. The hprot[1] access attribute is used directly to determine ips_supervisor_access."]
    MPL1,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl MPROT1R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            MPROT1R::MPL0 => 0,
            MPROT1R::MPL1 => 1,
            MPROT1R::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> MPROT1R {
        match value {
            0 => MPROT1R::MPL0,
            1 => MPROT1R::MPL1,
            i => MPROT1R::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `MPL0`"]
    #[inline]
    pub fn is_mpl0(&self) -> bool {
        *self == MPROT1R::MPL0
    }
    #[doc = "Checks if the value of the field is `MPL1`"]
    #[inline]
    pub fn is_mpl1(&self) -> bool {
        *self == MPROT1R::MPL1
    }
}
#[doc = "Possible values of the field `MPROT0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MPROT0R {
    #[doc = "Accesses from this master are forced to user-mode (ips_supervisor_access is forced to zero) regardless of the hprot[1] access attribute."]
    MPL0,
    #[doc = "Accesses from this master are not forced to user-mode. The hprot[1] access attribute is used directly to determine ips_supervisor_access."]
    MPL1,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl MPROT0R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            MPROT0R::MPL0 => 0,
            MPROT0R::MPL1 => 1,
            MPROT0R::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> MPROT0R {
        match value {
            0 => MPROT0R::MPL0,
            1 => MPROT0R::MPL1,
            i => MPROT0R::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `MPL0`"]
    #[inline]
    pub fn is_mpl0(&self) -> bool {
        *self == MPROT0R::MPL0
    }
    #[doc = "Checks if the value of the field is `MPL1`"]
    #[inline]
    pub fn is_mpl1(&self) -> bool {
        *self == MPROT0R::MPL1
    }
}
#[doc = "Values that can be written to the field `MPROT5`"]
pub enum MPROT5W {
    #[doc = "Accesses from this master are forced to user-mode (ips_supervisor_access is forced to zero) regardless of the hprot[1] access attribute."]
    MPL0,
    #[doc = "Accesses from this master are not forced to user-mode. The hprot[1] access attribute is used directly to determine ips_supervisor_access."]
    MPL1,
}
impl MPROT5W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            MPROT5W::MPL0 => 0,
            MPROT5W::MPL1 => 1,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MPROT5W<'a> {
    w: &'a mut W,
}
impl<'a> _MPROT5W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MPROT5W) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Accesses from this master are forced to user-mode (ips_supervisor_access is forced to zero) regardless of the hprot[1] access attribute."]
    #[inline]
    pub fn mpl0(self) -> &'a mut W {
        self.variant(MPROT5W::MPL0)
    }
    #[doc = "Accesses from this master are not forced to user-mode. The hprot[1] access attribute is used directly to determine ips_supervisor_access."]
    #[inline]
    pub fn mpl1(self) -> &'a mut W {
        self.variant(MPROT5W::MPL1)
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
#[doc = "Values that can be written to the field `MPROT3`"]
pub enum MPROT3W {
    #[doc = "Accesses from this master are forced to user-mode (ips_supervisor_access is forced to zero) regardless of the hprot[1] access attribute."]
    MPL0,
    #[doc = "Accesses from this master are not forced to user-mode. The hprot[1] access attribute is used directly to determine ips_supervisor_access."]
    MPL1,
}
impl MPROT3W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            MPROT3W::MPL0 => 0,
            MPROT3W::MPL1 => 1,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MPROT3W<'a> {
    w: &'a mut W,
}
impl<'a> _MPROT3W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MPROT3W) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Accesses from this master are forced to user-mode (ips_supervisor_access is forced to zero) regardless of the hprot[1] access attribute."]
    #[inline]
    pub fn mpl0(self) -> &'a mut W {
        self.variant(MPROT3W::MPL0)
    }
    #[doc = "Accesses from this master are not forced to user-mode. The hprot[1] access attribute is used directly to determine ips_supervisor_access."]
    #[inline]
    pub fn mpl1(self) -> &'a mut W {
        self.variant(MPROT3W::MPL1)
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
#[doc = "Values that can be written to the field `MPROT2`"]
pub enum MPROT2W {
    #[doc = "Accesses from this master are forced to user-mode (ips_supervisor_access is forced to zero) regardless of the hprot[1] access attribute."]
    MPL0,
    #[doc = "Accesses from this master are not forced to user-mode. The hprot[1] access attribute is used directly to determine ips_supervisor_access."]
    MPL1,
}
impl MPROT2W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            MPROT2W::MPL0 => 0,
            MPROT2W::MPL1 => 1,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MPROT2W<'a> {
    w: &'a mut W,
}
impl<'a> _MPROT2W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MPROT2W) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Accesses from this master are forced to user-mode (ips_supervisor_access is forced to zero) regardless of the hprot[1] access attribute."]
    #[inline]
    pub fn mpl0(self) -> &'a mut W {
        self.variant(MPROT2W::MPL0)
    }
    #[doc = "Accesses from this master are not forced to user-mode. The hprot[1] access attribute is used directly to determine ips_supervisor_access."]
    #[inline]
    pub fn mpl1(self) -> &'a mut W {
        self.variant(MPROT2W::MPL1)
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
#[doc = "Values that can be written to the field `MPROT1`"]
pub enum MPROT1W {
    #[doc = "Accesses from this master are forced to user-mode (ips_supervisor_access is forced to zero) regardless of the hprot[1] access attribute."]
    MPL0,
    #[doc = "Accesses from this master are not forced to user-mode. The hprot[1] access attribute is used directly to determine ips_supervisor_access."]
    MPL1,
}
impl MPROT1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            MPROT1W::MPL0 => 0,
            MPROT1W::MPL1 => 1,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MPROT1W<'a> {
    w: &'a mut W,
}
impl<'a> _MPROT1W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MPROT1W) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Accesses from this master are forced to user-mode (ips_supervisor_access is forced to zero) regardless of the hprot[1] access attribute."]
    #[inline]
    pub fn mpl0(self) -> &'a mut W {
        self.variant(MPROT1W::MPL0)
    }
    #[doc = "Accesses from this master are not forced to user-mode. The hprot[1] access attribute is used directly to determine ips_supervisor_access."]
    #[inline]
    pub fn mpl1(self) -> &'a mut W {
        self.variant(MPROT1W::MPL1)
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
#[doc = "Values that can be written to the field `MPROT0`"]
pub enum MPROT0W {
    #[doc = "Accesses from this master are forced to user-mode (ips_supervisor_access is forced to zero) regardless of the hprot[1] access attribute."]
    MPL0,
    #[doc = "Accesses from this master are not forced to user-mode. The hprot[1] access attribute is used directly to determine ips_supervisor_access."]
    MPL1,
}
impl MPROT0W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            MPROT0W::MPL0 => 0,
            MPROT0W::MPL1 => 1,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MPROT0W<'a> {
    w: &'a mut W,
}
impl<'a> _MPROT0W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MPROT0W) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Accesses from this master are forced to user-mode (ips_supervisor_access is forced to zero) regardless of the hprot[1] access attribute."]
    #[inline]
    pub fn mpl0(self) -> &'a mut W {
        self.variant(MPROT0W::MPL0)
    }
    #[doc = "Accesses from this master are not forced to user-mode. The hprot[1] access attribute is used directly to determine ips_supervisor_access."]
    #[inline]
    pub fn mpl1(self) -> &'a mut W {
        self.variant(MPROT0W::MPL1)
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
    #[doc = "Bits 8:11 - Master 5 Priviledge, Buffer, Read, Write Control."]
    #[inline]
    pub fn mprot5(&self) -> MPROT5R {
        MPROT5R::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 16:19 - Master 3 Priviledge, Buffer, Read, Write Control."]
    #[inline]
    pub fn mprot3(&self) -> MPROT3R {
        MPROT3R::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 20:23 - Master 2 Priviledge, Buffer, Read, Write Control"]
    #[inline]
    pub fn mprot2(&self) -> MPROT2R {
        MPROT2R::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 24:27 - Master 1 Priviledge, Buffer, Read, Write Control"]
    #[inline]
    pub fn mprot1(&self) -> MPROT1R {
        MPROT1R::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 28:31 - Master 0 Priviledge, Buffer, Read, Write Control"]
    #[inline]
    pub fn mprot0(&self) -> MPROT0R {
        MPROT0R::_from({
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
        W { bits: 1996488704 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 8:11 - Master 5 Priviledge, Buffer, Read, Write Control."]
    #[inline]
    pub fn mprot5(&mut self) -> _MPROT5W {
        _MPROT5W { w: self }
    }
    #[doc = "Bits 16:19 - Master 3 Priviledge, Buffer, Read, Write Control."]
    #[inline]
    pub fn mprot3(&mut self) -> _MPROT3W {
        _MPROT3W { w: self }
    }
    #[doc = "Bits 20:23 - Master 2 Priviledge, Buffer, Read, Write Control"]
    #[inline]
    pub fn mprot2(&mut self) -> _MPROT2W {
        _MPROT2W { w: self }
    }
    #[doc = "Bits 24:27 - Master 1 Priviledge, Buffer, Read, Write Control"]
    #[inline]
    pub fn mprot1(&mut self) -> _MPROT1W {
        _MPROT1W { w: self }
    }
    #[doc = "Bits 28:31 - Master 0 Priviledge, Buffer, Read, Write Control"]
    #[inline]
    pub fn mprot0(&mut self) -> _MPROT0W {
        _MPROT0W { w: self }
    }
}
