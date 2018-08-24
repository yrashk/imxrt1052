#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CSICR1 {
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
#[doc = "Possible values of the field `PIXEL_BIT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIXEL_BITR {
    #[doc = "8-bit data for each pixel"]
    PIXEL_BIT_0,
    #[doc = "10-bit data for each pixel"]
    PIXEL_BIT_1,
}
impl PIXEL_BITR {
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
            PIXEL_BITR::PIXEL_BIT_0 => false,
            PIXEL_BITR::PIXEL_BIT_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PIXEL_BITR {
        match value {
            false => PIXEL_BITR::PIXEL_BIT_0,
            true => PIXEL_BITR::PIXEL_BIT_1,
        }
    }
    #[doc = "Checks if the value of the field is `PIXEL_BIT_0`"]
    #[inline]
    pub fn is_pixel_bit_0(&self) -> bool {
        *self == PIXEL_BITR::PIXEL_BIT_0
    }
    #[doc = "Checks if the value of the field is `PIXEL_BIT_1`"]
    #[inline]
    pub fn is_pixel_bit_1(&self) -> bool {
        *self == PIXEL_BITR::PIXEL_BIT_1
    }
}
#[doc = "Possible values of the field `REDGE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REDGER {
    #[doc = "Pixel data is latched at the falling edge of CSI_PIXCLK"]
    REDGE_0,
    #[doc = "Pixel data is latched at the rising edge of CSI_PIXCLK"]
    REDGE_1,
}
impl REDGER {
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
            REDGER::REDGE_0 => false,
            REDGER::REDGE_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> REDGER {
        match value {
            false => REDGER::REDGE_0,
            true => REDGER::REDGE_1,
        }
    }
    #[doc = "Checks if the value of the field is `REDGE_0`"]
    #[inline]
    pub fn is_redge_0(&self) -> bool {
        *self == REDGER::REDGE_0
    }
    #[doc = "Checks if the value of the field is `REDGE_1`"]
    #[inline]
    pub fn is_redge_1(&self) -> bool {
        *self == REDGER::REDGE_1
    }
}
#[doc = "Possible values of the field `INV_PCLK`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INV_PCLKR {
    #[doc = "CSI_PIXCLK is directly applied to internal circuitry"]
    INV_PCLK_0,
    #[doc = "CSI_PIXCLK is inverted before applied to internal circuitry"]
    INV_PCLK_1,
}
impl INV_PCLKR {
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
            INV_PCLKR::INV_PCLK_0 => false,
            INV_PCLKR::INV_PCLK_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> INV_PCLKR {
        match value {
            false => INV_PCLKR::INV_PCLK_0,
            true => INV_PCLKR::INV_PCLK_1,
        }
    }
    #[doc = "Checks if the value of the field is `INV_PCLK_0`"]
    #[inline]
    pub fn is_inv_pclk_0(&self) -> bool {
        *self == INV_PCLKR::INV_PCLK_0
    }
    #[doc = "Checks if the value of the field is `INV_PCLK_1`"]
    #[inline]
    pub fn is_inv_pclk_1(&self) -> bool {
        *self == INV_PCLKR::INV_PCLK_1
    }
}
#[doc = "Possible values of the field `INV_DATA`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INV_DATAR {
    #[doc = "CSI_D[7:0] data lines are directly applied to internal circuitry"]
    INV_DATA_0,
    #[doc = "CSI_D[7:0] data lines are inverted before applied to internal circuitry"]
    INV_DATA_1,
}
impl INV_DATAR {
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
            INV_DATAR::INV_DATA_0 => false,
            INV_DATAR::INV_DATA_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> INV_DATAR {
        match value {
            false => INV_DATAR::INV_DATA_0,
            true => INV_DATAR::INV_DATA_1,
        }
    }
    #[doc = "Checks if the value of the field is `INV_DATA_0`"]
    #[inline]
    pub fn is_inv_data_0(&self) -> bool {
        *self == INV_DATAR::INV_DATA_0
    }
    #[doc = "Checks if the value of the field is `INV_DATA_1`"]
    #[inline]
    pub fn is_inv_data_1(&self) -> bool {
        *self == INV_DATAR::INV_DATA_1
    }
}
#[doc = "Possible values of the field `GCLK_MODE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GCLK_MODER {
    #[doc = "Non-gated clock mode. All incoming pixel clocks are valid. HSYNC is ignored."]
    GCLK_MODE_0,
    #[doc = "Gated clock mode. Pixel clock signal is valid only when HSYNC is active."]
    GCLK_MODE_1,
}
impl GCLK_MODER {
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
            GCLK_MODER::GCLK_MODE_0 => false,
            GCLK_MODER::GCLK_MODE_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> GCLK_MODER {
        match value {
            false => GCLK_MODER::GCLK_MODE_0,
            true => GCLK_MODER::GCLK_MODE_1,
        }
    }
    #[doc = "Checks if the value of the field is `GCLK_MODE_0`"]
    #[inline]
    pub fn is_gclk_mode_0(&self) -> bool {
        *self == GCLK_MODER::GCLK_MODE_0
    }
    #[doc = "Checks if the value of the field is `GCLK_MODE_1`"]
    #[inline]
    pub fn is_gclk_mode_1(&self) -> bool {
        *self == GCLK_MODER::GCLK_MODE_1
    }
}
#[doc = r" Value of the field"]
pub struct CLR_RXFIFOR {
    bits: bool,
}
impl CLR_RXFIFOR {
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
pub struct CLR_STATFIFOR {
    bits: bool,
}
impl CLR_STATFIFOR {
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
#[doc = "Possible values of the field `PACK_DIR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PACK_DIRR {
    #[doc = "Pack from LSB first. For image data, 0x11, 0x22, 0x33, 0x44, it will appear as 0x44332211 in RX FIFO. For stat data, 0xAAAA, 0xBBBB, it will appear as 0xBBBBAAAA in STAT FIFO."]
    PACK_DIR_0,
    #[doc = "Pack from MSB first. For image data, 0x11, 0x22, 0x33, 0x44, it will appear as 0x11223344 in RX FIFO. For stat data, 0xAAAA, 0xBBBB, it will appear as 0xAAAABBBB in STAT FIFO."]
    PACK_DIR_1,
}
impl PACK_DIRR {
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
            PACK_DIRR::PACK_DIR_0 => false,
            PACK_DIRR::PACK_DIR_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PACK_DIRR {
        match value {
            false => PACK_DIRR::PACK_DIR_0,
            true => PACK_DIRR::PACK_DIR_1,
        }
    }
    #[doc = "Checks if the value of the field is `PACK_DIR_0`"]
    #[inline]
    pub fn is_pack_dir_0(&self) -> bool {
        *self == PACK_DIRR::PACK_DIR_0
    }
    #[doc = "Checks if the value of the field is `PACK_DIR_1`"]
    #[inline]
    pub fn is_pack_dir_1(&self) -> bool {
        *self == PACK_DIRR::PACK_DIR_1
    }
}
#[doc = "Possible values of the field `FCC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FCCR {
    #[doc = "Asynchronous FIFO clear is selected."]
    FCC_0,
    #[doc = "Synchronous FIFO clear is selected."]
    FCC_1,
}
impl FCCR {
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
            FCCR::FCC_0 => false,
            FCCR::FCC_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> FCCR {
        match value {
            false => FCCR::FCC_0,
            true => FCCR::FCC_1,
        }
    }
    #[doc = "Checks if the value of the field is `FCC_0`"]
    #[inline]
    pub fn is_fcc_0(&self) -> bool {
        *self == FCCR::FCC_0
    }
    #[doc = "Checks if the value of the field is `FCC_1`"]
    #[inline]
    pub fn is_fcc_1(&self) -> bool {
        *self == FCCR::FCC_1
    }
}
#[doc = "Possible values of the field `CCIR_EN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CCIR_ENR {
    #[doc = "Traditional interface is selected. Timing interface logic is used to latch data."]
    CCIR_EN_0,
    #[doc = "CCIR656 interface is selected."]
    CCIR_EN_1,
}
impl CCIR_ENR {
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
            CCIR_ENR::CCIR_EN_0 => false,
            CCIR_ENR::CCIR_EN_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CCIR_ENR {
        match value {
            false => CCIR_ENR::CCIR_EN_0,
            true => CCIR_ENR::CCIR_EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `CCIR_EN_0`"]
    #[inline]
    pub fn is_ccir_en_0(&self) -> bool {
        *self == CCIR_ENR::CCIR_EN_0
    }
    #[doc = "Checks if the value of the field is `CCIR_EN_1`"]
    #[inline]
    pub fn is_ccir_en_1(&self) -> bool {
        *self == CCIR_ENR::CCIR_EN_1
    }
}
#[doc = "Possible values of the field `HSYNC_POL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HSYNC_POLR {
    #[doc = "HSYNC is active low"]
    HSYNC_POL_0,
    #[doc = "HSYNC is active high"]
    HSYNC_POL_1,
}
impl HSYNC_POLR {
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
            HSYNC_POLR::HSYNC_POL_0 => false,
            HSYNC_POLR::HSYNC_POL_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> HSYNC_POLR {
        match value {
            false => HSYNC_POLR::HSYNC_POL_0,
            true => HSYNC_POLR::HSYNC_POL_1,
        }
    }
    #[doc = "Checks if the value of the field is `HSYNC_POL_0`"]
    #[inline]
    pub fn is_hsync_pol_0(&self) -> bool {
        *self == HSYNC_POLR::HSYNC_POL_0
    }
    #[doc = "Checks if the value of the field is `HSYNC_POL_1`"]
    #[inline]
    pub fn is_hsync_pol_1(&self) -> bool {
        *self == HSYNC_POLR::HSYNC_POL_1
    }
}
#[doc = "Possible values of the field `SOF_INTEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SOF_INTENR {
    #[doc = "SOF interrupt disable"]
    SOF_INTEN_0,
    #[doc = "SOF interrupt enable"]
    SOF_INTEN_1,
}
impl SOF_INTENR {
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
            SOF_INTENR::SOF_INTEN_0 => false,
            SOF_INTENR::SOF_INTEN_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SOF_INTENR {
        match value {
            false => SOF_INTENR::SOF_INTEN_0,
            true => SOF_INTENR::SOF_INTEN_1,
        }
    }
    #[doc = "Checks if the value of the field is `SOF_INTEN_0`"]
    #[inline]
    pub fn is_sof_inten_0(&self) -> bool {
        *self == SOF_INTENR::SOF_INTEN_0
    }
    #[doc = "Checks if the value of the field is `SOF_INTEN_1`"]
    #[inline]
    pub fn is_sof_inten_1(&self) -> bool {
        *self == SOF_INTENR::SOF_INTEN_1
    }
}
#[doc = "Possible values of the field `SOF_POL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SOF_POLR {
    #[doc = "SOF interrupt is generated on SOF falling edge"]
    SOF_POL_0,
    #[doc = "SOF interrupt is generated on SOF rising edge"]
    SOF_POL_1,
}
impl SOF_POLR {
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
            SOF_POLR::SOF_POL_0 => false,
            SOF_POLR::SOF_POL_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SOF_POLR {
        match value {
            false => SOF_POLR::SOF_POL_0,
            true => SOF_POLR::SOF_POL_1,
        }
    }
    #[doc = "Checks if the value of the field is `SOF_POL_0`"]
    #[inline]
    pub fn is_sof_pol_0(&self) -> bool {
        *self == SOF_POLR::SOF_POL_0
    }
    #[doc = "Checks if the value of the field is `SOF_POL_1`"]
    #[inline]
    pub fn is_sof_pol_1(&self) -> bool {
        *self == SOF_POLR::SOF_POL_1
    }
}
#[doc = "Possible values of the field `RXFF_INTEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXFF_INTENR {
    #[doc = "RxFIFO full interrupt disable"]
    RXFF_INTEN_0,
    #[doc = "RxFIFO full interrupt enable"]
    RXFF_INTEN_1,
}
impl RXFF_INTENR {
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
            RXFF_INTENR::RXFF_INTEN_0 => false,
            RXFF_INTENR::RXFF_INTEN_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RXFF_INTENR {
        match value {
            false => RXFF_INTENR::RXFF_INTEN_0,
            true => RXFF_INTENR::RXFF_INTEN_1,
        }
    }
    #[doc = "Checks if the value of the field is `RXFF_INTEN_0`"]
    #[inline]
    pub fn is_rxff_inten_0(&self) -> bool {
        *self == RXFF_INTENR::RXFF_INTEN_0
    }
    #[doc = "Checks if the value of the field is `RXFF_INTEN_1`"]
    #[inline]
    pub fn is_rxff_inten_1(&self) -> bool {
        *self == RXFF_INTENR::RXFF_INTEN_1
    }
}
#[doc = "Possible values of the field `FB1_DMA_DONE_INTEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FB1_DMA_DONE_INTENR {
    #[doc = "Frame Buffer1 DMA Transfer Done interrupt disable"]
    FB1_DMA_DONE_INTEN_0,
    #[doc = "Frame Buffer1 DMA Transfer Done interrupt enable"]
    FB1_DMA_DONE_INTEN_1,
}
impl FB1_DMA_DONE_INTENR {
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
            FB1_DMA_DONE_INTENR::FB1_DMA_DONE_INTEN_0 => false,
            FB1_DMA_DONE_INTENR::FB1_DMA_DONE_INTEN_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> FB1_DMA_DONE_INTENR {
        match value {
            false => FB1_DMA_DONE_INTENR::FB1_DMA_DONE_INTEN_0,
            true => FB1_DMA_DONE_INTENR::FB1_DMA_DONE_INTEN_1,
        }
    }
    #[doc = "Checks if the value of the field is `FB1_DMA_DONE_INTEN_0`"]
    #[inline]
    pub fn is_fb1_dma_done_inten_0(&self) -> bool {
        *self == FB1_DMA_DONE_INTENR::FB1_DMA_DONE_INTEN_0
    }
    #[doc = "Checks if the value of the field is `FB1_DMA_DONE_INTEN_1`"]
    #[inline]
    pub fn is_fb1_dma_done_inten_1(&self) -> bool {
        *self == FB1_DMA_DONE_INTENR::FB1_DMA_DONE_INTEN_1
    }
}
#[doc = "Possible values of the field `FB2_DMA_DONE_INTEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FB2_DMA_DONE_INTENR {
    #[doc = "Frame Buffer2 DMA Transfer Done interrupt disable"]
    FB2_DMA_DONE_INTEN_0,
    #[doc = "Frame Buffer2 DMA Transfer Done interrupt enable"]
    FB2_DMA_DONE_INTEN_1,
}
impl FB2_DMA_DONE_INTENR {
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
            FB2_DMA_DONE_INTENR::FB2_DMA_DONE_INTEN_0 => false,
            FB2_DMA_DONE_INTENR::FB2_DMA_DONE_INTEN_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> FB2_DMA_DONE_INTENR {
        match value {
            false => FB2_DMA_DONE_INTENR::FB2_DMA_DONE_INTEN_0,
            true => FB2_DMA_DONE_INTENR::FB2_DMA_DONE_INTEN_1,
        }
    }
    #[doc = "Checks if the value of the field is `FB2_DMA_DONE_INTEN_0`"]
    #[inline]
    pub fn is_fb2_dma_done_inten_0(&self) -> bool {
        *self == FB2_DMA_DONE_INTENR::FB2_DMA_DONE_INTEN_0
    }
    #[doc = "Checks if the value of the field is `FB2_DMA_DONE_INTEN_1`"]
    #[inline]
    pub fn is_fb2_dma_done_inten_1(&self) -> bool {
        *self == FB2_DMA_DONE_INTENR::FB2_DMA_DONE_INTEN_1
    }
}
#[doc = "Possible values of the field `STATFF_INTEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum STATFF_INTENR {
    #[doc = "STATFIFO full interrupt disable"]
    STATFF_INTEN_0,
    #[doc = "STATFIFO full interrupt enable"]
    STATFF_INTEN_1,
}
impl STATFF_INTENR {
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
            STATFF_INTENR::STATFF_INTEN_0 => false,
            STATFF_INTENR::STATFF_INTEN_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> STATFF_INTENR {
        match value {
            false => STATFF_INTENR::STATFF_INTEN_0,
            true => STATFF_INTENR::STATFF_INTEN_1,
        }
    }
    #[doc = "Checks if the value of the field is `STATFF_INTEN_0`"]
    #[inline]
    pub fn is_statff_inten_0(&self) -> bool {
        *self == STATFF_INTENR::STATFF_INTEN_0
    }
    #[doc = "Checks if the value of the field is `STATFF_INTEN_1`"]
    #[inline]
    pub fn is_statff_inten_1(&self) -> bool {
        *self == STATFF_INTENR::STATFF_INTEN_1
    }
}
#[doc = "Possible values of the field `SFF_DMA_DONE_INTEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SFF_DMA_DONE_INTENR {
    #[doc = "STATFIFO DMA Transfer Done interrupt disable"]
    SFF_DMA_DONE_INTEN_0,
    #[doc = "STATFIFO DMA Transfer Done interrupt enable"]
    SFF_DMA_DONE_INTEN_1,
}
impl SFF_DMA_DONE_INTENR {
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
            SFF_DMA_DONE_INTENR::SFF_DMA_DONE_INTEN_0 => false,
            SFF_DMA_DONE_INTENR::SFF_DMA_DONE_INTEN_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SFF_DMA_DONE_INTENR {
        match value {
            false => SFF_DMA_DONE_INTENR::SFF_DMA_DONE_INTEN_0,
            true => SFF_DMA_DONE_INTENR::SFF_DMA_DONE_INTEN_1,
        }
    }
    #[doc = "Checks if the value of the field is `SFF_DMA_DONE_INTEN_0`"]
    #[inline]
    pub fn is_sff_dma_done_inten_0(&self) -> bool {
        *self == SFF_DMA_DONE_INTENR::SFF_DMA_DONE_INTEN_0
    }
    #[doc = "Checks if the value of the field is `SFF_DMA_DONE_INTEN_1`"]
    #[inline]
    pub fn is_sff_dma_done_inten_1(&self) -> bool {
        *self == SFF_DMA_DONE_INTENR::SFF_DMA_DONE_INTEN_1
    }
}
#[doc = "Possible values of the field `RF_OR_INTEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RF_OR_INTENR {
    #[doc = "RxFIFO overrun interrupt is disabled"]
    RF_OR_INTEN_0,
    #[doc = "RxFIFO overrun interrupt is enabled"]
    RF_OR_INTEN_1,
}
impl RF_OR_INTENR {
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
            RF_OR_INTENR::RF_OR_INTEN_0 => false,
            RF_OR_INTENR::RF_OR_INTEN_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RF_OR_INTENR {
        match value {
            false => RF_OR_INTENR::RF_OR_INTEN_0,
            true => RF_OR_INTENR::RF_OR_INTEN_1,
        }
    }
    #[doc = "Checks if the value of the field is `RF_OR_INTEN_0`"]
    #[inline]
    pub fn is_rf_or_inten_0(&self) -> bool {
        *self == RF_OR_INTENR::RF_OR_INTEN_0
    }
    #[doc = "Checks if the value of the field is `RF_OR_INTEN_1`"]
    #[inline]
    pub fn is_rf_or_inten_1(&self) -> bool {
        *self == RF_OR_INTENR::RF_OR_INTEN_1
    }
}
#[doc = "Possible values of the field `SF_OR_INTEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SF_OR_INTENR {
    #[doc = "STATFIFO overrun interrupt is disabled"]
    SF_OR_INTEN_0,
    #[doc = "STATFIFO overrun interrupt is enabled"]
    SF_OR_INTEN_1,
}
impl SF_OR_INTENR {
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
            SF_OR_INTENR::SF_OR_INTEN_0 => false,
            SF_OR_INTENR::SF_OR_INTEN_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SF_OR_INTENR {
        match value {
            false => SF_OR_INTENR::SF_OR_INTEN_0,
            true => SF_OR_INTENR::SF_OR_INTEN_1,
        }
    }
    #[doc = "Checks if the value of the field is `SF_OR_INTEN_0`"]
    #[inline]
    pub fn is_sf_or_inten_0(&self) -> bool {
        *self == SF_OR_INTENR::SF_OR_INTEN_0
    }
    #[doc = "Checks if the value of the field is `SF_OR_INTEN_1`"]
    #[inline]
    pub fn is_sf_or_inten_1(&self) -> bool {
        *self == SF_OR_INTENR::SF_OR_INTEN_1
    }
}
#[doc = "Possible values of the field `COF_INT_EN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum COF_INT_ENR {
    #[doc = "COF interrupt is disabled"]
    COF_INT_EN_0,
    #[doc = "COF interrupt is enabled"]
    COF_INT_EN_1,
}
impl COF_INT_ENR {
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
            COF_INT_ENR::COF_INT_EN_0 => false,
            COF_INT_ENR::COF_INT_EN_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> COF_INT_ENR {
        match value {
            false => COF_INT_ENR::COF_INT_EN_0,
            true => COF_INT_ENR::COF_INT_EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `COF_INT_EN_0`"]
    #[inline]
    pub fn is_cof_int_en_0(&self) -> bool {
        *self == COF_INT_ENR::COF_INT_EN_0
    }
    #[doc = "Checks if the value of the field is `COF_INT_EN_1`"]
    #[inline]
    pub fn is_cof_int_en_1(&self) -> bool {
        *self == COF_INT_ENR::COF_INT_EN_1
    }
}
#[doc = "Possible values of the field `CCIR_MODE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CCIR_MODER {
    #[doc = "Progressive mode is selected"]
    CCIR_MODE_0,
    #[doc = "Interlace mode is selected"]
    CCIR_MODE_1,
}
impl CCIR_MODER {
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
            CCIR_MODER::CCIR_MODE_0 => false,
            CCIR_MODER::CCIR_MODE_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CCIR_MODER {
        match value {
            false => CCIR_MODER::CCIR_MODE_0,
            true => CCIR_MODER::CCIR_MODE_1,
        }
    }
    #[doc = "Checks if the value of the field is `CCIR_MODE_0`"]
    #[inline]
    pub fn is_ccir_mode_0(&self) -> bool {
        *self == CCIR_MODER::CCIR_MODE_0
    }
    #[doc = "Checks if the value of the field is `CCIR_MODE_1`"]
    #[inline]
    pub fn is_ccir_mode_1(&self) -> bool {
        *self == CCIR_MODER::CCIR_MODE_1
    }
}
#[doc = "Possible values of the field `PrP_IF_EN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PRP_IF_ENR {
    #[doc = "CSI to PrP bus is disabled"]
    PRP_IF_EN_0,
    #[doc = "CSI to PrP bus is enabled"]
    PRP_IF_EN_1,
}
impl PRP_IF_ENR {
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
            PRP_IF_ENR::PRP_IF_EN_0 => false,
            PRP_IF_ENR::PRP_IF_EN_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PRP_IF_ENR {
        match value {
            false => PRP_IF_ENR::PRP_IF_EN_0,
            true => PRP_IF_ENR::PRP_IF_EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `PRP_IF_EN_0`"]
    #[inline]
    pub fn is_pr_p_if_en_0(&self) -> bool {
        *self == PRP_IF_ENR::PRP_IF_EN_0
    }
    #[doc = "Checks if the value of the field is `PRP_IF_EN_1`"]
    #[inline]
    pub fn is_pr_p_if_en_1(&self) -> bool {
        *self == PRP_IF_ENR::PRP_IF_EN_1
    }
}
#[doc = "Possible values of the field `EOF_INT_EN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EOF_INT_ENR {
    #[doc = "EOF interrupt is disabled."]
    EOF_INT_EN_0,
    #[doc = "EOF interrupt is generated when RX count value is reached."]
    EOF_INT_EN_1,
}
impl EOF_INT_ENR {
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
            EOF_INT_ENR::EOF_INT_EN_0 => false,
            EOF_INT_ENR::EOF_INT_EN_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> EOF_INT_ENR {
        match value {
            false => EOF_INT_ENR::EOF_INT_EN_0,
            true => EOF_INT_ENR::EOF_INT_EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `EOF_INT_EN_0`"]
    #[inline]
    pub fn is_eof_int_en_0(&self) -> bool {
        *self == EOF_INT_ENR::EOF_INT_EN_0
    }
    #[doc = "Checks if the value of the field is `EOF_INT_EN_1`"]
    #[inline]
    pub fn is_eof_int_en_1(&self) -> bool {
        *self == EOF_INT_ENR::EOF_INT_EN_1
    }
}
#[doc = "Possible values of the field `EXT_VSYNC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EXT_VSYNCR {
    #[doc = "Internal VSYNC mode"]
    EXT_VSYNC_0,
    #[doc = "External VSYNC mode"]
    EXT_VSYNC_1,
}
impl EXT_VSYNCR {
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
            EXT_VSYNCR::EXT_VSYNC_0 => false,
            EXT_VSYNCR::EXT_VSYNC_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> EXT_VSYNCR {
        match value {
            false => EXT_VSYNCR::EXT_VSYNC_0,
            true => EXT_VSYNCR::EXT_VSYNC_1,
        }
    }
    #[doc = "Checks if the value of the field is `EXT_VSYNC_0`"]
    #[inline]
    pub fn is_ext_vsync_0(&self) -> bool {
        *self == EXT_VSYNCR::EXT_VSYNC_0
    }
    #[doc = "Checks if the value of the field is `EXT_VSYNC_1`"]
    #[inline]
    pub fn is_ext_vsync_1(&self) -> bool {
        *self == EXT_VSYNCR::EXT_VSYNC_1
    }
}
#[doc = "Possible values of the field `SWAP16_EN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SWAP16_ENR {
    #[doc = "Disable swapping"]
    SWAP16_EN_0,
    #[doc = "Enable swapping"]
    SWAP16_EN_1,
}
impl SWAP16_ENR {
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
            SWAP16_ENR::SWAP16_EN_0 => false,
            SWAP16_ENR::SWAP16_EN_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SWAP16_ENR {
        match value {
            false => SWAP16_ENR::SWAP16_EN_0,
            true => SWAP16_ENR::SWAP16_EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `SWAP16_EN_0`"]
    #[inline]
    pub fn is_swap16_en_0(&self) -> bool {
        *self == SWAP16_ENR::SWAP16_EN_0
    }
    #[doc = "Checks if the value of the field is `SWAP16_EN_1`"]
    #[inline]
    pub fn is_swap16_en_1(&self) -> bool {
        *self == SWAP16_ENR::SWAP16_EN_1
    }
}
#[doc = "Values that can be written to the field `PIXEL_BIT`"]
pub enum PIXEL_BITW {
    #[doc = "8-bit data for each pixel"]
    PIXEL_BIT_0,
    #[doc = "10-bit data for each pixel"]
    PIXEL_BIT_1,
}
impl PIXEL_BITW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PIXEL_BITW::PIXEL_BIT_0 => false,
            PIXEL_BITW::PIXEL_BIT_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PIXEL_BITW<'a> {
    w: &'a mut W,
}
impl<'a> _PIXEL_BITW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PIXEL_BITW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "8-bit data for each pixel"]
    #[inline]
    pub fn pixel_bit_0(self) -> &'a mut W {
        self.variant(PIXEL_BITW::PIXEL_BIT_0)
    }
    #[doc = "10-bit data for each pixel"]
    #[inline]
    pub fn pixel_bit_1(self) -> &'a mut W {
        self.variant(PIXEL_BITW::PIXEL_BIT_1)
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
#[doc = "Values that can be written to the field `REDGE`"]
pub enum REDGEW {
    #[doc = "Pixel data is latched at the falling edge of CSI_PIXCLK"]
    REDGE_0,
    #[doc = "Pixel data is latched at the rising edge of CSI_PIXCLK"]
    REDGE_1,
}
impl REDGEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            REDGEW::REDGE_0 => false,
            REDGEW::REDGE_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _REDGEW<'a> {
    w: &'a mut W,
}
impl<'a> _REDGEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: REDGEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Pixel data is latched at the falling edge of CSI_PIXCLK"]
    #[inline]
    pub fn redge_0(self) -> &'a mut W {
        self.variant(REDGEW::REDGE_0)
    }
    #[doc = "Pixel data is latched at the rising edge of CSI_PIXCLK"]
    #[inline]
    pub fn redge_1(self) -> &'a mut W {
        self.variant(REDGEW::REDGE_1)
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
#[doc = "Values that can be written to the field `INV_PCLK`"]
pub enum INV_PCLKW {
    #[doc = "CSI_PIXCLK is directly applied to internal circuitry"]
    INV_PCLK_0,
    #[doc = "CSI_PIXCLK is inverted before applied to internal circuitry"]
    INV_PCLK_1,
}
impl INV_PCLKW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            INV_PCLKW::INV_PCLK_0 => false,
            INV_PCLKW::INV_PCLK_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _INV_PCLKW<'a> {
    w: &'a mut W,
}
impl<'a> _INV_PCLKW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: INV_PCLKW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "CSI_PIXCLK is directly applied to internal circuitry"]
    #[inline]
    pub fn inv_pclk_0(self) -> &'a mut W {
        self.variant(INV_PCLKW::INV_PCLK_0)
    }
    #[doc = "CSI_PIXCLK is inverted before applied to internal circuitry"]
    #[inline]
    pub fn inv_pclk_1(self) -> &'a mut W {
        self.variant(INV_PCLKW::INV_PCLK_1)
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
#[doc = "Values that can be written to the field `INV_DATA`"]
pub enum INV_DATAW {
    #[doc = "CSI_D[7:0] data lines are directly applied to internal circuitry"]
    INV_DATA_0,
    #[doc = "CSI_D[7:0] data lines are inverted before applied to internal circuitry"]
    INV_DATA_1,
}
impl INV_DATAW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            INV_DATAW::INV_DATA_0 => false,
            INV_DATAW::INV_DATA_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _INV_DATAW<'a> {
    w: &'a mut W,
}
impl<'a> _INV_DATAW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: INV_DATAW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "CSI_D[7:0] data lines are directly applied to internal circuitry"]
    #[inline]
    pub fn inv_data_0(self) -> &'a mut W {
        self.variant(INV_DATAW::INV_DATA_0)
    }
    #[doc = "CSI_D[7:0] data lines are inverted before applied to internal circuitry"]
    #[inline]
    pub fn inv_data_1(self) -> &'a mut W {
        self.variant(INV_DATAW::INV_DATA_1)
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
#[doc = "Values that can be written to the field `GCLK_MODE`"]
pub enum GCLK_MODEW {
    #[doc = "Non-gated clock mode. All incoming pixel clocks are valid. HSYNC is ignored."]
    GCLK_MODE_0,
    #[doc = "Gated clock mode. Pixel clock signal is valid only when HSYNC is active."]
    GCLK_MODE_1,
}
impl GCLK_MODEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            GCLK_MODEW::GCLK_MODE_0 => false,
            GCLK_MODEW::GCLK_MODE_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _GCLK_MODEW<'a> {
    w: &'a mut W,
}
impl<'a> _GCLK_MODEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: GCLK_MODEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Non-gated clock mode. All incoming pixel clocks are valid. HSYNC is ignored."]
    #[inline]
    pub fn gclk_mode_0(self) -> &'a mut W {
        self.variant(GCLK_MODEW::GCLK_MODE_0)
    }
    #[doc = "Gated clock mode. Pixel clock signal is valid only when HSYNC is active."]
    #[inline]
    pub fn gclk_mode_1(self) -> &'a mut W {
        self.variant(GCLK_MODEW::GCLK_MODE_1)
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
pub struct _CLR_RXFIFOW<'a> {
    w: &'a mut W,
}
impl<'a> _CLR_RXFIFOW<'a> {
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
pub struct _CLR_STATFIFOW<'a> {
    w: &'a mut W,
}
impl<'a> _CLR_STATFIFOW<'a> {
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
#[doc = "Values that can be written to the field `PACK_DIR`"]
pub enum PACK_DIRW {
    #[doc = "Pack from LSB first. For image data, 0x11, 0x22, 0x33, 0x44, it will appear as 0x44332211 in RX FIFO. For stat data, 0xAAAA, 0xBBBB, it will appear as 0xBBBBAAAA in STAT FIFO."]
    PACK_DIR_0,
    #[doc = "Pack from MSB first. For image data, 0x11, 0x22, 0x33, 0x44, it will appear as 0x11223344 in RX FIFO. For stat data, 0xAAAA, 0xBBBB, it will appear as 0xAAAABBBB in STAT FIFO."]
    PACK_DIR_1,
}
impl PACK_DIRW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PACK_DIRW::PACK_DIR_0 => false,
            PACK_DIRW::PACK_DIR_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PACK_DIRW<'a> {
    w: &'a mut W,
}
impl<'a> _PACK_DIRW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PACK_DIRW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Pack from LSB first. For image data, 0x11, 0x22, 0x33, 0x44, it will appear as 0x44332211 in RX FIFO. For stat data, 0xAAAA, 0xBBBB, it will appear as 0xBBBBAAAA in STAT FIFO."]
    #[inline]
    pub fn pack_dir_0(self) -> &'a mut W {
        self.variant(PACK_DIRW::PACK_DIR_0)
    }
    #[doc = "Pack from MSB first. For image data, 0x11, 0x22, 0x33, 0x44, it will appear as 0x11223344 in RX FIFO. For stat data, 0xAAAA, 0xBBBB, it will appear as 0xAAAABBBB in STAT FIFO."]
    #[inline]
    pub fn pack_dir_1(self) -> &'a mut W {
        self.variant(PACK_DIRW::PACK_DIR_1)
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
#[doc = "Values that can be written to the field `FCC`"]
pub enum FCCW {
    #[doc = "Asynchronous FIFO clear is selected."]
    FCC_0,
    #[doc = "Synchronous FIFO clear is selected."]
    FCC_1,
}
impl FCCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            FCCW::FCC_0 => false,
            FCCW::FCC_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FCCW<'a> {
    w: &'a mut W,
}
impl<'a> _FCCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FCCW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Asynchronous FIFO clear is selected."]
    #[inline]
    pub fn fcc_0(self) -> &'a mut W {
        self.variant(FCCW::FCC_0)
    }
    #[doc = "Synchronous FIFO clear is selected."]
    #[inline]
    pub fn fcc_1(self) -> &'a mut W {
        self.variant(FCCW::FCC_1)
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
#[doc = "Values that can be written to the field `CCIR_EN`"]
pub enum CCIR_ENW {
    #[doc = "Traditional interface is selected. Timing interface logic is used to latch data."]
    CCIR_EN_0,
    #[doc = "CCIR656 interface is selected."]
    CCIR_EN_1,
}
impl CCIR_ENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CCIR_ENW::CCIR_EN_0 => false,
            CCIR_ENW::CCIR_EN_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CCIR_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _CCIR_ENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CCIR_ENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Traditional interface is selected. Timing interface logic is used to latch data."]
    #[inline]
    pub fn ccir_en_0(self) -> &'a mut W {
        self.variant(CCIR_ENW::CCIR_EN_0)
    }
    #[doc = "CCIR656 interface is selected."]
    #[inline]
    pub fn ccir_en_1(self) -> &'a mut W {
        self.variant(CCIR_ENW::CCIR_EN_1)
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
#[doc = "Values that can be written to the field `HSYNC_POL`"]
pub enum HSYNC_POLW {
    #[doc = "HSYNC is active low"]
    HSYNC_POL_0,
    #[doc = "HSYNC is active high"]
    HSYNC_POL_1,
}
impl HSYNC_POLW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            HSYNC_POLW::HSYNC_POL_0 => false,
            HSYNC_POLW::HSYNC_POL_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _HSYNC_POLW<'a> {
    w: &'a mut W,
}
impl<'a> _HSYNC_POLW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: HSYNC_POLW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "HSYNC is active low"]
    #[inline]
    pub fn hsync_pol_0(self) -> &'a mut W {
        self.variant(HSYNC_POLW::HSYNC_POL_0)
    }
    #[doc = "HSYNC is active high"]
    #[inline]
    pub fn hsync_pol_1(self) -> &'a mut W {
        self.variant(HSYNC_POLW::HSYNC_POL_1)
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
        const OFFSET: u8 = 11;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `SOF_INTEN`"]
