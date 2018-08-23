#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CSICR18 {
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
#[doc = "Possible values of the field `DEINTERLACE_EN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DEINTERLACE_ENR {
    #[doc = "Deinterlace disabled"]
    DEINTERLACE_EN_0,
    #[doc = "Deinterlace enabled"]
    DEINTERLACE_EN_1,
}
impl DEINTERLACE_ENR {
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
            DEINTERLACE_ENR::DEINTERLACE_EN_0 => false,
            DEINTERLACE_ENR::DEINTERLACE_EN_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DEINTERLACE_ENR {
        match value {
            false => DEINTERLACE_ENR::DEINTERLACE_EN_0,
            true => DEINTERLACE_ENR::DEINTERLACE_EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `DEINTERLACE_EN_0`"]
    #[inline]
    pub fn is_deinterlace_en_0(&self) -> bool {
        *self == DEINTERLACE_ENR::DEINTERLACE_EN_0
    }
    #[doc = "Checks if the value of the field is `DEINTERLACE_EN_1`"]
    #[inline]
    pub fn is_deinterlace_en_1(&self) -> bool {
        *self == DEINTERLACE_ENR::DEINTERLACE_EN_1
    }
}
#[doc = r" Value of the field"]
pub struct PARALLEL24_ENR {
    bits: bool,
}
impl PARALLEL24_ENR {
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
pub struct BASEADDR_SWITCH_ENR {
    bits: bool,
}
impl BASEADDR_SWITCH_ENR {
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
#[doc = "Possible values of the field `BASEADDR_SWITCH_SEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BASEADDR_SWITCH_SELR {
    #[doc = "Switching base address at the edge of the vsync"]
    BASEADDR_SWITCH_SEL_0,
    #[doc = "Switching base address at the edge of the first data of each frame"]
    BASEADDR_SWITCH_SEL_1,
}
impl BASEADDR_SWITCH_SELR {
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
            BASEADDR_SWITCH_SELR::BASEADDR_SWITCH_SEL_0 => false,
            BASEADDR_SWITCH_SELR::BASEADDR_SWITCH_SEL_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> BASEADDR_SWITCH_SELR {
        match value {
            false => BASEADDR_SWITCH_SELR::BASEADDR_SWITCH_SEL_0,
            true => BASEADDR_SWITCH_SELR::BASEADDR_SWITCH_SEL_1,
        }
    }
    #[doc = "Checks if the value of the field is `BASEADDR_SWITCH_SEL_0`"]
    #[inline]
    pub fn is_baseaddr_switch_sel_0(&self) -> bool {
        *self == BASEADDR_SWITCH_SELR::BASEADDR_SWITCH_SEL_0
    }
    #[doc = "Checks if the value of the field is `BASEADDR_SWITCH_SEL_1`"]
    #[inline]
    pub fn is_baseaddr_switch_sel_1(&self) -> bool {
        *self == BASEADDR_SWITCH_SELR::BASEADDR_SWITCH_SEL_1
    }
}
#[doc = "Possible values of the field `FIELD0_DONE_IE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FIELD0_DONE_IER {
    #[doc = "Interrupt disabled"]
    FIELD0_DONE_IE_0,
    #[doc = "Interrupt enabled"]
    FIELD0_DONE_IE_1,
}
impl FIELD0_DONE_IER {
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
            FIELD0_DONE_IER::FIELD0_DONE_IE_0 => false,
            FIELD0_DONE_IER::FIELD0_DONE_IE_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> FIELD0_DONE_IER {
        match value {
            false => FIELD0_DONE_IER::FIELD0_DONE_IE_0,
            true => FIELD0_DONE_IER::FIELD0_DONE_IE_1,
        }
    }
    #[doc = "Checks if the value of the field is `FIELD0_DONE_IE_0`"]
    #[inline]
    pub fn is_field0_done_ie_0(&self) -> bool {
        *self == FIELD0_DONE_IER::FIELD0_DONE_IE_0
    }
    #[doc = "Checks if the value of the field is `FIELD0_DONE_IE_1`"]
    #[inline]
    pub fn is_field0_done_ie_1(&self) -> bool {
        *self == FIELD0_DONE_IER::FIELD0_DONE_IE_1
    }
}
#[doc = "Possible values of the field `DMA_FIELD1_DONE_IE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DMA_FIELD1_DONE_IER {
    #[doc = "Interrupt disabled"]
    DMA_FIELD1_DONE_IE_0,
    #[doc = "Interrupt enabled"]
    DMA_FIELD1_DONE_IE_1,
}
impl DMA_FIELD1_DONE_IER {
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
            DMA_FIELD1_DONE_IER::DMA_FIELD1_DONE_IE_0 => false,
            DMA_FIELD1_DONE_IER::DMA_FIELD1_DONE_IE_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DMA_FIELD1_DONE_IER {
        match value {
            false => DMA_FIELD1_DONE_IER::DMA_FIELD1_DONE_IE_0,
            true => DMA_FIELD1_DONE_IER::DMA_FIELD1_DONE_IE_1,
        }
    }
    #[doc = "Checks if the value of the field is `DMA_FIELD1_DONE_IE_0`"]
    #[inline]
    pub fn is_dma_field1_done_ie_0(&self) -> bool {
        *self == DMA_FIELD1_DONE_IER::DMA_FIELD1_DONE_IE_0
    }
    #[doc = "Checks if the value of the field is `DMA_FIELD1_DONE_IE_1`"]
    #[inline]
    pub fn is_dma_field1_done_ie_1(&self) -> bool {
        *self == DMA_FIELD1_DONE_IER::DMA_FIELD1_DONE_IE_1
    }
}
#[doc = "Possible values of the field `LAST_DMA_REQ_SEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LAST_DMA_REQ_SELR {
    #[doc = "fifo_full_level"]
    LAST_DMA_REQ_SEL_0,
    #[doc = "hburst_length"]
    LAST_DMA_REQ_SEL_1,
}
impl LAST_DMA_REQ_SELR {
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
            LAST_DMA_REQ_SELR::LAST_DMA_REQ_SEL_0 => false,
            LAST_DMA_REQ_SELR::LAST_DMA_REQ_SEL_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> LAST_DMA_REQ_SELR {
        match value {
            false => LAST_DMA_REQ_SELR::LAST_DMA_REQ_SEL_0,
            true => LAST_DMA_REQ_SELR::LAST_DMA_REQ_SEL_1,
        }
    }
    #[doc = "Checks if the value of the field is `LAST_DMA_REQ_SEL_0`"]
    #[inline]
    pub fn is_last_dma_req_sel_0(&self) -> bool {
        *self == LAST_DMA_REQ_SELR::LAST_DMA_REQ_SEL_0
    }
    #[doc = "Checks if the value of the field is `LAST_DMA_REQ_SEL_1`"]
    #[inline]
    pub fn is_last_dma_req_sel_1(&self) -> bool {
        *self == LAST_DMA_REQ_SELR::LAST_DMA_REQ_SEL_1
    }
}
#[doc = r" Value of the field"]
pub struct BASEADDR_CHANGE_ERROR_IER {
    bits: bool,
}
impl BASEADDR_CHANGE_ERROR_IER {
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
#[doc = "Possible values of the field `RGB888A_FORMAT_SEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RGB888A_FORMAT_SELR {
    #[doc = "{8'h0, data[23:0]}"]
    RGB888A_FORMAT_SEL_0,
    #[doc = "{data[23:0], 8'h0}"]
    RGB888A_FORMAT_SEL_1,
}
impl RGB888A_FORMAT_SELR {
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
            RGB888A_FORMAT_SELR::RGB888A_FORMAT_SEL_0 => false,
            RGB888A_FORMAT_SELR::RGB888A_FORMAT_SEL_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RGB888A_FORMAT_SELR {
        match value {
            false => RGB888A_FORMAT_SELR::RGB888A_FORMAT_SEL_0,
            true => RGB888A_FORMAT_SELR::RGB888A_FORMAT_SEL_1,
        }
    }
    #[doc = "Checks if the value of the field is `RGB888A_FORMAT_SEL_0`"]
    #[inline]
    pub fn is_rgb888a_format_sel_0(&self) -> bool {
        *self == RGB888A_FORMAT_SELR::RGB888A_FORMAT_SEL_0
    }
    #[doc = "Checks if the value of the field is `RGB888A_FORMAT_SEL_1`"]
    #[inline]
    pub fn is_rgb888a_format_sel_1(&self) -> bool {
        *self == RGB888A_FORMAT_SELR::RGB888A_FORMAT_SEL_1
    }
}
#[doc = r" Value of the field"]
pub struct AHB_HPROTR {
    bits: u8,
}
impl AHB_HPROTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Possible values of the field `CSI_LCDIF_BUFFER_LINES`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CSI_LCDIF_BUFFER_LINESR {
    #[doc = "4 lines"]
    CSI_LCDIF_BUFFER_LINES_0,
    #[doc = "8 lines"]
    CSI_LCDIF_BUFFER_LINES_1,
    #[doc = "16 lines"]
    CSI_LCDIF_BUFFER_LINES_2,
    #[doc = "16 lines"]
    CSI_LCDIF_BUFFER_LINES_3,
}
impl CSI_LCDIF_BUFFER_LINESR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            CSI_LCDIF_BUFFER_LINESR::CSI_LCDIF_BUFFER_LINES_0 => 0,
            CSI_LCDIF_BUFFER_LINESR::CSI_LCDIF_BUFFER_LINES_1 => 1,
            CSI_LCDIF_BUFFER_LINESR::CSI_LCDIF_BUFFER_LINES_2 => 2,
            CSI_LCDIF_BUFFER_LINESR::CSI_LCDIF_BUFFER_LINES_3 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> CSI_LCDIF_BUFFER_LINESR {
        match value {
            0 => CSI_LCDIF_BUFFER_LINESR::CSI_LCDIF_BUFFER_LINES_0,
            1 => CSI_LCDIF_BUFFER_LINESR::CSI_LCDIF_BUFFER_LINES_1,
            2 => CSI_LCDIF_BUFFER_LINESR::CSI_LCDIF_BUFFER_LINES_2,
            3 => CSI_LCDIF_BUFFER_LINESR::CSI_LCDIF_BUFFER_LINES_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `CSI_LCDIF_BUFFER_LINES_0`"]
    #[inline]
    pub fn is_csi_lcdif_buffer_lines_0(&self) -> bool {
        *self == CSI_LCDIF_BUFFER_LINESR::CSI_LCDIF_BUFFER_LINES_0
    }
    #[doc = "Checks if the value of the field is `CSI_LCDIF_BUFFER_LINES_1`"]
    #[inline]
    pub fn is_csi_lcdif_buffer_lines_1(&self) -> bool {
        *self == CSI_LCDIF_BUFFER_LINESR::CSI_LCDIF_BUFFER_LINES_1
    }
    #[doc = "Checks if the value of the field is `CSI_LCDIF_BUFFER_LINES_2`"]
    #[inline]
    pub fn is_csi_lcdif_buffer_lines_2(&self) -> bool {
        *self == CSI_LCDIF_BUFFER_LINESR::CSI_LCDIF_BUFFER_LINES_2
    }
    #[doc = "Checks if the value of the field is `CSI_LCDIF_BUFFER_LINES_3`"]
    #[inline]
    pub fn is_csi_lcdif_buffer_lines_3(&self) -> bool {
        *self == CSI_LCDIF_BUFFER_LINESR::CSI_LCDIF_BUFFER_LINES_3
    }
}
#[doc = "Possible values of the field `MASK_OPTION`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MASK_OPTIONR {
    #[doc = "Writing to memory from first completely frame, when using this option, the CSI_ENABLE should be 1."]
    MASK_OPTION_0,
    #[doc = "Writing to memory when CSI_ENABLE is 1."]
    MASK_OPTION_1,
    #[doc = "Writing to memory from second completely frame, when using this option, the CSI_ENABLE should be 1."]
    MASK_OPTION_2,
    #[doc = "Writing to memory when data comes in, not matter the CSI_ENABLE is 1 or 0."]
    MASK_OPTION_3,
}
impl MASK_OPTIONR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            MASK_OPTIONR::MASK_OPTION_0 => 0,
            MASK_OPTIONR::MASK_OPTION_1 => 1,
            MASK_OPTIONR::MASK_OPTION_2 => 2,
            MASK_OPTIONR::MASK_OPTION_3 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> MASK_OPTIONR {
        match value {
            0 => MASK_OPTIONR::MASK_OPTION_0,
            1 => MASK_OPTIONR::MASK_OPTION_1,
            2 => MASK_OPTIONR::MASK_OPTION_2,
            3 => MASK_OPTIONR::MASK_OPTION_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `MASK_OPTION_0`"]
    #[inline]
    pub fn is_mask_option_0(&self) -> bool {
        *self == MASK_OPTIONR::MASK_OPTION_0
    }
    #[doc = "Checks if the value of the field is `MASK_OPTION_1`"]
    #[inline]
    pub fn is_mask_option_1(&self) -> bool {
        *self == MASK_OPTIONR::MASK_OPTION_1
    }
    #[doc = "Checks if the value of the field is `MASK_OPTION_2`"]
    #[inline]
    pub fn is_mask_option_2(&self) -> bool {
        *self == MASK_OPTIONR::MASK_OPTION_2
    }
    #[doc = "Checks if the value of the field is `MASK_OPTION_3`"]
    #[inline]
    pub fn is_mask_option_3(&self) -> bool {
        *self == MASK_OPTIONR::MASK_OPTION_3
    }
}
#[doc = r" Value of the field"]
pub struct CSI_ENABLER {
    bits: bool,
}
impl CSI_ENABLER {
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
#[doc = "Values that can be written to the field `DEINTERLACE_EN`"]
pub enum DEINTERLACE_ENW {
    #[doc = "Deinterlace disabled"]
    DEINTERLACE_EN_0,
    #[doc = "Deinterlace enabled"]
    DEINTERLACE_EN_1,
}
impl DEINTERLACE_ENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DEINTERLACE_ENW::DEINTERLACE_EN_0 => false,
            DEINTERLACE_ENW::DEINTERLACE_EN_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DEINTERLACE_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _DEINTERLACE_ENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DEINTERLACE_ENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Deinterlace disabled"]
    #[inline]
    pub fn deinterlace_en_0(self) -> &'a mut W {
        self.variant(DEINTERLACE_ENW::DEINTERLACE_EN_0)
    }
    #[doc = "Deinterlace enabled"]
    #[inline]
    pub fn deinterlace_en_1(self) -> &'a mut W {
        self.variant(DEINTERLACE_ENW::DEINTERLACE_EN_1)
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
#[doc = r" Proxy"]
pub struct _PARALLEL24_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _PARALLEL24_ENW<'a> {
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
pub struct _BASEADDR_SWITCH_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _BASEADDR_SWITCH_ENW<'a> {
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
#[doc = "Values that can be written to the field `BASEADDR_SWITCH_SEL`"]
pub enum BASEADDR_SWITCH_SELW {
    #[doc = "Switching base address at the edge of the vsync"]
    BASEADDR_SWITCH_SEL_0,
    #[doc = "Switching base address at the edge of the first data of each frame"]
    BASEADDR_SWITCH_SEL_1,
}
impl BASEADDR_SWITCH_SELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            BASEADDR_SWITCH_SELW::BASEADDR_SWITCH_SEL_0 => false,
            BASEADDR_SWITCH_SELW::BASEADDR_SWITCH_SEL_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _BASEADDR_SWITCH_SELW<'a> {
    w: &'a mut W,
}
impl<'a> _BASEADDR_SWITCH_SELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: BASEADDR_SWITCH_SELW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Switching base address at the edge of the vsync"]
    #[inline]
    pub fn baseaddr_switch_sel_0(self) -> &'a mut W {
        self.variant(BASEADDR_SWITCH_SELW::BASEADDR_SWITCH_SEL_0)
    }
    #[doc = "Switching base address at the edge of the first data of each frame"]
    #[inline]
    pub fn baseaddr_switch_sel_1(self) -> &'a mut W {
        self.variant(BASEADDR_SWITCH_SELW::BASEADDR_SWITCH_SEL_1)
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
        const OFFSET: u8 = 5;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `FIELD0_DONE_IE`"]
