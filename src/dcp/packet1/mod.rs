#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::PACKET1 {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = r" Value of the field"]
pub struct INTERRUPTR {
    bits: bool,
}
impl INTERRUPTR {
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
pub struct DECR_SEMAPHORER {
    bits: bool,
}
impl DECR_SEMAPHORER {
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
pub struct CHAINR {
    bits: bool,
}
impl CHAINR {
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
pub struct CHAIN_CONTIGUOUSR {
    bits: bool,
}
impl CHAIN_CONTIGUOUSR {
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
pub struct ENABLE_MEMCOPYR {
    bits: bool,
}
impl ENABLE_MEMCOPYR {
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
pub struct ENABLE_CIPHERR {
    bits: bool,
}
impl ENABLE_CIPHERR {
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
pub struct ENABLE_HASHR {
    bits: bool,
}
impl ENABLE_HASHR {
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
pub struct ENABLE_BLITR {
    bits: bool,
}
impl ENABLE_BLITR {
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
#[doc = "Possible values of the field `CIPHER_ENCRYPT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CIPHER_ENCRYPTR {
    #[doc = "DECRYPT"]
    DECRYPT,
    #[doc = "ENCRYPT"]
    ENCRYPT,
}
impl CIPHER_ENCRYPTR {
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
            CIPHER_ENCRYPTR::DECRYPT => false,
            CIPHER_ENCRYPTR::ENCRYPT => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CIPHER_ENCRYPTR {
        match value {
            false => CIPHER_ENCRYPTR::DECRYPT,
            true => CIPHER_ENCRYPTR::ENCRYPT,
        }
    }
    #[doc = "Checks if the value of the field is `DECRYPT`"]
    #[inline]
    pub fn is_decrypt(&self) -> bool {
        *self == CIPHER_ENCRYPTR::DECRYPT
    }
    #[doc = "Checks if the value of the field is `ENCRYPT`"]
    #[inline]
    pub fn is_encrypt(&self) -> bool {
        *self == CIPHER_ENCRYPTR::ENCRYPT
    }
}
#[doc = r" Value of the field"]
pub struct CIPHER_INITR {
    bits: bool,
}
impl CIPHER_INITR {
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
pub struct OTP_KEYR {
    bits: bool,
}
impl OTP_KEYR {
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
pub struct PAYLOAD_KEYR {
    bits: bool,
}
impl PAYLOAD_KEYR {
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
pub struct HASH_INITR {
    bits: bool,
}
impl HASH_INITR {
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
pub struct HASH_TERMR {
    bits: bool,
}
impl HASH_TERMR {
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
pub struct CHECK_HASHR {
    bits: bool,
}
impl CHECK_HASHR {
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
#[doc = "Possible values of the field `HASH_OUTPUT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HASH_OUTPUTR {
    #[doc = "INPUT"]
    INPUT,
    #[doc = "OUTPUT"]
    OUTPUT,
}
impl HASH_OUTPUTR {
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
            HASH_OUTPUTR::INPUT => false,
            HASH_OUTPUTR::OUTPUT => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> HASH_OUTPUTR {
        match value {
            false => HASH_OUTPUTR::INPUT,
            true => HASH_OUTPUTR::OUTPUT,
        }
    }
    #[doc = "Checks if the value of the field is `INPUT`"]
    #[inline]
    pub fn is_input(&self) -> bool {
        *self == HASH_OUTPUTR::INPUT
    }
    #[doc = "Checks if the value of the field is `OUTPUT`"]
    #[inline]
    pub fn is_output(&self) -> bool {
        *self == HASH_OUTPUTR::OUTPUT
    }
}
#[doc = r" Value of the field"]
pub struct CONSTANT_FILLR {
    bits: bool,
}
impl CONSTANT_FILLR {
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
pub struct TEST_SEMA_IRQR {
    bits: bool,
}
impl TEST_SEMA_IRQR {
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
pub struct KEY_BYTESWAPR {
    bits: bool,
}
impl KEY_BYTESWAPR {
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
pub struct KEY_WORDSWAPR {
    bits: bool,
}
impl KEY_WORDSWAPR {
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
pub struct INPUT_BYTESWAPR {
    bits: bool,
}
impl INPUT_BYTESWAPR {
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
pub struct INPUT_WORDSWAPR {
    bits: bool,
}
impl INPUT_WORDSWAPR {
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
pub struct OUTPUT_BYTESWAPR {
    bits: bool,
}
impl OUTPUT_BYTESWAPR {
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
pub struct OUTPUT_WORDSWAPR {
    bits: bool,
}
impl OUTPUT_WORDSWAPR {
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
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - Reflects whether the channel must issue an interrupt upon the completion of the packet."]
    #[inline]
    pub fn interrupt(&self) -> INTERRUPTR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        INTERRUPTR { bits }
    }
    #[doc = "Bit 1 - Reflects whether the channel's semaphore must be decremented at the end of the current operation"]
    #[inline]
    pub fn decr_semaphore(&self) -> DECR_SEMAPHORER {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        DECR_SEMAPHORER { bits }
    }
    #[doc = "Bit 2 - Reflects whether the next command pointer register must be loaded into the channel's current descriptor pointer"]
    #[inline]
    pub fn chain(&self) -> CHAINR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        CHAINR { bits }
    }
    #[doc = "Bit 3 - Reflects whether the next packet's address is located following this packet's payload."]
    #[inline]
    pub fn chain_contiguous(&self) -> CHAIN_CONTIGUOUSR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        CHAIN_CONTIGUOUSR { bits }
    }
    #[doc = "Bit 4 - Reflects whether the selected hashing function should be enabled for this operation."]
    #[inline]
    pub fn enable_memcopy(&self) -> ENABLE_MEMCOPYR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        ENABLE_MEMCOPYR { bits }
    }
    #[doc = "Bit 5 - Reflects whether the selected cipher function must be enabled for this operation."]
    #[inline]
    pub fn enable_cipher(&self) -> ENABLE_CIPHERR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        ENABLE_CIPHERR { bits }
    }
    #[doc = "Bit 6 - Reflects whether the selected hashing function must be enabled for this operation."]
    #[inline]
    pub fn enable_hash(&self) -> ENABLE_HASHR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        ENABLE_HASHR { bits }
    }
    #[doc = "Bit 7 - Reflects whether the DCP must perform a blit operation"]
    #[inline]
    pub fn enable_blit(&self) -> ENABLE_BLITR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        ENABLE_BLITR { bits }
    }
    #[doc = "Bit 8 - When the cipher block is enabled, this bit indicates whether the operation is encryption or decryption"]
    #[inline]
    pub fn cipher_encrypt(&self) -> CIPHER_ENCRYPTR {
        CIPHER_ENCRYPTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 9 - Reflects whether the cipher block must load the initialization vector from the payload for this operation"]
    #[inline]
    pub fn cipher_init(&self) -> CIPHER_INITR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        CIPHER_INITR { bits }
    }
    #[doc = "Bit 10 - Reflects whether a hardware-based key must be used"]
    #[inline]
    pub fn otp_key(&self) -> OTP_KEYR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        OTP_KEYR { bits }
    }
    #[doc = "Bit 11 - When set, it indicates the payload contains the key"]
    #[inline]
    pub fn payload_key(&self) -> PAYLOAD_KEYR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        PAYLOAD_KEYR { bits }
    }
    #[doc = "Bit 12 - Reflects whether the current hashing block is the initial block in the hashing operation, so the hash registers must be initialized before the operation"]
    #[inline]
    pub fn hash_init(&self) -> HASH_INITR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        HASH_INITR { bits }
    }
    #[doc = "Bit 13 - Reflects whether the current hashing block is the final block in the hashing operation, so the hash padding must be applied by the hardware"]
    #[inline]
    pub fn hash_term(&self) -> HASH_TERMR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        HASH_TERMR { bits }
    }
    #[doc = "Bit 14 - Reflects whether the calculated hash value must be compared to the hash provided in the payload."]
    #[inline]
    pub fn check_hash(&self) -> CHECK_HASHR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        CHECK_HASHR { bits }
    }
    #[doc = "Bit 15 - When the hashing is enabled, this bit controls whether the input or output data is hashed."]
    #[inline]
    pub fn hash_output(&self) -> HASH_OUTPUTR {
        HASH_OUTPUTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 15;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 16 - When this bit is set (MEMCOPY and BLIT modes only), the DCP simply fills the destination buffer with the value found in the source address field"]
    #[inline]
    pub fn constant_fill(&self) -> CONSTANT_FILLR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        CONSTANT_FILLR { bits }
    }
    #[doc = "Bit 17 - This bit is used to test the channel semaphore transition to 0. FOR TEST USE ONLY!"]
    #[inline]
    pub fn test_sema_irq(&self) -> TEST_SEMA_IRQR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 17;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        TEST_SEMA_IRQR { bits }
    }
    #[doc = "Bit 18 - Reflects whether the DCP engine swaps the key bytes (big-endian key)."]
    #[inline]
    pub fn key_byteswap(&self) -> KEY_BYTESWAPR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        KEY_BYTESWAPR { bits }
    }
    #[doc = "Bit 19 - Reflects whether the DCP engine swaps the key words (big-endian key)."]
    #[inline]
    pub fn key_wordswap(&self) -> KEY_WORDSWAPR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 19;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        KEY_WORDSWAPR { bits }
    }
    #[doc = "Bit 20 - Reflects whether the DCP engine byteswaps the input data (big-endian data)."]
    #[inline]
    pub fn input_byteswap(&self) -> INPUT_BYTESWAPR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        INPUT_BYTESWAPR { bits }
    }
    #[doc = "Bit 21 - Reflects whether the DCP engine wordswaps the input data (big-endian data)."]
    #[inline]
    pub fn input_wordswap(&self) -> INPUT_WORDSWAPR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 21;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        INPUT_WORDSWAPR { bits }
    }
    #[doc = "Bit 22 - Reflects whether the DCP engine byteswaps the output data (big-endian data)."]
    #[inline]
    pub fn output_byteswap(&self) -> OUTPUT_BYTESWAPR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 22;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        OUTPUT_BYTESWAPR { bits }
    }
    #[doc = "Bit 23 - Reflects whether the DCP engine wordswaps the output data (big-endian data)."]
    #[inline]
    pub fn output_wordswap(&self) -> OUTPUT_WORDSWAPR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 23;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        OUTPUT_WORDSWAPR { bits }
    }
    #[doc = "Bits 24:31 - Packet Tag"]
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