pub enum SOF_INTENW {
    #[doc = "SOF interrupt disable"]
    SOF_INTEN_0,
    #[doc = "SOF interrupt enable"]
    SOF_INTEN_1,
}
impl SOF_INTENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SOF_INTENW::SOF_INTEN_0 => false,
            SOF_INTENW::SOF_INTEN_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SOF_INTENW<'a> {
    w: &'a mut W,
}
impl<'a> _SOF_INTENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SOF_INTENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "SOF interrupt disable"]
    #[inline]
    pub fn sof_inten_0(self) -> &'a mut W {
        self.variant(SOF_INTENW::SOF_INTEN_0)
    }
    #[doc = "SOF interrupt enable"]
    #[inline]
    pub fn sof_inten_1(self) -> &'a mut W {
        self.variant(SOF_INTENW::SOF_INTEN_1)
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
        const OFFSET: u8 = 16;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `SOF_POL`"]
pub enum SOF_POLW {
    #[doc = "SOF interrupt is generated on SOF falling edge"]
    SOF_POL_0,
    #[doc = "SOF interrupt is generated on SOF rising edge"]
    SOF_POL_1,
}
impl SOF_POLW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SOF_POLW::SOF_POL_0 => false,
            SOF_POLW::SOF_POL_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SOF_POLW<'a> {
    w: &'a mut W,
}
impl<'a> _SOF_POLW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SOF_POLW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "SOF interrupt is generated on SOF falling edge"]
    #[inline]
    pub fn sof_pol_0(self) -> &'a mut W {
        self.variant(SOF_POLW::SOF_POL_0)
    }
    #[doc = "SOF interrupt is generated on SOF rising edge"]
    #[inline]
    pub fn sof_pol_1(self) -> &'a mut W {
        self.variant(SOF_POLW::SOF_POL_1)
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
        const OFFSET: u8 = 17;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `RXFF_INTEN`"]
