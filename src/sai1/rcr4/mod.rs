#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::RCR4 {
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
#[doc = "Possible values of the field `FSD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FSDR {
    #[doc = "Frame Sync is generated externally in Slave mode."]
    FSD_0,
    #[doc = "Frame Sync is generated internally in Master mode."]
    FSD_1,
}
impl FSDR {
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
            FSDR::FSD_0 => false,
            FSDR::FSD_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> FSDR {
        match value {
            false => FSDR::FSD_0,
            true => FSDR::FSD_1,
        }
    }
    #[doc = "Checks if the value of the field is `FSD_0`"]
    #[inline]
    pub fn is_fsd_0(&self) -> bool {
        *self == FSDR::FSD_0
    }
    #[doc = "Checks if the value of the field is `FSD_1`"]
    #[inline]
    pub fn is_fsd_1(&self) -> bool {
        *self == FSDR::FSD_1
    }
}
#[doc = "Possible values of the field `FSP`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FSPR {
    #[doc = "Frame sync is active high."]
    FSP_0,
    #[doc = "Frame sync is active low."]
    FSP_1,
}
impl FSPR {
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
            FSPR::FSP_0 => false,
            FSPR::FSP_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> FSPR {
        match value {
            false => FSPR::FSP_0,
            true => FSPR::FSP_1,
        }
    }
    #[doc = "Checks if the value of the field is `FSP_0`"]
    #[inline]
    pub fn is_fsp_0(&self) -> bool {
        *self == FSPR::FSP_0
    }
    #[doc = "Checks if the value of the field is `FSP_1`"]
    #[inline]
    pub fn is_fsp_1(&self) -> bool {
        *self == FSPR::FSP_1
    }
}
#[doc = "Possible values of the field `ONDEM`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ONDEMR {
    #[doc = "Internal frame sync is generated continuously."]
    ONDEM_0,
    #[doc = "Internal frame sync is generated when the FIFO warning flag is clear."]
    ONDEM_1,
}
impl ONDEMR {
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
            ONDEMR::ONDEM_0 => false,
            ONDEMR::ONDEM_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ONDEMR {
        match value {
            false => ONDEMR::ONDEM_0,
            true => ONDEMR::ONDEM_1,
        }
    }
    #[doc = "Checks if the value of the field is `ONDEM_0`"]
    #[inline]
    pub fn is_ondem_0(&self) -> bool {
        *self == ONDEMR::ONDEM_0
    }
    #[doc = "Checks if the value of the field is `ONDEM_1`"]
    #[inline]
    pub fn is_ondem_1(&self) -> bool {
        *self == ONDEMR::ONDEM_1
    }
}
#[doc = "Possible values of the field `FSE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FSER {
    #[doc = "Frame sync asserts with the first bit of the frame."]
    FSE_0,
    #[doc = "Frame sync asserts one bit before the first bit of the frame."]
    FSE_1,
}
impl FSER {
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
            FSER::FSE_0 => false,
            FSER::FSE_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> FSER {
        match value {
            false => FSER::FSE_0,
            true => FSER::FSE_1,
        }
    }
    #[doc = "Checks if the value of the field is `FSE_0`"]
    #[inline]
    pub fn is_fse_0(&self) -> bool {
        *self == FSER::FSE_0
    }
    #[doc = "Checks if the value of the field is `FSE_1`"]
    #[inline]
    pub fn is_fse_1(&self) -> bool {
        *self == FSER::FSE_1
    }
}
#[doc = "Possible values of the field `MF`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MFR {
    #[doc = "LSB is received first."]
    MF_0,
    #[doc = "MSB is received first."]
    MF_1,
}
impl MFR {
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
            MFR::MF_0 => false,
            MFR::MF_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> MFR {
        match value {
            false => MFR::MF_0,
            true => MFR::MF_1,
        }
    }
    #[doc = "Checks if the value of the field is `MF_0`"]
    #[inline]
    pub fn is_mf_0(&self) -> bool {
        *self == MFR::MF_0
    }
    #[doc = "Checks if the value of the field is `MF_1`"]
    #[inline]
    pub fn is_mf_1(&self) -> bool {
        *self == MFR::MF_1
    }
}
#[doc = r" Value of the field"]
pub struct SYWDR {
    bits: u8,
}
impl SYWDR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct FRSZR {
    bits: u8,
}
impl FRSZR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Possible values of the field `FPACK`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FPACKR {
    #[doc = "FIFO packing is disabled"]
    FPACK_0,
    #[doc = "8-bit FIFO packing is enabled"]
    FPACK_2,
    #[doc = "16-bit FIFO packing is enabled"]
    FPACK_3,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl FPACKR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            FPACKR::FPACK_0 => 0,
            FPACKR::FPACK_2 => 2,
            FPACKR::FPACK_3 => 3,
            FPACKR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> FPACKR {
        match value {
            0 => FPACKR::FPACK_0,
            2 => FPACKR::FPACK_2,
            3 => FPACKR::FPACK_3,
            i => FPACKR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `FPACK_0`"]
    #[inline]
    pub fn is_fpack_0(&self) -> bool {
        *self == FPACKR::FPACK_0
    }
    #[doc = "Checks if the value of the field is `FPACK_2`"]
    #[inline]
    pub fn is_fpack_2(&self) -> bool {
        *self == FPACKR::FPACK_2
    }
    #[doc = "Checks if the value of the field is `FPACK_3`"]
    #[inline]
    pub fn is_fpack_3(&self) -> bool {
        *self == FPACKR::FPACK_3
    }
}
#[doc = "Possible values of the field `FCOMB`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FCOMBR {
    #[doc = "FIFO combine mode disabled."]
    FCOMB_0,
    #[doc = "FIFO combine mode enabled on FIFO writes (from receive shift registers)."]
    FCOMB_1,
    #[doc = "FIFO combine mode enabled on FIFO reads (by software)."]
    FCOMB_2,
    #[doc = "FIFO combine mode enabled on FIFO writes (from receive shift registers) and reads (by software)."]
    FCOMB_3,
}
impl FCOMBR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            FCOMBR::FCOMB_0 => 0,
            FCOMBR::FCOMB_1 => 1,
            FCOMBR::FCOMB_2 => 2,
            FCOMBR::FCOMB_3 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> FCOMBR {
        match value {
            0 => FCOMBR::FCOMB_0,
            1 => FCOMBR::FCOMB_1,
            2 => FCOMBR::FCOMB_2,
            3 => FCOMBR::FCOMB_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `FCOMB_0`"]
    #[inline]
    pub fn is_fcomb_0(&self) -> bool {
        *self == FCOMBR::FCOMB_0
    }
    #[doc = "Checks if the value of the field is `FCOMB_1`"]
    #[inline]
    pub fn is_fcomb_1(&self) -> bool {
        *self == FCOMBR::FCOMB_1
    }
    #[doc = "Checks if the value of the field is `FCOMB_2`"]
    #[inline]
    pub fn is_fcomb_2(&self) -> bool {
        *self == FCOMBR::FCOMB_2
    }
    #[doc = "Checks if the value of the field is `FCOMB_3`"]
    #[inline]
    pub fn is_fcomb_3(&self) -> bool {
        *self == FCOMBR::FCOMB_3
    }
}
#[doc = "Possible values of the field `FCONT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FCONTR {
    #[doc = "On FIFO error, the SAI will continue from the start of the next frame after the FIFO error flag has been cleared."]
    FCONT_0,
    #[doc = "On FIFO error, the SAI will continue from the same word that caused the FIFO error to set after the FIFO warning flag has been cleared."]
    FCONT_1,
}
impl FCONTR {
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
            FCONTR::FCONT_0 => false,
            FCONTR::FCONT_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> FCONTR {
        match value {
            false => FCONTR::FCONT_0,
            true => FCONTR::FCONT_1,
        }
    }
    #[doc = "Checks if the value of the field is `FCONT_0`"]
    #[inline]
    pub fn is_fcont_0(&self) -> bool {
        *self == FCONTR::FCONT_0
    }
    #[doc = "Checks if the value of the field is `FCONT_1`"]
    #[inline]
    pub fn is_fcont_1(&self) -> bool {
        *self == FCONTR::FCONT_1
    }
}
#[doc = "Values that can be written to the field `FSD`"]
pub enum FSDW {
    #[doc = "Frame Sync is generated externally in Slave mode."]
    FSD_0,
    #[doc = "Frame Sync is generated internally in Master mode."]
    FSD_1,
}
impl FSDW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            FSDW::FSD_0 => false,
            FSDW::FSD_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FSDW<'a> {
    w: &'a mut W,
}
impl<'a> _FSDW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FSDW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Frame Sync is generated externally in Slave mode."]
    #[inline]
    pub fn fsd_0(self) -> &'a mut W {
        self.variant(FSDW::FSD_0)
    }
    #[doc = "Frame Sync is generated internally in Master mode."]
    #[inline]
    pub fn fsd_1(self) -> &'a mut W {
        self.variant(FSDW::FSD_1)
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
#[doc = "Values that can be written to the field `FSP`"]
pub enum FSPW {
    #[doc = "Frame sync is active high."]
    FSP_0,
    #[doc = "Frame sync is active low."]
    FSP_1,
}
impl FSPW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            FSPW::FSP_0 => false,
            FSPW::FSP_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FSPW<'a> {
    w: &'a mut W,
}
impl<'a> _FSPW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FSPW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Frame sync is active high."]
    #[inline]
    pub fn fsp_0(self) -> &'a mut W {
        self.variant(FSPW::FSP_0)
    }
    #[doc = "Frame sync is active low."]
    #[inline]
    pub fn fsp_1(self) -> &'a mut W {
        self.variant(FSPW::FSP_1)
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
        const OFFSET: u8 = 1;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `ONDEM`"]
