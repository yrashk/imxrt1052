#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CH2STAT {
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
pub struct HASH_MISMATCHR {
    bits: bool,
}
impl HASH_MISMATCHR {
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
pub struct ERROR_SETUPR {
    bits: bool,
}
impl ERROR_SETUPR {
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
pub struct ERROR_PACKETR {
    bits: bool,
}
impl ERROR_PACKETR {
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
pub struct ERROR_SRCR {
    bits: bool,
}
impl ERROR_SRCR {
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
pub struct ERROR_DSTR {
    bits: bool,
}
impl ERROR_DSTR {
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
pub struct ERROR_PAGEFAULTR {
    bits: bool,
}
impl ERROR_PAGEFAULTR {
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
#[doc = "Possible values of the field `ERROR_CODE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ERROR_CODER {
    #[doc = "Error is signalled because the next pointer is 0x00000000."]
    NEXT_CHAIN_IS_0,
    #[doc = "Error is signalled because the semaphore is of a non-zero value and neither of the chain bits is set."]
    NO_CHAIN,
    #[doc = "Error is signalled because an error was reported while reading/writing the context buffer."]
    CONTEXT_ERROR,
    #[doc = "Error is signalled because an error was reported while reading/writing the payload."]
    PAYLOAD_ERROR,
    #[doc = "Error is signalled because the control packet specifies an invalid mode select (for instance, blit + hash)."]
    INVALID_MODE,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl ERROR_CODER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            ERROR_CODER::NEXT_CHAIN_IS_0 => 1,
            ERROR_CODER::NO_CHAIN => 2,
            ERROR_CODER::CONTEXT_ERROR => 3,
            ERROR_CODER::PAYLOAD_ERROR => 4,
            ERROR_CODER::INVALID_MODE => 5,
            ERROR_CODER::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> ERROR_CODER {
        match value {
            1 => ERROR_CODER::NEXT_CHAIN_IS_0,
            2 => ERROR_CODER::NO_CHAIN,
            3 => ERROR_CODER::CONTEXT_ERROR,
            4 => ERROR_CODER::PAYLOAD_ERROR,
            5 => ERROR_CODER::INVALID_MODE,
            i => ERROR_CODER::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `NEXT_CHAIN_IS_0`"]
    #[inline]
    pub fn is_next_chain_is_0(&self) -> bool {
        *self == ERROR_CODER::NEXT_CHAIN_IS_0
    }
    #[doc = "Checks if the value of the field is `NO_CHAIN`"]
    #[inline]
    pub fn is_no_chain(&self) -> bool {
        *self == ERROR_CODER::NO_CHAIN
    }
    #[doc = "Checks if the value of the field is `CONTEXT_ERROR`"]
    #[inline]
    pub fn is_context_error(&self) -> bool {
        *self == ERROR_CODER::CONTEXT_ERROR
    }
    #[doc = "Checks if the value of the field is `PAYLOAD_ERROR`"]
    #[inline]
    pub fn is_payload_error(&self) -> bool {
        *self == ERROR_CODER::PAYLOAD_ERROR
    }
    #[doc = "Checks if the value of the field is `INVALID_MODE`"]
    #[inline]
    pub fn is_invalid_mode(&self) -> bool {
        *self == ERROR_CODER::INVALID_MODE
    }
}
#[doc = r" Value of the field"]
pub struct TAGR {
    bits: u8,
}
impl TAGR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Proxy"]
pub struct _HASH_MISMATCHW<'a> {
    w: &'a mut W,
}
impl<'a> _HASH_MISMATCHW<'a> {
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
pub struct _ERROR_SETUPW<'a> {
    w: &'a mut W,
}
impl<'a> _ERROR_SETUPW<'a> {
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
pub struct _ERROR_PACKETW<'a> {
    w: &'a mut W,
}
impl<'a> _ERROR_PACKETW<'a> {
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
#[doc = r" Proxy"]
pub struct _ERROR_SRCW<'a> {
    w: &'a mut W,
}
impl<'a> _ERROR_SRCW<'a> {
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
pub struct _ERROR_DSTW<'a> {
    w: &'a mut W,
}
impl<'a> _ERROR_DSTW<'a> {
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
#[doc = r" Proxy"]
pub struct _ERROR_PAGEFAULTW<'a> {
    w: &'a mut W,
}
impl<'a> _ERROR_PAGEFAULTW<'a> {
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
#[doc = "Values that can be written to the field `ERROR_CODE`"]
pub enum ERROR_CODEW {
    #[doc = "Error is signalled because the next pointer is 0x00000000."]
    NEXT_CHAIN_IS_0,
    #[doc = "Error is signalled because the semaphore is of a non-zero value and neither of the chain bits is set."]
    NO_CHAIN,
    #[doc = "Error is signalled because an error was reported while reading/writing the context buffer."]
    CONTEXT_ERROR,
    #[doc = "Error is signalled because an error was reported while reading/writing the payload."]
    PAYLOAD_ERROR,
    #[doc = "Error is signalled because the control packet specifies an invalid mode select (for instance, blit + hash)."]
    INVALID_MODE,
}
impl ERROR_CODEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            ERROR_CODEW::NEXT_CHAIN_IS_0 => 1,
            ERROR_CODEW::NO_CHAIN => 2,
            ERROR_CODEW::CONTEXT_ERROR => 3,
            ERROR_CODEW::PAYLOAD_ERROR => 4,
            ERROR_CODEW::INVALID_MODE => 5,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ERROR_CODEW<'a> {
    w: &'a mut W,
}
impl<'a> _ERROR_CODEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ERROR_CODEW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Error is signalled because the next pointer is 0x00000000."]
    #[inline]
    pub fn next_chain_is_0(self) -> &'a mut W {
        self.variant(ERROR_CODEW::NEXT_CHAIN_IS_0)
    }
    #[doc = "Error is signalled because the semaphore is of a non-zero value and neither of the chain bits is set."]
    #[inline]
    pub fn no_chain(self) -> &'a mut W {
        self.variant(ERROR_CODEW::NO_CHAIN)
    }
    #[doc = "Error is signalled because an error was reported while reading/writing the context buffer."]
    #[inline]
    pub fn context_error(self) -> &'a mut W {
        self.variant(ERROR_CODEW::CONTEXT_ERROR)
    }
    #[doc = "Error is signalled because an error was reported while reading/writing the payload."]
    #[inline]
    pub fn payload_error(self) -> &'a mut W {
        self.variant(ERROR_CODEW::PAYLOAD_ERROR)
    }
    #[doc = "Error is signalled because the control packet specifies an invalid mode select (for instance, blit + hash)."]
    #[inline]
    pub fn invalid_mode(self) -> &'a mut W {
        self.variant(ERROR_CODEW::INVALID_MODE)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 255;
        const OFFSET: u8 = 16;
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
    #[doc = "Bit 1 - This bit indicates that a hashing check operation is mismatched for the control packets that enable the HASH_CHECK bit"]
    #[inline]
    pub fn hash_mismatch(&self) -> HASH_MISMATCHR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        HASH_MISMATCHR { bits }
    }
    #[doc = "Bit 2 - This bit indicates that the hardware detected an invalid programming configuration (such as a buffer length that is not a multiple of the natural data size for the operation)"]
    #[inline]
    pub fn error_setup(&self) -> ERROR_SETUPR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        ERROR_SETUPR { bits }
    }
    #[doc = "Bit 3 - This bit indicates that a bus error occurred when reading the packet or payload, or when writing the status back to the packet paylaod"]
    #[inline]
    pub fn error_packet(&self) -> ERROR_PACKETR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        ERROR_PACKETR { bits }
    }
    #[doc = "Bit 4 - This bit indicates that a bus error occurred when reading from the source buffer"]
    #[inline]
    pub fn error_src(&self) -> ERROR_SRCR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        ERROR_SRCR { bits }
    }
    #[doc = "Bit 5 - This bit indicates that a bus error occurred when storing to the destination buffer"]
    #[inline]
    pub fn error_dst(&self) -> ERROR_DSTR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        ERROR_DSTR { bits }
    }
    #[doc = "Bit 6 - This bit indicates that a page fault occurred while converting a virtual address to a physical address"]
    #[inline]
    pub fn error_pagefault(&self) -> ERROR_PAGEFAULTR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        ERROR_PAGEFAULTR { bits }
    }
    #[doc = "Bits 16:23 - Indicates additional error codes for some of the error conditions."]
    #[inline]
    pub fn error_code(&self) -> ERROR_CODER {
        ERROR_CODER::_from({
            const MASK: u8 = 255;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 24:31 - Indicates the tag from the last completed packet in the command structure."]
    #[inline]
    pub fn tag(&self) -> TAGR {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        TAGR { bits }
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
    #[doc = "Bit 1 - This bit indicates that a hashing check operation is mismatched for the control packets that enable the HASH_CHECK bit"]
    #[inline]
    pub fn hash_mismatch(&mut self) -> _HASH_MISMATCHW {
        _HASH_MISMATCHW { w: self }
    }
    #[doc = "Bit 2 - This bit indicates that the hardware detected an invalid programming configuration (such as a buffer length that is not a multiple of the natural data size for the operation)"]
    #[inline]
    pub fn error_setup(&mut self) -> _ERROR_SETUPW {
        _ERROR_SETUPW { w: self }
    }
    #[doc = "Bit 3 - This bit indicates that a bus error occurred when reading the packet or payload, or when writing the status back to the packet paylaod"]
    #[inline]
    pub fn error_packet(&mut self) -> _ERROR_PACKETW {
        _ERROR_PACKETW { w: self }
    }
    #[doc = "Bit 4 - This bit indicates that a bus error occurred when reading from the source buffer"]
    #[inline]
    pub fn error_src(&mut self) -> _ERROR_SRCW {
        _ERROR_SRCW { w: self }
    }
    #[doc = "Bit 5 - This bit indicates that a bus error occurred when storing to the destination buffer"]
    #[inline]
    pub fn error_dst(&mut self) -> _ERROR_DSTW {
        _ERROR_DSTW { w: self }
    }
    #[doc = "Bit 6 - This bit indicates that a page fault occurred while converting a virtual address to a physical address"]
    #[inline]
    pub fn error_pagefault(&mut self) -> _ERROR_PAGEFAULTW {
        _ERROR_PAGEFAULTW { w: self }
    }
    #[doc = "Bits 16:23 - Indicates additional error codes for some of the error conditions."]
    #[inline]
    pub fn error_code(&mut self) -> _ERROR_CODEW {
        _ERROR_CODEW { w: self }
    }
}