pub enum RXFF_INTENW {
    #[doc = "RxFIFO full interrupt disable"]
    RXFF_INTEN_0,
    #[doc = "RxFIFO full interrupt enable"]
    RXFF_INTEN_1,
}
impl RXFF_INTENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RXFF_INTENW::RXFF_INTEN_0 => false,
            RXFF_INTENW::RXFF_INTEN_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RXFF_INTENW<'a> {
    w: &'a mut W,
}
impl<'a> _RXFF_INTENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RXFF_INTENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "RxFIFO full interrupt disable"]
    #[inline]
    pub fn rxff_inten_0(self) -> &'a mut W {
        self.variant(RXFF_INTENW::RXFF_INTEN_0)
    }
    #[doc = "RxFIFO full interrupt enable"]
    #[inline]
    pub fn rxff_inten_1(self) -> &'a mut W {
        self.variant(RXFF_INTENW::RXFF_INTEN_1)
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
        const OFFSET: u8 = 18;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `FB1_DMA_DONE_INTEN`"]
pub enum FB1_DMA_DONE_INTENW {
    #[doc = "Frame Buffer1 DMA Transfer Done interrupt disable"]
    FB1_DMA_DONE_INTEN_0,
    #[doc = "Frame Buffer1 DMA Transfer Done interrupt enable"]
    FB1_DMA_DONE_INTEN_1,
}
impl FB1_DMA_DONE_INTENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            FB1_DMA_DONE_INTENW::FB1_DMA_DONE_INTEN_0 => false,
            FB1_DMA_DONE_INTENW::FB1_DMA_DONE_INTEN_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FB1_DMA_DONE_INTENW<'a> {
    w: &'a mut W,
}
impl<'a> _FB1_DMA_DONE_INTENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FB1_DMA_DONE_INTENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Frame Buffer1 DMA Transfer Done interrupt disable"]
    #[inline]
    pub fn fb1_dma_done_inten_0(self) -> &'a mut W {
        self.variant(FB1_DMA_DONE_INTENW::FB1_DMA_DONE_INTEN_0)
    }
    #[doc = "Frame Buffer1 DMA Transfer Done interrupt enable"]
    #[inline]
    pub fn fb1_dma_done_inten_1(self) -> &'a mut W {
        self.variant(FB1_DMA_DONE_INTENW::FB1_DMA_DONE_INTEN_1)
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
        const OFFSET: u8 = 19;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `FB2_DMA_DONE_INTEN`"]