pub enum FIELD0_DONE_IEW {
    #[doc = "Interrupt disabled"]
    FIELD0_DONE_IE_0,
    #[doc = "Interrupt enabled"]
    FIELD0_DONE_IE_1,
}
impl FIELD0_DONE_IEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            FIELD0_DONE_IEW::FIELD0_DONE_IE_0 => false,
            FIELD0_DONE_IEW::FIELD0_DONE_IE_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FIELD0_DONE_IEW<'a> {
    w: &'a mut W,
}
impl<'a> _FIELD0_DONE_IEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FIELD0_DONE_IEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Interrupt disabled"]
    #[inline]
    pub fn field0_done_ie_0(self) -> &'a mut W {
        self.variant(FIELD0_DONE_IEW::FIELD0_DONE_IE_0)
    }
    #[doc = "Interrupt enabled"]
    #[inline]
    pub fn field0_done_ie_1(self) -> &'a mut W {
        self.variant(FIELD0_DONE_IEW::FIELD0_DONE_IE_1)
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
        const OFFSET: u8 = 6;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `DMA_FIELD1_DONE_IE`"]
pub enum DMA_FIELD1_DONE_IEW {
    #[doc = "Interrupt disabled"]
    DMA_FIELD1_DONE_IE_0,
    #[doc = "Interrupt enabled"]
    DMA_FIELD1_DONE_IE_1,
}
impl DMA_FIELD1_DONE_IEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DMA_FIELD1_DONE_IEW::DMA_FIELD1_DONE_IE_0 => false,
            DMA_FIELD1_DONE_IEW::DMA_FIELD1_DONE_IE_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DMA_FIELD1_DONE_IEW<'a> {
    w: &'a mut W,
}
impl<'a> _DMA_FIELD1_DONE_IEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DMA_FIELD1_DONE_IEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Interrupt disabled"]
    #[inline]
    pub fn dma_field1_done_ie_0(self) -> &'a mut W {
        self.variant(DMA_FIELD1_DONE_IEW::DMA_FIELD1_DONE_IE_0)
    }
    #[doc = "Interrupt enabled"]
    #[inline]
    pub fn dma_field1_done_ie_1(self) -> &'a mut W {
        self.variant(DMA_FIELD1_DONE_IEW::DMA_FIELD1_DONE_IE_1)
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
        const OFFSET: u8 = 7;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `LAST_DMA_REQ_SEL`"]
