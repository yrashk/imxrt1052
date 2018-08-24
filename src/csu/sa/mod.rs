#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::SA {
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
#[doc = "Possible values of the field `NSA_DMA`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NSA_DMAR {
    #[doc = "Secure access for the corresponding type-1 master"]
    NSA_DMA_0,
    #[doc = "Non-secure access for the corresponding type-1 master"]
    NSA_DMA_1,
}
impl NSA_DMAR {
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
            NSA_DMAR::NSA_DMA_0 => false,
            NSA_DMAR::NSA_DMA_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> NSA_DMAR {
        match value {
            false => NSA_DMAR::NSA_DMA_0,
            true => NSA_DMAR::NSA_DMA_1,
        }
    }
    #[doc = "Checks if the value of the field is `NSA_DMA_0`"]
    #[inline]
    pub fn is_nsa_dma_0(&self) -> bool {
        *self == NSA_DMAR::NSA_DMA_0
    }
    #[doc = "Checks if the value of the field is `NSA_DMA_1`"]
    #[inline]
    pub fn is_nsa_dma_1(&self) -> bool {
        *self == NSA_DMAR::NSA_DMA_1
    }
}
#[doc = "Possible values of the field `L_DMA`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum L_DMAR {
    #[doc = "No lock-the adjacent (next lower) bit can be written by the software."]
    L_DMA_0,
    #[doc = "Lock-the adjacent (next lower) bit can't be written by the software."]
    L_DMA_1,
}
impl L_DMAR {
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
            L_DMAR::L_DMA_0 => false,
            L_DMAR::L_DMA_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> L_DMAR {
        match value {
            false => L_DMAR::L_DMA_0,
            true => L_DMAR::L_DMA_1,
        }
    }
    #[doc = "Checks if the value of the field is `L_DMA_0`"]
    #[inline]
    pub fn is_l_dma_0(&self) -> bool {
        *self == L_DMAR::L_DMA_0
    }
    #[doc = "Checks if the value of the field is `L_DMA_1`"]
    #[inline]
    pub fn is_l_dma_1(&self) -> bool {
        *self == L_DMAR::L_DMA_1
    }
}
#[doc = "Possible values of the field `NSA_LCDIF`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NSA_LCDIFR {
    #[doc = "Secure access for the corresponding type-1 master"]
    NSA_LCDIF_0,
    #[doc = "Non-secure access for the corresponding type-1 master"]
    NSA_LCDIF_1,
}
impl NSA_LCDIFR {
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
            NSA_LCDIFR::NSA_LCDIF_0 => false,
            NSA_LCDIFR::NSA_LCDIF_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> NSA_LCDIFR {
        match value {
            false => NSA_LCDIFR::NSA_LCDIF_0,
            true => NSA_LCDIFR::NSA_LCDIF_1,
        }
    }
    #[doc = "Checks if the value of the field is `NSA_LCDIF_0`"]
    #[inline]
    pub fn is_nsa_lcdif_0(&self) -> bool {
        *self == NSA_LCDIFR::NSA_LCDIF_0
    }
    #[doc = "Checks if the value of the field is `NSA_LCDIF_1`"]
    #[inline]
    pub fn is_nsa_lcdif_1(&self) -> bool {
        *self == NSA_LCDIFR::NSA_LCDIF_1
    }
}
#[doc = "Possible values of the field `L_LCDIF`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum L_LCDIFR {
    #[doc = "No lock-the adjacent (next lower) bit can be written by the software."]
    L_LCDIF_0,
    #[doc = "Lock-the adjacent (next lower) bit can't be written by the software."]
    L_LCDIF_1,
}
impl L_LCDIFR {
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
            L_LCDIFR::L_LCDIF_0 => false,
            L_LCDIFR::L_LCDIF_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> L_LCDIFR {
        match value {
            false => L_LCDIFR::L_LCDIF_0,
            true => L_LCDIFR::L_LCDIF_1,
        }
    }
    #[doc = "Checks if the value of the field is `L_LCDIF_0`"]
    #[inline]
    pub fn is_l_lcdif_0(&self) -> bool {
        *self == L_LCDIFR::L_LCDIF_0
    }
    #[doc = "Checks if the value of the field is `L_LCDIF_1`"]
    #[inline]
    pub fn is_l_lcdif_1(&self) -> bool {
        *self == L_LCDIFR::L_LCDIF_1
    }
}
#[doc = "Possible values of the field `NSA_CSI`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NSA_CSIR {
    #[doc = "Secure access for the corresponding type-1 master"]
    NSA_CSI_0,
    #[doc = "Non-secure access for the corresponding type-1 master"]
    NSA_CSI_1,
}
impl NSA_CSIR {
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
            NSA_CSIR::NSA_CSI_0 => false,
            NSA_CSIR::NSA_CSI_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> NSA_CSIR {
        match value {
            false => NSA_CSIR::NSA_CSI_0,
            true => NSA_CSIR::NSA_CSI_1,
        }
    }
    #[doc = "Checks if the value of the field is `NSA_CSI_0`"]
    #[inline]
    pub fn is_nsa_csi_0(&self) -> bool {
        *self == NSA_CSIR::NSA_CSI_0
    }
    #[doc = "Checks if the value of the field is `NSA_CSI_1`"]
    #[inline]
    pub fn is_nsa_csi_1(&self) -> bool {
        *self == NSA_CSIR::NSA_CSI_1
    }
}
#[doc = "Possible values of the field `L_CSI`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum L_CSIR {
    #[doc = "No lock-the adjacent (next lower) bit can be written by the software."]
    L_CSI_0,
    #[doc = "Lock-the adjacent (next lower) bit can't be written by the software."]
    L_CSI_1,
}
impl L_CSIR {
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
            L_CSIR::L_CSI_0 => false,
            L_CSIR::L_CSI_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> L_CSIR {
        match value {
            false => L_CSIR::L_CSI_0,
            true => L_CSIR::L_CSI_1,
        }
    }
    #[doc = "Checks if the value of the field is `L_CSI_0`"]
    #[inline]
    pub fn is_l_csi_0(&self) -> bool {
        *self == L_CSIR::L_CSI_0
    }
    #[doc = "Checks if the value of the field is `L_CSI_1`"]
    #[inline]
    pub fn is_l_csi_1(&self) -> bool {
        *self == L_CSIR::L_CSI_1
    }
}
#[doc = "Possible values of the field `NSA_PXP`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NSA_PXPR {
    #[doc = "Secure access for the corresponding type-1 master"]
    NSA_PXP_0,
    #[doc = "Non-secure access for the corresponding type-1 master"]
    NSA_PXP_1,
}
impl NSA_PXPR {
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
            NSA_PXPR::NSA_PXP_0 => false,
            NSA_PXPR::NSA_PXP_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> NSA_PXPR {
        match value {
            false => NSA_PXPR::NSA_PXP_0,
            true => NSA_PXPR::NSA_PXP_1,
        }
    }
    #[doc = "Checks if the value of the field is `NSA_PXP_0`"]
    #[inline]
    pub fn is_nsa_pxp_0(&self) -> bool {
        *self == NSA_PXPR::NSA_PXP_0
    }
    #[doc = "Checks if the value of the field is `NSA_PXP_1`"]
    #[inline]
    pub fn is_nsa_pxp_1(&self) -> bool {
        *self == NSA_PXPR::NSA_PXP_1
    }
}
#[doc = "Possible values of the field `L_PXP`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum L_PXPR {
    #[doc = "No lock-the adjacent (next lower) bit can be written by the software."]
    L_PXP_0,
    #[doc = "Lock-the adjacent (next lower) bit can't be written by the software."]
    L_PXP_1,
}
impl L_PXPR {
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
            L_PXPR::L_PXP_0 => false,
            L_PXPR::L_PXP_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> L_PXPR {
        match value {
            false => L_PXPR::L_PXP_0,
            true => L_PXPR::L_PXP_1,
        }
    }
    #[doc = "Checks if the value of the field is `L_PXP_0`"]
    #[inline]
    pub fn is_l_pxp_0(&self) -> bool {
        *self == L_PXPR::L_PXP_0
    }
    #[doc = "Checks if the value of the field is `L_PXP_1`"]
    #[inline]
    pub fn is_l_pxp_1(&self) -> bool {
        *self == L_PXPR::L_PXP_1
    }
}
#[doc = "Possible values of the field `NSA_DCP`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NSA_DCPR {
    #[doc = "Secure access for the corresponding type-1 master"]
    NSA_DCP_0,
    #[doc = "Non-secure access for the corresponding type-1 master"]
    NSA_DCP_1,
}
impl NSA_DCPR {
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
            NSA_DCPR::NSA_DCP_0 => false,
            NSA_DCPR::NSA_DCP_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> NSA_DCPR {
        match value {
            false => NSA_DCPR::NSA_DCP_0,
            true => NSA_DCPR::NSA_DCP_1,
        }
    }
    #[doc = "Checks if the value of the field is `NSA_DCP_0`"]
    #[inline]
    pub fn is_nsa_dcp_0(&self) -> bool {
        *self == NSA_DCPR::NSA_DCP_0
    }
    #[doc = "Checks if the value of the field is `NSA_DCP_1`"]
    #[inline]
    pub fn is_nsa_dcp_1(&self) -> bool {
        *self == NSA_DCPR::NSA_DCP_1
    }
}
#[doc = "Possible values of the field `L_DCP`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum L_DCPR {
    #[doc = "No lock-the adjacent (next lower) bit can be written by the software."]
    L_DCP_0,
    #[doc = "Lock-the adjacent (next lower) bit can't be written by the software."]
    L_DCP_1,
}
impl L_DCPR {
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
            L_DCPR::L_DCP_0 => false,
            L_DCPR::L_DCP_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> L_DCPR {
        match value {
            false => L_DCPR::L_DCP_0,
            true => L_DCPR::L_DCP_1,
        }
    }
    #[doc = "Checks if the value of the field is `L_DCP_0`"]
    #[inline]
    pub fn is_l_dcp_0(&self) -> bool {
        *self == L_DCPR::L_DCP_0
    }
    #[doc = "Checks if the value of the field is `L_DCP_1`"]
    #[inline]
    pub fn is_l_dcp_1(&self) -> bool {
        *self == L_DCPR::L_DCP_1
    }
}
#[doc = "Possible values of the field `NSA_ENET`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NSA_ENETR {
    #[doc = "Secure access for the corresponding type-1 master"]
    NSA_ENET_0,
    #[doc = "Non-secure access for the corresponding type-1 master"]
    NSA_ENET_1,
}
impl NSA_ENETR {
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
            NSA_ENETR::NSA_ENET_0 => false,
            NSA_ENETR::NSA_ENET_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> NSA_ENETR {
        match value {
            false => NSA_ENETR::NSA_ENET_0,
            true => NSA_ENETR::NSA_ENET_1,
        }
    }
    #[doc = "Checks if the value of the field is `NSA_ENET_0`"]
    #[inline]
    pub fn is_nsa_enet_0(&self) -> bool {
        *self == NSA_ENETR::NSA_ENET_0
    }
    #[doc = "Checks if the value of the field is `NSA_ENET_1`"]
    #[inline]
    pub fn is_nsa_enet_1(&self) -> bool {
        *self == NSA_ENETR::NSA_ENET_1
    }
}
#[doc = "Possible values of the field `L_ENET`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum L_ENETR {
    #[doc = "No lock-the adjacent (next lower) bit can be written by the software."]
    L_ENET_0,
    #[doc = "Lock-the adjacent (next lower) bit can't be written by the software."]
    L_ENET_1,
}
impl L_ENETR {
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
            L_ENETR::L_ENET_0 => false,
            L_ENETR::L_ENET_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> L_ENETR {
        match value {
            false => L_ENETR::L_ENET_0,
            true => L_ENETR::L_ENET_1,
        }
    }
    #[doc = "Checks if the value of the field is `L_ENET_0`"]
    #[inline]
    pub fn is_l_enet_0(&self) -> bool {
        *self == L_ENETR::L_ENET_0
    }
    #[doc = "Checks if the value of the field is `L_ENET_1`"]
    #[inline]
    pub fn is_l_enet_1(&self) -> bool {
        *self == L_ENETR::L_ENET_1
    }
}
#[doc = "Possible values of the field `NSA_USDHC1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NSA_USDHC1R {
    #[doc = "Secure access for the corresponding type-1 master"]
    NSA_USDHC1_0,
    #[doc = "Non-secure access for the corresponding type-1 master"]
    NSA_USDHC1_1,
}
impl NSA_USDHC1R {
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
            NSA_USDHC1R::NSA_USDHC1_0 => false,
            NSA_USDHC1R::NSA_USDHC1_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> NSA_USDHC1R {
        match value {
            false => NSA_USDHC1R::NSA_USDHC1_0,
            true => NSA_USDHC1R::NSA_USDHC1_1,
        }
    }
    #[doc = "Checks if the value of the field is `NSA_USDHC1_0`"]
    #[inline]
    pub fn is_nsa_usdhc1_0(&self) -> bool {
        *self == NSA_USDHC1R::NSA_USDHC1_0
    }
    #[doc = "Checks if the value of the field is `NSA_USDHC1_1`"]
    #[inline]
    pub fn is_nsa_usdhc1_1(&self) -> bool {
        *self == NSA_USDHC1R::NSA_USDHC1_1
    }
}
#[doc = "Possible values of the field `L_USDHC1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum L_USDHC1R {
    #[doc = "No lock-the adjacent (next lower) bit can be written by the software."]
    L_USDHC1_0,
    #[doc = "Lock-the adjacent (next lower) bit can't be written by the software."]
    L_USDHC1_1,
}
impl L_USDHC1R {
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
            L_USDHC1R::L_USDHC1_0 => false,
            L_USDHC1R::L_USDHC1_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> L_USDHC1R {
        match value {
            false => L_USDHC1R::L_USDHC1_0,
            true => L_USDHC1R::L_USDHC1_1,
        }
    }
    #[doc = "Checks if the value of the field is `L_USDHC1_0`"]
    #[inline]
    pub fn is_l_usdhc1_0(&self) -> bool {
        *self == L_USDHC1R::L_USDHC1_0
    }
    #[doc = "Checks if the value of the field is `L_USDHC1_1`"]
    #[inline]
    pub fn is_l_usdhc1_1(&self) -> bool {
        *self == L_USDHC1R::L_USDHC1_1
    }
}
#[doc = "Possible values of the field `NSA_USDHC2`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NSA_USDHC2R {
    #[doc = "Secure access for the corresponding type-1 master"]
    NSA_USDHC2_0,
    #[doc = "Non-secure access for the corresponding type-1 master"]
    NSA_USDHC2_1,
}
impl NSA_USDHC2R {
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
            NSA_USDHC2R::NSA_USDHC2_0 => false,
            NSA_USDHC2R::NSA_USDHC2_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> NSA_USDHC2R {
        match value {
            false => NSA_USDHC2R::NSA_USDHC2_0,
            true => NSA_USDHC2R::NSA_USDHC2_1,
        }
    }
    #[doc = "Checks if the value of the field is `NSA_USDHC2_0`"]
    #[inline]
    pub fn is_nsa_usdhc2_0(&self) -> bool {
        *self == NSA_USDHC2R::NSA_USDHC2_0
    }
    #[doc = "Checks if the value of the field is `NSA_USDHC2_1`"]
    #[inline]
    pub fn is_nsa_usdhc2_1(&self) -> bool {
        *self == NSA_USDHC2R::NSA_USDHC2_1
    }
}
#[doc = "Possible values of the field `L_USDHC2`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum L_USDHC2R {
    #[doc = "No lock-the adjacent (next lower) bit can be written by the software."]
    L_USDHC2_0,
    #[doc = "Lock-the adjacent (next lower) bit can't be written by the software."]
    L_USDHC2_1,
}
impl L_USDHC2R {
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
            L_USDHC2R::L_USDHC2_0 => false,
            L_USDHC2R::L_USDHC2_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> L_USDHC2R {
        match value {
            false => L_USDHC2R::L_USDHC2_0,
            true => L_USDHC2R::L_USDHC2_1,
        }
    }
    #[doc = "Checks if the value of the field is `L_USDHC2_0`"]
    #[inline]
    pub fn is_l_usdhc2_0(&self) -> bool {
        *self == L_USDHC2R::L_USDHC2_0
    }
    #[doc = "Checks if the value of the field is `L_USDHC2_1`"]
    #[inline]
    pub fn is_l_usdhc2_1(&self) -> bool {
        *self == L_USDHC2R::L_USDHC2_1
    }
}
#[doc = "Possible values of the field `NSA_TPSMP`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NSA_TPSMPR {
    #[doc = "Secure access for the corresponding type-1 master"]
    NSA_TPSMP_0,
    #[doc = "Non-secure access for the corresponding type-1 master"]
    NSA_TPSMP_1,
}
impl NSA_TPSMPR {
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
            NSA_TPSMPR::NSA_TPSMP_0 => false,
            NSA_TPSMPR::NSA_TPSMP_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> NSA_TPSMPR {
        match value {
            false => NSA_TPSMPR::NSA_TPSMP_0,
            true => NSA_TPSMPR::NSA_TPSMP_1,
        }
    }
    #[doc = "Checks if the value of the field is `NSA_TPSMP_0`"]
    #[inline]
    pub fn is_nsa_tpsmp_0(&self) -> bool {
        *self == NSA_TPSMPR::NSA_TPSMP_0
    }
    #[doc = "Checks if the value of the field is `NSA_TPSMP_1`"]
    #[inline]
    pub fn is_nsa_tpsmp_1(&self) -> bool {
        *self == NSA_TPSMPR::NSA_TPSMP_1
    }
}
#[doc = "Possible values of the field `L_TPSMP`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum L_TPSMPR {
    #[doc = "No lock-the adjacent (next lower) bit can be written by the software."]
    L_TPSMP_0,
    #[doc = "Lock-the adjacent (next lower) bit can't be written by the software."]
    L_TPSMP_1,
}
impl L_TPSMPR {
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
            L_TPSMPR::L_TPSMP_0 => false,
            L_TPSMPR::L_TPSMP_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> L_TPSMPR {
        match value {
            false => L_TPSMPR::L_TPSMP_0,
            true => L_TPSMPR::L_TPSMP_1,
        }
    }
    #[doc = "Checks if the value of the field is `L_TPSMP_0`"]
    #[inline]
    pub fn is_l_tpsmp_0(&self) -> bool {
        *self == L_TPSMPR::L_TPSMP_0
    }
    #[doc = "Checks if the value of the field is `L_TPSMP_1`"]
    #[inline]
    pub fn is_l_tpsmp_1(&self) -> bool {
        *self == L_TPSMPR::L_TPSMP_1
    }
}
#[doc = "Possible values of the field `NSA_USB`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NSA_USBR {
    #[doc = "Secure access for the corresponding type-1 master"]
    NSA_USB_0,
    #[doc = "Non-secure access for the corresponding type-1 master"]
    NSA_USB_1,
}
impl NSA_USBR {
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
            NSA_USBR::NSA_USB_0 => false,
            NSA_USBR::NSA_USB_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> NSA_USBR {
        match value {
            false => NSA_USBR::NSA_USB_0,
            true => NSA_USBR::NSA_USB_1,
        }
    }
    #[doc = "Checks if the value of the field is `NSA_USB_0`"]
    #[inline]
    pub fn is_nsa_usb_0(&self) -> bool {
        *self == NSA_USBR::NSA_USB_0
    }
    #[doc = "Checks if the value of the field is `NSA_USB_1`"]
    #[inline]
    pub fn is_nsa_usb_1(&self) -> bool {
        *self == NSA_USBR::NSA_USB_1
    }
}
#[doc = "Possible values of the field `L_USB`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum L_USBR {
    #[doc = "No lock-the adjacent (next lower) bit can be written by the software."]
    L_USB_0,
    #[doc = "Lock-the adjacent (next lower) bit can't be written by the software."]
    L_USB_1,
}
impl L_USBR {
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
            L_USBR::L_USB_0 => false,
            L_USBR::L_USB_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> L_USBR {
        match value {
            false => L_USBR::L_USB_0,
            true => L_USBR::L_USB_1,
        }
    }
    #[doc = "Checks if the value of the field is `L_USB_0`"]
    #[inline]
    pub fn is_l_usb_0(&self) -> bool {
        *self == L_USBR::L_USB_0
    }
    #[doc = "Checks if the value of the field is `L_USB_1`"]
    #[inline]
    pub fn is_l_usb_1(&self) -> bool {
        *self == L_USBR::L_USB_1
    }
}
#[doc = "Values that can be written to the field `NSA_DMA`"]
pub enum NSA_DMAW {
    #[doc = "Secure access for the corresponding type-1 master"]
    NSA_DMA_0,
    #[doc = "Non-secure access for the corresponding type-1 master"]
    NSA_DMA_1,
}
impl NSA_DMAW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            NSA_DMAW::NSA_DMA_0 => false,
            NSA_DMAW::NSA_DMA_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _NSA_DMAW<'a> {
    w: &'a mut W,
}
impl<'a> _NSA_DMAW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: NSA_DMAW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Secure access for the corresponding type-1 master"]
    #[inline]
    pub fn nsa_dma_0(self) -> &'a mut W {
        self.variant(NSA_DMAW::NSA_DMA_0)
    }
    #[doc = "Non-secure access for the corresponding type-1 master"]
    #[inline]
    pub fn nsa_dma_1(self) -> &'a mut W {
        self.variant(NSA_DMAW::NSA_DMA_1)
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
#[doc = "Values that can be written to the field `L_DMA`"]
pub enum L_DMAW {
    #[doc = "No lock-the adjacent (next lower) bit can be written by the software."]
    L_DMA_0,
    #[doc = "Lock-the adjacent (next lower) bit can't be written by the software."]
    L_DMA_1,
}
impl L_DMAW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            L_DMAW::L_DMA_0 => false,
            L_DMAW::L_DMA_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _L_DMAW<'a> {
    w: &'a mut W,
}
impl<'a> _L_DMAW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: L_DMAW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No lock-the adjacent (next lower) bit can be written by the software."]
    #[inline]
    pub fn l_dma_0(self) -> &'a mut W {
        self.variant(L_DMAW::L_DMA_0)
    }
    #[doc = "Lock-the adjacent (next lower) bit can't be written by the software."]
    #[inline]
    pub fn l_dma_1(self) -> &'a mut W {
        self.variant(L_DMAW::L_DMA_1)
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
#[doc = "Values that can be written to the field `NSA_LCDIF`"]
pub enum NSA_LCDIFW {
    #[doc = "Secure access for the corresponding type-1 master"]
    NSA_LCDIF_0,
    #[doc = "Non-secure access for the corresponding type-1 master"]
    NSA_LCDIF_1,
}
impl NSA_LCDIFW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            NSA_LCDIFW::NSA_LCDIF_0 => false,
            NSA_LCDIFW::NSA_LCDIF_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _NSA_LCDIFW<'a> {
    w: &'a mut W,
}
impl<'a> _NSA_LCDIFW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: NSA_LCDIFW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Secure access for the corresponding type-1 master"]
    #[inline]
    pub fn nsa_lcdif_0(self) -> &'a mut W {
        self.variant(NSA_LCDIFW::NSA_LCDIF_0)
    }
    #[doc = "Non-secure access for the corresponding type-1 master"]
    #[inline]
    pub fn nsa_lcdif_1(self) -> &'a mut W {
        self.variant(NSA_LCDIFW::NSA_LCDIF_1)
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
#[doc = "Values that can be written to the field `L_LCDIF`"]
pub enum L_LCDIFW {
    #[doc = "No lock-the adjacent (next lower) bit can be written by the software."]
    L_LCDIF_0,
    #[doc = "Lock-the adjacent (next lower) bit can't be written by the software."]
    L_LCDIF_1,
}
impl L_LCDIFW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            L_LCDIFW::L_LCDIF_0 => false,
            L_LCDIFW::L_LCDIF_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _L_LCDIFW<'a> {
    w: &'a mut W,
}
impl<'a> _L_LCDIFW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: L_LCDIFW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No lock-the adjacent (next lower) bit can be written by the software."]
    #[inline]
    pub fn l_lcdif_0(self) -> &'a mut W {
        self.variant(L_LCDIFW::L_LCDIF_0)
    }
    #[doc = "Lock-the adjacent (next lower) bit can't be written by the software."]
    #[inline]
    pub fn l_lcdif_1(self) -> &'a mut W {
        self.variant(L_LCDIFW::L_LCDIF_1)
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
#[doc = "Values that can be written to the field `NSA_CSI`"]
pub enum NSA_CSIW {
    #[doc = "Secure access for the corresponding type-1 master"]
    NSA_CSI_0,
    #[doc = "Non-secure access for the corresponding type-1 master"]
    NSA_CSI_1,
}
impl NSA_CSIW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            NSA_CSIW::NSA_CSI_0 => false,
            NSA_CSIW::NSA_CSI_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _NSA_CSIW<'a> {
    w: &'a mut W,
}
impl<'a> _NSA_CSIW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: NSA_CSIW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Secure access for the corresponding type-1 master"]
    #[inline]
    pub fn nsa_csi_0(self) -> &'a mut W {
        self.variant(NSA_CSIW::NSA_CSI_0)
    }
    #[doc = "Non-secure access for the corresponding type-1 master"]
    #[inline]
    pub fn nsa_csi_1(self) -> &'a mut W {
        self.variant(NSA_CSIW::NSA_CSI_1)
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
#[doc = "Values that can be written to the field `L_CSI`"]
pub enum L_CSIW {
    #[doc = "No lock-the adjacent (next lower) bit can be written by the software."]
    L_CSI_0,
    #[doc = "Lock-the adjacent (next lower) bit can't be written by the software."]
    L_CSI_1,
}
impl L_CSIW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            L_CSIW::L_CSI_0 => false,
            L_CSIW::L_CSI_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _L_CSIW<'a> {
    w: &'a mut W,
}
impl<'a> _L_CSIW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: L_CSIW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No lock-the adjacent (next lower) bit can be written by the software."]
    #[inline]
    pub fn l_csi_0(self) -> &'a mut W {
        self.variant(L_CSIW::L_CSI_0)
    }
    #[doc = "Lock-the adjacent (next lower) bit can't be written by the software."]
    #[inline]
    pub fn l_csi_1(self) -> &'a mut W {
        self.variant(L_CSIW::L_CSI_1)
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
#[doc = "Values that can be written to the field `NSA_PXP`"]
pub enum NSA_PXPW {
    #[doc = "Secure access for the corresponding type-1 master"]
    NSA_PXP_0,
    #[doc = "Non-secure access for the corresponding type-1 master"]
    NSA_PXP_1,
}
impl NSA_PXPW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            NSA_PXPW::NSA_PXP_0 => false,
            NSA_PXPW::NSA_PXP_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _NSA_PXPW<'a> {
    w: &'a mut W,
}
impl<'a> _NSA_PXPW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: NSA_PXPW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Secure access for the corresponding type-1 master"]
    #[inline]
    pub fn nsa_pxp_0(self) -> &'a mut W {
        self.variant(NSA_PXPW::NSA_PXP_0)
    }
    #[doc = "Non-secure access for the corresponding type-1 master"]
    #[inline]
    pub fn nsa_pxp_1(self) -> &'a mut W {
        self.variant(NSA_PXPW::NSA_PXP_1)
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
#[doc = "Values that can be written to the field `L_PXP`"]
pub enum L_PXPW {
    #[doc = "No lock-the adjacent (next lower) bit can be written by the software."]
    L_PXP_0,
    #[doc = "Lock-the adjacent (next lower) bit can't be written by the software."]
    L_PXP_1,
}
impl L_PXPW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            L_PXPW::L_PXP_0 => false,
            L_PXPW::L_PXP_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _L_PXPW<'a> {
    w: &'a mut W,
}
impl<'a> _L_PXPW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: L_PXPW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No lock-the adjacent (next lower) bit can be written by the software."]
    #[inline]
    pub fn l_pxp_0(self) -> &'a mut W {
        self.variant(L_PXPW::L_PXP_0)
    }
    #[doc = "Lock-the adjacent (next lower) bit can't be written by the software."]
    #[inline]
    pub fn l_pxp_1(self) -> &'a mut W {
        self.variant(L_PXPW::L_PXP_1)
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
        const OFFSET: u8 = 9;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `NSA_DCP`"]