pub enum FB2_DMA_DONE_INTENW {
    #[doc = "Frame Buffer2 DMA Transfer Done interrupt disable"]
    FB2_DMA_DONE_INTEN_0,
    #[doc = "Frame Buffer2 DMA Transfer Done interrupt enable"]
    FB2_DMA_DONE_INTEN_1,
}
impl FB2_DMA_DONE_INTENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            FB2_DMA_DONE_INTENW::FB2_DMA_DONE_INTEN_0 => false,
            FB2_DMA_DONE_INTENW::FB2_DMA_DONE_INTEN_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FB2_DMA_DONE_INTENW<'a> {
    w: &'a mut W,
}
impl<'a> _FB2_DMA_DONE_INTENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FB2_DMA_DONE_INTENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Frame Buffer2 DMA Transfer Done interrupt disable"]
    #[inline]
    pub fn fb2_dma_done_inten_0(self) -> &'a mut W {
        self.variant(FB2_DMA_DONE_INTENW::FB2_DMA_DONE_INTEN_0)
    }
    #[doc = "Frame Buffer2 DMA Transfer Done interrupt enable"]
    #[inline]
    pub fn fb2_dma_done_inten_1(self) -> &'a mut W {
        self.variant(FB2_DMA_DONE_INTENW::FB2_DMA_DONE_INTEN_1)
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
        const OFFSET: u8 = 20;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `STATFF_INTEN`"]