pub enum LAST_DMA_REQ_SELW {
    #[doc = "fifo_full_level"]
    LAST_DMA_REQ_SEL_0,
    #[doc = "hburst_length"]
    LAST_DMA_REQ_SEL_1,
}
impl LAST_DMA_REQ_SELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            LAST_DMA_REQ_SELW::LAST_DMA_REQ_SEL_0 => false,
            LAST_DMA_REQ_SELW::LAST_DMA_REQ_SEL_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _LAST_DMA_REQ_SELW<'a> {
    w: &'a mut W,
}
impl<'a> _LAST_DMA_REQ_SELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: LAST_DMA_REQ_SELW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "fifo_full_level"]
    #[inline]
    pub fn last_dma_req_sel_0(self) -> &'a mut W {
        self.variant(LAST_DMA_REQ_SELW::LAST_DMA_REQ_SEL_0)
    }
    #[doc = "hburst_length"]
    #[inline]
    pub fn last_dma_req_sel_1(self) -> &'a mut W {
        self.variant(LAST_DMA_REQ_SELW::LAST_DMA_REQ_SEL_1)
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
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _BASEADDR_CHANGE_ERROR_IEW<'a> {
    w: &'a mut W,
}
impl<'a> _BASEADDR_CHANGE_ERROR_IEW<'a> {
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
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `RGB888A_FORMAT_SEL`"]
pub enum RGB888A_FORMAT_SELW {
    #[doc = "{8'h0, data[23:0]}"]
    RGB888A_FORMAT_SEL_0,
    #[doc = "{data[23:0], 8'h0}"]
    RGB888A_FORMAT_SEL_1,
}
impl RGB888A_FORMAT_SELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RGB888A_FORMAT_SELW::RGB888A_FORMAT_SEL_0 => false,
            RGB888A_FORMAT_SELW::RGB888A_FORMAT_SEL_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RGB888A_FORMAT_SELW<'a> {
    w: &'a mut W,
}
impl<'a> _RGB888A_FORMAT_SELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RGB888A_FORMAT_SELW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "{8'h0, data[23:0]}"]
    #[inline]
    pub fn rgb888a_format_sel_0(self) -> &'a mut W {
        self.variant(RGB888A_FORMAT_SELW::RGB888A_FORMAT_SEL_0)
    }
    #[doc = "{data[23:0], 8'h0}"]
    #[inline]
    pub fn rgb888a_format_sel_1(self) -> &'a mut W {
        self.variant(RGB888A_FORMAT_SELW::RGB888A_FORMAT_SEL_1)
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
        const OFFSET: u8 = 10;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _AHB_HPROTW<'a> {
    w: &'a mut W,
}
impl<'a> _AHB_HPROTW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 12;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `CSI_LCDIF_BUFFER_LINES`"]
pub enum CSI_LCDIF_BUFFER_LINESW {
    #[doc = "4 lines"]
    CSI_LCDIF_BUFFER_LINES_0,
    #[doc = "8 lines"]
    CSI_LCDIF_BUFFER_LINES_1,
    #[doc = "16 lines"]
    CSI_LCDIF_BUFFER_LINES_2,
    #[doc = "16 lines"]
    CSI_LCDIF_BUFFER_LINES_3,
}
impl CSI_LCDIF_BUFFER_LINESW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            CSI_LCDIF_BUFFER_LINESW::CSI_LCDIF_BUFFER_LINES_0 => 0,
            CSI_LCDIF_BUFFER_LINESW::CSI_LCDIF_BUFFER_LINES_1 => 1,
            CSI_LCDIF_BUFFER_LINESW::CSI_LCDIF_BUFFER_LINES_2 => 2,
            CSI_LCDIF_BUFFER_LINESW::CSI_LCDIF_BUFFER_LINES_3 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CSI_LCDIF_BUFFER_LINESW<'a> {
    w: &'a mut W,
}
impl<'a> _CSI_LCDIF_BUFFER_LINESW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CSI_LCDIF_BUFFER_LINESW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "4 lines"]
    #[inline]
    pub fn csi_lcdif_buffer_lines_0(self) -> &'a mut W {
        self.variant(CSI_LCDIF_BUFFER_LINESW::CSI_LCDIF_BUFFER_LINES_0)
    }
    #[doc = "8 lines"]
    #[inline]
    pub fn csi_lcdif_buffer_lines_1(self) -> &'a mut W {
        self.variant(CSI_LCDIF_BUFFER_LINESW::CSI_LCDIF_BUFFER_LINES_1)
    }
    #[doc = "16 lines"]
    #[inline]
    pub fn csi_lcdif_buffer_lines_2(self) -> &'a mut W {
        self.variant(CSI_LCDIF_BUFFER_LINESW::CSI_LCDIF_BUFFER_LINES_2)
    }
    #[doc = "16 lines"]
    #[inline]
    pub fn csi_lcdif_buffer_lines_3(self) -> &'a mut W {
        self.variant(CSI_LCDIF_BUFFER_LINESW::CSI_LCDIF_BUFFER_LINES_3)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 16;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `MASK_OPTION`"]
