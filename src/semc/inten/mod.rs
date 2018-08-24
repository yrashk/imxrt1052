#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::INTEN {
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
#[doc = r" Value of the field"]
pub struct IPCMDDONEENR {
    bits: bool,
}
impl IPCMDDONEENR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
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
}
#[doc = r" Value of the field"]
pub struct IPCMDERRENR {
    bits: bool,
}
impl IPCMDERRENR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
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
}
#[doc = r" Value of the field"]
pub struct AXICMDERRENR {
    bits: bool,
}
impl AXICMDERRENR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
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
}
#[doc = r" Value of the field"]
pub struct AXIBUSERRENR {
    bits: bool,
}
impl AXIBUSERRENR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
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
}
#[doc = "Possible values of the field `NDPAGEENDEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NDPAGEENDENR {
    #[doc = "Disable"]
    NDPAGEENDEN_0,
    #[doc = "Enable"]
    NDPAGEENDEN_1,
}
impl NDPAGEENDENR {
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
            NDPAGEENDENR::NDPAGEENDEN_0 => false,
            NDPAGEENDENR::NDPAGEENDEN_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> NDPAGEENDENR {
        match value {
            false => NDPAGEENDENR::NDPAGEENDEN_0,
            true => NDPAGEENDENR::NDPAGEENDEN_1,
        }
    }
    #[doc = "Checks if the value of the field is `NDPAGEENDEN_0`"]
    #[inline]
    pub fn is_ndpageenden_0(&self) -> bool {
        *self == NDPAGEENDENR::NDPAGEENDEN_0
    }
    #[doc = "Checks if the value of the field is `NDPAGEENDEN_1`"]
    #[inline]
    pub fn is_ndpageenden_1(&self) -> bool {
        *self == NDPAGEENDENR::NDPAGEENDEN_1
    }
}
#[doc = "Possible values of the field `NDNOPENDEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NDNOPENDENR {
    #[doc = "Disable"]
    NDNOPENDEN_0,
    #[doc = "Enable"]
    NDNOPENDEN_1,
}
impl NDNOPENDENR {
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
            NDNOPENDENR::NDNOPENDEN_0 => false,
            NDNOPENDENR::NDNOPENDEN_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> NDNOPENDENR {
        match value {
            false => NDNOPENDENR::NDNOPENDEN_0,
            true => NDNOPENDENR::NDNOPENDEN_1,
        }
    }
    #[doc = "Checks if the value of the field is `NDNOPENDEN_0`"]
    #[inline]
    pub fn is_ndnopenden_0(&self) -> bool {
        *self == NDNOPENDENR::NDNOPENDEN_0
    }
    #[doc = "Checks if the value of the field is `NDNOPENDEN_1`"]
    #[inline]
    pub fn is_ndnopenden_1(&self) -> bool {
        *self == NDNOPENDENR::NDNOPENDEN_1
    }
}
#[doc = r" Proxy"]
pub struct _IPCMDDONEENW<'a> {
    w: &'a mut W,
}
impl<'a> _IPCMDDONEENW<'a> {
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
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _IPCMDERRENW<'a> {
    w: &'a mut W,
}
impl<'a> _IPCMDERRENW<'a> {
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
        const OFFSET: u8 = 1;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _AXICMDERRENW<'a> {
    w: &'a mut W,
}
impl<'a> _AXICMDERRENW<'a> {
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
#[doc = r" Proxy"]
pub struct _AXIBUSERRENW<'a> {
    w: &'a mut W,
}
impl<'a> _AXIBUSERRENW<'a> {
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
#[doc = "Values that can be written to the field `NDPAGEENDEN`"]
pub enum NDPAGEENDENW {
    #[doc = "Disable"]
    NDPAGEENDEN_0,
    #[doc = "Enable"]
    NDPAGEENDEN_1,
}
impl NDPAGEENDENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            NDPAGEENDENW::NDPAGEENDEN_0 => false,
            NDPAGEENDENW::NDPAGEENDEN_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _NDPAGEENDENW<'a> {
    w: &'a mut W,
}
impl<'a> _NDPAGEENDENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: NDPAGEENDENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable"]
    #[inline]
    pub fn ndpageenden_0(self) -> &'a mut W {
        self.variant(NDPAGEENDENW::NDPAGEENDEN_0)
    }
    #[doc = "Enable"]
    #[inline]
    pub fn ndpageenden_1(self) -> &'a mut W {
        self.variant(NDPAGEENDENW::NDPAGEENDEN_1)
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
        const OFFSET: u8 = 4;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `NDNOPENDEN`"]