pub enum STATFF_INTENW {
    #[doc = "STATFIFO full interrupt disable"]
    STATFF_INTEN_0,
    #[doc = "STATFIFO full interrupt enable"]
    STATFF_INTEN_1,
}
impl STATFF_INTENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            STATFF_INTENW::STATFF_INTEN_0 => false,
            STATFF_INTENW::STATFF_INTEN_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _STATFF_INTENW<'a> {
    w: &'a mut W,
}
impl<'a> _STATFF_INTENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: STATFF_INTENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "STATFIFO full interrupt disable"]
    #[inline]
    pub fn statff_inten_0(self) -> &'a mut W {
        self.variant(STATFF_INTENW::STATFF_INTEN_0)
    }
    #[doc = "STATFIFO full interrupt enable"]
    #[inline]
    pub fn statff_inten_1(self) -> &'a mut W {
        self.variant(STATFF_INTENW::STATFF_INTEN_1)
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
        const OFFSET: u8 = 21;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `SFF_DMA_DONE_INTEN`"]
pub enum SFF_DMA_DONE_INTENW {
    #[doc = "STATFIFO DMA Transfer Done interrupt disable"]
    SFF_DMA_DONE_INTEN_0,
    #[doc = "STATFIFO DMA Transfer Done interrupt enable"]
    SFF_DMA_DONE_INTEN_1,
}
impl SFF_DMA_DONE_INTENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SFF_DMA_DONE_INTENW::SFF_DMA_DONE_INTEN_0 => false,
            SFF_DMA_DONE_INTENW::SFF_DMA_DONE_INTEN_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SFF_DMA_DONE_INTENW<'a> {
    w: &'a mut W,
}
impl<'a> _SFF_DMA_DONE_INTENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SFF_DMA_DONE_INTENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "STATFIFO DMA Transfer Done interrupt disable"]
    #[inline]
    pub fn sff_dma_done_inten_0(self) -> &'a mut W {
        self.variant(SFF_DMA_DONE_INTENW::SFF_DMA_DONE_INTEN_0)
    }
    #[doc = "STATFIFO DMA Transfer Done interrupt enable"]
    #[inline]
    pub fn sff_dma_done_inten_1(self) -> &'a mut W {
        self.variant(SFF_DMA_DONE_INTENW::SFF_DMA_DONE_INTEN_1)
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
        const OFFSET: u8 = 22;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `RF_OR_INTEN`"]
