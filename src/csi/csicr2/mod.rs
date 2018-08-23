#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CSICR2 {
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
pub struct HSCR {
    bits: u8,
}
impl HSCR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct VSCR {
    bits: u8,
}
impl VSCR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Possible values of the field `LVRM`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LVRMR {
    #[doc = "512 x 384"]
    LVRM_0,
    #[doc = "448 x 336"]
    LVRM_1,
    #[doc = "384 x 288"]
    LVRM_2,
    #[doc = "384 x 256"]
    LVRM_3,
    #[doc = "320 x 240"]
    LVRM_4,
    #[doc = "288 x 216"]
    LVRM_5,
    #[doc = "400 x 300"]
    LVRM_6,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl LVRMR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            LVRMR::LVRM_0 => 0,
            LVRMR::LVRM_1 => 1,
            LVRMR::LVRM_2 => 2,
            LVRMR::LVRM_3 => 3,
            LVRMR::LVRM_4 => 4,
            LVRMR::LVRM_5 => 5,
            LVRMR::LVRM_6 => 6,
            LVRMR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> LVRMR {
        match value {
            0 => LVRMR::LVRM_0,
            1 => LVRMR::LVRM_1,
            2 => LVRMR::LVRM_2,
            3 => LVRMR::LVRM_3,
            4 => LVRMR::LVRM_4,
            5 => LVRMR::LVRM_5,
            6 => LVRMR::LVRM_6,
            i => LVRMR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `LVRM_0`"]
    #[inline]
    pub fn is_lvrm_0(&self) -> bool {
        *self == LVRMR::LVRM_0
    }
    #[doc = "Checks if the value of the field is `LVRM_1`"]
    #[inline]
    pub fn is_lvrm_1(&self) -> bool {
        *self == LVRMR::LVRM_1
    }
    #[doc = "Checks if the value of the field is `LVRM_2`"]
    #[inline]
    pub fn is_lvrm_2(&self) -> bool {
        *self == LVRMR::LVRM_2
    }
    #[doc = "Checks if the value of the field is `LVRM_3`"]
    #[inline]
    pub fn is_lvrm_3(&self) -> bool {
        *self == LVRMR::LVRM_3
    }
    #[doc = "Checks if the value of the field is `LVRM_4`"]
    #[inline]
    pub fn is_lvrm_4(&self) -> bool {
        *self == LVRMR::LVRM_4
    }
    #[doc = "Checks if the value of the field is `LVRM_5`"]
    #[inline]
    pub fn is_lvrm_5(&self) -> bool {
        *self == LVRMR::LVRM_5
    }
    #[doc = "Checks if the value of the field is `LVRM_6`"]
    #[inline]
    pub fn is_lvrm_6(&self) -> bool {
        *self == LVRMR::LVRM_6
    }
}
#[doc = "Possible values of the field `BTS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BTSR {
    #[doc = "GR"]
    BTS_0,
    #[doc = "RG"]
    BTS_1,
    #[doc = "BG"]
    BTS_2,
    #[doc = "GB"]
    BTS_3,
}
impl BTSR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            BTSR::BTS_0 => 0,
            BTSR::BTS_1 => 1,
            BTSR::BTS_2 => 2,
            BTSR::BTS_3 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> BTSR {
        match value {
            0 => BTSR::BTS_0,
            1 => BTSR::BTS_1,
            2 => BTSR::BTS_2,
            3 => BTSR::BTS_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `BTS_0`"]
    #[inline]
    pub fn is_bts_0(&self) -> bool {
        *self == BTSR::BTS_0
    }
    #[doc = "Checks if the value of the field is `BTS_1`"]
    #[inline]
    pub fn is_bts_1(&self) -> bool {
        *self == BTSR::BTS_1
    }
    #[doc = "Checks if the value of the field is `BTS_2`"]
    #[inline]
    pub fn is_bts_2(&self) -> bool {
        *self == BTSR::BTS_2
    }
    #[doc = "Checks if the value of the field is `BTS_3`"]
    #[inline]
    pub fn is_bts_3(&self) -> bool {
        *self == BTSR::BTS_3
    }
}
#[doc = "Possible values of the field `SCE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SCER {
    #[doc = "Skip count disable"]
    SCE_0,
    #[doc = "Skip count enable"]
    SCE_1,
}
impl SCER {
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
            SCER::SCE_0 => false,
            SCER::SCE_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SCER {
        match value {
            false => SCER::SCE_0,
            true => SCER::SCE_1,
        }
    }
    #[doc = "Checks if the value of the field is `SCE_0`"]
    #[inline]
    pub fn is_sce_0(&self) -> bool {
        *self == SCER::SCE_0
    }
    #[doc = "Checks if the value of the field is `SCE_1`"]
    #[inline]
    pub fn is_sce_1(&self) -> bool {
        *self == SCER::SCE_1
    }
}
#[doc = "Possible values of the field `AFS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AFSR {
    #[doc = "Abs Diff on consecutive green pixels"]
    AFS_0,
    #[doc = "Abs Diff on every third green pixels"]
    AFS_1,
    #[doc = "Abs Diff on every four green pixels"]
    AFS_2,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl AFSR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            AFSR::AFS_0 => 0,
            AFSR::AFS_1 => 1,
            AFSR::AFS_2 => 2,
            AFSR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> AFSR {
        match value {
            0 => AFSR::AFS_0,
            1 => AFSR::AFS_1,
            2 => AFSR::AFS_2,
            i => AFSR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `AFS_0`"]
    #[inline]
    pub fn is_afs_0(&self) -> bool {
        *self == AFSR::AFS_0
    }
    #[doc = "Checks if the value of the field is `AFS_1`"]
    #[inline]
    pub fn is_afs_1(&self) -> bool {
        *self == AFSR::AFS_1
    }
    #[doc = "Checks if the value of the field is `AFS_2`"]
    #[inline]
    pub fn is_afs_2(&self) -> bool {
        *self == AFSR::AFS_2
    }
}
#[doc = "Possible values of the field `DRM`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DRMR {
    #[doc = "Stats grid of 8 x 6"]
    DRM_0,
    #[doc = "Stats grid of 8 x 12"]
    DRM_1,
}
impl DRMR {
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
            DRMR::DRM_0 => false,
            DRMR::DRM_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DRMR {
        match value {
            false => DRMR::DRM_0,
            true => DRMR::DRM_1,
        }
    }
    #[doc = "Checks if the value of the field is `DRM_0`"]
    #[inline]
    pub fn is_drm_0(&self) -> bool {
        *self == DRMR::DRM_0
    }
    #[doc = "Checks if the value of the field is `DRM_1`"]
    #[inline]
    pub fn is_drm_1(&self) -> bool {
        *self == DRMR::DRM_1
    }
}
#[doc = "Possible values of the field `DMA_BURST_TYPE_SFF`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DMA_BURST_TYPE_SFFR {
    #[doc = "INCR8"]
    DMA_BURST_TYPE_SFF_0,
    #[doc = "INCR4"]
    DMA_BURST_TYPE_SFF_1,
    #[doc = "INCR16"]
    DMA_BURST_TYPE_SFF_3,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl DMA_BURST_TYPE_SFFR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            DMA_BURST_TYPE_SFFR::DMA_BURST_TYPE_SFF_0 => 0,
            DMA_BURST_TYPE_SFFR::DMA_BURST_TYPE_SFF_1 => 1,
            DMA_BURST_TYPE_SFFR::DMA_BURST_TYPE_SFF_3 => 3,
            DMA_BURST_TYPE_SFFR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> DMA_BURST_TYPE_SFFR {
        match value {
            0 => DMA_BURST_TYPE_SFFR::DMA_BURST_TYPE_SFF_0,
            1 => DMA_BURST_TYPE_SFFR::DMA_BURST_TYPE_SFF_1,
            3 => DMA_BURST_TYPE_SFFR::DMA_BURST_TYPE_SFF_3,
            i => DMA_BURST_TYPE_SFFR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `DMA_BURST_TYPE_SFF_0`"]
    #[inline]
    pub fn is_dma_burst_type_sff_0(&self) -> bool {
        *self == DMA_BURST_TYPE_SFFR::DMA_BURST_TYPE_SFF_0
    }
    #[doc = "Checks if the value of the field is `DMA_BURST_TYPE_SFF_1`"]
    #[inline]
    pub fn is_dma_burst_type_sff_1(&self) -> bool {
        *self == DMA_BURST_TYPE_SFFR::DMA_BURST_TYPE_SFF_1
    }
    #[doc = "Checks if the value of the field is `DMA_BURST_TYPE_SFF_3`"]
    #[inline]
    pub fn is_dma_burst_type_sff_3(&self) -> bool {
        *self == DMA_BURST_TYPE_SFFR::DMA_BURST_TYPE_SFF_3
    }
}
#[doc = "Possible values of the field `DMA_BURST_TYPE_RFF`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DMA_BURST_TYPE_RFFR {
    #[doc = "INCR8"]
    DMA_BURST_TYPE_RFF_0,
    #[doc = "INCR4"]
    DMA_BURST_TYPE_RFF_1,
    #[doc = "INCR16"]
    DMA_BURST_TYPE_RFF_3,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl DMA_BURST_TYPE_RFFR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            DMA_BURST_TYPE_RFFR::DMA_BURST_TYPE_RFF_0 => 0,
            DMA_BURST_TYPE_RFFR::DMA_BURST_TYPE_RFF_1 => 1,
            DMA_BURST_TYPE_RFFR::DMA_BURST_TYPE_RFF_3 => 3,
            DMA_BURST_TYPE_RFFR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> DMA_BURST_TYPE_RFFR {
        match value {
            0 => DMA_BURST_TYPE_RFFR::DMA_BURST_TYPE_RFF_0,
            1 => DMA_BURST_TYPE_RFFR::DMA_BURST_TYPE_RFF_1,
            3 => DMA_BURST_TYPE_RFFR::DMA_BURST_TYPE_RFF_3,
            i => DMA_BURST_TYPE_RFFR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `DMA_BURST_TYPE_RFF_0`"]
    #[inline]
    pub fn is_dma_burst_type_rff_0(&self) -> bool {
        *self == DMA_BURST_TYPE_RFFR::DMA_BURST_TYPE_RFF_0
    }
    #[doc = "Checks if the value of the field is `DMA_BURST_TYPE_RFF_1`"]
    #[inline]
    pub fn is_dma_burst_type_rff_1(&self) -> bool {
        *self == DMA_BURST_TYPE_RFFR::DMA_BURST_TYPE_RFF_1
    }
    #[doc = "Checks if the value of the field is `DMA_BURST_TYPE_RFF_3`"]
    #[inline]
    pub fn is_dma_burst_type_rff_3(&self) -> bool {
        *self == DMA_BURST_TYPE_RFFR::DMA_BURST_TYPE_RFF_3
    }
}
#[doc = r" Proxy"]
pub struct _HSCW<'a> {
    w: &'a mut W,
}
impl<'a> _HSCW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 255;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _VSCW<'a> {
    w: &'a mut W,
}
impl<'a> _VSCW<'a> {
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
#[doc = "Values that can be written to the field `LVRM`"]
pub enum LVRMW {
    #[doc = "512 x 384"]
    LVRM_0,
    #[doc = "448 x 336"]
    LVRM_1,
    #[doc = "384 x 288"]
    LVRM_2,
    #[doc = "384 x 256"]
    LVRM_3,
    #[doc = "320 x 240"]
    LVRM_4,
    #[doc = "288 x 216"]
    LVRM_5,
    #[doc = "400 x 300"]
    LVRM_6,
}
impl LVRMW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            LVRMW::LVRM_0 => 0,
            LVRMW::LVRM_1 => 1,
            LVRMW::LVRM_2 => 2,
            LVRMW::LVRM_3 => 3,
            LVRMW::LVRM_4 => 4,
            LVRMW::LVRM_5 => 5,
            LVRMW::LVRM_6 => 6,
        }
    }
}
#[doc = r" Proxy"]
pub struct _LVRMW<'a> {
    w: &'a mut W,
}
impl<'a> _LVRMW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: LVRMW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "512 x 384"]
    #[inline]
    pub fn lvrm_0(self) -> &'a mut W {
        self.variant(LVRMW::LVRM_0)
    }
    #[doc = "448 x 336"]
    #[inline]
    pub fn lvrm_1(self) -> &'a mut W {
        self.variant(LVRMW::LVRM_1)
    }
    #[doc = "384 x 288"]
    #[inline]
    pub fn lvrm_2(self) -> &'a mut W {
        self.variant(LVRMW::LVRM_2)
    }
    #[doc = "384 x 256"]
    #[inline]
    pub fn lvrm_3(self) -> &'a mut W {
        self.variant(LVRMW::LVRM_3)
    }
    #[doc = "320 x 240"]
    #[inline]
    pub fn lvrm_4(self) -> &'a mut W {
        self.variant(LVRMW::LVRM_4)
    }
    #[doc = "288 x 216"]
    #[inline]
    pub fn lvrm_5(self) -> &'a mut W {
        self.variant(LVRMW::LVRM_5)
    }
    #[doc = "400 x 300"]
    #[inline]
    pub fn lvrm_6(self) -> &'a mut W {
        self.variant(LVRMW::LVRM_6)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 16;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `BTS`"]