pub enum MASK_OPTIONW {
    #[doc = "Writing to memory from first completely frame, when using this option, the CSI_ENABLE should be 1."]
    MASK_OPTION_0,
    #[doc = "Writing to memory when CSI_ENABLE is 1."]
    MASK_OPTION_1,
    #[doc = "Writing to memory from second completely frame, when using this option, the CSI_ENABLE should be 1."]
    MASK_OPTION_2,
    #[doc = "Writing to memory when data comes in, not matter the CSI_ENABLE is 1 or 0."]
    MASK_OPTION_3,
}
impl MASK_OPTIONW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            MASK_OPTIONW::MASK_OPTION_0 => 0,
            MASK_OPTIONW::MASK_OPTION_1 => 1,
            MASK_OPTIONW::MASK_OPTION_2 => 2,
            MASK_OPTIONW::MASK_OPTION_3 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MASK_OPTIONW<'a> {
    w: &'a mut W,
}
impl<'a> _MASK_OPTIONW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MASK_OPTIONW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Writing to memory from first completely frame, when using this option, the CSI_ENABLE should be 1."]
    #[inline]
    pub fn mask_option_0(self) -> &'a mut W {
        self.variant(MASK_OPTIONW::MASK_OPTION_0)
    }
    #[doc = "Writing to memory when CSI_ENABLE is 1."]
    #[inline]
    pub fn mask_option_1(self) -> &'a mut W {
        self.variant(MASK_OPTIONW::MASK_OPTION_1)
    }
    #[doc = "Writing to memory from second completely frame, when using this option, the CSI_ENABLE should be 1."]
    #[inline]
    pub fn mask_option_2(self) -> &'a mut W {
        self.variant(MASK_OPTIONW::MASK_OPTION_2)
    }
    #[doc = "Writing to memory when data comes in, not matter the CSI_ENABLE is 1 or 0."]
    #[inline]
    pub fn mask_option_3(self) -> &'a mut W {
        self.variant(MASK_OPTIONW::MASK_OPTION_3)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 18;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _CSI_ENABLEW<'a> {
    w: &'a mut W,
}
impl<'a> _CSI_ENABLEW<'a> {
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
        const OFFSET: u8 = 31;
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
    #[doc = "Bit 2 - This bit is used to select the output method When input is standard CCIR656 video."]
    #[inline]
    pub fn deinterlace_en(&self) -> DEINTERLACE_ENR {
        DEINTERLACE_ENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - When input is parallel rgb888/yuv444 24bit, this bit can be enabled."]
    #[inline]
    pub fn parallel24_en(&self) -> PARALLEL24_ENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        PARALLEL24_ENR { bits }
    }
    #[doc = "Bit 4 - When this bit is enabled, CSI DMA will switch the base address according to BASEADDR_SWITCH_SEL rather than atomically by DMA completed"]
    #[inline]
    pub fn baseaddr_switch_en(&self) -> BASEADDR_SWITCH_ENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        BASEADDR_SWITCH_ENR { bits }
    }
    #[doc = "Bit 5 - CSI 2 base addresses switching method. When using this bit, BASEADDR_SWITCH_EN is 1."]
    #[inline]
    pub fn baseaddr_switch_sel(&self) -> BASEADDR_SWITCH_SELR {
        BASEADDR_SWITCH_SELR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 6 - In interlace mode, fileld 0 means interrupt enabled."]
    #[inline]
    pub fn field0_done_ie(&self) -> FIELD0_DONE_IER {
        FIELD0_DONE_IER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 7 - When in interlace mode, field 1 done interrupt enable."]
    #[inline]
    pub fn dma_field1_done_ie(&self) -> DMA_FIELD1_DONE_IER {
        DMA_FIELD1_DONE_IER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 8 - Choosing the last DMA request condition."]
    #[inline]
    pub fn last_dma_req_sel(&self) -> LAST_DMA_REQ_SELR {
        LAST_DMA_REQ_SELR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 9 - Base address change error interrupt enable signal."]
    #[inline]
    pub fn baseaddr_change_error_ie(&self) -> BASEADDR_CHANGE_ERROR_IER {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        BASEADDR_CHANGE_ERROR_IER { bits }
    }
    #[doc = "Bit 10 - Output is 32-bit format."]
    #[inline]
    pub fn rgb888a_format_sel(&self) -> RGB888A_FORMAT_SELR {
        RGB888A_FORMAT_SELR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 12:15 - Hprot value in AHB bus protocol."]
    #[inline]
    pub fn ahb_hprot(&self) -> AHB_HPROTR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        AHB_HPROTR { bits }
    }
    #[doc = "Bits 16:17 - The number of lines are used in handshake mode with LCDIF."]
    #[inline]
    pub fn csi_lcdif_buffer_lines(&self) -> CSI_LCDIF_BUFFER_LINESR {
        CSI_LCDIF_BUFFER_LINESR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 18:19 - These bits used to choose the method to mask the CSI input."]
    #[inline]
    pub fn mask_option(&self) -> MASK_OPTIONR {
        MASK_OPTIONR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 31 - CSI global enable signal"]
    #[inline]
    pub fn csi_enable(&self) -> CSI_ENABLER {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 31;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        CSI_ENABLER { bits }
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 184320 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 2 - This bit is used to select the output method When input is standard CCIR656 video."]
    #[inline]
    pub fn deinterlace_en(&mut self) -> _DEINTERLACE_ENW {
        _DEINTERLACE_ENW { w: self }
    }
    #[doc = "Bit 3 - When input is parallel rgb888/yuv444 24bit, this bit can be enabled."]
    #[inline]
    pub fn parallel24_en(&mut self) -> _PARALLEL24_ENW {
        _PARALLEL24_ENW { w: self }
    }
    #[doc = "Bit 4 - When this bit is enabled, CSI DMA will switch the base address according to BASEADDR_SWITCH_SEL rather than atomically by DMA completed"]
    #[inline]
    pub fn baseaddr_switch_en(&mut self) -> _BASEADDR_SWITCH_ENW {
        _BASEADDR_SWITCH_ENW { w: self }
    }
    #[doc = "Bit 5 - CSI 2 base addresses switching method. When using this bit, BASEADDR_SWITCH_EN is 1."]
    #[inline]
    pub fn baseaddr_switch_sel(&mut self) -> _BASEADDR_SWITCH_SELW {
        _BASEADDR_SWITCH_SELW { w: self }
    }
    #[doc = "Bit 6 - In interlace mode, fileld 0 means interrupt enabled."]
    #[inline]
    pub fn field0_done_ie(&mut self) -> _FIELD0_DONE_IEW {
        _FIELD0_DONE_IEW { w: self }
    }
    #[doc = "Bit 7 - When in interlace mode, field 1 done interrupt enable."]
    #[inline]
    pub fn dma_field1_done_ie(&mut self) -> _DMA_FIELD1_DONE_IEW {
        _DMA_FIELD1_DONE_IEW { w: self }
    }
    #[doc = "Bit 8 - Choosing the last DMA request condition."]
    #[inline]
    pub fn last_dma_req_sel(&mut self) -> _LAST_DMA_REQ_SELW {
        _LAST_DMA_REQ_SELW { w: self }
    }
    #[doc = "Bit 9 - Base address change error interrupt enable signal."]
    #[inline]
    pub fn baseaddr_change_error_ie(&mut self) -> _BASEADDR_CHANGE_ERROR_IEW {
        _BASEADDR_CHANGE_ERROR_IEW { w: self }
    }
    #[doc = "Bit 10 - Output is 32-bit format."]
    #[inline]
    pub fn rgb888a_format_sel(&mut self) -> _RGB888A_FORMAT_SELW {
        _RGB888A_FORMAT_SELW { w: self }
    }
    #[doc = "Bits 12:15 - Hprot value in AHB bus protocol."]
    #[inline]
    pub fn ahb_hprot(&mut self) -> _AHB_HPROTW {
        _AHB_HPROTW { w: self }
    }
    #[doc = "Bits 16:17 - The number of lines are used in handshake mode with LCDIF."]
    #[inline]
    pub fn csi_lcdif_buffer_lines(&mut self) -> _CSI_LCDIF_BUFFER_LINESW {
        _CSI_LCDIF_BUFFER_LINESW { w: self }
    }
    #[doc = "Bits 18:19 - These bits used to choose the method to mask the CSI input."]
    #[inline]
    pub fn mask_option(&mut self) -> _MASK_OPTIONW {
        _MASK_OPTIONW { w: self }
    }
    #[doc = "Bit 31 - CSI global enable signal"]
    #[inline]
    pub fn csi_enable(&mut self) -> _CSI_ENABLEW {
        _CSI_ENABLEW { w: self }
    }
}