pub enum RF_OR_INTENW {
    #[doc = "RxFIFO overrun interrupt is disabled"]
    RF_OR_INTEN_0,
    #[doc = "RxFIFO overrun interrupt is enabled"]
    RF_OR_INTEN_1,
}
impl RF_OR_INTENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RF_OR_INTENW::RF_OR_INTEN_0 => false,
            RF_OR_INTENW::RF_OR_INTEN_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RF_OR_INTENW<'a> {
    w: &'a mut W,
}
impl<'a> _RF_OR_INTENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RF_OR_INTENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "RxFIFO overrun interrupt is disabled"]
    #[inline]
    pub fn rf_or_inten_0(self) -> &'a mut W {
        self.variant(RF_OR_INTENW::RF_OR_INTEN_0)
    }
    #[doc = "RxFIFO overrun interrupt is enabled"]
    #[inline]
    pub fn rf_or_inten_1(self) -> &'a mut W {
        self.variant(RF_OR_INTENW::RF_OR_INTEN_1)
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
        const OFFSET: u8 = 24;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `SF_OR_INTEN`"]
pub enum SF_OR_INTENW {
    #[doc = "STATFIFO overrun interrupt is disabled"]
    SF_OR_INTEN_0,
    #[doc = "STATFIFO overrun interrupt is enabled"]
    SF_OR_INTEN_1,
}
impl SF_OR_INTENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SF_OR_INTENW::SF_OR_INTEN_0 => false,
            SF_OR_INTENW::SF_OR_INTEN_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SF_OR_INTENW<'a> {
    w: &'a mut W,
}
impl<'a> _SF_OR_INTENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SF_OR_INTENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "STATFIFO overrun interrupt is disabled"]
    #[inline]
    pub fn sf_or_inten_0(self) -> &'a mut W {
        self.variant(SF_OR_INTENW::SF_OR_INTEN_0)
    }
    #[doc = "STATFIFO overrun interrupt is enabled"]
    #[inline]
    pub fn sf_or_inten_1(self) -> &'a mut W {
        self.variant(SF_OR_INTENW::SF_OR_INTEN_1)
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
        const OFFSET: u8 = 25;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `COF_INT_EN`"]
