#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CNTR {
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
#[doc = "Possible values of the field `MEGA_PDN_REQ`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MEGA_PDN_REQR {
    #[doc = "No Request"]
    MEGA_PDN_REQ_0,
    #[doc = "Request power down sequence"]
    MEGA_PDN_REQ_1,
}
impl MEGA_PDN_REQR {
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
            MEGA_PDN_REQR::MEGA_PDN_REQ_0 => false,
            MEGA_PDN_REQR::MEGA_PDN_REQ_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> MEGA_PDN_REQR {
        match value {
            false => MEGA_PDN_REQR::MEGA_PDN_REQ_0,
            true => MEGA_PDN_REQR::MEGA_PDN_REQ_1,
        }
    }
    #[doc = "Checks if the value of the field is `MEGA_PDN_REQ_0`"]
    #[inline]
    pub fn is_mega_pdn_req_0(&self) -> bool {
        *self == MEGA_PDN_REQR::MEGA_PDN_REQ_0
    }
    #[doc = "Checks if the value of the field is `MEGA_PDN_REQ_1`"]
    #[inline]
    pub fn is_mega_pdn_req_1(&self) -> bool {
        *self == MEGA_PDN_REQR::MEGA_PDN_REQ_1
    }
}
#[doc = "Possible values of the field `MEGA_PUP_REQ`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MEGA_PUP_REQR {
    #[doc = "No Request"]
    MEGA_PUP_REQ_0,
    #[doc = "Request power up sequence"]
    MEGA_PUP_REQ_1,
}
impl MEGA_PUP_REQR {
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
            MEGA_PUP_REQR::MEGA_PUP_REQ_0 => false,
            MEGA_PUP_REQR::MEGA_PUP_REQ_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> MEGA_PUP_REQR {
        match value {
            false => MEGA_PUP_REQR::MEGA_PUP_REQ_0,
            true => MEGA_PUP_REQR::MEGA_PUP_REQ_1,
        }
    }
    #[doc = "Checks if the value of the field is `MEGA_PUP_REQ_0`"]
    #[inline]
    pub fn is_mega_pup_req_0(&self) -> bool {
        *self == MEGA_PUP_REQR::MEGA_PUP_REQ_0
    }
    #[doc = "Checks if the value of the field is `MEGA_PUP_REQ_1`"]
    #[inline]
    pub fn is_mega_pup_req_1(&self) -> bool {
        *self == MEGA_PUP_REQR::MEGA_PUP_REQ_1
    }
}
#[doc = "Possible values of the field `PDRAM0_PGE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PDRAM0_PGER {
    #[doc = "FlexRAM PDRAM0 domain (bank1-7) will keep power on even if CPU core is power down."]
    PDRAM0_PGE_0,
    #[doc = "FlexRAM PDRAM0 domain (bank1-7) will be power down once when CPU core is power down."]
    PDRAM0_PGE_1,
}
impl PDRAM0_PGER {
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
            PDRAM0_PGER::PDRAM0_PGE_0 => false,
            PDRAM0_PGER::PDRAM0_PGE_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PDRAM0_PGER {
        match value {
            false => PDRAM0_PGER::PDRAM0_PGE_0,
            true => PDRAM0_PGER::PDRAM0_PGE_1,
        }
    }
    #[doc = "Checks if the value of the field is `PDRAM0_PGE_0`"]
    #[inline]
    pub fn is_pdram0_pge_0(&self) -> bool {
        *self == PDRAM0_PGER::PDRAM0_PGE_0
    }
    #[doc = "Checks if the value of the field is `PDRAM0_PGE_1`"]
    #[inline]
    pub fn is_pdram0_pge_1(&self) -> bool {
        *self == PDRAM0_PGER::PDRAM0_PGE_1
    }
}
#[doc = "Values that can be written to the field `MEGA_PDN_REQ`"]
pub enum MEGA_PDN_REQW {
    #[doc = "No Request"]
    MEGA_PDN_REQ_0,
    #[doc = "Request power down sequence"]
    MEGA_PDN_REQ_1,
}
impl MEGA_PDN_REQW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            MEGA_PDN_REQW::MEGA_PDN_REQ_0 => false,
            MEGA_PDN_REQW::MEGA_PDN_REQ_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MEGA_PDN_REQW<'a> {
    w: &'a mut W,
}
impl<'a> _MEGA_PDN_REQW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MEGA_PDN_REQW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No Request"]
    #[inline]
    pub fn mega_pdn_req_0(self) -> &'a mut W {
        self.variant(MEGA_PDN_REQW::MEGA_PDN_REQ_0)
    }
    #[doc = "Request power down sequence"]
    #[inline]
    pub fn mega_pdn_req_1(self) -> &'a mut W {
        self.variant(MEGA_PDN_REQW::MEGA_PDN_REQ_1)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 2;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `MEGA_PUP_REQ`"]
