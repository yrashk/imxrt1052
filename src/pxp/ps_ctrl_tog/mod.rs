#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::PS_CTRL_TOG {
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
#[doc = "Possible values of the field `FORMAT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FORMATR {
    #[doc = "32-bit pixels (unpacked 24-bit format)"]
    RGB888,
    #[doc = "16-bit pixels"]
    RGB555,
    #[doc = "16-bit pixels"]
    RGB444,
    #[doc = "16-bit pixels"]
    RGB565,
    #[doc = "32-bit pixels (1-plane XYUV unpacked)"]
    YUV1P444,
    #[doc = "16-bit pixels (1-plane U0,Y0,V0,Y1 interleaved bytes)"]
    UYVY1P422,
    #[doc = "16-bit pixels (1-plane V0,Y0,U0,Y1 interleaved bytes)"]
    VYUY1P422,
    #[doc = "8-bit monochrome pixels (1-plane Y luma output)"]
    Y8,
    #[doc = "4-bit monochrome pixels (1-plane Y luma, 4 bit truncation)"]
    Y4,
    #[doc = "16-bit pixels (2-plane UV interleaved bytes)"]
    YUV2P422,
    #[doc = "16-bit pixels (2-plane UV)"]
    YUV2P420,
    #[doc = "16-bit pixels (2-plane VU interleaved bytes)"]
    YVU2P422,
    #[doc = "16-bit pixels (2-plane VU)"]
    YVU2P420,
    #[doc = "16-bit pixels (3-plane format)"]
    YUV422,
    #[doc = "16-bit pixels (3-plane format)"]
    YUV420,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl FORMATR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            FORMATR::RGB888 => 4,
            FORMATR::RGB555 => 12,
            FORMATR::RGB444 => 13,
            FORMATR::RGB565 => 14,
            FORMATR::YUV1P444 => 16,
            FORMATR::UYVY1P422 => 18,
            FORMATR::VYUY1P422 => 19,
            FORMATR::Y8 => 20,
            FORMATR::Y4 => 21,
            FORMATR::YUV2P422 => 24,
            FORMATR::YUV2P420 => 25,
            FORMATR::YVU2P422 => 26,
            FORMATR::YVU2P420 => 27,
            FORMATR::YUV422 => 30,
            FORMATR::YUV420 => 31,
            FORMATR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> FORMATR {
        match value {
            4 => FORMATR::RGB888,
            12 => FORMATR::RGB555,
            13 => FORMATR::RGB444,
            14 => FORMATR::RGB565,
            16 => FORMATR::YUV1P444,
            18 => FORMATR::UYVY1P422,
            19 => FORMATR::VYUY1P422,
            20 => FORMATR::Y8,
            21 => FORMATR::Y4,
            24 => FORMATR::YUV2P422,
            25 => FORMATR::YUV2P420,
            26 => FORMATR::YVU2P422,
            27 => FORMATR::YVU2P420,
            30 => FORMATR::YUV422,
            31 => FORMATR::YUV420,
            i => FORMATR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `RGB888`"]
    #[inline]
    pub fn is_rgb888(&self) -> bool {
        *self == FORMATR::RGB888
    }
    #[doc = "Checks if the value of the field is `RGB555`"]
    #[inline]
    pub fn is_rgb555(&self) -> bool {
        *self == FORMATR::RGB555
    }
    #[doc = "Checks if the value of the field is `RGB444`"]
    #[inline]
    pub fn is_rgb444(&self) -> bool {
        *self == FORMATR::RGB444
    }
    #[doc = "Checks if the value of the field is `RGB565`"]
    #[inline]
    pub fn is_rgb565(&self) -> bool {
        *self == FORMATR::RGB565
    }
    #[doc = "Checks if the value of the field is `YUV1P444`"]
    #[inline]
    pub fn is_yuv1p444(&self) -> bool {
        *self == FORMATR::YUV1P444
    }
    #[doc = "Checks if the value of the field is `UYVY1P422`"]
    #[inline]
    pub fn is_uyvy1p422(&self) -> bool {
        *self == FORMATR::UYVY1P422
    }
    #[doc = "Checks if the value of the field is `VYUY1P422`"]
    #[inline]
    pub fn is_vyuy1p422(&self) -> bool {
        *self == FORMATR::VYUY1P422
    }
    #[doc = "Checks if the value of the field is `Y8`"]
    #[inline]
    pub fn is_y8(&self) -> bool {
        *self == FORMATR::Y8
    }
    #[doc = "Checks if the value of the field is `Y4`"]
    #[inline]
    pub fn is_y4(&self) -> bool {
        *self == FORMATR::Y4
    }
    #[doc = "Checks if the value of the field is `YUV2P422`"]
    #[inline]
    pub fn is_yuv2p422(&self) -> bool {
        *self == FORMATR::YUV2P422
    }
    #[doc = "Checks if the value of the field is `YUV2P420`"]
    #[inline]
    pub fn is_yuv2p420(&self) -> bool {
        *self == FORMATR::YUV2P420
    }
    #[doc = "Checks if the value of the field is `YVU2P422`"]
    #[inline]
    pub fn is_yvu2p422(&self) -> bool {
        *self == FORMATR::YVU2P422
    }
    #[doc = "Checks if the value of the field is `YVU2P420`"]
    #[inline]
    pub fn is_yvu2p420(&self) -> bool {
        *self == FORMATR::YVU2P420
    }
    #[doc = "Checks if the value of the field is `YUV422`"]
    #[inline]
    pub fn is_yuv422(&self) -> bool {
        *self == FORMATR::YUV422
    }
    #[doc = "Checks if the value of the field is `YUV420`"]
    #[inline]
    pub fn is_yuv420(&self) -> bool {
        *self == FORMATR::YUV420
    }
}
#[doc = r" Value of the field"]
pub struct WB_SWAPR {
    bits: bool,
}
impl WB_SWAPR {
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
pub struct RSVD0R {
    bits: u8,
}
impl RSVD0R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Possible values of the field `DECY`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DECYR {
    #[doc = "Disable pre-decimation filter."]
    DISABLE,
    #[doc = "Decimate PS by 2."]
    DECY2,
    #[doc = "Decimate PS by 4."]
    DECY4,
    #[doc = "Decimate PS by 8."]
    DECY8,
}
impl DECYR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            DECYR::DISABLE => 0,
            DECYR::DECY2 => 1,
            DECYR::DECY4 => 2,
            DECYR::DECY8 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> DECYR {
        match value {
            0 => DECYR::DISABLE,
            1 => DECYR::DECY2,
            2 => DECYR::DECY4,
            3 => DECYR::DECY8,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline]
    pub fn is_disable(&self) -> bool {
        *self == DECYR::DISABLE
    }
    #[doc = "Checks if the value of the field is `DECY2`"]
    #[inline]
    pub fn is_decy2(&self) -> bool {
        *self == DECYR::DECY2
    }
    #[doc = "Checks if the value of the field is `DECY4`"]
    #[inline]
    pub fn is_decy4(&self) -> bool {
        *self == DECYR::DECY4
    }
    #[doc = "Checks if the value of the field is `DECY8`"]
    #[inline]
    pub fn is_decy8(&self) -> bool {
        *self == DECYR::DECY8
    }
}
#[doc = "Possible values of the field `DECX`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DECXR {
    #[doc = "Disable pre-decimation filter."]
    DISABLE,
    #[doc = "Decimate PS by 2."]
    DECX2,
    #[doc = "Decimate PS by 4."]
    DECX4,
    #[doc = "Decimate PS by 8."]
    DECX8,
}
impl DECXR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            DECXR::DISABLE => 0,
            DECXR::DECX2 => 1,
            DECXR::DECX4 => 2,
            DECXR::DECX8 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> DECXR {
        match value {
            0 => DECXR::DISABLE,
            1 => DECXR::DECX2,
            2 => DECXR::DECX4,
            3 => DECXR::DECX8,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline]
    pub fn is_disable(&self) -> bool {
        *self == DECXR::DISABLE
    }
    #[doc = "Checks if the value of the field is `DECX2`"]
    #[inline]
    pub fn is_decx2(&self) -> bool {
        *self == DECXR::DECX2
    }
    #[doc = "Checks if the value of the field is `DECX4`"]
    #[inline]
    pub fn is_decx4(&self) -> bool {
        *self == DECXR::DECX4
    }
    #[doc = "Checks if the value of the field is `DECX8`"]
    #[inline]
    pub fn is_decx8(&self) -> bool {
        *self == DECXR::DECX8
    }
}
#[doc = r" Value of the field"]
pub struct RSVD1R {
    bits: u32,
}
impl RSVD1R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
}
#[doc = "Values that can be written to the field `FORMAT`"]
pub enum FORMATW {
    #[doc = "32-bit pixels (unpacked 24-bit format)"]
    RGB888,
    #[doc = "16-bit pixels"]
    RGB555,
    #[doc = "16-bit pixels"]
    RGB444,
    #[doc = "16-bit pixels"]
    RGB565,
    #[doc = "32-bit pixels (1-plane XYUV unpacked)"]
    YUV1P444,
    #[doc = "16-bit pixels (1-plane U0,Y0,V0,Y1 interleaved bytes)"]
    UYVY1P422,
    #[doc = "16-bit pixels (1-plane V0,Y0,U0,Y1 interleaved bytes)"]
    VYUY1P422,
    #[doc = "8-bit monochrome pixels (1-plane Y luma output)"]
    Y8,
    #[doc = "4-bit monochrome pixels (1-plane Y luma, 4 bit truncation)"]
    Y4,
    #[doc = "16-bit pixels (2-plane UV interleaved bytes)"]
    YUV2P422,
    #[doc = "16-bit pixels (2-plane UV)"]
    YUV2P420,
    #[doc = "16-bit pixels (2-plane VU interleaved bytes)"]
    YVU2P422,
    #[doc = "16-bit pixels (2-plane VU)"]
    YVU2P420,
    #[doc = "16-bit pixels (3-plane format)"]
    YUV422,
    #[doc = "16-bit pixels (3-plane format)"]
    YUV420,
}
impl FORMATW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            FORMATW::RGB888 => 4,
            FORMATW::RGB555 => 12,
            FORMATW::RGB444 => 13,
            FORMATW::RGB565 => 14,
            FORMATW::YUV1P444 => 16,
            FORMATW::UYVY1P422 => 18,
            FORMATW::VYUY1P422 => 19,
            FORMATW::Y8 => 20,
            FORMATW::Y4 => 21,
            FORMATW::YUV2P422 => 24,
            FORMATW::YUV2P420 => 25,
            FORMATW::YVU2P422 => 26,
            FORMATW::YVU2P420 => 27,
            FORMATW::YUV422 => 30,
            FORMATW::YUV420 => 31,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FORMATW<'a> {
    w: &'a mut W,
}
impl<'a> _FORMATW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FORMATW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "32-bit pixels (unpacked 24-bit format)"]
    #[inline]
    pub fn rgb888(self) -> &'a mut W {
        self.variant(FORMATW::RGB888)
    }
    #[doc = "16-bit pixels"]
    #[inline]
    pub fn rgb555(self) -> &'a mut W {
        self.variant(FORMATW::RGB555)
    }
    #[doc = "16-bit pixels"]
    #[inline]
    pub fn rgb444(self) -> &'a mut W {
        self.variant(FORMATW::RGB444)
    }
    #[doc = "16-bit pixels"]
    #[inline]
    pub fn rgb565(self) -> &'a mut W {
        self.variant(FORMATW::RGB565)
    }
    #[doc = "32-bit pixels (1-plane XYUV unpacked)"]
    #[inline]
    pub fn yuv1p444(self) -> &'a mut W {
        self.variant(FORMATW::YUV1P444)
    }
    #[doc = "16-bit pixels (1-plane U0,Y0,V0,Y1 interleaved bytes)"]
    #[inline]
    pub fn uyvy1p422(self) -> &'a mut W {
        self.variant(FORMATW::UYVY1P422)
    }
    #[doc = "16-bit pixels (1-plane V0,Y0,U0,Y1 interleaved bytes)"]
    #[inline]
    pub fn vyuy1p422(self) -> &'a mut W {
        self.variant(FORMATW::VYUY1P422)
    }
    #[doc = "8-bit monochrome pixels (1-plane Y luma output)"]
    #[inline]
    pub fn y8(self) -> &'a mut W {
        self.variant(FORMATW::Y8)
    }
    #[doc = "4-bit monochrome pixels (1-plane Y luma, 4 bit truncation)"]
    #[inline]
    pub fn y4(self) -> &'a mut W {
        self.variant(FORMATW::Y4)
    }
    #[doc = "16-bit pixels (2-plane UV interleaved bytes)"]
    #[inline]
    pub fn yuv2p422(self) -> &'a mut W {
        self.variant(FORMATW::YUV2P422)
    }
    #[doc = "16-bit pixels (2-plane UV)"]
    #[inline]
    pub fn yuv2p420(self) -> &'a mut W {
        self.variant(FORMATW::YUV2P420)
    }
    #[doc = "16-bit pixels (2-plane VU interleaved bytes)"]
    #[inline]
    pub fn yvu2p422(self) -> &'a mut W {
        self.variant(FORMATW::YVU2P422)
    }
    #[doc = "16-bit pixels (2-plane VU)"]
    #[inline]
    pub fn yvu2p420(self) -> &'a mut W {
        self.variant(FORMATW::YVU2P420)
    }
    #[doc = "16-bit pixels (3-plane format)"]
    #[inline]
    pub fn yuv422(self) -> &'a mut W {
        self.variant(FORMATW::YUV422)
    }
    #[doc = "16-bit pixels (3-plane format)"]
    #[inline]
    pub fn yuv420(self) -> &'a mut W {
        self.variant(FORMATW::YUV420)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 31;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _WB_SWAPW<'a> {
    w: &'a mut W,
}
impl<'a> _WB_SWAPW<'a> {
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
#[doc = "Values that can be written to the field `DECY`"]
pub enum DECYW {
    #[doc = "Disable pre-decimation filter."]
    DISABLE,
    #[doc = "Decimate PS by 2."]
    DECY2,
    #[doc = "Decimate PS by 4."]
    DECY4,
    #[doc = "Decimate PS by 8."]
    DECY8,
}
impl DECYW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            DECYW::DISABLE => 0,
            DECYW::DECY2 => 1,
            DECYW::DECY4 => 2,
            DECYW::DECY8 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DECYW<'a> {
    w: &'a mut W,
}
impl<'a> _DECYW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DECYW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Disable pre-decimation filter."]
    #[inline]
    pub fn disable(self) -> &'a mut W {
        self.variant(DECYW::DISABLE)
    }
    #[doc = "Decimate PS by 2."]
    #[inline]
    pub fn decy2(self) -> &'a mut W {
        self.variant(DECYW::DECY2)
    }
    #[doc = "Decimate PS by 4."]
    #[inline]
    pub fn decy4(self) -> &'a mut W {
        self.variant(DECYW::DECY4)
    }
    #[doc = "Decimate PS by 8."]
    #[inline]
    pub fn decy8(self) -> &'a mut W {
        self.variant(DECYW::DECY8)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `DECX`"]
