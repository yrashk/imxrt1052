#[doc = r" Value read from the register"]
pub struct R {
    bits: u16,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u16,
}
impl super::SM2DMAEN {
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
pub struct CX0DER {
    bits: bool,
}
impl CX0DER {
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
pub struct CX1DER {
    bits: bool,
}
impl CX1DER {
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
pub struct CB0DER {
    bits: bool,
}
impl CB0DER {
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
pub struct CB1DER {
    bits: bool,
}
impl CB1DER {
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
pub struct CA0DER {
    bits: bool,
}
impl CA0DER {
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
pub struct CA1DER {
    bits: bool,
}
impl CA1DER {
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
#[doc = "Possible values of the field `CAPTDE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CAPTDER {
    #[doc = "Read DMA requests disabled."]
    CAPTDE_0,
    #[doc = "Exceeding a FIFO watermark sets the DMA read request. This requires at least one of DMAEN[CA1DE], DMAEN[CA0DE], DMAEN[CB1DE], DMAEN[CB0DE], DMAEN[CX1DE], or DMAEN[CX0DE] to also be set in order to determine to which watermark(s) the DMA request is sensitive."]
    CAPTDE_1,
    #[doc = "A local sync (VAL1 matches counter) sets the read DMA request."]
    CAPTDE_2,
    #[doc = "A local reload (STS[RF] being set) sets the read DMA request."]
    CAPTDE_3,
}
impl CAPTDER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            CAPTDER::CAPTDE_0 => 0,
            CAPTDER::CAPTDE_1 => 1,
            CAPTDER::CAPTDE_2 => 2,
            CAPTDER::CAPTDE_3 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> CAPTDER {
        match value {
            0 => CAPTDER::CAPTDE_0,
            1 => CAPTDER::CAPTDE_1,
            2 => CAPTDER::CAPTDE_2,
            3 => CAPTDER::CAPTDE_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `CAPTDE_0`"]
    #[inline]
    pub fn is_captde_0(&self) -> bool {
        *self == CAPTDER::CAPTDE_0
    }
    #[doc = "Checks if the value of the field is `CAPTDE_1`"]
    #[inline]
    pub fn is_captde_1(&self) -> bool {
        *self == CAPTDER::CAPTDE_1
    }
    #[doc = "Checks if the value of the field is `CAPTDE_2`"]
    #[inline]
    pub fn is_captde_2(&self) -> bool {
        *self == CAPTDER::CAPTDE_2
    }
    #[doc = "Checks if the value of the field is `CAPTDE_3`"]
    #[inline]
    pub fn is_captde_3(&self) -> bool {
        *self == CAPTDER::CAPTDE_3
    }
}
#[doc = "Possible values of the field `FAND`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FANDR {
    #[doc = "Selected FIFO watermarks are OR'ed together."]
    FAND_0,
    #[doc = "Selected FIFO watermarks are AND'ed together."]
    FAND_1,
}
impl FANDR {
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
            FANDR::FAND_0 => false,
            FANDR::FAND_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> FANDR {
        match value {
            false => FANDR::FAND_0,
            true => FANDR::FAND_1,
        }
    }
    #[doc = "Checks if the value of the field is `FAND_0`"]
    #[inline]
    pub fn is_fand_0(&self) -> bool {
        *self == FANDR::FAND_0
    }
    #[doc = "Checks if the value of the field is `FAND_1`"]
    #[inline]
    pub fn is_fand_1(&self) -> bool {
        *self == FANDR::FAND_1
    }
}
#[doc = "Possible values of the field `VALDE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VALDER {
    #[doc = "DMA write requests disabled"]
    VALDE_0,
    #[doc = "no description available"]
    VALDE_1,
}
impl VALDER {
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
            VALDER::VALDE_0 => false,
            VALDER::VALDE_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> VALDER {
        match value {
            false => VALDER::VALDE_0,
            true => VALDER::VALDE_1,
        }
    }
    #[doc = "Checks if the value of the field is `VALDE_0`"]
    #[inline]
    pub fn is_valde_0(&self) -> bool {
        *self == VALDER::VALDE_0
    }
    #[doc = "Checks if the value of the field is `VALDE_1`"]
    #[inline]
    pub fn is_valde_1(&self) -> bool {
        *self == VALDER::VALDE_1
    }
}
#[doc = r" Proxy"]
pub struct _CX0DEW<'a> {
    w: &'a mut W,
}
impl<'a> _CX0DEW<'a> {
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
        self.w.bits &= !((MASK as u16) << OFFSET);
        self.w.bits |= ((value & MASK) as u16) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _CX1DEW<'a> {
    w: &'a mut W,
}
impl<'a> _CX1DEW<'a> {
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
        self.w.bits &= !((MASK as u16) << OFFSET);
        self.w.bits |= ((value & MASK) as u16) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _CB0DEW<'a> {
    w: &'a mut W,
}
impl<'a> _CB0DEW<'a> {
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
        self.w.bits &= !((MASK as u16) << OFFSET);
        self.w.bits |= ((value & MASK) as u16) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _CB1DEW<'a> {
    w: &'a mut W,
}
impl<'a> _CB1DEW<'a> {
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
        self.w.bits &= !((MASK as u16) << OFFSET);
        self.w.bits |= ((value & MASK) as u16) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _CA0DEW<'a> {
    w: &'a mut W,
}
impl<'a> _CA0DEW<'a> {
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
        self.w.bits &= !((MASK as u16) << OFFSET);
        self.w.bits |= ((value & MASK) as u16) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _CA1DEW<'a> {
    w: &'a mut W,
}
impl<'a> _CA1DEW<'a> {
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
        self.w.bits &= !((MASK as u16) << OFFSET);
        self.w.bits |= ((value & MASK) as u16) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `CAPTDE`"]
pub enum CAPTDEW {
    #[doc = "Read DMA requests disabled."]
    CAPTDE_0,
    #[doc = "Exceeding a FIFO watermark sets the DMA read request. This requires at least one of DMAEN[CA1DE], DMAEN[CA0DE], DMAEN[CB1DE], DMAEN[CB0DE], DMAEN[CX1DE], or DMAEN[CX0DE] to also be set in order to determine to which watermark(s) the DMA request is sensitive."]
    CAPTDE_1,
    #[doc = "A local sync (VAL1 matches counter) sets the read DMA request."]
    CAPTDE_2,
    #[doc = "A local reload (STS[RF] being set) sets the read DMA request."]
    CAPTDE_3,
}
impl CAPTDEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            CAPTDEW::CAPTDE_0 => 0,
            CAPTDEW::CAPTDE_1 => 1,
            CAPTDEW::CAPTDE_2 => 2,
            CAPTDEW::CAPTDE_3 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CAPTDEW<'a> {
    w: &'a mut W,
}
impl<'a> _CAPTDEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CAPTDEW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Read DMA requests disabled."]
    #[inline]
    pub fn captde_0(self) -> &'a mut W {
        self.variant(CAPTDEW::CAPTDE_0)
    }
    #[doc = "Exceeding a FIFO watermark sets the DMA read request. This requires at least one of DMAEN[CA1DE], DMAEN[CA0DE], DMAEN[CB1DE], DMAEN[CB0DE], DMAEN[CX1DE], or DMAEN[CX0DE] to also be set in order to determine to which watermark(s) the DMA request is sensitive."]
    #[inline]
    pub fn captde_1(self) -> &'a mut W {
        self.variant(CAPTDEW::CAPTDE_1)
    }
    #[doc = "A local sync (VAL1 matches counter) sets the read DMA request."]
    #[inline]
    pub fn captde_2(self) -> &'a mut W {
        self.variant(CAPTDEW::CAPTDE_2)
    }
    #[doc = "A local reload (STS[RF] being set) sets the read DMA request."]
    #[inline]
    pub fn captde_3(self) -> &'a mut W {
        self.variant(CAPTDEW::CAPTDE_3)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 6;
        self.w.bits &= !((MASK as u16) << OFFSET);
        self.w.bits |= ((value & MASK) as u16) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `FAND`"]
pub enum FANDW {
    #[doc = "Selected FIFO watermarks are OR'ed together."]
    FAND_0,
    #[doc = "Selected FIFO watermarks are AND'ed together."]
    FAND_1,
}
impl FANDW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            FANDW::FAND_0 => false,
            FANDW::FAND_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FANDW<'a> {
    w: &'a mut W,
}
impl<'a> _FANDW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FANDW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Selected FIFO watermarks are OR'ed together."]
    #[inline]
    pub fn fand_0(self) -> &'a mut W {
        self.variant(FANDW::FAND_0)
    }
    #[doc = "Selected FIFO watermarks are AND'ed together."]
    #[inline]
    pub fn fand_1(self) -> &'a mut W {
        self.variant(FANDW::FAND_1)
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
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u16) << OFFSET);
        self.w.bits |= ((value & MASK) as u16) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `VALDE`"]
