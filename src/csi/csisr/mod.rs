#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CSISR {
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
#[doc = "Possible values of the field `DRDY`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DRDYR {
    #[doc = "No data (word) is ready"]
    DRDY_0,
    #[doc = "At least 1 datum (word) is ready in RXFIFO."]
    DRDY_1,
}
impl DRDYR {
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
            DRDYR::DRDY_0 => false,
            DRDYR::DRDY_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DRDYR {
        match value {
            false => DRDYR::DRDY_0,
            true => DRDYR::DRDY_1,
        }
    }
    #[doc = "Checks if the value of the field is `DRDY_0`"]
    #[inline]
    pub fn is_drdy_0(&self) -> bool {
        *self == DRDYR::DRDY_0
    }
    #[doc = "Checks if the value of the field is `DRDY_1`"]
    #[inline]
    pub fn is_drdy_1(&self) -> bool {
        *self == DRDYR::DRDY_1
    }
}
#[doc = "Possible values of the field `ECC_INT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ECC_INTR {
    #[doc = "No error detected"]
    ECC_INT_0,
    #[doc = "Error is detected in CCIR coding"]
    ECC_INT_1,
}
impl ECC_INTR {
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
            ECC_INTR::ECC_INT_0 => false,
            ECC_INTR::ECC_INT_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ECC_INTR {
        match value {
            false => ECC_INTR::ECC_INT_0,
            true => ECC_INTR::ECC_INT_1,
        }
    }
    #[doc = "Checks if the value of the field is `ECC_INT_0`"]
    #[inline]
    pub fn is_ecc_int_0(&self) -> bool {
        *self == ECC_INTR::ECC_INT_0
    }
    #[doc = "Checks if the value of the field is `ECC_INT_1`"]
    #[inline]
    pub fn is_ecc_int_1(&self) -> bool {
        *self == ECC_INTR::ECC_INT_1
    }
}
#[doc = "Possible values of the field `HRESP_ERR_INT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HRESP_ERR_INTR {
    #[doc = "No hresponse error."]
    HRESP_ERR_INT_0,
    #[doc = "Hresponse error is detected."]
    HRESP_ERR_INT_1,
}
impl HRESP_ERR_INTR {
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
            HRESP_ERR_INTR::HRESP_ERR_INT_0 => false,
            HRESP_ERR_INTR::HRESP_ERR_INT_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> HRESP_ERR_INTR {
        match value {
            false => HRESP_ERR_INTR::HRESP_ERR_INT_0,
            true => HRESP_ERR_INTR::HRESP_ERR_INT_1,
        }
    }
    #[doc = "Checks if the value of the field is `HRESP_ERR_INT_0`"]
    #[inline]
    pub fn is_hresp_err_int_0(&self) -> bool {
        *self == HRESP_ERR_INTR::HRESP_ERR_INT_0
    }
    #[doc = "Checks if the value of the field is `HRESP_ERR_INT_1`"]
    #[inline]
    pub fn is_hresp_err_int_1(&self) -> bool {
        *self == HRESP_ERR_INTR::HRESP_ERR_INT_1
    }
}
#[doc = "Possible values of the field `COF_INT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum COF_INTR {
    #[doc = "Video field has no change."]
    COF_INT_0,
    #[doc = "Change of video field is detected."]
    COF_INT_1,
}
impl COF_INTR {
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
            COF_INTR::COF_INT_0 => false,
            COF_INTR::COF_INT_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> COF_INTR {
        match value {
            false => COF_INTR::COF_INT_0,
            true => COF_INTR::COF_INT_1,
        }
    }
    #[doc = "Checks if the value of the field is `COF_INT_0`"]
    #[inline]
    pub fn is_cof_int_0(&self) -> bool {
        *self == COF_INTR::COF_INT_0
    }
    #[doc = "Checks if the value of the field is `COF_INT_1`"]
    #[inline]
    pub fn is_cof_int_1(&self) -> bool {
        *self == COF_INTR::COF_INT_1
    }
}
#[doc = "Possible values of the field `F1_INT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum F1_INTR {
    #[doc = "Field 1 of video is not detected."]
    F1_INT_0,
    #[doc = "Field 1 of video is about to start."]
    F1_INT_1,
}
impl F1_INTR {
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
            F1_INTR::F1_INT_0 => false,
            F1_INTR::F1_INT_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> F1_INTR {
        match value {
            false => F1_INTR::F1_INT_0,
            true => F1_INTR::F1_INT_1,
        }
    }
    #[doc = "Checks if the value of the field is `F1_INT_0`"]
    #[inline]
    pub fn is_f1_int_0(&self) -> bool {
        *self == F1_INTR::F1_INT_0
    }
    #[doc = "Checks if the value of the field is `F1_INT_1`"]
    #[inline]
    pub fn is_f1_int_1(&self) -> bool {
        *self == F1_INTR::F1_INT_1
    }
}
#[doc = "Possible values of the field `F2_INT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum F2_INTR {
    #[doc = "Field 2 of video is not detected"]
    F2_INT_0,
    #[doc = "Field 2 of video is about to start"]
    F2_INT_1,
}
impl F2_INTR {
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
            F2_INTR::F2_INT_0 => false,
            F2_INTR::F2_INT_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> F2_INTR {
        match value {
            false => F2_INTR::F2_INT_0,
            true => F2_INTR::F2_INT_1,
        }
    }
    #[doc = "Checks if the value of the field is `F2_INT_0`"]
    #[inline]
    pub fn is_f2_int_0(&self) -> bool {
        *self == F2_INTR::F2_INT_0
    }
    #[doc = "Checks if the value of the field is `F2_INT_1`"]
    #[inline]
    pub fn is_f2_int_1(&self) -> bool {
        *self == F2_INTR::F2_INT_1
    }
}
#[doc = "Possible values of the field `SOF_INT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SOF_INTR {
    #[doc = "SOF is not detected."]
    SOF_INT_0,
    #[doc = "SOF is detected."]
    SOF_INT_1,
}
impl SOF_INTR {
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
            SOF_INTR::SOF_INT_0 => false,
            SOF_INTR::SOF_INT_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SOF_INTR {
        match value {
            false => SOF_INTR::SOF_INT_0,
            true => SOF_INTR::SOF_INT_1,
        }
    }
    #[doc = "Checks if the value of the field is `SOF_INT_0`"]
    #[inline]
    pub fn is_sof_int_0(&self) -> bool {
        *self == SOF_INTR::SOF_INT_0
    }
    #[doc = "Checks if the value of the field is `SOF_INT_1`"]
    #[inline]
    pub fn is_sof_int_1(&self) -> bool {
        *self == SOF_INTR::SOF_INT_1
    }
}
#[doc = "Possible values of the field `EOF_INT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EOF_INTR {
    #[doc = "EOF is not detected."]
    EOF_INT_0,
    #[doc = "EOF is detected."]
    EOF_INT_1,
}
impl EOF_INTR {
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
            EOF_INTR::EOF_INT_0 => false,
            EOF_INTR::EOF_INT_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> EOF_INTR {
        match value {
            false => EOF_INTR::EOF_INT_0,
            true => EOF_INTR::EOF_INT_1,
        }
    }
    #[doc = "Checks if the value of the field is `EOF_INT_0`"]
    #[inline]
    pub fn is_eof_int_0(&self) -> bool {
        *self == EOF_INTR::EOF_INT_0
    }
    #[doc = "Checks if the value of the field is `EOF_INT_1`"]
    #[inline]
    pub fn is_eof_int_1(&self) -> bool {
        *self == EOF_INTR::EOF_INT_1
    }
}
#[doc = "Possible values of the field `RxFF_INT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXFF_INTR {
    #[doc = "RxFIFO is not full."]
    RXFF_INT_0,
    #[doc = "RxFIFO is full."]
    RXFF_INT_1,
}
impl RXFF_INTR {
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
            RXFF_INTR::RXFF_INT_0 => false,
            RXFF_INTR::RXFF_INT_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RXFF_INTR {
        match value {
            false => RXFF_INTR::RXFF_INT_0,
            true => RXFF_INTR::RXFF_INT_1,
        }
    }
    #[doc = "Checks if the value of the field is `RXFF_INT_0`"]
    #[inline]
    pub fn is_rx_ff_int_0(&self) -> bool {
        *self == RXFF_INTR::RXFF_INT_0
    }
    #[doc = "Checks if the value of the field is `RXFF_INT_1`"]
    #[inline]
    pub fn is_rx_ff_int_1(&self) -> bool {
        *self == RXFF_INTR::RXFF_INT_1
    }
}
#[doc = "Possible values of the field `DMA_TSF_DONE_FB1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DMA_TSF_DONE_FB1R {
    #[doc = "DMA transfer is not completed."]
    DMA_TSF_DONE_FB1_0,
    #[doc = "DMA transfer is completed."]
    DMA_TSF_DONE_FB1_1,
}
impl DMA_TSF_DONE_FB1R {
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
            DMA_TSF_DONE_FB1R::DMA_TSF_DONE_FB1_0 => false,
            DMA_TSF_DONE_FB1R::DMA_TSF_DONE_FB1_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DMA_TSF_DONE_FB1R {
        match value {
            false => DMA_TSF_DONE_FB1R::DMA_TSF_DONE_FB1_0,
            true => DMA_TSF_DONE_FB1R::DMA_TSF_DONE_FB1_1,
        }
    }
    #[doc = "Checks if the value of the field is `DMA_TSF_DONE_FB1_0`"]
    #[inline]
    pub fn is_dma_tsf_done_fb1_0(&self) -> bool {
        *self == DMA_TSF_DONE_FB1R::DMA_TSF_DONE_FB1_0
    }
    #[doc = "Checks if the value of the field is `DMA_TSF_DONE_FB1_1`"]
    #[inline]
    pub fn is_dma_tsf_done_fb1_1(&self) -> bool {
        *self == DMA_TSF_DONE_FB1R::DMA_TSF_DONE_FB1_1
    }
}
#[doc = "Possible values of the field `DMA_TSF_DONE_FB2`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DMA_TSF_DONE_FB2R {
    #[doc = "DMA transfer is not completed."]
    DMA_TSF_DONE_FB2_0,
    #[doc = "DMA transfer is completed."]
    DMA_TSF_DONE_FB2_1,
}
impl DMA_TSF_DONE_FB2R {
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
            DMA_TSF_DONE_FB2R::DMA_TSF_DONE_FB2_0 => false,
            DMA_TSF_DONE_FB2R::DMA_TSF_DONE_FB2_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DMA_TSF_DONE_FB2R {
        match value {
            false => DMA_TSF_DONE_FB2R::DMA_TSF_DONE_FB2_0,
            true => DMA_TSF_DONE_FB2R::DMA_TSF_DONE_FB2_1,
        }
    }
    #[doc = "Checks if the value of the field is `DMA_TSF_DONE_FB2_0`"]
    #[inline]
    pub fn is_dma_tsf_done_fb2_0(&self) -> bool {
        *self == DMA_TSF_DONE_FB2R::DMA_TSF_DONE_FB2_0
    }
    #[doc = "Checks if the value of the field is `DMA_TSF_DONE_FB2_1`"]
    #[inline]
    pub fn is_dma_tsf_done_fb2_1(&self) -> bool {
        *self == DMA_TSF_DONE_FB2R::DMA_TSF_DONE_FB2_1
    }
}
#[doc = "Possible values of the field `STATFF_INT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum STATFF_INTR {
    #[doc = "STATFIFO is not full."]
    STATFF_INT_0,
    #[doc = "STATFIFO is full."]
    STATFF_INT_1,
}
impl STATFF_INTR {
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
            STATFF_INTR::STATFF_INT_0 => false,
            STATFF_INTR::STATFF_INT_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> STATFF_INTR {
        match value {
            false => STATFF_INTR::STATFF_INT_0,
            true => STATFF_INTR::STATFF_INT_1,
        }
    }
    #[doc = "Checks if the value of the field is `STATFF_INT_0`"]
    #[inline]
    pub fn is_statff_int_0(&self) -> bool {
        *self == STATFF_INTR::STATFF_INT_0
    }
    #[doc = "Checks if the value of the field is `STATFF_INT_1`"]
    #[inline]
    pub fn is_statff_int_1(&self) -> bool {
        *self == STATFF_INTR::STATFF_INT_1
    }
}
#[doc = "Possible values of the field `DMA_TSF_DONE_SFF`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DMA_TSF_DONE_SFFR {
    #[doc = "DMA transfer is not completed."]
    DMA_TSF_DONE_SFF_0,
    #[doc = "DMA transfer is completed."]
    DMA_TSF_DONE_SFF_1,
}
impl DMA_TSF_DONE_SFFR {
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
            DMA_TSF_DONE_SFFR::DMA_TSF_DONE_SFF_0 => false,
            DMA_TSF_DONE_SFFR::DMA_TSF_DONE_SFF_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DMA_TSF_DONE_SFFR {
        match value {
            false => DMA_TSF_DONE_SFFR::DMA_TSF_DONE_SFF_0,
            true => DMA_TSF_DONE_SFFR::DMA_TSF_DONE_SFF_1,
        }
    }
    #[doc = "Checks if the value of the field is `DMA_TSF_DONE_SFF_0`"]
    #[inline]
    pub fn is_dma_tsf_done_sff_0(&self) -> bool {
        *self == DMA_TSF_DONE_SFFR::DMA_TSF_DONE_SFF_0
    }
    #[doc = "Checks if the value of the field is `DMA_TSF_DONE_SFF_1`"]
    #[inline]
    pub fn is_dma_tsf_done_sff_1(&self) -> bool {
        *self == DMA_TSF_DONE_SFFR::DMA_TSF_DONE_SFF_1
    }
}
#[doc = "Possible values of the field `RF_OR_INT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RF_OR_INTR {
    #[doc = "RXFIFO has not overflowed."]
    RF_OR_INT_0,
    #[doc = "RXFIFO has overflowed."]
    RF_OR_INT_1,
}
impl RF_OR_INTR {
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
            RF_OR_INTR::RF_OR_INT_0 => false,
            RF_OR_INTR::RF_OR_INT_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RF_OR_INTR {
        match value {
            false => RF_OR_INTR::RF_OR_INT_0,
            true => RF_OR_INTR::RF_OR_INT_1,
        }
    }
    #[doc = "Checks if the value of the field is `RF_OR_INT_0`"]
    #[inline]
    pub fn is_rf_or_int_0(&self) -> bool {
        *self == RF_OR_INTR::RF_OR_INT_0
    }
    #[doc = "Checks if the value of the field is `RF_OR_INT_1`"]
    #[inline]
    pub fn is_rf_or_int_1(&self) -> bool {
        *self == RF_OR_INTR::RF_OR_INT_1
    }
}
#[doc = "Possible values of the field `SF_OR_INT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SF_OR_INTR {
    #[doc = "STATFIFO has not overflowed."]
    SF_OR_INT_0,
    #[doc = "STATFIFO has overflowed."]
    SF_OR_INT_1,
}
impl SF_OR_INTR {
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
            SF_OR_INTR::SF_OR_INT_0 => false,
            SF_OR_INTR::SF_OR_INT_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SF_OR_INTR {
        match value {
            false => SF_OR_INTR::SF_OR_INT_0,
            true => SF_OR_INTR::SF_OR_INT_1,
        }
    }
    #[doc = "Checks if the value of the field is `SF_OR_INT_0`"]
    #[inline]
    pub fn is_sf_or_int_0(&self) -> bool {
        *self == SF_OR_INTR::SF_OR_INT_0
    }
    #[doc = "Checks if the value of the field is `SF_OR_INT_1`"]
    #[inline]
    pub fn is_sf_or_int_1(&self) -> bool {
        *self == SF_OR_INTR::SF_OR_INT_1
    }
}
#[doc = r" Value of the field"]
pub struct DMA_FIELD1_DONER {
    bits: bool,
}
impl DMA_FIELD1_DONER {
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
pub struct DMA_FIELD0_DONER {
    bits: bool,
}
impl DMA_FIELD0_DONER {
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
pub struct BASEADDR_CHHANGE_ERRORR {
    bits: bool,
}
impl BASEADDR_CHHANGE_ERRORR {
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
#[doc = "Values that can be written to the field `DRDY`"]
pub enum DRDYW {
    #[doc = "No data (word) is ready"]
    DRDY_0,
    #[doc = "At least 1 datum (word) is ready in RXFIFO."]
    DRDY_1,
}
impl DRDYW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DRDYW::DRDY_0 => false,
            DRDYW::DRDY_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DRDYW<'a> {
    w: &'a mut W,
}
impl<'a> _DRDYW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DRDYW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No data (word) is ready"]
    #[inline]
    pub fn drdy_0(self) -> &'a mut W {
        self.variant(DRDYW::DRDY_0)
    }
    #[doc = "At least 1 datum (word) is ready in RXFIFO."]
    #[inline]
    pub fn drdy_1(self) -> &'a mut W {
        self.variant(DRDYW::DRDY_1)
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
#[doc = "Values that can be written to the field `ECC_INT`"]
pub enum ECC_INTW {
    #[doc = "No error detected"]
    ECC_INT_0,
    #[doc = "Error is detected in CCIR coding"]
    ECC_INT_1,
}
impl ECC_INTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ECC_INTW::ECC_INT_0 => false,
            ECC_INTW::ECC_INT_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ECC_INTW<'a> {
    w: &'a mut W,
}
impl<'a> _ECC_INTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ECC_INTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No error detected"]
    #[inline]
    pub fn ecc_int_0(self) -> &'a mut W {
        self.variant(ECC_INTW::ECC_INT_0)
    }
    #[doc = "Error is detected in CCIR coding"]
    #[inline]
    pub fn ecc_int_1(self) -> &'a mut W {
        self.variant(ECC_INTW::ECC_INT_1)
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
#[doc = "Values that can be written to the field `HRESP_ERR_INT`"]
pub enum HRESP_ERR_INTW {
    #[doc = "No hresponse error."]
    HRESP_ERR_INT_0,
    #[doc = "Hresponse error is detected."]
    HRESP_ERR_INT_1,
}
impl HRESP_ERR_INTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            HRESP_ERR_INTW::HRESP_ERR_INT_0 => false,
            HRESP_ERR_INTW::HRESP_ERR_INT_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _HRESP_ERR_INTW<'a> {
    w: &'a mut W,
}
impl<'a> _HRESP_ERR_INTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: HRESP_ERR_INTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No hresponse error."]
    #[inline]
    pub fn hresp_err_int_0(self) -> &'a mut W {
        self.variant(HRESP_ERR_INTW::HRESP_ERR_INT_0)
    }
    #[doc = "Hresponse error is detected."]
    #[inline]
    pub fn hresp_err_int_1(self) -> &'a mut W {
        self.variant(HRESP_ERR_INTW::HRESP_ERR_INT_1)
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
#[doc = "Values that can be written to the field `COF_INT`"]
pub enum COF_INTW {
    #[doc = "Video field has no change."]
    COF_INT_0,
    #[doc = "Change of video field is detected."]
    COF_INT_1,
}
impl COF_INTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            COF_INTW::COF_INT_0 => false,
            COF_INTW::COF_INT_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _COF_INTW<'a> {
    w: &'a mut W,
}
impl<'a> _COF_INTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: COF_INTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Video field has no change."]
    #[inline]
    pub fn cof_int_0(self) -> &'a mut W {
        self.variant(COF_INTW::COF_INT_0)
    }
    #[doc = "Change of video field is detected."]
    #[inline]
    pub fn cof_int_1(self) -> &'a mut W {
        self.variant(COF_INTW::COF_INT_1)
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
        const OFFSET: u8 = 13;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `F1_INT`"]