pub enum MEGA_PUP_REQW {
    #[doc = "No Request"]
    MEGA_PUP_REQ_0,
    #[doc = "Request power up sequence"]
    MEGA_PUP_REQ_1,
}
impl MEGA_PUP_REQW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            MEGA_PUP_REQW::MEGA_PUP_REQ_0 => false,
            MEGA_PUP_REQW::MEGA_PUP_REQ_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MEGA_PUP_REQW<'a> {
    w: &'a mut W,
}
impl<'a> _MEGA_PUP_REQW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MEGA_PUP_REQW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No Request"]
    #[inline]
    pub fn mega_pup_req_0(self) -> &'a mut W {
        self.variant(MEGA_PUP_REQW::MEGA_PUP_REQ_0)
    }
    #[doc = "Request power up sequence"]
    #[inline]
    pub fn mega_pup_req_1(self) -> &'a mut W {
        self.variant(MEGA_PUP_REQW::MEGA_PUP_REQ_1)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 3;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `PDRAM0_PGE`"]
pub enum PDRAM0_PGEW {
    #[doc = "FlexRAM PDRAM0 domain (bank1-7) will keep power on even if CPU core is power down."]
    PDRAM0_PGE_0,
    #[doc = "FlexRAM PDRAM0 domain (bank1-7) will be power down once when CPU core is power down."]
    PDRAM0_PGE_1,
}
impl PDRAM0_PGEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PDRAM0_PGEW::PDRAM0_PGE_0 => false,
            PDRAM0_PGEW::PDRAM0_PGE_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PDRAM0_PGEW<'a> {
    w: &'a mut W,
}
impl<'a> _PDRAM0_PGEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PDRAM0_PGEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "FlexRAM PDRAM0 domain (bank1-7) will keep power on even if CPU core is power down."]
    #[inline]
    pub fn pdram0_pge_0(self) -> &'a mut W {
        self.variant(PDRAM0_PGEW::PDRAM0_PGE_0)
    }
    #[doc = "FlexRAM PDRAM0 domain (bank1-7) will be power down once when CPU core is power down."]
    #[inline]
    pub fn pdram0_pge_1(self) -> &'a mut W {
        self.variant(PDRAM0_PGEW::PDRAM0_PGE_1)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 22;
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
    #[doc = "Bit 2 - MEGA domain power down request"]
    #[inline]
    pub fn mega_pdn_req(&self) -> MEGA_PDN_REQR {
        MEGA_PDN_REQR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - MEGA domain power up request"]
    #[inline]
    pub fn mega_pup_req(&self) -> MEGA_PUP_REQR {
        MEGA_PUP_REQR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 22 - FlexRAM PDRAM0 Power Gate Enable"]
    #[inline]
    pub fn pdram0_pge(&self) -> PDRAM0_PGER {
        PDRAM0_PGER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 22;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 5373952 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 2 - MEGA domain power down request"]
    #[inline]
    pub fn mega_pdn_req(&mut self) -> _MEGA_PDN_REQW {
        _MEGA_PDN_REQW { w: self }
    }
    #[doc = "Bit 3 - MEGA domain power up request"]
    #[inline]
    pub fn mega_pup_req(&mut self) -> _MEGA_PUP_REQW {
        _MEGA_PUP_REQW { w: self }
    }
    #[doc = "Bit 22 - FlexRAM PDRAM0 Power Gate Enable"]
    #[inline]
    pub fn pdram0_pge(&mut self) -> _PDRAM0_PGEW {
        _PDRAM0_PGEW { w: self }
    }
}