pub enum DECXW {
    #[doc = "Disable pre-decimation filter."]
    DISABLE,
    #[doc = "Decimate PS by 2."]
    DECX2,
    #[doc = "Decimate PS by 4."]
    DECX4,
    #[doc = "Decimate PS by 8."]
    DECX8,
}
impl DECXW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            DECXW::DISABLE => 0,
            DECXW::DECX2 => 1,
            DECXW::DECX4 => 2,
            DECXW::DECX8 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DECXW<'a> {
    w: &'a mut W,
}
impl<'a> _DECXW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DECXW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Disable pre-decimation filter."]
    #[inline]
    pub fn disable(self) -> &'a mut W {
        self.variant(DECXW::DISABLE)
    }
    #[doc = "Decimate PS by 2."]
    #[inline]
    pub fn decx2(self) -> &'a mut W {
        self.variant(DECXW::DECX2)
    }
    #[doc = "Decimate PS by 4."]
    #[inline]
    pub fn decx4(self) -> &'a mut W {
        self.variant(DECXW::DECX4)
    }
    #[doc = "Decimate PS by 8."]
    #[inline]
    pub fn decx8(self) -> &'a mut W {
        self.variant(DECXW::DECX8)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 10;
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
    #[doc = "Bits 0:4 - PS buffer format. To select between YUV and YCbCr formats, see bit 31 of the CSC1_COEF0 register."]
    #[inline]
    pub fn format(&self) -> FORMATR {
        FORMATR::_from({
            const MASK: u8 = 31;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 5 - Swap bytes in words. For each 16 bit word, the two bytes will be swapped."]
    #[inline]
    pub fn wb_swap(&self) -> WB_SWAPR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        WB_SWAPR { bits }
    }
    #[doc = "Bits 6:7 - Reserved, always set to zero."]
    #[inline]
    pub fn rsvd0(&self) -> RSVD0R {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        RSVD0R { bits }
    }
    #[doc = "Bits 8:9 - Verticle pre decimation filter control."]
    #[inline]
    pub fn decy(&self) -> DECYR {
        DECYR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 10:11 - Horizontal pre decimation filter control."]
    #[inline]
    pub fn decx(&self) -> DECXR {
        DECXR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 12:31 - Reserved, always set to zero."]
    #[inline]
    pub fn rsvd1(&self) -> RSVD1R {
        let bits = {
            const MASK: u32 = 1048575;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) as u32
        };
        RSVD1R { bits }
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
    #[doc = "Bits 0:4 - PS buffer format. To select between YUV and YCbCr formats, see bit 31 of the CSC1_COEF0 register."]
    #[inline]
    pub fn format(&mut self) -> _FORMATW {
        _FORMATW { w: self }
    }
    #[doc = "Bit 5 - Swap bytes in words. For each 16 bit word, the two bytes will be swapped."]
    #[inline]
    pub fn wb_swap(&mut self) -> _WB_SWAPW {
        _WB_SWAPW { w: self }
    }
    #[doc = "Bits 8:9 - Verticle pre decimation filter control."]
    #[inline]
    pub fn decy(&mut self) -> _DECYW {
        _DECYW { w: self }
    }
    #[doc = "Bits 10:11 - Horizontal pre decimation filter control."]
    #[inline]
    pub fn decx(&mut self) -> _DECXW {
        _DECXW { w: self }
    }
}
