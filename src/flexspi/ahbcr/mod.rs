#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::AHBCR {
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
#[doc = "Possible values of the field `APAREN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum APARENR {
    #[doc = "Flash will be accessed in Individual mode."]
    APAREN_0,
    #[doc = "Flash will be accessed in Parallel mode."]
    APAREN_1,
}
impl APARENR {
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
            APARENR::APAREN_0 => false,
            APARENR::APAREN_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> APARENR {
        match value {
            false => APARENR::APAREN_0,
            true => APARENR::APAREN_1,
        }
    }
    #[doc = "Checks if the value of the field is `APAREN_0`"]
    #[inline]
    pub fn is_aparen_0(&self) -> bool {
        *self == APARENR::APAREN_0
    }
    #[doc = "Checks if the value of the field is `APAREN_1`"]
    #[inline]
    pub fn is_aparen_1(&self) -> bool {
        *self == APARENR::APAREN_1
    }
}
#[doc = "Possible values of the field `CACHABLEEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CACHABLEENR {
    #[doc = "Disabled. When there is AHB bus cachable read access, FlexSPI will not check whether it hit AHB TX Buffer."]
    CACHABLEEN_0,
    #[doc = "Enabled. When there is AHB bus cachable read access, FlexSPI will check whether it hit AHB TX Buffer first."]
    CACHABLEEN_1,
}
impl CACHABLEENR {
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
            CACHABLEENR::CACHABLEEN_0 => false,
            CACHABLEENR::CACHABLEEN_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CACHABLEENR {
        match value {
            false => CACHABLEENR::CACHABLEEN_0,
            true => CACHABLEENR::CACHABLEEN_1,
        }
    }
    #[doc = "Checks if the value of the field is `CACHABLEEN_0`"]
    #[inline]
    pub fn is_cachableen_0(&self) -> bool {
        *self == CACHABLEENR::CACHABLEEN_0
    }
    #[doc = "Checks if the value of the field is `CACHABLEEN_1`"]
    #[inline]
    pub fn is_cachableen_1(&self) -> bool {
        *self == CACHABLEENR::CACHABLEEN_1
    }
}
#[doc = "Possible values of the field `BUFFERABLEEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BUFFERABLEENR {
    #[doc = "Disabled. For all AHB write access (no matter bufferable or non-bufferable ), FlexSPI will return AHB Bus ready after all data is transmitted to External device and AHB command finished."]
    BUFFERABLEEN_0,
    #[doc = "Enabled. For AHB bufferable write access, FlexSPI will return AHB Bus ready when the AHB command is granted by arbitrator and will not wait for AHB command finished."]
    BUFFERABLEEN_1,
}
impl BUFFERABLEENR {
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
            BUFFERABLEENR::BUFFERABLEEN_0 => false,
            BUFFERABLEENR::BUFFERABLEEN_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> BUFFERABLEENR {
        match value {
            false => BUFFERABLEENR::BUFFERABLEEN_0,
            true => BUFFERABLEENR::BUFFERABLEEN_1,
        }
    }
    #[doc = "Checks if the value of the field is `BUFFERABLEEN_0`"]
    #[inline]
    pub fn is_bufferableen_0(&self) -> bool {
        *self == BUFFERABLEENR::BUFFERABLEEN_0
    }
    #[doc = "Checks if the value of the field is `BUFFERABLEEN_1`"]
    #[inline]
    pub fn is_bufferableen_1(&self) -> bool {
        *self == BUFFERABLEENR::BUFFERABLEEN_1
    }
}
#[doc = r" Value of the field"]
pub struct PREFETCHENR {
    bits: bool,
}
impl PREFETCHENR {
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
#[doc = "Possible values of the field `READADDROPT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum READADDROPTR {
    #[doc = "There is AHB read burst start address alignment limitation when flash is accessed in parallel mode or flash is wordaddressable."]
    READADDROPT_0,
    #[doc = "There is no AHB read burst start address alignment limitation. FlexSPI will fetch more datas than AHB burst required to meet the alignment requirement."]
    READADDROPT_1,
}
impl READADDROPTR {
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
            READADDROPTR::READADDROPT_0 => false,
            READADDROPTR::READADDROPT_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> READADDROPTR {
        match value {
            false => READADDROPTR::READADDROPT_0,
            true => READADDROPTR::READADDROPT_1,
        }
    }
    #[doc = "Checks if the value of the field is `READADDROPT_0`"]
    #[inline]
    pub fn is_readaddropt_0(&self) -> bool {
        *self == READADDROPTR::READADDROPT_0
    }
    #[doc = "Checks if the value of the field is `READADDROPT_1`"]
    #[inline]
    pub fn is_readaddropt_1(&self) -> bool {
        *self == READADDROPTR::READADDROPT_1
    }
}
#[doc = "Values that can be written to the field `APAREN`"]
pub enum APARENW {
    #[doc = "Flash will be accessed in Individual mode."]
    APAREN_0,
    #[doc = "Flash will be accessed in Parallel mode."]
    APAREN_1,
}
impl APARENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            APARENW::APAREN_0 => false,
            APARENW::APAREN_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _APARENW<'a> {
    w: &'a mut W,
}
impl<'a> _APARENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: APARENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Flash will be accessed in Individual mode."]
    #[inline]
    pub fn aparen_0(self) -> &'a mut W {
        self.variant(APARENW::APAREN_0)
    }
    #[doc = "Flash will be accessed in Parallel mode."]
    #[inline]
    pub fn aparen_1(self) -> &'a mut W {
        self.variant(APARENW::APAREN_1)
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
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `CACHABLEEN`"]
pub enum CACHABLEENW {
    #[doc = "Disabled. When there is AHB bus cachable read access, FlexSPI will not check whether it hit AHB TX Buffer."]
    CACHABLEEN_0,
    #[doc = "Enabled. When there is AHB bus cachable read access, FlexSPI will check whether it hit AHB TX Buffer first."]
    CACHABLEEN_1,
}
impl CACHABLEENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CACHABLEENW::CACHABLEEN_0 => false,
            CACHABLEENW::CACHABLEEN_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CACHABLEENW<'a> {
    w: &'a mut W,
}
impl<'a> _CACHABLEENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CACHABLEENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disabled. When there is AHB bus cachable read access, FlexSPI will not check whether it hit AHB TX Buffer."]
    #[inline]
    pub fn cachableen_0(self) -> &'a mut W {
        self.variant(CACHABLEENW::CACHABLEEN_0)
    }
    #[doc = "Enabled. When there is AHB bus cachable read access, FlexSPI will check whether it hit AHB TX Buffer first."]
    #[inline]
    pub fn cachableen_1(self) -> &'a mut W {
        self.variant(CACHABLEENW::CACHABLEEN_1)
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
#[doc = "Values that can be written to the field `BUFFERABLEEN`"]
pub enum BUFFERABLEENW {
    #[doc = "Disabled. For all AHB write access (no matter bufferable or non-bufferable ), FlexSPI will return AHB Bus ready after all data is transmitted to External device and AHB command finished."]
    BUFFERABLEEN_0,
    #[doc = "Enabled. For AHB bufferable write access, FlexSPI will return AHB Bus ready when the AHB command is granted by arbitrator and will not wait for AHB command finished."]
    BUFFERABLEEN_1,
}
impl BUFFERABLEENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            BUFFERABLEENW::BUFFERABLEEN_0 => false,
            BUFFERABLEENW::BUFFERABLEEN_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _BUFFERABLEENW<'a> {
    w: &'a mut W,
}
impl<'a> _BUFFERABLEENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: BUFFERABLEENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disabled. For all AHB write access (no matter bufferable or non-bufferable ), FlexSPI will return AHB Bus ready after all data is transmitted to External device and AHB command finished."]
    #[inline]
    pub fn bufferableen_0(self) -> &'a mut W {
        self.variant(BUFFERABLEENW::BUFFERABLEEN_0)
    }
    #[doc = "Enabled. For AHB bufferable write access, FlexSPI will return AHB Bus ready when the AHB command is granted by arbitrator and will not wait for AHB command finished."]
    #[inline]
    pub fn bufferableen_1(self) -> &'a mut W {
        self.variant(BUFFERABLEENW::BUFFERABLEEN_1)
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
#[doc = r" Proxy"]
pub struct _PREFETCHENW<'a> {
    w: &'a mut W,
}
impl<'a> _PREFETCHENW<'a> {
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
#[doc = "Values that can be written to the field `READADDROPT`"]
pub enum READADDROPTW {
    #[doc = "There is AHB read burst start address alignment limitation when flash is accessed in parallel mode or flash is wordaddressable."]
    READADDROPT_0,
    #[doc = "There is no AHB read burst start address alignment limitation. FlexSPI will fetch more datas than AHB burst required to meet the alignment requirement."]
    READADDROPT_1,
}
impl READADDROPTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            READADDROPTW::READADDROPT_0 => false,
            READADDROPTW::READADDROPT_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _READADDROPTW<'a> {
    w: &'a mut W,
}
impl<'a> _READADDROPTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: READADDROPTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "There is AHB read burst start address alignment limitation when flash is accessed in parallel mode or flash is wordaddressable."]
    #[inline]
    pub fn readaddropt_0(self) -> &'a mut W {
        self.variant(READADDROPTW::READADDROPT_0)
    }
    #[doc = "There is no AHB read burst start address alignment limitation. FlexSPI will fetch more datas than AHB burst required to meet the alignment requirement."]
    #[inline]
    pub fn readaddropt_1(self) -> &'a mut W {
        self.variant(READADDROPTW::READADDROPT_1)
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
        const OFFSET: u8 = 6;
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
    #[doc = "Bit 0 - Parallel mode enabled for AHB triggered Command (both read and write) ."]
    #[inline]
    pub fn aparen(&self) -> APARENR {
        APARENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - Enable AHB bus cachable read access support."]
    #[inline]
    pub fn cachableen(&self) -> CACHABLEENR {
        CACHABLEENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 4 - Enable AHB bus bufferable write access support. This field affects the last beat of AHB write access, refer for more details about AHB bufferable write."]
    #[inline]
    pub fn bufferableen(&self) -> BUFFERABLEENR {
        BUFFERABLEENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 5 - AHB Read Prefetch Enable."]
    #[inline]
    pub fn prefetchen(&self) -> PREFETCHENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        PREFETCHENR { bits }
    }
    #[doc = "Bit 6 - AHB Read Address option bit. This option bit is intend to remove AHB burst start address alignment limitation."]
    #[inline]
    pub fn readaddropt(&self) -> READADDROPTR {
        READADDROPTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 24 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - Parallel mode enabled for AHB triggered Command (both read and write) ."]
    #[inline]
    pub fn aparen(&mut self) -> _APARENW {
        _APARENW { w: self }
    }
    #[doc = "Bit 3 - Enable AHB bus cachable read access support."]
    #[inline]
    pub fn cachableen(&mut self) -> _CACHABLEENW {
        _CACHABLEENW { w: self }
    }
    #[doc = "Bit 4 - Enable AHB bus bufferable write access support. This field affects the last beat of AHB write access, refer for more details about AHB bufferable write."]
    #[inline]
    pub fn bufferableen(&mut self) -> _BUFFERABLEENW {
        _BUFFERABLEENW { w: self }
    }
    #[doc = "Bit 5 - AHB Read Prefetch Enable."]
    #[inline]
    pub fn prefetchen(&mut self) -> _PREFETCHENW {
        _PREFETCHENW { w: self }
    }
    #[doc = "Bit 6 - AHB Read Address option bit. This option bit is intend to remove AHB burst start address alignment limitation."]
    #[inline]
    pub fn readaddropt(&mut self) -> _READADDROPTW {
        _READADDROPTW { w: self }
    }
}