pub enum ONDEMW {
    #[doc = "Internal frame sync is generated continuously."]
    ONDEM_0,
    #[doc = "Internal frame sync is generated when the FIFO warning flag is clear."]
    ONDEM_1,
}
impl ONDEMW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ONDEMW::ONDEM_0 => false,
            ONDEMW::ONDEM_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ONDEMW<'a> {
    w: &'a mut W,
}
impl<'a> _ONDEMW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ONDEMW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Internal frame sync is generated continuously."]
    #[inline]
    pub fn ondem_0(self) -> &'a mut W {
        self.variant(ONDEMW::ONDEM_0)
    }
    #[doc = "Internal frame sync is generated when the FIFO warning flag is clear."]
    #[inline]
    pub fn ondem_1(self) -> &'a mut W {
        self.variant(ONDEMW::ONDEM_1)
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
#[doc = "Values that can be written to the field `FSE`"]
pub enum FSEW {
    #[doc = "Frame sync asserts with the first bit of the frame."]
    FSE_0,
    #[doc = "Frame sync asserts one bit before the first bit of the frame."]
    FSE_1,
}
impl FSEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            FSEW::FSE_0 => false,
            FSEW::FSE_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FSEW<'a> {
    w: &'a mut W,
}
impl<'a> _FSEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FSEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Frame sync asserts with the first bit of the frame."]
    #[inline]
    pub fn fse_0(self) -> &'a mut W {
        self.variant(FSEW::FSE_0)
    }
    #[doc = "Frame sync asserts one bit before the first bit of the frame."]
    #[inline]
    pub fn fse_1(self) -> &'a mut W {
        self.variant(FSEW::FSE_1)
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
#[doc = "Values that can be written to the field `MF`"]
pub enum MFW {
    #[doc = "LSB is received first."]
    MF_0,
    #[doc = "MSB is received first."]
    MF_1,
}
impl MFW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            MFW::MF_0 => false,
            MFW::MF_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MFW<'a> {
    w: &'a mut W,
}
impl<'a> _MFW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MFW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "LSB is received first."]
    #[inline]
    pub fn mf_0(self) -> &'a mut W {
        self.variant(MFW::MF_0)
    }
    #[doc = "MSB is received first."]
    #[inline]
    pub fn mf_1(self) -> &'a mut W {
        self.variant(MFW::MF_1)
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
pub struct _SYWDW<'a> {
    w: &'a mut W,
}
impl<'a> _SYWDW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 31;
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _FRSZW<'a> {
    w: &'a mut W,
}
impl<'a> _FRSZW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 31;
        const OFFSET: u8 = 16;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `FPACK`"]