pub enum BTSW {
    #[doc = "GR"]
    BTS_0,
    #[doc = "RG"]
    BTS_1,
    #[doc = "BG"]
    BTS_2,
    #[doc = "GB"]
    BTS_3,
}
impl BTSW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            BTSW::BTS_0 => 0,
            BTSW::BTS_1 => 1,
            BTSW::BTS_2 => 2,
            BTSW::BTS_3 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _BTSW<'a> {
    w: &'a mut W,
}
impl<'a> _BTSW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: BTSW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "GR"]
    #[inline]
    pub fn bts_0(self) -> &'a mut W {
        self.variant(BTSW::BTS_0)
    }
    #[doc = "RG"]
    #[inline]
    pub fn bts_1(self) -> &'a mut W {
        self.variant(BTSW::BTS_1)
    }
    #[doc = "BG"]
    #[inline]
    pub fn bts_2(self) -> &'a mut W {
        self.variant(BTSW::BTS_2)
    }
    #[doc = "GB"]
    #[inline]
    pub fn bts_3(self) -> &'a mut W {
        self.variant(BTSW::BTS_3)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 19;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `SCE`"]
pub enum SCEW {
    #[doc = "Skip count disable"]
    SCE_0,
    #[doc = "Skip count enable"]
    SCE_1,
}
impl SCEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SCEW::SCE_0 => false,
            SCEW::SCE_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SCEW<'a> {
    w: &'a mut W,
}
impl<'a> _SCEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SCEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Skip count disable"]
    #[inline]
    pub fn sce_0(self) -> &'a mut W {
        self.variant(SCEW::SCE_0)
    }
    #[doc = "Skip count enable"]
    #[inline]
    pub fn sce_1(self) -> &'a mut W {
        self.variant(SCEW::SCE_1)
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
        const OFFSET: u8 = 23;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `AFS`"]