pub enum F1_INTW {
    #[doc = "Field 1 of video is not detected."]
    F1_INT_0,
    #[doc = "Field 1 of video is about to start."]
    F1_INT_1,
}
impl F1_INTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            F1_INTW::F1_INT_0 => false,
            F1_INTW::F1_INT_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _F1_INTW<'a> {
    w: &'a mut W,
}
impl<'a> _F1_INTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: F1_INTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Field 1 of video is not detected."]
    #[inline]
    pub fn f1_int_0(self) -> &'a mut W {
        self.variant(F1_INTW::F1_INT_0)
    }
    #[doc = "Field 1 of video is about to start."]
    #[inline]
    pub fn f1_int_1(self) -> &'a mut W {
        self.variant(F1_INTW::F1_INT_1)
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
        const OFFSET: u8 = 14;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `F2_INT`"]
pub enum F2_INTW {
    #[doc = "Field 2 of video is not detected"]
    F2_INT_0,
    #[doc = "Field 2 of video is about to start"]
    F2_INT_1,
}
impl F2_INTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            F2_INTW::F2_INT_0 => false,
            F2_INTW::F2_INT_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _F2_INTW<'a> {
    w: &'a mut W,
}
impl<'a> _F2_INTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: F2_INTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Field 2 of video is not detected"]
    #[inline]
    pub fn f2_int_0(self) -> &'a mut W {
        self.variant(F2_INTW::F2_INT_0)
    }
    #[doc = "Field 2 of video is about to start"]
    #[inline]
    pub fn f2_int_1(self) -> &'a mut W {
        self.variant(F2_INTW::F2_INT_1)
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
        const OFFSET: u8 = 15;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `SOF_INT`"]
