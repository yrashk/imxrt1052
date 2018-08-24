#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::AS_CTRL {
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
pub struct RSVD0R {
    bits: bool,
}
impl RSVD0R {
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
#[doc = "Possible values of the field `ALPHA_CTRL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ALPHA_CTRLR {
    #[doc = "Indicates that the AS pixel alpha value will be used to blend the AS with PS. The ALPHA field is ignored."]
    EMBEDDED,
    #[doc = "Indicates that the value in the ALPHA field should be used instead of the alpha values present in the input pixels."]
    OVERRIDE,
    #[doc = "Indicates that the value in the ALPHA field should be used to scale all pixel alpha values. Each pixel alpha is multiplied by the value in the ALPHA field."]
    MULTIPLY,
    #[doc = "Enable ROPs. The ROP field indicates an operation to be performed on the alpha surface and PS pixels."]
    ROPS,
}
impl ALPHA_CTRLR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            ALPHA_CTRLR::EMBEDDED => 0,
            ALPHA_CTRLR::OVERRIDE => 1,
            ALPHA_CTRLR::MULTIPLY => 2,
            ALPHA_CTRLR::ROPS => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> ALPHA_CTRLR {
        match value {
            0 => ALPHA_CTRLR::EMBEDDED,
            1 => ALPHA_CTRLR::OVERRIDE,
            2 => ALPHA_CTRLR::MULTIPLY,
            3 => ALPHA_CTRLR::ROPS,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `EMBEDDED`"]
    #[inline]
    pub fn is_embedded(&self) -> bool {
        *self == ALPHA_CTRLR::EMBEDDED
    }
    #[doc = "Checks if the value of the field is `OVERRIDE`"]
    #[inline]
    pub fn is_override_(&self) -> bool {
        *self == ALPHA_CTRLR::OVERRIDE
    }
    #[doc = "Checks if the value of the field is `MULTIPLY`"]
    #[inline]
    pub fn is_multiply(&self) -> bool {
        *self == ALPHA_CTRLR::MULTIPLY
    }
    #[doc = "Checks if the value of the field is `ROPS`"]
    #[inline]
    pub fn is_rops(&self) -> bool {
        *self == ALPHA_CTRLR::ROPS
    }
}
#[doc = r" Value of the field"]
pub struct ENABLE_COLORKEYR {
    bits: bool,
}
impl ENABLE_COLORKEYR {
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
#[doc = "Possible values of the field `FORMAT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FORMATR {
    #[doc = "32-bit pixels with alpha"]
    ARGB8888,
    #[doc = "32-bit pixels without alpha (unpacked 24-bit format)"]
    RGB888,
    #[doc = "16-bit pixels with alpha"]
    ARGB1555,
    #[doc = "16-bit pixels with alpha"]
    ARGB4444,
    #[doc = "16-bit pixels without alpha"]
    RGB555,
    #[doc = "16-bit pixels without alpha"]
    RGB444,
    #[doc = "16-bit pixels without alpha"]
    RGB565,
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
            FORMATR::ARGB1555 => 8,
            FORMATR::ARGB4444 => 9,
            FORMATR::RGB555 => 12,
            FORMATR::RGB444 => 13,
            FORMATR::RGB565 => 14,
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
            8 => FORMATR::ARGB1555,
            9 => FORMATR::ARGB4444,
            12 => FORMATR::RGB555,
            13 => FORMATR::RGB444,
            14 => FORMATR::RGB565,
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
#[doc = "Possible values of the field `ROP`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ROPR {
    #[doc = "AS AND PS"]
    MASKAS,
    #[doc = "nAS AND PS"]
    MASKNOTAS,
    #[doc = "AS AND nPS"]
    MASKASNOT,
    #[doc = "AS OR PS"]
    MERGEAS,
    #[doc = "nAS OR PS"]
    MERGENOTAS,
    #[doc = "AS OR nPS"]
    MERGEASNOT,
    #[doc = "nAS"]
    NOTCOPYAS,
    #[doc = "nPS"]
    NOT,
    #[doc = "AS NAND PS"]
    NOTMASKAS,
    #[doc = "AS NOR PS"]
    NOTMERGEAS,
    #[doc = "AS XOR PS"]
    XORAS,
    #[doc = "AS XNOR PS"]
    NOTXORAS,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl ROPR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            ROPR::MASKAS => 0,
            ROPR::MASKNOTAS => 1,
            ROPR::MASKASNOT => 2,
            ROPR::MERGEAS => 3,
            ROPR::MERGENOTAS => 4,
            ROPR::MERGEASNOT => 5,
            ROPR::NOTCOPYAS => 6,
            ROPR::NOT => 7,
            ROPR::NOTMASKAS => 8,
            ROPR::NOTMERGEAS => 9,
            ROPR::XORAS => 10,
            ROPR::NOTXORAS => 11,
            ROPR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> ROPR {
        match value {
            0 => ROPR::MASKAS,
            1 => ROPR::MASKNOTAS,
            2 => ROPR::MASKASNOT,
            3 => ROPR::MERGEAS,
            4 => ROPR::MERGENOTAS,
            5 => ROPR::MERGEASNOT,
            6 => ROPR::NOTCOPYAS,
            7 => ROPR::NOT,
            8 => ROPR::NOTMASKAS,
            9 => ROPR::NOTMERGEAS,
            10 => ROPR::XORAS,
            11 => ROPR::NOTXORAS,
            i => ROPR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `MASKAS`"]
    #[inline]
    pub fn is_maskas(&self) -> bool {
        *self == ROPR::MASKAS
    }
    #[doc = "Checks if the value of the field is `MASKNOTAS`"]
    #[inline]
    pub fn is_masknotas(&self) -> bool {
        *self == ROPR::MASKNOTAS
    }
    #[doc = "Checks if the value of the field is `MASKASNOT`"]
    #[inline]
    pub fn is_maskasnot(&self) -> bool {
        *self == ROPR::MASKASNOT
    }
    #[doc = "Checks if the value of the field is `MERGEAS`"]
    #[inline]
    pub fn is_mergeas(&self) -> bool {
        *self == ROPR::MERGEAS
    }
    #[doc = "Checks if the value of the field is `MERGENOTAS`"]
    #[inline]
    pub fn is_mergenotas(&self) -> bool {
        *self == ROPR::MERGENOTAS
    }
    #[doc = "Checks if the value of the field is `MERGEASNOT`"]
    #[inline]
    pub fn is_mergeasnot(&self) -> bool {
        *self == ROPR::MERGEASNOT
    }
    #[doc = "Checks if the value of the field is `NOTCOPYAS`"]
    #[inline]
    pub fn is_notcopyas(&self) -> bool {
        *self == ROPR::NOTCOPYAS
    }
    #[doc = "Checks if the value of the field is `NOT`"]
    #[inline]
    pub fn is_not(&self) -> bool {
        *self == ROPR::NOT
    }
    #[doc = "Checks if the value of the field is `NOTMASKAS`"]
    #[inline]
    pub fn is_notmaskas(&self) -> bool {
        *self == ROPR::NOTMASKAS
    }
    #[doc = "Checks if the value of the field is `NOTMERGEAS`"]
    #[inline]
    pub fn is_notmergeas(&self) -> bool {
        *self == ROPR::NOTMERGEAS
    }
    #[doc = "Checks if the value of the field is `XORAS`"]
    #[inline]
    pub fn is_xoras(&self) -> bool {
        *self == ROPR::XORAS
    }
    #[doc = "Checks if the value of the field is `NOTXORAS`"]
    #[inline]
    pub fn is_notxoras(&self) -> bool {
        *self == ROPR::NOTXORAS
    }
}
#[doc = r" Value of the field"]
pub struct ALPHA_INVERTR {
    bits: bool,
}
impl ALPHA_INVERTR {
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
#[doc = "Values that can be written to the field `ALPHA_CTRL`"]
pub enum ALPHA_CTRLW {
    #[doc = "Indicates that the AS pixel alpha value will be used to blend the AS with PS. The ALPHA field is ignored."]
    EMBEDDED,
    #[doc = "Indicates that the value in the ALPHA field should be used instead of the alpha values present in the input pixels."]
    OVERRIDE,
    #[doc = "Indicates that the value in the ALPHA field should be used to scale all pixel alpha values. Each pixel alpha is multiplied by the value in the ALPHA field."]
    MULTIPLY,
    #[doc = "Enable ROPs. The ROP field indicates an operation to be performed on the alpha surface and PS pixels."]
    ROPS,
}
impl ALPHA_CTRLW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            ALPHA_CTRLW::EMBEDDED => 0,
            ALPHA_CTRLW::OVERRIDE => 1,
            ALPHA_CTRLW::MULTIPLY => 2,
            ALPHA_CTRLW::ROPS => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ALPHA_CTRLW<'a> {
    w: &'a mut W,
}
impl<'a> _ALPHA_CTRLW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ALPHA_CTRLW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Indicates that the AS pixel alpha value will be used to blend the AS with PS. The ALPHA field is ignored."]
    #[inline]
    pub fn embedded(self) -> &'a mut W {
        self.variant(ALPHA_CTRLW::EMBEDDED)
    }
    #[doc = "Indicates that the value in the ALPHA field should be used instead of the alpha values present in the input pixels."]
    #[inline]
    pub fn override_(self) -> &'a mut W {
        self.variant(ALPHA_CTRLW::OVERRIDE)
    }
    #[doc = "Indicates that the value in the ALPHA field should be used to scale all pixel alpha values. Each pixel alpha is multiplied by the value in the ALPHA field."]
    #[inline]
    pub fn multiply(self) -> &'a mut W {
        self.variant(ALPHA_CTRLW::MULTIPLY)
    }
    #[doc = "Enable ROPs. The ROP field indicates an operation to be performed on the alpha surface and PS pixels."]
    #[inline]
    pub fn rops(self) -> &'a mut W {
        self.variant(ALPHA_CTRLW::ROPS)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 1;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _ENABLE_COLORKEYW<'a> {
    w: &'a mut W,
}
impl<'a> _ENABLE_COLORKEYW<'a> {
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
#[doc = "Values that can be written to the field `FORMAT`"]
pub enum FORMATW {
    #[doc = "32-bit pixels with alpha"]
    ARGB8888,
    #[doc = "32-bit pixels without alpha (unpacked 24-bit format)"]
    RGB888,
    #[doc = "16-bit pixels with alpha"]
    ARGB1555,
    #[doc = "16-bit pixels with alpha"]
    ARGB4444,
    #[doc = "16-bit pixels without alpha"]
    RGB555,
    #[doc = "16-bit pixels without alpha"]
    RGB444,
    #[doc = "16-bit pixels without alpha"]
    RGB565,
}
impl FORMATW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            FORMATW::ARGB8888 => 0,
            FORMATW::RGB888 => 4,
            FORMATW::ARGB1555 => 8,
            FORMATW::ARGB4444 => 9,
            FORMATW::RGB555 => 12,
            FORMATW::RGB444 => 13,
            FORMATW::RGB565 => 14,
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
    #[doc = "32-bit pixels with alpha"]
    #[inline]
    pub fn argb8888(self) -> &'a mut W {
        self.variant(FORMATW::ARGB8888)
    }
    #[doc = "32-bit pixels without alpha (unpacked 24-bit format)"]
    #[inline]
    pub fn rgb888(self) -> &'a mut W {
        self.variant(FORMATW::RGB888)
    }
    #[doc = "16-bit pixels with alpha"]
    #[inline]
    pub fn argb1555(self) -> &'a mut W {
        self.variant(FORMATW::ARGB1555)
    }
    #[doc = "16-bit pixels with alpha"]
    #[inline]
    pub fn argb4444(self) -> &'a mut W {
        self.variant(FORMATW::ARGB4444)
    }
    #[doc = "16-bit pixels without alpha"]
    #[inline]
    pub fn rgb555(self) -> &'a mut W {
        self.variant(FORMATW::RGB555)
    }
    #[doc = "16-bit pixels without alpha"]
    #[inline]
    pub fn rgb444(self) -> &'a mut W {
        self.variant(FORMATW::RGB444)
    }
    #[doc = "16-bit pixels without alpha"]
    #[inline]
    pub fn rgb565(self) -> &'a mut W {
        self.variant(FORMATW::RGB565)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 4;
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
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `ROP`"]
pub enum ROPW {
    #[doc = "AS AND PS"]
    MASKAS,
    #[doc = "nAS AND PS"]
    MASKNOTAS,
    #[doc = "AS AND nPS"]
    MASKASNOT,
    #[doc = "AS OR PS"]
    MERGEAS,
    #[doc = "nAS OR PS"]
    MERGENOTAS,
    #[doc = "AS OR nPS"]
    MERGEASNOT,
    #[doc = "nAS"]
    NOTCOPYAS,
    #[doc = "nPS"]
    NOT,
    #[doc = "AS NAND PS"]
    NOTMASKAS,
    #[doc = "AS NOR PS"]
    NOTMERGEAS,
    #[doc = "AS XOR PS"]
    XORAS,
    #[doc = "AS XNOR PS"]
    NOTXORAS,
}
impl ROPW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            ROPW::MASKAS => 0,
            ROPW::MASKNOTAS => 1,
            ROPW::MASKASNOT => 2,
            ROPW::MERGEAS => 3,
            ROPW::MERGENOTAS => 4,
            ROPW::MERGEASNOT => 5,
            ROPW::NOTCOPYAS => 6,
            ROPW::NOT => 7,
            ROPW::NOTMASKAS => 8,
            ROPW::NOTMERGEAS => 9,
            ROPW::XORAS => 10,
            ROPW::NOTXORAS => 11,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ROPW<'a> {
    w: &'a mut W,
}
impl<'a> _ROPW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ROPW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "AS AND PS"]
    #[inline]
    pub fn maskas(self) -> &'a mut W {
        self.variant(ROPW::MASKAS)
    }
    #[doc = "nAS AND PS"]
    #[inline]
    pub fn masknotas(self) -> &'a mut W {
        self.variant(ROPW::MASKNOTAS)
    }
    #[doc = "AS AND nPS"]
    #[inline]
    pub fn maskasnot(self) -> &'a mut W {
        self.variant(ROPW::MASKASNOT)
    }
    #[doc = "AS OR PS"]
    #[inline]
    pub fn mergeas(self) -> &'a mut W {
        self.variant(ROPW::MERGEAS)
    }
    #[doc = "nAS OR PS"]
    #[inline]
    pub fn mergenotas(self) -> &'a mut W {
        self.variant(ROPW::MERGENOTAS)
    }
    #[doc = "AS OR nPS"]
    #[inline]
    pub fn mergeasnot(self) -> &'a mut W {
        self.variant(ROPW::MERGEASNOT)
    }
    #[doc = "nAS"]
    #[inline]
    pub fn notcopyas(self) -> &'a mut W {
        self.variant(ROPW::NOTCOPYAS)
    }
    #[doc = "nPS"]
    #[inline]
    pub fn not(self) -> &'a mut W {
        self.variant(ROPW::NOT)
    }
    #[doc = "AS NAND PS"]
    #[inline]
    pub fn notmaskas(self) -> &'a mut W {
        self.variant(ROPW::NOTMASKAS)
    }
    #[doc = "AS NOR PS"]
    #[inline]
    pub fn notmergeas(self) -> &'a mut W {
        self.variant(ROPW::NOTMERGEAS)
    }
    #[doc = "AS XOR PS"]
    #[inline]
    pub fn xoras(self) -> &'a mut W {
        self.variant(ROPW::XORAS)
    }
    #[doc = "AS XNOR PS"]
    #[inline]
    pub fn notxoras(self) -> &'a mut W {
        self.variant(ROPW::NOTXORAS)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 16;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _ALPHA_INVERTW<'a> {
    w: &'a mut W,
}
impl<'a> _ALPHA_INVERTW<'a> {
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
        const OFFSET: u8 = 20;
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
    #[doc = "Bit 0 - Reserved, always set to zero."]
    #[inline]
    pub fn rsvd0(&self) -> RSVD0R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        RSVD0R { bits }
    }
    #[doc = "Bits 1:2 - Determines how the alpha value is constructed for this alpha surface"]
    #[inline]
    pub fn alpha_ctrl(&self) -> ALPHA_CTRLR {
        ALPHA_CTRLR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 3 - Indicates that colorkey functionality is enabled for this alpha surface"]
    #[inline]
    pub fn enable_colorkey(&self) -> ENABLE_COLORKEYR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        ENABLE_COLORKEYR { bits }
    }
    #[doc = "Bits 4:7 - Indicates the input buffer format for AS."]
    #[inline]
    pub fn format(&self) -> FORMATR {
        FORMATR::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 8:15 - Alpha modifier used when the ALPHA_MULTIPLY or ALPHA_OVERRIDE values are programmed in PXP_AS_CTRL[ALPHA_CTRL]"]
    #[inline]
    pub fn alpha(&self) -> ALPHAR {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        ALPHAR { bits }
    }
    #[doc = "Bits 16:19 - Indicates a raster operation to perform when enabled"]
    #[inline]
    pub fn rop(&self) -> ROPR {
        ROPR::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 20 - Setting this bit to logic 0 will not alter the alpha value"]
    #[inline]
    pub fn alpha_invert(&self) -> ALPHA_INVERTR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        ALPHA_INVERTR { bits }
    }
    #[doc = "Bits 21:31 - Reserved, always set to zero."]
    #[inline]
    pub fn rsvd1(&self) -> RSVD1R {
        let bits = {
            const MASK: u16 = 2047;
            const OFFSET: u8 = 21;
            ((self.bits >> OFFSET) & MASK as u32) as u16
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
    #[doc = "Bits 1:2 - Determines how the alpha value is constructed for this alpha surface"]
    #[inline]
    pub fn alpha_ctrl(&mut self) -> _ALPHA_CTRLW {
        _ALPHA_CTRLW { w: self }
    }
    #[doc = "Bit 3 - Indicates that colorkey functionality is enabled for this alpha surface"]
    #[inline]
    pub fn enable_colorkey(&mut self) -> _ENABLE_COLORKEYW {
        _ENABLE_COLORKEYW { w: self }
    }
    #[doc = "Bits 4:7 - Indicates the input buffer format for AS."]
    #[inline]
    pub fn format(&mut self) -> _FORMATW {
        _FORMATW { w: self }
    }
    #[doc = "Bits 8:15 - Alpha modifier used when the ALPHA_MULTIPLY or ALPHA_OVERRIDE values are programmed in PXP_AS_CTRL[ALPHA_CTRL]"]
    #[inline]
    pub fn alpha(&mut self) -> _ALPHAW {
        _ALPHAW { w: self }
    }
    #[doc = "Bits 16:19 - Indicates a raster operation to perform when enabled"]
    #[inline]
    pub fn rop(&mut self) -> _ROPW {
        _ROPW { w: self }
    }
    #[doc = "Bit 20 - Setting this bit to logic 0 will not alter the alpha value"]
    #[inline]
    pub fn alpha_invert(&mut self) -> _ALPHA_INVERTW {
        _ALPHA_INVERTW { w: self }
    }
}