pub enum NSA_DCPW {
    #[doc = "Secure access for the corresponding type-1 master"]
    NSA_DCP_0,
    #[doc = "Non-secure access for the corresponding type-1 master"]
    NSA_DCP_1,
}
impl NSA_DCPW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            NSA_DCPW::NSA_DCP_0 => false,
            NSA_DCPW::NSA_DCP_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _NSA_DCPW<'a> {
    w: &'a mut W,
}
impl<'a> _NSA_DCPW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: NSA_DCPW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Secure access for the corresponding type-1 master"]
    #[inline]
    pub fn nsa_dcp_0(self) -> &'a mut W {
        self.variant(NSA_DCPW::NSA_DCP_0)
    }
    #[doc = "Non-secure access for the corresponding type-1 master"]
    #[inline]
    pub fn nsa_dcp_1(self) -> &'a mut W {
        self.variant(NSA_DCPW::NSA_DCP_1)
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
#[doc = "Values that can be written to the field `L_DCP`"]
pub enum L_DCPW {
    #[doc = "No lock-the adjacent (next lower) bit can be written by the software."]
    L_DCP_0,
    #[doc = "Lock-the adjacent (next lower) bit can't be written by the software."]
    L_DCP_1,
}
impl L_DCPW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            L_DCPW::L_DCP_0 => false,
            L_DCPW::L_DCP_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _L_DCPW<'a> {
    w: &'a mut W,
}
impl<'a> _L_DCPW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: L_DCPW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No lock-the adjacent (next lower) bit can be written by the software."]
    #[inline]
    pub fn l_dcp_0(self) -> &'a mut W {
        self.variant(L_DCPW::L_DCP_0)
    }
    #[doc = "Lock-the adjacent (next lower) bit can't be written by the software."]
    #[inline]
    pub fn l_dcp_1(self) -> &'a mut W {
        self.variant(L_DCPW::L_DCP_1)
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
#[doc = "Values that can be written to the field `NSA_ENET`"]
pub enum NSA_ENETW {
    #[doc = "Secure access for the corresponding type-1 master"]
    NSA_ENET_0,
    #[doc = "Non-secure access for the corresponding type-1 master"]
    NSA_ENET_1,
}
impl NSA_ENETW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            NSA_ENETW::NSA_ENET_0 => false,
            NSA_ENETW::NSA_ENET_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _NSA_ENETW<'a> {
    w: &'a mut W,
}
impl<'a> _NSA_ENETW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: NSA_ENETW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Secure access for the corresponding type-1 master"]
    #[inline]
    pub fn nsa_enet_0(self) -> &'a mut W {
        self.variant(NSA_ENETW::NSA_ENET_0)
    }
    #[doc = "Non-secure access for the corresponding type-1 master"]
    #[inline]
    pub fn nsa_enet_1(self) -> &'a mut W {
        self.variant(NSA_ENETW::NSA_ENET_1)
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
#[doc = "Values that can be written to the field `L_ENET`"]
pub enum L_ENETW {
    #[doc = "No lock-the adjacent (next lower) bit can be written by the software."]
    L_ENET_0,
    #[doc = "Lock-the adjacent (next lower) bit can't be written by the software."]
    L_ENET_1,
}
impl L_ENETW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            L_ENETW::L_ENET_0 => false,
            L_ENETW::L_ENET_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _L_ENETW<'a> {
    w: &'a mut W,
}
impl<'a> _L_ENETW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: L_ENETW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No lock-the adjacent (next lower) bit can be written by the software."]
    #[inline]
    pub fn l_enet_0(self) -> &'a mut W {
        self.variant(L_ENETW::L_ENET_0)
    }
    #[doc = "Lock-the adjacent (next lower) bit can't be written by the software."]
    #[inline]
    pub fn l_enet_1(self) -> &'a mut W {
        self.variant(L_ENETW::L_ENET_1)
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
#[doc = "Values that can be written to the field `NSA_USDHC1`"]
pub enum NSA_USDHC1W {
    #[doc = "Secure access for the corresponding type-1 master"]
    NSA_USDHC1_0,
    #[doc = "Non-secure access for the corresponding type-1 master"]
    NSA_USDHC1_1,
}
impl NSA_USDHC1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            NSA_USDHC1W::NSA_USDHC1_0 => false,
            NSA_USDHC1W::NSA_USDHC1_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _NSA_USDHC1W<'a> {
    w: &'a mut W,
}
impl<'a> _NSA_USDHC1W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: NSA_USDHC1W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Secure access for the corresponding type-1 master"]
    #[inline]
    pub fn nsa_usdhc1_0(self) -> &'a mut W {
        self.variant(NSA_USDHC1W::NSA_USDHC1_0)
    }
    #[doc = "Non-secure access for the corresponding type-1 master"]
    #[inline]
    pub fn nsa_usdhc1_1(self) -> &'a mut W {
        self.variant(NSA_USDHC1W::NSA_USDHC1_1)
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
#[doc = "Values that can be written to the field `L_USDHC1`"]
pub enum L_USDHC1W {
    #[doc = "No lock-the adjacent (next lower) bit can be written by the software."]
    L_USDHC1_0,
    #[doc = "Lock-the adjacent (next lower) bit can't be written by the software."]
    L_USDHC1_1,
}
impl L_USDHC1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            L_USDHC1W::L_USDHC1_0 => false,
            L_USDHC1W::L_USDHC1_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _L_USDHC1W<'a> {
    w: &'a mut W,
}
impl<'a> _L_USDHC1W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: L_USDHC1W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No lock-the adjacent (next lower) bit can be written by the software."]
    #[inline]
    pub fn l_usdhc1_0(self) -> &'a mut W {
        self.variant(L_USDHC1W::L_USDHC1_0)
    }
    #[doc = "Lock-the adjacent (next lower) bit can't be written by the software."]
    #[inline]
    pub fn l_usdhc1_1(self) -> &'a mut W {
        self.variant(L_USDHC1W::L_USDHC1_1)
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
#[doc = "Values that can be written to the field `NSA_USDHC2`"]
pub enum NSA_USDHC2W {
    #[doc = "Secure access for the corresponding type-1 master"]
    NSA_USDHC2_0,
    #[doc = "Non-secure access for the corresponding type-1 master"]
    NSA_USDHC2_1,
}
impl NSA_USDHC2W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            NSA_USDHC2W::NSA_USDHC2_0 => false,
            NSA_USDHC2W::NSA_USDHC2_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _NSA_USDHC2W<'a> {
    w: &'a mut W,
}
impl<'a> _NSA_USDHC2W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: NSA_USDHC2W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Secure access for the corresponding type-1 master"]
    #[inline]
    pub fn nsa_usdhc2_0(self) -> &'a mut W {
        self.variant(NSA_USDHC2W::NSA_USDHC2_0)
    }
    #[doc = "Non-secure access for the corresponding type-1 master"]
    #[inline]
    pub fn nsa_usdhc2_1(self) -> &'a mut W {
        self.variant(NSA_USDHC2W::NSA_USDHC2_1)
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
#[doc = "Values that can be written to the field `L_USDHC2`"]
pub enum L_USDHC2W {
    #[doc = "No lock-the adjacent (next lower) bit can be written by the software."]
    L_USDHC2_0,
    #[doc = "Lock-the adjacent (next lower) bit can't be written by the software."]
    L_USDHC2_1,
}
impl L_USDHC2W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            L_USDHC2W::L_USDHC2_0 => false,
            L_USDHC2W::L_USDHC2_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _L_USDHC2W<'a> {
    w: &'a mut W,
}
impl<'a> _L_USDHC2W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: L_USDHC2W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No lock-the adjacent (next lower) bit can be written by the software."]
    #[inline]
    pub fn l_usdhc2_0(self) -> &'a mut W {
        self.variant(L_USDHC2W::L_USDHC2_0)
    }
    #[doc = "Lock-the adjacent (next lower) bit can't be written by the software."]
    #[inline]
    pub fn l_usdhc2_1(self) -> &'a mut W {
        self.variant(L_USDHC2W::L_USDHC2_1)
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
#[doc = "Values that can be written to the field `NSA_TPSMP`"]
pub enum NSA_TPSMPW {
    #[doc = "Secure access for the corresponding type-1 master"]
    NSA_TPSMP_0,
    #[doc = "Non-secure access for the corresponding type-1 master"]
    NSA_TPSMP_1,
}
impl NSA_TPSMPW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            NSA_TPSMPW::NSA_TPSMP_0 => false,
            NSA_TPSMPW::NSA_TPSMP_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _NSA_TPSMPW<'a> {
    w: &'a mut W,
}
impl<'a> _NSA_TPSMPW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: NSA_TPSMPW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Secure access for the corresponding type-1 master"]
    #[inline]
    pub fn nsa_tpsmp_0(self) -> &'a mut W {
        self.variant(NSA_TPSMPW::NSA_TPSMP_0)
    }
    #[doc = "Non-secure access for the corresponding type-1 master"]
    #[inline]
    pub fn nsa_tpsmp_1(self) -> &'a mut W {
        self.variant(NSA_TPSMPW::NSA_TPSMP_1)
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
#[doc = "Values that can be written to the field `L_TPSMP`"]
pub enum L_TPSMPW {
    #[doc = "No lock-the adjacent (next lower) bit can be written by the software."]
    L_TPSMP_0,
    #[doc = "Lock-the adjacent (next lower) bit can't be written by the software."]
    L_TPSMP_1,
}
impl L_TPSMPW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            L_TPSMPW::L_TPSMP_0 => false,
            L_TPSMPW::L_TPSMP_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _L_TPSMPW<'a> {
    w: &'a mut W,
}
impl<'a> _L_TPSMPW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: L_TPSMPW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No lock-the adjacent (next lower) bit can be written by the software."]
    #[inline]
    pub fn l_tpsmp_0(self) -> &'a mut W {
        self.variant(L_TPSMPW::L_TPSMP_0)
    }
    #[doc = "Lock-the adjacent (next lower) bit can't be written by the software."]
    #[inline]
    pub fn l_tpsmp_1(self) -> &'a mut W {
        self.variant(L_TPSMPW::L_TPSMP_1)
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
#[doc = "Values that can be written to the field `NSA_USB`"]
pub enum NSA_USBW {
    #[doc = "Secure access for the corresponding type-1 master"]
    NSA_USB_0,
    #[doc = "Non-secure access for the corresponding type-1 master"]
    NSA_USB_1,
}
impl NSA_USBW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            NSA_USBW::NSA_USB_0 => false,
            NSA_USBW::NSA_USB_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _NSA_USBW<'a> {
    w: &'a mut W,
}
impl<'a> _NSA_USBW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: NSA_USBW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Secure access for the corresponding type-1 master"]
    #[inline]
    pub fn nsa_usb_0(self) -> &'a mut W {
        self.variant(NSA_USBW::NSA_USB_0)
    }
    #[doc = "Non-secure access for the corresponding type-1 master"]
    #[inline]
    pub fn nsa_usb_1(self) -> &'a mut W {
        self.variant(NSA_USBW::NSA_USB_1)
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
#[doc = "Values that can be written to the field `L_USB`"]
pub enum L_USBW {
    #[doc = "No lock-the adjacent (next lower) bit can be written by the software."]
    L_USB_0,
    #[doc = "Lock-the adjacent (next lower) bit can't be written by the software."]
    L_USB_1,
}
impl L_USBW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            L_USBW::L_USB_0 => false,
            L_USBW::L_USB_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _L_USBW<'a> {
    w: &'a mut W,
}
impl<'a> _L_USBW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: L_USBW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No lock-the adjacent (next lower) bit can be written by the software."]
    #[inline]
    pub fn l_usb_0(self) -> &'a mut W {
        self.variant(L_USBW::L_USB_0)
    }
    #[doc = "Lock-the adjacent (next lower) bit can't be written by the software."]
    #[inline]
    pub fn l_usb_1(self) -> &'a mut W {
        self.variant(L_USBW::L_USB_1)
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
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 2 - Non-secure access policy indicator bit"]
    #[inline]
    pub fn nsa_dma(&self) -> NSA_DMAR {
        NSA_DMAR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - Lock bit set by the TZ software for the eDMA"]
    #[inline]
    pub fn l_dma(&self) -> L_DMAR {
        L_DMAR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 4 - Non-secure access policy indicator bit"]
    #[inline]
    pub fn nsa_lcdif(&self) -> NSA_LCDIFR {
        NSA_LCDIFR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 5 - Lock bit set by the TZ software for the LCDIF"]
    #[inline]
    pub fn l_lcdif(&self) -> L_LCDIFR {
        L_LCDIFR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 6 - Non-secure access policy indicator bit"]
    #[inline]
    pub fn nsa_csi(&self) -> NSA_CSIR {
        NSA_CSIR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 7 - Lock bit set by the TZ software for the CSI"]
    #[inline]
    pub fn l_csi(&self) -> L_CSIR {
        L_CSIR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 8 - Non-Secure Access Policy indicator bit"]
    #[inline]
    pub fn nsa_pxp(&self) -> NSA_PXPR {
        NSA_PXPR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 9 - Lock bit set by the TZ software for the PXP"]
    #[inline]
    pub fn l_pxp(&self) -> L_PXPR {
        L_PXPR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 10 - Non-secure access policy indicator bit"]
    #[inline]
    pub fn nsa_dcp(&self) -> NSA_DCPR {
        NSA_DCPR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 11 - Lock bit set by the TZ software for the DCP"]
    #[inline]
    pub fn l_dcp(&self) -> L_DCPR {
        L_DCPR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 14 - Non-secure access policy indicator bit"]
    #[inline]
    pub fn nsa_enet(&self) -> NSA_ENETR {
        NSA_ENETR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 15 - Lock bit set by the TZ software for the ENET1 and ENET2"]
    #[inline]
    pub fn l_enet(&self) -> L_ENETR {
        L_ENETR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 15;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 16 - Non-secure access policy indicator bit"]
    #[inline]
    pub fn nsa_usdhc1(&self) -> NSA_USDHC1R {
        NSA_USDHC1R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 17 - Lock bit set by the TZ software for the USDHC1"]
    #[inline]
    pub fn l_usdhc1(&self) -> L_USDHC1R {
        L_USDHC1R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 17;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 18 - Non-secure access policy indicator bit"]
    #[inline]
    pub fn nsa_usdhc2(&self) -> NSA_USDHC2R {
        NSA_USDHC2R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 19 - Lock bit set by the TZ software for the USDHC2"]
    #[inline]
    pub fn l_usdhc2(&self) -> L_USDHC2R {
        L_USDHC2R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 19;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 20 - Non-secure access policy indicator bit"]
    #[inline]
    pub fn nsa_tpsmp(&self) -> NSA_TPSMPR {
        NSA_TPSMPR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 21 - Lock bit set by the TZ software for the TPSMP"]
    #[inline]
    pub fn l_tpsmp(&self) -> L_TPSMPR {
        L_TPSMPR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 21;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 22 - Non-secure access policy indicator bit"]
    #[inline]
    pub fn nsa_usb(&self) -> NSA_USBR {
        NSA_USBR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 22;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 23 - Lock bit set by the TZ software for the USB"]
    #[inline]
    pub fn l_usb(&self) -> L_USBR {
        L_USBR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 23;
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
    #[doc = "Bit 2 - Non-secure access policy indicator bit"]
    #[inline]
    pub fn nsa_dma(&mut self) -> _NSA_DMAW {
        _NSA_DMAW { w: self }
    }
    #[doc = "Bit 3 - Lock bit set by the TZ software for the eDMA"]
    #[inline]
    pub fn l_dma(&mut self) -> _L_DMAW {
        _L_DMAW { w: self }
    }
    #[doc = "Bit 4 - Non-secure access policy indicator bit"]
    #[inline]
    pub fn nsa_lcdif(&mut self) -> _NSA_LCDIFW {
        _NSA_LCDIFW { w: self }
    }
    #[doc = "Bit 5 - Lock bit set by the TZ software for the LCDIF"]
    #[inline]
    pub fn l_lcdif(&mut self) -> _L_LCDIFW {
        _L_LCDIFW { w: self }
    }
    #[doc = "Bit 6 - Non-secure access policy indicator bit"]
    #[inline]
    pub fn nsa_csi(&mut self) -> _NSA_CSIW {
        _NSA_CSIW { w: self }
    }
    #[doc = "Bit 7 - Lock bit set by the TZ software for the CSI"]
    #[inline]
    pub fn l_csi(&mut self) -> _L_CSIW {
        _L_CSIW { w: self }
    }
    #[doc = "Bit 8 - Non-Secure Access Policy indicator bit"]
    #[inline]
    pub fn nsa_pxp(&mut self) -> _NSA_PXPW {
        _NSA_PXPW { w: self }
    }
    #[doc = "Bit 9 - Lock bit set by the TZ software for the PXP"]
    #[inline]
    pub fn l_pxp(&mut self) -> _L_PXPW {
        _L_PXPW { w: self }
    }
    #[doc = "Bit 10 - Non-secure access policy indicator bit"]
    #[inline]
    pub fn nsa_dcp(&mut self) -> _NSA_DCPW {
        _NSA_DCPW { w: self }
    }
    #[doc = "Bit 11 - Lock bit set by the TZ software for the DCP"]
    #[inline]
    pub fn l_dcp(&mut self) -> _L_DCPW {
        _L_DCPW { w: self }
    }
    #[doc = "Bit 14 - Non-secure access policy indicator bit"]
    #[inline]
    pub fn nsa_enet(&mut self) -> _NSA_ENETW {
        _NSA_ENETW { w: self }
    }
    #[doc = "Bit 15 - Lock bit set by the TZ software for the ENET1 and ENET2"]
    #[inline]
    pub fn l_enet(&mut self) -> _L_ENETW {
        _L_ENETW { w: self }
    }
    #[doc = "Bit 16 - Non-secure access policy indicator bit"]
    #[inline]
    pub fn nsa_usdhc1(&mut self) -> _NSA_USDHC1W {
        _NSA_USDHC1W { w: self }
    }
    #[doc = "Bit 17 - Lock bit set by the TZ software for the USDHC1"]
    #[inline]
    pub fn l_usdhc1(&mut self) -> _L_USDHC1W {
        _L_USDHC1W { w: self }
    }
    #[doc = "Bit 18 - Non-secure access policy indicator bit"]
    #[inline]
    pub fn nsa_usdhc2(&mut self) -> _NSA_USDHC2W {
        _NSA_USDHC2W { w: self }
    }
    #[doc = "Bit 19 - Lock bit set by the TZ software for the USDHC2"]
    #[inline]
    pub fn l_usdhc2(&mut self) -> _L_USDHC2W {
        _L_USDHC2W { w: self }
    }
    #[doc = "Bit 20 - Non-secure access policy indicator bit"]
    #[inline]
    pub fn nsa_tpsmp(&mut self) -> _NSA_TPSMPW {
        _NSA_TPSMPW { w: self }
    }
    #[doc = "Bit 21 - Lock bit set by the TZ software for the TPSMP"]
    #[inline]
    pub fn l_tpsmp(&mut self) -> _L_TPSMPW {
        _L_TPSMPW { w: self }
    }
    #[doc = "Bit 22 - Non-secure access policy indicator bit"]
    #[inline]
    pub fn nsa_usb(&mut self) -> _NSA_USBW {
        _NSA_USBW { w: self }
    }
    #[doc = "Bit 23 - Lock bit set by the TZ software for the USB"]
    #[inline]
    pub fn l_usb(&mut self) -> _L_USBW {
        _L_USBW { w: self }
    }
}