pub enum SOF_INTW {
    #[doc = "SOF is not detected."]
    SOF_INT_0,
    #[doc = "SOF is detected."]
    SOF_INT_1,
}
impl SOF_INTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SOF_INTW::SOF_INT_0 => false,
            SOF_INTW::SOF_INT_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SOF_INTW<'a> {
    w: &'a mut W,
}
impl<'a> _SOF_INTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SOF_INTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "SOF is not detected."]
    #[inline]
    pub fn sof_int_0(self) -> &'a mut W {
        self.variant(SOF_INTW::SOF_INT_0)
    }
    #[doc = "SOF is detected."]
    #[inline]
    pub fn sof_int_1(self) -> &'a mut W {
        self.variant(SOF_INTW::SOF_INT_1)
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
#[doc = "Values that can be written to the field `EOF_INT`"]
pub enum EOF_INTW {
    #[doc = "EOF is not detected."]
    EOF_INT_0,
    #[doc = "EOF is detected."]
    EOF_INT_1,
}
impl EOF_INTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            EOF_INTW::EOF_INT_0 => false,
            EOF_INTW::EOF_INT_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _EOF_INTW<'a> {
    w: &'a mut W,
}
impl<'a> _EOF_INTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: EOF_INTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "EOF is not detected."]
    #[inline]
    pub fn eof_int_0(self) -> &'a mut W {
        self.variant(EOF_INTW::EOF_INT_0)
    }
    #[doc = "EOF is detected."]
    #[inline]
    pub fn eof_int_1(self) -> &'a mut W {
        self.variant(EOF_INTW::EOF_INT_1)
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
#[doc = "Values that can be written to the field `RxFF_INT`"]
pub enum RXFF_INTW {
    #[doc = "RxFIFO is not full."]
    RXFF_INT_0,
    #[doc = "RxFIFO is full."]
    RXFF_INT_1,
}
impl RXFF_INTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RXFF_INTW::RXFF_INT_0 => false,
            RXFF_INTW::RXFF_INT_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RXFF_INTW<'a> {
    w: &'a mut W,
}
impl<'a> _RXFF_INTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RXFF_INTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "RxFIFO is not full."]
    #[inline]
    pub fn rx_ff_int_0(self) -> &'a mut W {
        self.variant(RXFF_INTW::RXFF_INT_0)
    }
    #[doc = "RxFIFO is full."]
    #[inline]
    pub fn rx_ff_int_1(self) -> &'a mut W {
        self.variant(RXFF_INTW::RXFF_INT_1)
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
#[doc = "Values that can be written to the field `DMA_TSF_DONE_FB1`"]
pub enum DMA_TSF_DONE_FB1W {
    #[doc = "DMA transfer is not completed."]
    DMA_TSF_DONE_FB1_0,
    #[doc = "DMA transfer is completed."]
    DMA_TSF_DONE_FB1_1,
}
impl DMA_TSF_DONE_FB1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DMA_TSF_DONE_FB1W::DMA_TSF_DONE_FB1_0 => false,
            DMA_TSF_DONE_FB1W::DMA_TSF_DONE_FB1_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DMA_TSF_DONE_FB1W<'a> {
    w: &'a mut W,
}
impl<'a> _DMA_TSF_DONE_FB1W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DMA_TSF_DONE_FB1W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "DMA transfer is not completed."]
    #[inline]
    pub fn dma_tsf_done_fb1_0(self) -> &'a mut W {
        self.variant(DMA_TSF_DONE_FB1W::DMA_TSF_DONE_FB1_0)
    }
    #[doc = "DMA transfer is completed."]
    #[inline]
    pub fn dma_tsf_done_fb1_1(self) -> &'a mut W {
        self.variant(DMA_TSF_DONE_FB1W::DMA_TSF_DONE_FB1_1)
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
#[doc = "Values that can be written to the field `DMA_TSF_DONE_FB2`"]
pub enum DMA_TSF_DONE_FB2W {
    #[doc = "DMA transfer is not completed."]
    DMA_TSF_DONE_FB2_0,
    #[doc = "DMA transfer is completed."]
    DMA_TSF_DONE_FB2_1,
}
impl DMA_TSF_DONE_FB2W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DMA_TSF_DONE_FB2W::DMA_TSF_DONE_FB2_0 => false,
            DMA_TSF_DONE_FB2W::DMA_TSF_DONE_FB2_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DMA_TSF_DONE_FB2W<'a> {
    w: &'a mut W,
}
impl<'a> _DMA_TSF_DONE_FB2W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DMA_TSF_DONE_FB2W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "DMA transfer is not completed."]
    #[inline]
    pub fn dma_tsf_done_fb2_0(self) -> &'a mut W {
        self.variant(DMA_TSF_DONE_FB2W::DMA_TSF_DONE_FB2_0)
    }
    #[doc = "DMA transfer is completed."]
    #[inline]
    pub fn dma_tsf_done_fb2_1(self) -> &'a mut W {
        self.variant(DMA_TSF_DONE_FB2W::DMA_TSF_DONE_FB2_1)
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
#[doc = "Values that can be written to the field `STATFF_INT`"]
pub enum STATFF_INTW {
    #[doc = "STATFIFO is not full."]
    STATFF_INT_0,
    #[doc = "STATFIFO is full."]
    STATFF_INT_1,
}
impl STATFF_INTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            STATFF_INTW::STATFF_INT_0 => false,
            STATFF_INTW::STATFF_INT_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _STATFF_INTW<'a> {
    w: &'a mut W,
}
impl<'a> _STATFF_INTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: STATFF_INTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "STATFIFO is not full."]
    #[inline]
    pub fn statff_int_0(self) -> &'a mut W {
        self.variant(STATFF_INTW::STATFF_INT_0)
    }
    #[doc = "STATFIFO is full."]
    #[inline]
    pub fn statff_int_1(self) -> &'a mut W {
        self.variant(STATFF_INTW::STATFF_INT_1)
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
#[doc = "Values that can be written to the field `DMA_TSF_DONE_SFF`"]
pub enum DMA_TSF_DONE_SFFW {
    #[doc = "DMA transfer is not completed."]
    DMA_TSF_DONE_SFF_0,
    #[doc = "DMA transfer is completed."]
    DMA_TSF_DONE_SFF_1,
}
impl DMA_TSF_DONE_SFFW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DMA_TSF_DONE_SFFW::DMA_TSF_DONE_SFF_0 => false,
            DMA_TSF_DONE_SFFW::DMA_TSF_DONE_SFF_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DMA_TSF_DONE_SFFW<'a> {
    w: &'a mut W,
}
impl<'a> _DMA_TSF_DONE_SFFW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DMA_TSF_DONE_SFFW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "DMA transfer is not completed."]
    #[inline]
    pub fn dma_tsf_done_sff_0(self) -> &'a mut W {
        self.variant(DMA_TSF_DONE_SFFW::DMA_TSF_DONE_SFF_0)
    }
    #[doc = "DMA transfer is completed."]
    #[inline]
    pub fn dma_tsf_done_sff_1(self) -> &'a mut W {
        self.variant(DMA_TSF_DONE_SFFW::DMA_TSF_DONE_SFF_1)
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
#[doc = "Values that can be written to the field `RF_OR_INT`"]
pub enum RF_OR_INTW {
    #[doc = "RXFIFO has not overflowed."]
    RF_OR_INT_0,
    #[doc = "RXFIFO has overflowed."]
    RF_OR_INT_1,
}
impl RF_OR_INTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RF_OR_INTW::RF_OR_INT_0 => false,
            RF_OR_INTW::RF_OR_INT_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RF_OR_INTW<'a> {
    w: &'a mut W,
}
impl<'a> _RF_OR_INTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RF_OR_INTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "RXFIFO has not overflowed."]
    #[inline]
    pub fn rf_or_int_0(self) -> &'a mut W {
        self.variant(RF_OR_INTW::RF_OR_INT_0)
    }
    #[doc = "RXFIFO has overflowed."]
    #[inline]
    pub fn rf_or_int_1(self) -> &'a mut W {
        self.variant(RF_OR_INTW::RF_OR_INT_1)
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
#[doc = "Values that can be written to the field `SF_OR_INT`"]
pub enum SF_OR_INTW {
    #[doc = "STATFIFO has not overflowed."]
    SF_OR_INT_0,
    #[doc = "STATFIFO has overflowed."]
    SF_OR_INT_1,
}
impl SF_OR_INTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SF_OR_INTW::SF_OR_INT_0 => false,
            SF_OR_INTW::SF_OR_INT_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SF_OR_INTW<'a> {
    w: &'a mut W,
}
impl<'a> _SF_OR_INTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SF_OR_INTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "STATFIFO has not overflowed."]
    #[inline]
    pub fn sf_or_int_0(self) -> &'a mut W {
        self.variant(SF_OR_INTW::SF_OR_INT_0)
    }
    #[doc = "STATFIFO has overflowed."]
    #[inline]
    pub fn sf_or_int_1(self) -> &'a mut W {
        self.variant(SF_OR_INTW::SF_OR_INT_1)
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
#[doc = r" Proxy"]
pub struct _DMA_FIELD1_DONEW<'a> {
    w: &'a mut W,
}
impl<'a> _DMA_FIELD1_DONEW<'a> {
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
#[doc = r" Proxy"]
pub struct _DMA_FIELD0_DONEW<'a> {
    w: &'a mut W,
}
impl<'a> _DMA_FIELD0_DONEW<'a> {
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
#[doc = r" Proxy"]
pub struct _BASEADDR_CHHANGE_ERRORW<'a> {
    w: &'a mut W,
}
impl<'a> _BASEADDR_CHHANGE_ERRORW<'a> {
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
    #[doc = "Bit 0 - RXFIFO Data Ready"]
    #[inline]
    pub fn drdy(&self) -> DRDYR {
        DRDYR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - CCIR Error Interrupt"]
    #[inline]
    pub fn ecc_int(&self) -> ECC_INTR {
        ECC_INTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 7 - Hresponse Error Interrupt Status"]
    #[inline]
    pub fn hresp_err_int(&self) -> HRESP_ERR_INTR {
        HRESP_ERR_INTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 13 - Change Of Field Interrupt Status"]
    #[inline]
    pub fn cof_int(&self) -> COF_INTR {
        COF_INTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 14 - CCIR Field 1 Interrupt Status"]
    #[inline]
    pub fn f1_int(&self) -> F1_INTR {
        F1_INTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 15 - CCIR Field 2 Interrupt Status"]
    #[inline]
    pub fn f2_int(&self) -> F2_INTR {
        F2_INTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 15;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 16 - Start of Frame Interrupt Status. Indicates when SOF is detected. (Cleared by writing 1)"]
    #[inline]
    pub fn sof_int(&self) -> SOF_INTR {
        SOF_INTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 17 - End of Frame (EOF) Interrupt Status. Indicates when EOF is detected. (Cleared by writing 1)"]
    #[inline]
    pub fn eof_int(&self) -> EOF_INTR {
        EOF_INTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 17;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 18 - RXFIFO Full Interrupt Status"]
    #[inline]
    pub fn rx_ff_int(&self) -> RXFF_INTR {
        RXFF_INTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 19 - DMA Transfer Done in Frame Buffer1"]
    #[inline]
    pub fn dma_tsf_done_fb1(&self) -> DMA_TSF_DONE_FB1R {
        DMA_TSF_DONE_FB1R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 19;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 20 - DMA Transfer Done in Frame Buffer2"]
    #[inline]
    pub fn dma_tsf_done_fb2(&self) -> DMA_TSF_DONE_FB2R {
        DMA_TSF_DONE_FB2R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 21 - STATFIFO Full Interrupt Status"]
    #[inline]
    pub fn statff_int(&self) -> STATFF_INTR {
        STATFF_INTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 21;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 22 - DMA Transfer Done from StatFIFO"]
    #[inline]
    pub fn dma_tsf_done_sff(&self) -> DMA_TSF_DONE_SFFR {
        DMA_TSF_DONE_SFFR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 22;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 24 - RxFIFO Overrun Interrupt Status"]
    #[inline]
    pub fn rf_or_int(&self) -> RF_OR_INTR {
        RF_OR_INTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 25 - STATFIFO Overrun Interrupt Status"]
    #[inline]
    pub fn sf_or_int(&self) -> SF_OR_INTR {
        SF_OR_INTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 25;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 26 - When DMA field 0 is complete, this bit will be set to 1(clear by writing 1)."]
    #[inline]
    pub fn dma_field1_done(&self) -> DMA_FIELD1_DONER {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 26;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        DMA_FIELD1_DONER { bits }
    }
    #[doc = "Bit 27 - When DMA field 0 is complete, this bit will be set to 1(clear by writing 1)."]
    #[inline]
    pub fn dma_field0_done(&self) -> DMA_FIELD0_DONER {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 27;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        DMA_FIELD0_DONER { bits }
    }
    #[doc = "Bit 28 - When using base address switching enable, this bit will be 1 when switching occur before DMA complete"]
    #[inline]
    pub fn baseaddr_chhange_error(&self) -> BASEADDR_CHHANGE_ERRORR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 28;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        BASEADDR_CHHANGE_ERRORR { bits }
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 16384 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - RXFIFO Data Ready"]
    #[inline]
    pub fn drdy(&mut self) -> _DRDYW {
        _DRDYW { w: self }
    }
    #[doc = "Bit 1 - CCIR Error Interrupt"]
    #[inline]
    pub fn ecc_int(&mut self) -> _ECC_INTW {
        _ECC_INTW { w: self }
    }
    #[doc = "Bit 7 - Hresponse Error Interrupt Status"]
    #[inline]
    pub fn hresp_err_int(&mut self) -> _HRESP_ERR_INTW {
        _HRESP_ERR_INTW { w: self }
    }
    #[doc = "Bit 13 - Change Of Field Interrupt Status"]
    #[inline]
    pub fn cof_int(&mut self) -> _COF_INTW {
        _COF_INTW { w: self }
    }
    #[doc = "Bit 14 - CCIR Field 1 Interrupt Status"]
    #[inline]
    pub fn f1_int(&mut self) -> _F1_INTW {
        _F1_INTW { w: self }
    }
    #[doc = "Bit 15 - CCIR Field 2 Interrupt Status"]
    #[inline]
    pub fn f2_int(&mut self) -> _F2_INTW {
        _F2_INTW { w: self }
    }
    #[doc = "Bit 16 - Start of Frame Interrupt Status. Indicates when SOF is detected. (Cleared by writing 1)"]
    #[inline]
    pub fn sof_int(&mut self) -> _SOF_INTW {
        _SOF_INTW { w: self }
    }
    #[doc = "Bit 17 - End of Frame (EOF) Interrupt Status. Indicates when EOF is detected. (Cleared by writing 1)"]
    #[inline]
    pub fn eof_int(&mut self) -> _EOF_INTW {
        _EOF_INTW { w: self }
    }
    #[doc = "Bit 18 - RXFIFO Full Interrupt Status"]
    #[inline]
    pub fn rx_ff_int(&mut self) -> _RXFF_INTW {
        _RXFF_INTW { w: self }
    }
    #[doc = "Bit 19 - DMA Transfer Done in Frame Buffer1"]
    #[inline]
    pub fn dma_tsf_done_fb1(&mut self) -> _DMA_TSF_DONE_FB1W {
        _DMA_TSF_DONE_FB1W { w: self }
    }
    #[doc = "Bit 20 - DMA Transfer Done in Frame Buffer2"]
    #[inline]
    pub fn dma_tsf_done_fb2(&mut self) -> _DMA_TSF_DONE_FB2W {
        _DMA_TSF_DONE_FB2W { w: self }
    }
    #[doc = "Bit 21 - STATFIFO Full Interrupt Status"]
    #[inline]
    pub fn statff_int(&mut self) -> _STATFF_INTW {
        _STATFF_INTW { w: self }
    }
    #[doc = "Bit 22 - DMA Transfer Done from StatFIFO"]
    #[inline]
    pub fn dma_tsf_done_sff(&mut self) -> _DMA_TSF_DONE_SFFW {
        _DMA_TSF_DONE_SFFW { w: self }
    }
    #[doc = "Bit 24 - RxFIFO Overrun Interrupt Status"]
    #[inline]
    pub fn rf_or_int(&mut self) -> _RF_OR_INTW {
        _RF_OR_INTW { w: self }
    }
    #[doc = "Bit 25 - STATFIFO Overrun Interrupt Status"]
    #[inline]
    pub fn sf_or_int(&mut self) -> _SF_OR_INTW {
        _SF_OR_INTW { w: self }
    }
    #[doc = "Bit 26 - When DMA field 0 is complete, this bit will be set to 1(clear by writing 1)."]
    #[inline]
    pub fn dma_field1_done(&mut self) -> _DMA_FIELD1_DONEW {
        _DMA_FIELD1_DONEW { w: self }
    }
    #[doc = "Bit 27 - When DMA field 0 is complete, this bit will be set to 1(clear by writing 1)."]
    #[inline]
    pub fn dma_field0_done(&mut self) -> _DMA_FIELD0_DONEW {
        _DMA_FIELD0_DONEW { w: self }
    }
    #[doc = "Bit 28 - When using base address switching enable, this bit will be 1 when switching occur before DMA complete"]
    #[inline]
    pub fn baseaddr_chhange_error(&mut self) -> _BASEADDR_CHHANGE_ERRORW {
        _BASEADDR_CHHANGE_ERRORW { w: self }
    }
}