pub enum AFSW {
    #[doc = "Abs Diff on consecutive green pixels"]
    AFS_0,
    #[doc = "Abs Diff on every third green pixels"]
    AFS_1,
    #[doc = "Abs Diff on every four green pixels"]
    AFS_2,
}
impl AFSW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            AFSW::AFS_0 => 0,
            AFSW::AFS_1 => 1,
            AFSW::AFS_2 => 2,
        }
    }
}
#[doc = r" Proxy"]
pub struct _AFSW<'a> {
    w: &'a mut W,
}
impl<'a> _AFSW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: AFSW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Abs Diff on consecutive green pixels"]
    #[inline]
    pub fn afs_0(self) -> &'a mut W {
        self.variant(AFSW::AFS_0)
    }
    #[doc = "Abs Diff on every third green pixels"]
    #[inline]
    pub fn afs_1(self) -> &'a mut W {
        self.variant(AFSW::AFS_1)
    }
    #[doc = "Abs Diff on every four green pixels"]
    #[inline]
    pub fn afs_2(self) -> &'a mut W {
        self.variant(AFSW::AFS_2)
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
#[doc = "Values that can be written to the field `DRM`"]
pub enum DRMW {
    #[doc = "Stats grid of 8 x 6"]
    DRM_0,
    #[doc = "Stats grid of 8 x 12"]
    DRM_1,
}
impl DRMW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DRMW::DRM_0 => false,
            DRMW::DRM_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DRMW<'a> {
    w: &'a mut W,
}
impl<'a> _DRMW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DRMW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Stats grid of 8 x 6"]
    #[inline]
    pub fn drm_0(self) -> &'a mut W {
        self.variant(DRMW::DRM_0)
    }
    #[doc = "Stats grid of 8 x 12"]
    #[inline]
    pub fn drm_1(self) -> &'a mut W {
        self.variant(DRMW::DRM_1)
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
        const OFFSET: u8 = 26;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `DMA_BURST_TYPE_SFF`"]