pub enum NDNOPENDENW {
    #[doc = "Disable"]
    NDNOPENDEN_0,
    #[doc = "Enable"]
    NDNOPENDEN_1,
}
impl NDNOPENDENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            NDNOPENDENW::NDNOPENDEN_0 => false,
            NDNOPENDENW::NDNOPENDEN_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _NDNOPENDENW<'a> {
    w: &'a mut W,
}
impl<'a> _NDNOPENDENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: NDNOPENDENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable"]
    #[inline]
    pub fn ndnopenden_0(self) -> &'a mut W {
        self.variant(NDNOPENDENW::NDNOPENDEN_0)
    }
    #[doc = "Enable"]
    #[inline]
    pub fn ndnopenden_1(self) -> &'a mut W {
        self.variant(NDNOPENDENW::NDNOPENDEN_1)
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
        const OFFSET: u8 = 5;
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
    #[doc = "Bit 0 - IP command done interrupt enable"]
    #[inline]
    pub fn ipcmddoneen(&self) -> IPCMDDONEENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        IPCMDDONEENR { bits }
    }
    #[doc = "Bit 1 - IP command error interrupt enable"]
    #[inline]
    pub fn ipcmderren(&self) -> IPCMDERRENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        IPCMDERRENR { bits }
    }
    #[doc = "Bit 2 - AXI command error interrupt enable"]
    #[inline]
    pub fn axicmderren(&self) -> AXICMDERRENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        AXICMDERRENR { bits }
    }
    #[doc = "Bit 3 - AXI bus error interrupt enable"]
    #[inline]
    pub fn axibuserren(&self) -> AXIBUSERRENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        AXIBUSERRENR { bits }
    }
    #[doc = "Bit 4 - This bit enable/disable the NDPAGEEND interrupt generation."]
    #[inline]
    pub fn ndpageenden(&self) -> NDPAGEENDENR {
        NDPAGEENDENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 5 - This bit enable/disable the NDNOPEND interrupt generation."]
    #[inline]
    pub fn ndnopenden(&self) -> NDNOPENDENR {
        NDNOPENDENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
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
    #[doc = "Bit 0 - IP command done interrupt enable"]
    #[inline]
    pub fn ipcmddoneen(&mut self) -> _IPCMDDONEENW {
        _IPCMDDONEENW { w: self }
    }
    #[doc = "Bit 1 - IP command error interrupt enable"]
    #[inline]
    pub fn ipcmderren(&mut self) -> _IPCMDERRENW {
        _IPCMDERRENW { w: self }
    }
    #[doc = "Bit 2 - AXI command error interrupt enable"]
    #[inline]
    pub fn axicmderren(&mut self) -> _AXICMDERRENW {
        _AXICMDERRENW { w: self }
    }
    #[doc = "Bit 3 - AXI bus error interrupt enable"]
    #[inline]
    pub fn axibuserren(&mut self) -> _AXIBUSERRENW {
        _AXIBUSERRENW { w: self }
    }
    #[doc = "Bit 4 - This bit enable/disable the NDPAGEEND interrupt generation."]
    #[inline]
    pub fn ndpageenden(&mut self) -> _NDPAGEENDENW {
        _NDPAGEENDENW { w: self }
    }
    #[doc = "Bit 5 - This bit enable/disable the NDNOPEND interrupt generation."]
    #[inline]
    pub fn ndnopenden(&mut self) -> _NDNOPENDENW {
        _NDNOPENDENW { w: self }
    }
}
