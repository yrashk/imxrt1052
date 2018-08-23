#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::PACKET2 {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = "Possible values of the field `CIPHER_SELECT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CIPHER_SELECTR {
    #[doc = "AES128"]
    AES128,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl CIPHER_SELECTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            CIPHER_SELECTR::AES128 => 0,
            CIPHER_SELECTR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> CIPHER_SELECTR {
        match value {
            0 => CIPHER_SELECTR::AES128,
            i => CIPHER_SELECTR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `AES128`"]
    #[inline]
    pub fn is_aes128(&self) -> bool {
        *self == CIPHER_SELECTR::AES128
    }
}
#[doc = "Possible values of the field `CIPHER_MODE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CIPHER_MODER {
    #[doc = "ECB"]
    ECB,
    #[doc = "CBC"]
    CBC,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl CIPHER_MODER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            CIPHER_MODER::ECB => 0,
            CIPHER_MODER::CBC => 1,
            CIPHER_MODER::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> CIPHER_MODER {
        match value {
            0 => CIPHER_MODER::ECB,
            1 => CIPHER_MODER::CBC,
            i => CIPHER_MODER::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `ECB`"]
    #[inline]
    pub fn is_ecb(&self) -> bool {
        *self == CIPHER_MODER::ECB
    }
    #[doc = "Checks if the value of the field is `CBC`"]
    #[inline]
    pub fn is_cbc(&self) -> bool {
        *self == CIPHER_MODER::CBC
    }
}
#[doc = "Possible values of the field `KEY_SELECT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum KEY_SELECTR {
    #[doc = "KEY0"]
    KEY0,
    #[doc = "KEY1"]
    KEY1,
    #[doc = "KEY2"]
    KEY2,
    #[doc = "KEY3"]
    KEY3,
    #[doc = "UNIQUE_KEY"]
    UNIQUE_KEY,
    #[doc = "OTP_KEY"]
    OTP_KEY,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl KEY_SELECTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            KEY_SELECTR::KEY0 => 0,
            KEY_SELECTR::KEY1 => 1,
            KEY_SELECTR::KEY2 => 2,
            KEY_SELECTR::KEY3 => 3,
            KEY_SELECTR::UNIQUE_KEY => 254,
            KEY_SELECTR::OTP_KEY => 255,
            KEY_SELECTR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> KEY_SELECTR {
        match value {
            0 => KEY_SELECTR::KEY0,
            1 => KEY_SELECTR::KEY1,
            2 => KEY_SELECTR::KEY2,
            3 => KEY_SELECTR::KEY3,
            254 => KEY_SELECTR::UNIQUE_KEY,
            255 => KEY_SELECTR::OTP_KEY,
            i => KEY_SELECTR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `KEY0`"]
    #[inline]
    pub fn is_key0(&self) -> bool {
        *self == KEY_SELECTR::KEY0
    }
    #[doc = "Checks if the value of the field is `KEY1`"]
    #[inline]
    pub fn is_key1(&self) -> bool {
        *self == KEY_SELECTR::KEY1
    }
    #[doc = "Checks if the value of the field is `KEY2`"]
    #[inline]
    pub fn is_key2(&self) -> bool {
        *self == KEY_SELECTR::KEY2
    }
    #[doc = "Checks if the value of the field is `KEY3`"]
    #[inline]
    pub fn is_key3(&self) -> bool {
        *self == KEY_SELECTR::KEY3
    }
    #[doc = "Checks if the value of the field is `UNIQUE_KEY`"]
    #[inline]
    pub fn is_unique_key(&self) -> bool {
        *self == KEY_SELECTR::UNIQUE_KEY
    }
    #[doc = "Checks if the value of the field is `OTP_KEY`"]
    #[inline]
    pub fn is_otp_key(&self) -> bool {
        *self == KEY_SELECTR::OTP_KEY
    }
}
#[doc = "Possible values of the field `HASH_SELECT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HASH_SELECTR {
    #[doc = "SHA1"]
    SHA1,
    #[doc = "CRC32"]
    CRC32,
    #[doc = "SHA256"]
    SHA256,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl HASH_SELECTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            HASH_SELECTR::SHA1 => 0,
            HASH_SELECTR::CRC32 => 1,
            HASH_SELECTR::SHA256 => 2,
            HASH_SELECTR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> HASH_SELECTR {
        match value {
            0 => HASH_SELECTR::SHA1,
            1 => HASH_SELECTR::CRC32,
            2 => HASH_SELECTR::SHA256,
            i => HASH_SELECTR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `SHA1`"]
    #[inline]
    pub fn is_sha1(&self) -> bool {
        *self == HASH_SELECTR::SHA1
    }
    #[doc = "Checks if the value of the field is `CRC32`"]
    #[inline]
    pub fn is_crc32(&self) -> bool {
        *self == HASH_SELECTR::CRC32
    }
    #[doc = "Checks if the value of the field is `SHA256`"]
    #[inline]
    pub fn is_sha256(&self) -> bool {
        *self == HASH_SELECTR::SHA256
    }
}
#[doc = r" Value of the field"]
pub struct CIPHER_CFGR {
    bits: u8,
}
impl CIPHER_CFGR {
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
    #[doc = "Bits 0:3 - Cipher selection field"]
    #[inline]
    pub fn cipher_select(&self) -> CIPHER_SELECTR {
        CIPHER_SELECTR::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 4:7 - Cipher mode selection field. Reflects the mode of operation for the cipher operations."]
    #[inline]
    pub fn cipher_mode(&self) -> CIPHER_MODER {
        CIPHER_MODER::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 8:15 - Key selection field"]
    #[inline]
    pub fn key_select(&self) -> KEY_SELECTR {
        KEY_SELECTR::_from({
            const MASK: u8 = 255;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 16:19 - Hash Selection Field"]
    #[inline]
    pub fn hash_select(&self) -> HASH_SELECTR {
        HASH_SELECTR::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 24:31 - Cipher configuration bits. Optional configuration bits are required for the ciphers."]
    #[inline]
    pub fn cipher_cfg(&self) -> CIPHER_CFGR {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        CIPHER_CFGR { bits }
    }
}