pub enum DMA_BURST_TYPE_SFFW {
    #[doc = "INCR8"]
    DMA_BURST_TYPE_SFF_0,
    #[doc = "INCR4"]
    DMA_BURST_TYPE_SFF_1,
    #[doc = "INCR16"]
    DMA_BURST_TYPE_SFF_3,
}
impl DMA_BURST_TYPE_SFFW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            DMA_BURST_TYPE_SFFW::DMA_BURST_TYPE_SFF_0 => 0,
            DMA_BURST_TYPE_SFFW::DMA_BURST_TYPE_SFF_1 => 1,
            DMA_BURST_TYPE_SFFW::DMA_BURST_TYPE_SFF_3 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DMA_BURST_TYPE_SFFW<'a> {
    w: &'a mut W,
}
impl<'a> _DMA_BURST_TYPE_SFFW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DMA_BURST_TYPE_SFFW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "INCR8"]
    #[inline]
    pub fn dma_burst_type_sff_0(self) -> &'a mut W {
        self.variant(DMA_BURST_TYPE_SFFW::DMA_BURST_TYPE_SFF_0)
    }
    #[doc = "INCR4"]
    #[inline]
    pub fn dma_burst_type_sff_1(self) -> &'a mut W {
        self.variant(DMA_BURST_TYPE_SFFW::DMA_BURST_TYPE_SFF_1)
    }
    #[doc = "INCR16"]
    #[inline]
    pub fn dma_burst_type_sff_3(self) -> &'a mut W {
        self.variant(DMA_BURST_TYPE_SFFW::DMA_BURST_TYPE_SFF_3)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 28;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `DMA_BURST_TYPE_RFF`"]