pub enum COF_INT_ENW {
    #[doc = "COF interrupt is disabled"]
    COF_INT_EN_0,
    #[doc = "COF interrupt is enabled"]
    COF_INT_EN_1,
}
impl COF_INT_ENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            COF_INT_ENW::COF_INT_EN_0 => false,
            COF_INT_ENW::COF_INT_EN_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _COF_INT_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _COF_INT_ENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: COF_INT_ENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "COF interrupt is disabled"]
    #[inline]
    pub fn cof_int_en_0(self) -> &'a mut W {
        self.variant(COF_INT_ENW::COF_INT_EN_0)
    }
    #[doc = "COF interrupt is enabled"]
    #[inline]
    pub fn cof_int_en_1(self) -> &'a mut W {
        self.variant(COF_INT_ENW::COF_INT_EN_1)
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
#[doc = "Values that can be written to the field `CCIR_MODE`"]
pub enum CCIR_MODEW {
    #[doc = "Progressive mode is selected"]
    CCIR_MODE_0,
    #[doc = "Interlace mode is selected"]
    CCIR_MODE_1,
}
impl CCIR_MODEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CCIR_MODEW::CCIR_MODE_0 => false,
            CCIR_MODEW::CCIR_MODE_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CCIR_MODEW<'a> {
    w: &'a mut W,
}
impl<'a> _CCIR_MODEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CCIR_MODEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Progressive mode is selected"]
    #[inline]
    pub fn ccir_mode_0(self) -> &'a mut W {
        self.variant(CCIR_MODEW::CCIR_MODE_0)
    }
    #[doc = "Interlace mode is selected"]
    #[inline]
    pub fn ccir_mode_1(self) -> &'a mut W {
        self.variant(CCIR_MODEW::CCIR_MODE_1)
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
        const OFFSET: u8 = 27;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `PrP_IF_EN`"]
pub enum PRP_IF_ENW {
    #[doc = "CSI to PrP bus is disabled"]
    PRP_IF_EN_0,
    #[doc = "CSI to PrP bus is enabled"]
    PRP_IF_EN_1,
}
impl PRP_IF_ENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PRP_IF_ENW::PRP_IF_EN_0 => false,
            PRP_IF_ENW::PRP_IF_EN_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PRP_IF_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _PRP_IF_ENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PRP_IF_ENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "CSI to PrP bus is disabled"]
    #[inline]
    pub fn pr_p_if_en_0(self) -> &'a mut W {
        self.variant(PRP_IF_ENW::PRP_IF_EN_0)
    }
    #[doc = "CSI to PrP bus is enabled"]
    #[inline]
    pub fn pr_p_if_en_1(self) -> &'a mut W {
        self.variant(PRP_IF_ENW::PRP_IF_EN_1)
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
#[doc = "Values that can be written to the field `EOF_INT_EN`"]
pub enum EOF_INT_ENW {
    #[doc = "EOF interrupt is disabled."]
    EOF_INT_EN_0,
    #[doc = "EOF interrupt is generated when RX count value is reached."]
    EOF_INT_EN_1,
}
impl EOF_INT_ENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            EOF_INT_ENW::EOF_INT_EN_0 => false,
            EOF_INT_ENW::EOF_INT_EN_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _EOF_INT_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _EOF_INT_ENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: EOF_INT_ENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "EOF interrupt is disabled."]
    #[inline]
    pub fn eof_int_en_0(self) -> &'a mut W {
        self.variant(EOF_INT_ENW::EOF_INT_EN_0)
    }
    #[doc = "EOF interrupt is generated when RX count value is reached."]
    #[inline]
    pub fn eof_int_en_1(self) -> &'a mut W {
        self.variant(EOF_INT_ENW::EOF_INT_EN_1)
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
        const OFFSET: u8 = 29;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `EXT_VSYNC`"]
pub enum EXT_VSYNCW {
    #[doc = "Internal VSYNC mode"]
    EXT_VSYNC_0,
    #[doc = "External VSYNC mode"]
    EXT_VSYNC_1,
}
impl EXT_VSYNCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            EXT_VSYNCW::EXT_VSYNC_0 => false,
            EXT_VSYNCW::EXT_VSYNC_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _EXT_VSYNCW<'a> {
    w: &'a mut W,
}
impl<'a> _EXT_VSYNCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: EXT_VSYNCW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Internal VSYNC mode"]
    #[inline]
    pub fn ext_vsync_0(self) -> &'a mut W {
        self.variant(EXT_VSYNCW::EXT_VSYNC_0)
    }
    #[doc = "External VSYNC mode"]
    #[inline]
    pub fn ext_vsync_1(self) -> &'a mut W {
        self.variant(EXT_VSYNCW::EXT_VSYNC_1)
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
        const OFFSET: u8 = 30;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `SWAP16_EN`"]
