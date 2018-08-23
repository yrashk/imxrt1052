#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::OUT_CTRL {
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
    #[doc = "32-bit pixels"]
    ARGB8888,
    #[doc = "32-bit pixels (unpacked 24-bit pixel in 32 bit DWORD.)"]
    RGB888,
    #[doc = "24-bit pixels (packed 24-bit format)"]
    RGB888P,
    #[doc = "16-bit pixels"]
    ARGB1555,
    #[doc = "16-bit pixels"]
    ARGB4444,
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
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl FORMATR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            FORMATR::ARGB8888 => 0,
            FORMATR::RGB888 => 4,
            FORMATR::RGB888P => 5,
            FORMATR::ARGB1555 => 8,
            FORMATR::ARGB4444 => 9,
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
            FORMATR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> FORMATR {
        match value {
            0 => FORMATR::ARGB8888,
            4 => FORMATR::RGB888,
            5 => FORMATR::RGB888P,
            8 => FORMATR::ARGB1555,
            9 => FORMATR::ARGB4444,
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
            i => FORMATR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `ARGB8888`"]
    #[inline]
    pub fn is_argb8888(&self) -> bool {
        *self == FORMATR::ARGB8888
    }
    #[doc = "Checks if the value of the field is `RGB888`"]
    #[inline]
    pub fn is_rgb888(&self) -> bool {
        *self == FORMATR::RGB888
    }
    #[doc = "Checks if the value of the field is `RGB888P`"]
    #[inline]
    pub fn is_rgb888p(&self) -> bool {
        *self == FORMATR::RGB888P
    }
    #[doc = "Checks if the value of the field is `ARGB1555`"]
    #[inline]
    pub fn is_argb1555(&self) -> bool {
        *self == FORMATR::ARGB1555
    }
    #[doc = "Checks if the value of the field is `ARGB4444`"]
    #[inline]
    pub fn is_argb4444(&self) -> bool {
        *self == FORMATR::ARGB4444
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
#[doc = "Possible values of the field `INTERLACED_OUTPUT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INTERLACED_OUTPUTR {
    #[doc = "All data written in progressive format to the OUTBUF Pointer."]
    PROGRESSIVE,
    #[doc = "Interlaced output: only data for field 0 is written to the OUTBUF Pointer."]
    FIELD0,
    #[doc = "Interlaced output: only data for field 1 is written to the OUTBUF2 Pointer."]
    FIELD1,
    #[doc = "Interlaced output: data for field 0 is written to OUTBUF and data for field 1 is written to OUTBUF2."]
    INTERLACED,
}
impl INTERLACED_OUTPUTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            INTERLACED_OUTPUTR::PROGRESSIVE => 0,
            INTERLACED_OUTPUTR::FIELD0 => 1,
            INTERLACED_OUTPUTR::FIELD1 => 2,
            INTERLACED_OUTPUTR::INTERLACED => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> INTERLACED_OUTPUTR {
        match value {
            0 => INTERLACED_OUTPUTR::PROGRESSIVE,
            1 => INTERLACED_OUTPUTR::FIELD0,
            2 => INTERLACED_OUTPUTR::FIELD1,
            3 => INTERLACED_OUTPUTR::INTERLACED,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `PROGRESSIVE`"]
    #[inline]
    pub fn is_progressive(&self) -> bool {
        *self == INTERLACED_OUTPUTR::PROGRESSIVE
    }
    #[doc = "Checks if the value of the field is `FIELD0`"]
    #[inline]
    pub fn is_field0(&self) -> bool {
        *self == INTERLACED_OUTPUTR::FIELD0
    }
    #[doc = "Checks if the value of the field is `FIELD1`"]
    #[inline]
    pub fn is_field1(&self) -> bool {
        *self == INTERLACED_OUTPUTR::FIELD1
    }
    #[doc = "Checks if the value of the field is `INTERLACED`"]
    #[inline]
    pub fn is_interlaced(&self) -> bool {
        *self == INTERLACED_OUTPUTR::INTERLACED
    }
}
#[doc = r" Value of the field"]
pub struct RSVD1R {
    bits: u16,
}
impl RSVD1R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct ALPHA_OUTPUTR {
    bits: bool,
}
impl ALPHA_OUTPUTR {
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
pub struct ALPHAR {
    bits: u8,
}
impl ALPHAR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Values that can be written to the field `FORMAT`"]
pub enum FORMATW {
    #[doc = "32-bit pixels"]
    ARGB8888,
    #[doc = "32-bit pixels (unpacked 24-bit pixel in 32 bit DWORD.)"]
    RGB888,
    #[doc = "24-bit pixels (packed 24-bit format)"]
    RGB888P,
    #[doc = "16-bit pixels"]
    ARGB1555,
    #[doc = "16-bit pixels"]
    ARGB4444,
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
}
impl FORMATW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            FORMATW::ARGB8888 => 0,
            FORMATW::RGB888 => 4,
            FORMATW::RGB888P => 5,
            FORMATW::ARGB1555 => 8,
            FORMATW::ARGB4444 => 9,
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
    #[doc = "32-bit pixels"]
    #[inline]
    pub fn argb8888(self) -> &'a mut W {
        self.variant(FORMATW::ARGB8888)
    }
    #[doc = "32-bit pixels (unpacked 24-bit pixel in 32 bit DWORD.)"]
    #[inline]
    pub fn rgb888(self) -> &'a mut W {
        self.variant(FORMATW::RGB888)
    }
    #[doc = "24-bit pixels (packed 24-bit format)"]
    #[inline]
    pub fn rgb888p(self) -> &'a mut W {
        self.variant(FORMATW::RGB888P)
    }
    #[doc = "16-bit pixels"]
    #[inline]
    pub fn argb1555(self) -> &'a mut W {
        self.variant(FORMATW::ARGB1555)
    }
    #[doc = "16-bit pixels"]
    #[inline]
    pub fn argb4444(self) -> &'a mut W {
        self.variant(FORMATW::ARGB4444)
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
#[doc = "Values that can be written to the field `INTERLACED_OUTPUT`"]
pub enum INTERLACED_OUTPUTW {
    #[doc = "All data written in progressive format to the OUTBUF Pointer."]
    PROGRESSIVE,
    #[doc = "Interlaced output: only data for field 0 is written to the OUTBUF Pointer."]
    FIELD0,
    #[doc = "Interlaced output: only data for field 1 is written to the OUTBUF2 Pointer."]
    FIELD1,
    #[doc = "Interlaced output: data for field 0 is written to OUTBUF and data for field 1 is written to OUTBUF2."]
    INTERLACED,
}
impl INTERLACED_OUTPUTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            INTERLACED_OUTPUTW::PROGRESSIVE => 0,
            INTERLACED_OUTPUTW::FIELD0 => 1,
            INTERLACED_OUTPUTW::FIELD1 => 2,
            INTERLACED_OUTPUTW::INTERLACED => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _INTERLACED_OUTPUTW<'a> {
    w: &'a mut W,
}
impl<'a> _INTERLACED_OUTPUTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: INTERLACED_OUTPUTW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "All data written in progressive format to the OUTBUF Pointer."]
    #[inline]
    pub fn progressive(self) -> &'a mut W {
        self.variant(INTERLACED_OUTPUTW::PROGRESSIVE)
    }
    #[doc = "Interlaced output: only data for field 0 is written to the OUTBUF Pointer."]
    #[inline]
    pub fn field0(self) -> &'a mut W {
        self.variant(INTERLACED_OUTPUTW::FIELD0)
    }
    #[doc = "Interlaced output: only data for field 1 is written to the OUTBUF2 Pointer."]
    #[inline]
    pub fn field1(self) -> &'a mut W {
        self.variant(INTERLACED_OUTPUTW::FIELD1)
    }
    #[doc = "Interlaced output: data for field 0 is written to OUTBUF and data for field 1 is written to OUTBUF2."]
    #[inline]
    pub fn interlaced(self) -> &'a mut W {
        self.variant(INTERLACED_OUTPUTW::INTERLACED)
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
#[doc = r" Proxy"]
pub struct _ALPHA_OUTPUTW<'a> {
    w: &'a mut W,
}
impl<'a> _ALPHA_OUTPUTW<'a> {
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
        const OFFSET: u8 = 23;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _ALPHAW<'a> {
    w: &'a mut W,
}
impl<'a> _ALPHAW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 255;
        const OFFSET: u8 = 24;
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
    #[doc = "Bits 0:4 - Output framebuffer format"]
    #[inline]
    pub fn format(&self) -> FORMATR {
        FORMATR::_from({
            const MASK: u8 = 31;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 5:7 - Reserved, always set to zero."]
    #[inline]
    pub fn rsvd0(&self) -> RSVD0R {
        let bits = {
            const MASK: u8 = 7;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        RSVD0R { bits }
    }
    #[doc = "Bits 8:9 - Determines how the PXP writes it's output data"]
    #[inline]
    pub fn interlaced_output(&self) -> INTERLACED_OUTPUTR {
        INTERLACED_OUTPUTR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 10:22 - Reserved, always set to zero."]
    #[inline]
    pub fn rsvd1(&self) -> RSVD1R {
        let bits = {
            const MASK: u16 = 8191;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        };
        RSVD1R { bits }
    }
    #[doc = "Bit 23 - Indicates that alpha component in output buffer pixels should be overwritten by PXP_OUT_CTRL[ALPHA]"]
    #[inline]
    pub fn alpha_output(&self) -> ALPHA_OUTPUTR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 23;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        ALPHA_OUTPUTR { bits }
    }
    #[doc = "Bits 24:31 - When generating an output buffer with an alpha component, the value in this field will be used when enabled to override the alpha passed through the pixel data pipeline"]
    #[inline]
    pub fn alpha(&self) -> ALPHAR {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        ALPHAR { bits }
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
    #[doc = "Bits 0:4 - Output framebuffer format"]
    #[inline]
    pub fn format(&mut self) -> _FORMATW {
        _FORMATW { w: self }
    }
    #[doc = "Bits 8:9 - Determines how the PXP writes it's output data"]
    #[inline]
    pub fn interlaced_output(&mut self) -> _INTERLACED_OUTPUTW {
        _INTERLACED_OUTPUTW { w: self }
    }
    #[doc = "Bit 23 - Indicates that alpha component in output buffer pixels should be overwritten by PXP_OUT_CTRL[ALPHA]"]
    #[inline]
    pub fn alpha_output(&mut self) -> _ALPHA_OUTPUTW {
        _ALPHA_OUTPUTW { w: self }
    }
    #[doc = "Bits 24:31 - When generating an output buffer with an alpha component, the value in this field will be used when enabled to override the alpha passed through the pixel data pipeline"]
    #[inline]
    pub fn alpha(&mut self) -> _ALPHAW {
        _ALPHAW { w: self }
    }
}
