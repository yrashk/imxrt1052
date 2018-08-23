#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::CAPABILITY1 {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = "Possible values of the field `CIPHER_ALGORITHMS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CIPHER_ALGORITHMSR {
    #[doc = "AES128"]
    AES128,
    #[doc = r" Reserved"]
    _Reserved(u16),
}
impl CIPHER_ALGORITHMSR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        match *self {
            CIPHER_ALGORITHMSR::AES128 => 1,
            CIPHER_ALGORITHMSR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u16) -> CIPHER_ALGORITHMSR {
        match value {
            1 => CIPHER_ALGORITHMSR::AES128,
            i => CIPHER_ALGORITHMSR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `AES128`"]
    #[inline]
    pub fn is_aes128(&self) -> bool {
        *self == CIPHER_ALGORITHMSR::AES128
    }
}
#[doc = "Possible values of the field `HASH_ALGORITHMS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HASH_ALGORITHMSR {
    #[doc = "SHA1"]
    SHA1,
    #[doc = "CRC32"]
    CRC32,
    #[doc = "SHA256"]
    SHA256,
    #[doc = r" Reserved"]
    _Reserved(u16),
}
impl HASH_ALGORITHMSR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        match *self {
            HASH_ALGORITHMSR::SHA1 => 1,
            HASH_ALGORITHMSR::CRC32 => 2,
            HASH_ALGORITHMSR::SHA256 => 4,
            HASH_ALGORITHMSR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u16) -> HASH_ALGORITHMSR {
        match value {
            1 => HASH_ALGORITHMSR::SHA1,
            2 => HASH_ALGORITHMSR::CRC32,
            4 => HASH_ALGORITHMSR::SHA256,
            i => HASH_ALGORITHMSR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `SHA1`"]
    #[inline]
    pub fn is_sha1(&self) -> bool {
        *self == HASH_ALGORITHMSR::SHA1
    }
    #[doc = "Checks if the value of the field is `CRC32`"]
    #[inline]
    pub fn is_crc32(&self) -> bool {
        *self == HASH_ALGORITHMSR::CRC32
    }
    #[doc = "Checks if the value of the field is `SHA256`"]
    #[inline]
    pub fn is_sha256(&self) -> bool {
        *self == HASH_ALGORITHMSR::SHA256
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:15 - One-hot field indicating which cipher algorithms are available"]
    #[inline]
    pub fn cipher_algorithms(&self) -> CIPHER_ALGORITHMSR {
        CIPHER_ALGORITHMSR::_from({
            const MASK: u16 = 65535;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        })
    }
    #[doc = "Bits 16:31 - One-hot field indicating which hashing features are implemented in the hardware"]
    #[inline]
    pub fn hash_algorithms(&self) -> HASH_ALGORITHMSR {
        HASH_ALGORITHMSR::_from({
            const MASK: u16 = 65535;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        })
    }
}