pub enum VALDEW {
    #[doc = "DMA write requests disabled"]
    VALDE_0,
    #[doc = "no description available"]
    VALDE_1,
}
impl VALDEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            VALDEW::VALDE_0 => false,
            VALDEW::VALDE_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _VALDEW<'a> {
    w: &'a mut W,
}
impl<'a> _VALDEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: VALDEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "DMA write requests disabled"]
    #[inline]
    pub fn valde_0(self) -> &'a mut W {
        self.variant(VALDEW::VALDE_0)
    }
    #[doc = "no description available"]
    #[inline]
    pub fn valde_1(self) -> &'a mut W {
        self.variant(VALDEW::VALDE_1)
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
        const OFFSET: u8 = 9;
        self.w.bits &= !((MASK as u16) << OFFSET);
        self.w.bits |= ((value & MASK) as u16) << OFFSET;
        self.w
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }
    #[doc = "Bit 0 - Capture X0 FIFO DMA Enable"]
    #[inline]
    pub fn cx0de(&self) -> CX0DER {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        };
        CX0DER { bits }
    }
    #[doc = "Bit 1 - Capture X1 FIFO DMA Enable"]
    #[inline]
    pub fn cx1de(&self) -> CX1DER {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        };
        CX1DER { bits }
    }
    #[doc = "Bit 2 - Capture B0 FIFO DMA Enable"]
    #[inline]
    pub fn cb0de(&self) -> CB0DER {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        };
        CB0DER { bits }
    }
    #[doc = "Bit 3 - Capture B1 FIFO DMA Enable"]
    #[inline]
    pub fn cb1de(&self) -> CB1DER {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        };
        CB1DER { bits }
    }
    #[doc = "Bit 4 - Capture A0 FIFO DMA Enable"]
    #[inline]
    pub fn ca0de(&self) -> CA0DER {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        };
        CA0DER { bits }
    }
    #[doc = "Bit 5 - Capture A1 FIFO DMA Enable"]
    #[inline]
    pub fn ca1de(&self) -> CA1DER {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        };
        CA1DER { bits }
    }
    #[doc = "Bits 6:7 - Capture DMA Enable Source Select"]
    #[inline]
    pub fn captde(&self) -> CAPTDER {
        CAPTDER::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u16) as u8
        })
    }
    #[doc = "Bit 8 - FIFO Watermark AND Control"]
    #[inline]
    pub fn fand(&self) -> FANDR {
        FANDR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        })
    }
    #[doc = "Bit 9 - Value Registers DMA Enable"]
    #[inline]
    pub fn valde(&self) -> VALDER {
        VALDER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u16) != 0
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
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - Capture X0 FIFO DMA Enable"]
    #[inline]
    pub fn cx0de(&mut self) -> _CX0DEW {
        _CX0DEW { w: self }
    }
    #[doc = "Bit 1 - Capture X1 FIFO DMA Enable"]
    #[inline]
    pub fn cx1de(&mut self) -> _CX1DEW {
        _CX1DEW { w: self }
    }
    #[doc = "Bit 2 - Capture B0 FIFO DMA Enable"]
    #[inline]
    pub fn cb0de(&mut self) -> _CB0DEW {
        _CB0DEW { w: self }
    }
    #[doc = "Bit 3 - Capture B1 FIFO DMA Enable"]
    #[inline]
    pub fn cb1de(&mut self) -> _CB1DEW {
        _CB1DEW { w: self }
    }
    #[doc = "Bit 4 - Capture A0 FIFO DMA Enable"]
    #[inline]
    pub fn ca0de(&mut self) -> _CA0DEW {
        _CA0DEW { w: self }
    }
    #[doc = "Bit 5 - Capture A1 FIFO DMA Enable"]
    #[inline]
    pub fn ca1de(&mut self) -> _CA1DEW {
        _CA1DEW { w: self }
    }
    #[doc = "Bits 6:7 - Capture DMA Enable Source Select"]
    #[inline]
    pub fn captde(&mut self) -> _CAPTDEW {
        _CAPTDEW { w: self }
    }
    #[doc = "Bit 8 - FIFO Watermark AND Control"]
    #[inline]
    pub fn fand(&mut self) -> _FANDW {
        _FANDW { w: self }
    }
    #[doc = "Bit 9 - Value Registers DMA Enable"]
    #[inline]
    pub fn valde(&mut self) -> _VALDEW {
        _VALDEW { w: self }
    }
}