pub enum FPACKW {
    #[doc = "FIFO packing is disabled"]
    FPACK_0,
    #[doc = "8-bit FIFO packing is enabled"]
    FPACK_2,
    #[doc = "16-bit FIFO packing is enabled"]
    FPACK_3,
}
impl FPACKW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            FPACKW::FPACK_0 => 0,
            FPACKW::FPACK_2 => 2,
            FPACKW::FPACK_3 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FPACKW<'a> {
    w: &'a mut W,
}
impl<'a> _FPACKW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FPACKW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "FIFO packing is disabled"]
    #[inline]
    pub fn fpack_0(self) -> &'a mut W {
        self.variant(FPACKW::FPACK_0)
    }
    #[doc = "8-bit FIFO packing is enabled"]
    #[inline]
    pub fn fpack_2(self) -> &'a mut W {
        self.variant(FPACKW::FPACK_2)
    }
    #[doc = "16-bit FIFO packing is enabled"]
    #[inline]
    pub fn fpack_3(self) -> &'a mut W {
        self.variant(FPACKW::FPACK_3)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 24;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `FCOMB`"]
pub enum FCOMBW {
    #[doc = "FIFO combine mode disabled."]
    FCOMB_0,
    #[doc = "FIFO combine mode enabled on FIFO writes (from receive shift registers)."]
    FCOMB_1,
    #[doc = "FIFO combine mode enabled on FIFO reads (by software)."]
    FCOMB_2,
    #[doc = "FIFO combine mode enabled on FIFO writes (from receive shift registers) and reads (by software)."]
    FCOMB_3,
}
impl FCOMBW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            FCOMBW::FCOMB_0 => 0,
            FCOMBW::FCOMB_1 => 1,
            FCOMBW::FCOMB_2 => 2,
            FCOMBW::FCOMB_3 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FCOMBW<'a> {
    w: &'a mut W,
}
impl<'a> _FCOMBW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FCOMBW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "FIFO combine mode disabled."]
    #[inline]
    pub fn fcomb_0(self) -> &'a mut W {
        self.variant(FCOMBW::FCOMB_0)
    }
    #[doc = "FIFO combine mode enabled on FIFO writes (from receive shift registers)."]
    #[inline]
    pub fn fcomb_1(self) -> &'a mut W {
        self.variant(FCOMBW::FCOMB_1)
    }
    #[doc = "FIFO combine mode enabled on FIFO reads (by software)."]
    #[inline]
    pub fn fcomb_2(self) -> &'a mut W {
        self.variant(FCOMBW::FCOMB_2)
    }
    #[doc = "FIFO combine mode enabled on FIFO writes (from receive shift registers) and reads (by software)."]
    #[inline]
    pub fn fcomb_3(self) -> &'a mut W {
        self.variant(FCOMBW::FCOMB_3)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 26;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `FCONT`"]