pub enum SWAP16_ENW {
    #[doc = "Disable swapping"]
    SWAP16_EN_0,
    #[doc = "Enable swapping"]
    SWAP16_EN_1,
}
impl SWAP16_ENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SWAP16_ENW::SWAP16_EN_0 => false,
            SWAP16_ENW::SWAP16_EN_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SWAP16_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _SWAP16_ENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SWAP16_ENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable swapping"]
    #[inline]
    pub fn swap16_en_0(self) -> &'a mut W {
        self.variant(SWAP16_ENW::SWAP16_EN_0)
    }
    #[doc = "Enable swapping"]
    #[inline]
    pub fn swap16_en_1(self) -> &'a mut W {
        self.variant(SWAP16_ENW::SWAP16_EN_1)
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
    #[doc = "Bit 0 - Pixel Bit"]
    #[inline]
    pub fn pixel_bit(&self) -> PIXEL_BITR {
        PIXEL_BITR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - Valid Pixel Clock Edge Select"]
    #[inline]
    pub fn redge(&self) -> REDGER {
        REDGER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - Invert Pixel Clock Input"]
    #[inline]
    pub fn inv_pclk(&self) -> INV_PCLKR {
        INV_PCLKR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - Invert Data Input. This bit enables or disables internal inverters on the data lines."]
    #[inline]
    pub fn inv_data(&self) -> INV_DATAR {
        INV_DATAR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 4 - Gated Clock Mode Enable"]
    #[inline]
    pub fn gclk_mode(&self) -> GCLK_MODER {
        GCLK_MODER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 5 - Asynchronous RXFIFO Clear"]
    #[inline]
    pub fn clr_rxfifo(&self) -> CLR_RXFIFOR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        CLR_RXFIFOR { bits }
    }
    #[doc = "Bit 6 - Asynchronous STATFIFO Clear"]
    #[inline]
    pub fn clr_statfifo(&self) -> CLR_STATFIFOR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        CLR_STATFIFOR { bits }
    }
    #[doc = "Bit 7 - Data Packing Direction"]
    #[inline]
    pub fn pack_dir(&self) -> PACK_DIRR {
        PACK_DIRR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 8 - FIFO Clear Control"]
    #[inline]
    pub fn fcc(&self) -> FCCR {
        FCCR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 10 - CCIR656 Interface Enable"]
    #[inline]
    pub fn ccir_en(&self) -> CCIR_ENR {
        CCIR_ENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 11 - HSYNC Polarity Select"]
    #[inline]
    pub fn hsync_pol(&self) -> HSYNC_POLR {
        HSYNC_POLR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 16 - Start Of Frame (SOF) Interrupt Enable. This bit enables the SOF interrupt."]
    #[inline]
    pub fn sof_inten(&self) -> SOF_INTENR {
        SOF_INTENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 17 - SOF Interrupt Polarity. This bit controls the condition that generates an SOF interrupt."]
    #[inline]
    pub fn sof_pol(&self) -> SOF_POLR {
        SOF_POLR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 17;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 18 - RxFIFO Full Interrupt Enable. This bit enables the RxFIFO full interrupt."]
    #[inline]
    pub fn rxff_inten(&self) -> RXFF_INTENR {
        RXFF_INTENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 19 - Frame Buffer1 DMA Transfer Done Interrupt Enable"]
    #[inline]
    pub fn fb1_dma_done_inten(&self) -> FB1_DMA_DONE_INTENR {
        FB1_DMA_DONE_INTENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 19;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 20 - Frame Buffer2 DMA Transfer Done Interrupt Enable"]
    #[inline]
    pub fn fb2_dma_done_inten(&self) -> FB2_DMA_DONE_INTENR {
        FB2_DMA_DONE_INTENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 21 - STATFIFO Full Interrupt Enable. This bit enables the STAT FIFO interrupt."]
    #[inline]
    pub fn statff_inten(&self) -> STATFF_INTENR {
        STATFF_INTENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 21;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 22 - STATFIFO DMA Transfer Done Interrupt Enable"]
    #[inline]
    pub fn sff_dma_done_inten(&self) -> SFF_DMA_DONE_INTENR {
        SFF_DMA_DONE_INTENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 22;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 24 - RxFIFO Overrun Interrupt Enable. This bit enables the RX FIFO overrun interrupt."]
    #[inline]
    pub fn rf_or_inten(&self) -> RF_OR_INTENR {
        RF_OR_INTENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 25 - STAT FIFO Overrun Interrupt Enable. This bit enables the STATFIFO overrun interrupt."]
    #[inline]
    pub fn sf_or_inten(&self) -> SF_OR_INTENR {
        SF_OR_INTENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 25;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 26 - Change Of Image Field (COF) Interrupt Enable"]
    #[inline]
    pub fn cof_int_en(&self) -> COF_INT_ENR {
        COF_INT_ENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 26;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 27 - CCIR Mode Select"]
    #[inline]
    pub fn ccir_mode(&self) -> CCIR_MODER {
        CCIR_MODER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 27;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 28 - CSI-PrP Interface Enable"]
    #[inline]
    pub fn pr_p_if_en(&self) -> PRP_IF_ENR {
        PRP_IF_ENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 28;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 29 - End-of-Frame Interrupt Enable. This bit enables and disables the EOF interrupt."]
    #[inline]
    pub fn eof_int_en(&self) -> EOF_INT_ENR {
        EOF_INT_ENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 29;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 30 - External VSYNC Enable"]
    #[inline]
    pub fn ext_vsync(&self) -> EXT_VSYNCR {
        EXT_VSYNCR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 30;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 31 - SWAP 16-Bit Enable"]
    #[inline]
    pub fn swap16_en(&self) -> SWAP16_ENR {
        SWAP16_ENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 31;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 1073743872 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - Pixel Bit"]
    #[inline]
    pub fn pixel_bit(&mut self) -> _PIXEL_BITW {
        _PIXEL_BITW { w: self }
    }
    #[doc = "Bit 1 - Valid Pixel Clock Edge Select"]
    #[inline]
    pub fn redge(&mut self) -> _REDGEW {
        _REDGEW { w: self }
    }
    #[doc = "Bit 2 - Invert Pixel Clock Input"]
    #[inline]
    pub fn inv_pclk(&mut self) -> _INV_PCLKW {
        _INV_PCLKW { w: self }
    }
    #[doc = "Bit 3 - Invert Data Input. This bit enables or disables internal inverters on the data lines."]
    #[inline]
    pub fn inv_data(&mut self) -> _INV_DATAW {
        _INV_DATAW { w: self }
    }
    #[doc = "Bit 4 - Gated Clock Mode Enable"]
    #[inline]
    pub fn gclk_mode(&mut self) -> _GCLK_MODEW {
        _GCLK_MODEW { w: self }
    }
    #[doc = "Bit 5 - Asynchronous RXFIFO Clear"]
    #[inline]
    pub fn clr_rxfifo(&mut self) -> _CLR_RXFIFOW {
        _CLR_RXFIFOW { w: self }
    }
    #[doc = "Bit 6 - Asynchronous STATFIFO Clear"]
    #[inline]
    pub fn clr_statfifo(&mut self) -> _CLR_STATFIFOW {
        _CLR_STATFIFOW { w: self }
    }
    #[doc = "Bit 7 - Data Packing Direction"]
    #[inline]
    pub fn pack_dir(&mut self) -> _PACK_DIRW {
        _PACK_DIRW { w: self }
    }
    #[doc = "Bit 8 - FIFO Clear Control"]
    #[inline]
    pub fn fcc(&mut self) -> _FCCW {
        _FCCW { w: self }
    }
    #[doc = "Bit 10 - CCIR656 Interface Enable"]
    #[inline]
    pub fn ccir_en(&mut self) -> _CCIR_ENW {
        _CCIR_ENW { w: self }
    }
    #[doc = "Bit 11 - HSYNC Polarity Select"]
    #[inline]
    pub fn hsync_pol(&mut self) -> _HSYNC_POLW {
        _HSYNC_POLW { w: self }
    }
    #[doc = "Bit 16 - Start Of Frame (SOF) Interrupt Enable. This bit enables the SOF interrupt."]
    #[inline]
    pub fn sof_inten(&mut self) -> _SOF_INTENW {
        _SOF_INTENW { w: self }
    }
    #[doc = "Bit 17 - SOF Interrupt Polarity. This bit controls the condition that generates an SOF interrupt."]
    #[inline]
    pub fn sof_pol(&mut self) -> _SOF_POLW {
        _SOF_POLW { w: self }
    }
    #[doc = "Bit 18 - RxFIFO Full Interrupt Enable. This bit enables the RxFIFO full interrupt."]
    #[inline]
    pub fn rxff_inten(&mut self) -> _RXFF_INTENW {
        _RXFF_INTENW { w: self }
    }
    #[doc = "Bit 19 - Frame Buffer1 DMA Transfer Done Interrupt Enable"]
    #[inline]
    pub fn fb1_dma_done_inten(&mut self) -> _FB1_DMA_DONE_INTENW {
        _FB1_DMA_DONE_INTENW { w: self }
    }
    #[doc = "Bit 20 - Frame Buffer2 DMA Transfer Done Interrupt Enable"]
    #[inline]
    pub fn fb2_dma_done_inten(&mut self) -> _FB2_DMA_DONE_INTENW {
        _FB2_DMA_DONE_INTENW { w: self }
    }
    #[doc = "Bit 21 - STATFIFO Full Interrupt Enable. This bit enables the STAT FIFO interrupt."]
    #[inline]
    pub fn statff_inten(&mut self) -> _STATFF_INTENW {
        _STATFF_INTENW { w: self }
    }
    #[doc = "Bit 22 - STATFIFO DMA Transfer Done Interrupt Enable"]
    #[inline]
    pub fn sff_dma_done_inten(&mut self) -> _SFF_DMA_DONE_INTENW {
        _SFF_DMA_DONE_INTENW { w: self }
    }
    #[doc = "Bit 24 - RxFIFO Overrun Interrupt Enable. This bit enables the RX FIFO overrun interrupt."]
    #[inline]
    pub fn rf_or_inten(&mut self) -> _RF_OR_INTENW {
        _RF_OR_INTENW { w: self }
    }
    #[doc = "Bit 25 - STAT FIFO Overrun Interrupt Enable. This bit enables the STATFIFO overrun interrupt."]
    #[inline]
    pub fn sf_or_inten(&mut self) -> _SF_OR_INTENW {
        _SF_OR_INTENW { w: self }
    }
    #[doc = "Bit 26 - Change Of Image Field (COF) Interrupt Enable"]
    #[inline]
    pub fn cof_int_en(&mut self) -> _COF_INT_ENW {
        _COF_INT_ENW { w: self }
    }
    #[doc = "Bit 27 - CCIR Mode Select"]
    #[inline]
    pub fn ccir_mode(&mut self) -> _CCIR_MODEW {
        _CCIR_MODEW { w: self }
    }
    #[doc = "Bit 28 - CSI-PrP Interface Enable"]
    #[inline]
    pub fn pr_p_if_en(&mut self) -> _PRP_IF_ENW {
        _PRP_IF_ENW { w: self }
    }
    #[doc = "Bit 29 - End-of-Frame Interrupt Enable. This bit enables and disables the EOF interrupt."]
    #[inline]
    pub fn eof_int_en(&mut self) -> _EOF_INT_ENW {
        _EOF_INT_ENW { w: self }
    }
    #[doc = "Bit 30 - External VSYNC Enable"]
    #[inline]
    pub fn ext_vsync(&mut self) -> _EXT_VSYNCW {
        _EXT_VSYNCW { w: self }
    }
    #[doc = "Bit 31 - SWAP 16-Bit Enable"]
    #[inline]
    pub fn swap16_en(&mut self) -> _SWAP16_ENW {
        _SWAP16_ENW { w: self }
    }
}