pub enum DMA_BURST_TYPE_RFFW {
    #[doc = "INCR8"]
    DMA_BURST_TYPE_RFF_0,
    #[doc = "INCR4"]
    DMA_BURST_TYPE_RFF_1,
    #[doc = "INCR16"]
    DMA_BURST_TYPE_RFF_3,
}
impl DMA_BURST_TYPE_RFFW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            DMA_BURST_TYPE_RFFW::DMA_BURST_TYPE_RFF_0 => 0,
            DMA_BURST_TYPE_RFFW::DMA_BURST_TYPE_RFF_1 => 1,
            DMA_BURST_TYPE_RFFW::DMA_BURST_TYPE_RFF_3 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DMA_BURST_TYPE_RFFW<'a> {
    w: &'a mut W,
}
impl<'a> _DMA_BURST_TYPE_RFFW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DMA_BURST_TYPE_RFFW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "INCR8"]
    #[inline]
    pub fn dma_burst_type_rff_0(self) -> &'a mut W {
        self.variant(DMA_BURST_TYPE_RFFW::DMA_BURST_TYPE_RFF_0)
    }
    #[doc = "INCR4"]
    #[inline]
    pub fn dma_burst_type_rff_1(self) -> &'a mut W {
        self.variant(DMA_BURST_TYPE_RFFW::DMA_BURST_TYPE_RFF_1)
    }
    #[doc = "INCR16"]
    #[inline]
    pub fn dma_burst_type_rff_3(self) -> &'a mut W {
        self.variant(DMA_BURST_TYPE_RFFW::DMA_BURST_TYPE_RFF_3)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 30;
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
    #[doc = "Bits 0:7 - Horizontal Skip Count"]
    #[inline]
    pub fn hsc(&self) -> HSCR {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        HSCR { bits }
    }
    #[doc = "Bits 8:15 - Vertical Skip Count. Contains the number of rows to skip. SCE must be 1, otherwise VSC is ignored."]
    #[inline]
    pub fn vsc(&self) -> VSCR {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        VSCR { bits }
    }
    #[doc = "Bits 16:18 - Live View Resolution Mode. Selects the grid size used for live view resolution."]
    #[inline]
    pub fn lvrm(&self) -> LVRMR {
        LVRMR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 19:20 - Bayer Tile Start. Controls the Bayer pattern starting point."]
    #[inline]
    pub fn bts(&self) -> BTSR {
        BTSR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 19;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 23 - Skip Count Enable. Enables or disables the skip count feature."]
    #[inline]
    pub fn sce(&self) -> SCER {
        SCER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 23;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 24:25 - Auto Focus Spread. Selects which green pixels are used for auto-focus."]
    #[inline]
    pub fn afs(&self) -> AFSR {
        AFSR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 26 - Double Resolution Mode. Controls size of statistics grid."]
    #[inline]
    pub fn drm(&self) -> DRMR {
        DRMR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 26;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 28:29 - Burst Type of DMA Transfer from STATFIFO. Selects the burst type of DMA transfer from STATFIFO."]
    #[inline]
    pub fn dma_burst_type_sff(&self) -> DMA_BURST_TYPE_SFFR {
        DMA_BURST_TYPE_SFFR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 28;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 30:31 - Burst Type of DMA Transfer from RxFIFO. Selects the burst type of DMA transfer from RxFIFO."]
    #[inline]
    pub fn dma_burst_type_rff(&self) -> DMA_BURST_TYPE_RFFR {
        DMA_BURST_TYPE_RFFR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 30;
            ((self.bits >> OFFSET) & MASK as u32) as u8
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
    #[doc = "Bits 0:7 - Horizontal Skip Count"]
    #[inline]
    pub fn hsc(&mut self) -> _HSCW {
        _HSCW { w: self }
    }
    #[doc = "Bits 8:15 - Vertical Skip Count. Contains the number of rows to skip. SCE must be 1, otherwise VSC is ignored."]
    #[inline]
    pub fn vsc(&mut self) -> _VSCW {
        _VSCW { w: self }
    }
    #[doc = "Bits 16:18 - Live View Resolution Mode. Selects the grid size used for live view resolution."]
    #[inline]
    pub fn lvrm(&mut self) -> _LVRMW {
        _LVRMW { w: self }
    }
    #[doc = "Bits 19:20 - Bayer Tile Start. Controls the Bayer pattern starting point."]
    #[inline]
    pub fn bts(&mut self) -> _BTSW {
        _BTSW { w: self }
    }
    #[doc = "Bit 23 - Skip Count Enable. Enables or disables the skip count feature."]
    #[inline]
    pub fn sce(&mut self) -> _SCEW {
        _SCEW { w: self }
    }
    #[doc = "Bits 24:25 - Auto Focus Spread. Selects which green pixels are used for auto-focus."]
    #[inline]
    pub fn afs(&mut self) -> _AFSW {
        _AFSW { w: self }
    }
    #[doc = "Bit 26 - Double Resolution Mode. Controls size of statistics grid."]
    #[inline]
    pub fn drm(&mut self) -> _DRMW {
        _DRMW { w: self }
    }
    #[doc = "Bits 28:29 - Burst Type of DMA Transfer from STATFIFO. Selects the burst type of DMA transfer from STATFIFO."]
    #[inline]
    pub fn dma_burst_type_sff(&mut self) -> _DMA_BURST_TYPE_SFFW {
        _DMA_BURST_TYPE_SFFW { w: self }
    }
    #[doc = "Bits 30:31 - Burst Type of DMA Transfer from RxFIFO. Selects the burst type of DMA transfer from RxFIFO."]
    #[inline]
    pub fn dma_burst_type_rff(&mut self) -> _DMA_BURST_TYPE_RFFW {
        _DMA_BURST_TYPE_RFFW { w: self }
    }
}