pub enum FCONTW {
    #[doc = "On FIFO error, the SAI will continue from the start of the next frame after the FIFO error flag has been cleared."]
    FCONT_0,
    #[doc = "On FIFO error, the SAI will continue from the same word that caused the FIFO error to set after the FIFO warning flag has been cleared."]
    FCONT_1,
}
impl FCONTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            FCONTW::FCONT_0 => false,
            FCONTW::FCONT_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FCONTW<'a> {
    w: &'a mut W,
}
impl<'a> _FCONTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FCONTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "On FIFO error, the SAI will continue from the start of the next frame after the FIFO error flag has been cleared."]
    #[inline]
    pub fn fcont_0(self) -> &'a mut W {
        self.variant(FCONTW::FCONT_0)
    }
    #[doc = "On FIFO error, the SAI will continue from the same word that caused the FIFO error to set after the FIFO warning flag has been cleared."]
    #[inline]
    pub fn fcont_1(self) -> &'a mut W {
        self.variant(FCONTW::FCONT_1)
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
    #[doc = "Bit 0 - Frame Sync Direction"]
    #[inline]
    pub fn fsd(&self) -> FSDR {
        FSDR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - Frame Sync Polarity"]
    #[inline]
    pub fn fsp(&self) -> FSPR {
        FSPR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - On Demand Mode"]
    #[inline]
    pub fn ondem(&self) -> ONDEMR {
        ONDEMR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - Frame Sync Early"]
    #[inline]
    pub fn fse(&self) -> FSER {
        FSER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 4 - MSB First"]
    #[inline]
    pub fn mf(&self) -> MFR {
        MFR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 8:12 - Sync Width"]
    #[inline]
    pub fn sywd(&self) -> SYWDR {
        let bits = {
            const MASK: u8 = 31;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        SYWDR { bits }
    }
    #[doc = "Bits 16:20 - Frame Size"]
    #[inline]
    pub fn frsz(&self) -> FRSZR {
        let bits = {
            const MASK: u8 = 31;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        FRSZR { bits }
    }
    #[doc = "Bits 24:25 - FIFO Packing Mode"]
    #[inline]
    pub fn fpack(&self) -> FPACKR {
        FPACKR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 26:27 - FIFO Combine Mode"]
    #[inline]
    pub fn fcomb(&self) -> FCOMBR {
        FCOMBR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 26;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 28 - FIFO Continue on Error"]
    #[inline]
    pub fn fcont(&self) -> FCONTR {
        FCONTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 28;
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
    #[doc = "Bit 0 - Frame Sync Direction"]
    #[inline]
    pub fn fsd(&mut self) -> _FSDW {
        _FSDW { w: self }
    }
    #[doc = "Bit 1 - Frame Sync Polarity"]
    #[inline]
    pub fn fsp(&mut self) -> _FSPW {
        _FSPW { w: self }
    }
    #[doc = "Bit 2 - On Demand Mode"]
    #[inline]
    pub fn ondem(&mut self) -> _ONDEMW {
        _ONDEMW { w: self }
    }
    #[doc = "Bit 3 - Frame Sync Early"]
    #[inline]
    pub fn fse(&mut self) -> _FSEW {
        _FSEW { w: self }
    }
    #[doc = "Bit 4 - MSB First"]
    #[inline]
    pub fn mf(&mut self) -> _MFW {
        _MFW { w: self }
    }
    #[doc = "Bits 8:12 - Sync Width"]
    #[inline]
    pub fn sywd(&mut self) -> _SYWDW {
        _SYWDW { w: self }
    }
    #[doc = "Bits 16:20 - Frame Size"]
    #[inline]
    pub fn frsz(&mut self) -> _FRSZW {
        _FRSZW { w: self }
    }
    #[doc = "Bits 24:25 - FIFO Packing Mode"]
    #[inline]
    pub fn fpack(&mut self) -> _FPACKW {
        _FPACKW { w: self }
    }
    #[doc = "Bits 26:27 - FIFO Combine Mode"]
    #[inline]
    pub fn fcomb(&mut self) -> _FCOMBW {
        _FCOMBW { w: self }
    }
    #[doc = "Bit 28 - FIFO Continue on Error"]
    #[inline]
    pub fn fcont(&mut self) -> _FCONTW {
        _FCONTW { w: self }
    }
}
