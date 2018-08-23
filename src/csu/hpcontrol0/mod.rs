#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::HPCONTROL0 {
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
#[doc = "Possible values of the field `HPC_DMA`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HPC_DMAR {
    #[doc = "User mode for the corresponding master"]
    HPC_DMA_0,
    #[doc = "Supervisor mode for the corresponding master"]
    HPC_DMA_1,
}
impl HPC_DMAR {
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
            HPC_DMAR::HPC_DMA_0 => false,
            HPC_DMAR::HPC_DMA_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> HPC_DMAR {
        match value {
            false => HPC_DMAR::HPC_DMA_0,
            true => HPC_DMAR::HPC_DMA_1,
        }
    }
    #[doc = "Checks if the value of the field is `HPC_DMA_0`"]
    #[inline]
    pub fn is_hpc_dma_0(&self) -> bool {
        *self == HPC_DMAR::HPC_DMA_0
    }
    #[doc = "Checks if the value of the field is `HPC_DMA_1`"]
    #[inline]
    pub fn is_hpc_dma_1(&self) -> bool {
        *self == HPC_DMAR::HPC_DMA_1
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
#[doc = "Possible values of the field `HPC_LCDIF`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HPC_LCDIFR {
    #[doc = "User mode for the corresponding master"]
    HPC_LCDIF_0,
    #[doc = "Supervisor mode for the corresponding master"]
    HPC_LCDIF_1,
}
impl HPC_LCDIFR {
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
            HPC_LCDIFR::HPC_LCDIF_0 => false,
            HPC_LCDIFR::HPC_LCDIF_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> HPC_LCDIFR {
        match value {
            false => HPC_LCDIFR::HPC_LCDIF_0,
            true => HPC_LCDIFR::HPC_LCDIF_1,
        }
    }
    #[doc = "Checks if the value of the field is `HPC_LCDIF_0`"]
    #[inline]
    pub fn is_hpc_lcdif_0(&self) -> bool {
        *self == HPC_LCDIFR::HPC_LCDIF_0
    }
    #[doc = "Checks if the value of the field is `HPC_LCDIF_1`"]
    #[inline]
    pub fn is_hpc_lcdif_1(&self) -> bool {
        *self == HPC_LCDIFR::HPC_LCDIF_1
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
#[doc = "Possible values of the field `HPC_CSI`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HPC_CSIR {
    #[doc = "User mode for the corresponding master"]
    HPC_CSI_0,
    #[doc = "Supervisor mode for the corresponding master"]
    HPC_CSI_1,
}
impl HPC_CSIR {
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
            HPC_CSIR::HPC_CSI_0 => false,
            HPC_CSIR::HPC_CSI_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> HPC_CSIR {
        match value {
            false => HPC_CSIR::HPC_CSI_0,
            true => HPC_CSIR::HPC_CSI_1,
        }
    }
    #[doc = "Checks if the value of the field is `HPC_CSI_0`"]
    #[inline]
    pub fn is_hpc_csi_0(&self) -> bool {
        *self == HPC_CSIR::HPC_CSI_0
    }
    #[doc = "Checks if the value of the field is `HPC_CSI_1`"]
    #[inline]
    pub fn is_hpc_csi_1(&self) -> bool {
        *self == HPC_CSIR::HPC_CSI_1
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
#[doc = "Possible values of the field `HPC_PXP`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HPC_PXPR {
    #[doc = "User mode for the corresponding master"]
    HPC_PXP_0,
    #[doc = "Supervisor mode for the corresponding master"]
    HPC_PXP_1,
}
impl HPC_PXPR {
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
            HPC_PXPR::HPC_PXP_0 => false,
            HPC_PXPR::HPC_PXP_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> HPC_PXPR {
        match value {
            false => HPC_PXPR::HPC_PXP_0,
            true => HPC_PXPR::HPC_PXP_1,
        }
    }
    #[doc = "Checks if the value of the field is `HPC_PXP_0`"]
    #[inline]
    pub fn is_hpc_pxp_0(&self) -> bool {
        *self == HPC_PXPR::HPC_PXP_0
    }
    #[doc = "Checks if the value of the field is `HPC_PXP_1`"]
    #[inline]
    pub fn is_hpc_pxp_1(&self) -> bool {
        *self == HPC_PXPR::HPC_PXP_1
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
#[doc = "Possible values of the field `HPC_DCP`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HPC_DCPR {
    #[doc = "User mode for the corresponding master"]
    HPC_DCP_0,
    #[doc = "Supervisor mode for the corresponding master"]
    HPC_DCP_1,
}
impl HPC_DCPR {
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
            HPC_DCPR::HPC_DCP_0 => false,
            HPC_DCPR::HPC_DCP_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> HPC_DCPR {
        match value {
            false => HPC_DCPR::HPC_DCP_0,
            true => HPC_DCPR::HPC_DCP_1,
        }
    }
    #[doc = "Checks if the value of the field is `HPC_DCP_0`"]
    #[inline]
    pub fn is_hpc_dcp_0(&self) -> bool {
        *self == HPC_DCPR::HPC_DCP_0
    }
    #[doc = "Checks if the value of the field is `HPC_DCP_1`"]
    #[inline]
    pub fn is_hpc_dcp_1(&self) -> bool {
        *self == HPC_DCPR::HPC_DCP_1
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
#[doc = "Possible values of the field `HPC_ENET`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HPC_ENETR {
    #[doc = "User mode for the corresponding master"]
    HPC_ENET_0,
    #[doc = "Supervisor mode for the corresponding master"]
    HPC_ENET_1,
}
impl HPC_ENETR {
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
            HPC_ENETR::HPC_ENET_0 => false,
            HPC_ENETR::HPC_ENET_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> HPC_ENETR {
        match value {
            false => HPC_ENETR::HPC_ENET_0,
            true => HPC_ENETR::HPC_ENET_1,
        }
    }
    #[doc = "Checks if the value of the field is `HPC_ENET_0`"]
    #[inline]
    pub fn is_hpc_enet_0(&self) -> bool {
        *self == HPC_ENETR::HPC_ENET_0
    }
    #[doc = "Checks if the value of the field is `HPC_ENET_1`"]
    #[inline]
    pub fn is_hpc_enet_1(&self) -> bool {
        *self == HPC_ENETR::HPC_ENET_1
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
#[doc = "Possible values of the field `HPC_USDHC1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HPC_USDHC1R {
    #[doc = "User mode for the corresponding master"]
    HPC_USDHC1_0,
    #[doc = "Supervisor mode for the corresponding master"]
    HPC_USDHC1_1,
}
impl HPC_USDHC1R {
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
            HPC_USDHC1R::HPC_USDHC1_0 => false,
            HPC_USDHC1R::HPC_USDHC1_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> HPC_USDHC1R {
        match value {
            false => HPC_USDHC1R::HPC_USDHC1_0,
            true => HPC_USDHC1R::HPC_USDHC1_1,
        }
    }
    #[doc = "Checks if the value of the field is `HPC_USDHC1_0`"]
    #[inline]
    pub fn is_hpc_usdhc1_0(&self) -> bool {
        *self == HPC_USDHC1R::HPC_USDHC1_0
    }
    #[doc = "Checks if the value of the field is `HPC_USDHC1_1`"]
    #[inline]
    pub fn is_hpc_usdhc1_1(&self) -> bool {
        *self == HPC_USDHC1R::HPC_USDHC1_1
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
#[doc = "Possible values of the field `HPC_USDHC2`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HPC_USDHC2R {
    #[doc = "User mode for the corresponding master"]
    HPC_USDHC2_0,
    #[doc = "Supervisor mode for the corresponding master"]
    HPC_USDHC2_1,
}
impl HPC_USDHC2R {
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
            HPC_USDHC2R::HPC_USDHC2_0 => false,
            HPC_USDHC2R::HPC_USDHC2_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> HPC_USDHC2R {
        match value {
            false => HPC_USDHC2R::HPC_USDHC2_0,
            true => HPC_USDHC2R::HPC_USDHC2_1,
        }
    }
    #[doc = "Checks if the value of the field is `HPC_USDHC2_0`"]
    #[inline]
    pub fn is_hpc_usdhc2_0(&self) -> bool {
        *self == HPC_USDHC2R::HPC_USDHC2_0
    }
    #[doc = "Checks if the value of the field is `HPC_USDHC2_1`"]
    #[inline]
    pub fn is_hpc_usdhc2_1(&self) -> bool {
        *self == HPC_USDHC2R::HPC_USDHC2_1
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
#[doc = "Possible values of the field `HPC_TPSMP`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HPC_TPSMPR {
    #[doc = "User mode for the corresponding master"]
    HPC_TPSMP_0,
    #[doc = "Supervisor mode for the corresponding master"]
    HPC_TPSMP_1,
}
impl HPC_TPSMPR {
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
            HPC_TPSMPR::HPC_TPSMP_0 => false,
            HPC_TPSMPR::HPC_TPSMP_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> HPC_TPSMPR {
        match value {
            false => HPC_TPSMPR::HPC_TPSMP_0,
            true => HPC_TPSMPR::HPC_TPSMP_1,
        }
    }
    #[doc = "Checks if the value of the field is `HPC_TPSMP_0`"]
    #[inline]
    pub fn is_hpc_tpsmp_0(&self) -> bool {
        *self == HPC_TPSMPR::HPC_TPSMP_0
    }
    #[doc = "Checks if the value of the field is `HPC_TPSMP_1`"]
    #[inline]
    pub fn is_hpc_tpsmp_1(&self) -> bool {
        *self == HPC_TPSMPR::HPC_TPSMP_1
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
#[doc = "Possible values of the field `HPC_USB`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HPC_USBR {
    #[doc = "User mode for the corresponding master"]
    HPC_USB_0,
    #[doc = "Supervisor mode for the corresponding master"]
    HPC_USB_1,
}
impl HPC_USBR {
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
            HPC_USBR::HPC_USB_0 => false,
            HPC_USBR::HPC_USB_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> HPC_USBR {
        match value {
            false => HPC_USBR::HPC_USB_0,
            true => HPC_USBR::HPC_USB_1,
        }
    }
    #[doc = "Checks if the value of the field is `HPC_USB_0`"]
    #[inline]
    pub fn is_hpc_usb_0(&self) -> bool {
        *self == HPC_USBR::HPC_USB_0
    }
    #[doc = "Checks if the value of the field is `HPC_USB_1`"]
    #[inline]
    pub fn is_hpc_usb_1(&self) -> bool {
        *self == HPC_USBR::HPC_USB_1
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
#[doc = "Values that can be written to the field `HPC_DMA`"]
pub enum HPC_DMAW {
    #[doc = "User mode for the corresponding master"]
    HPC_DMA_0,
    #[doc = "Supervisor mode for the corresponding master"]
    HPC_DMA_1,
}
impl HPC_DMAW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            HPC_DMAW::HPC_DMA_0 => false,
            HPC_DMAW::HPC_DMA_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _HPC_DMAW<'a> {
    w: &'a mut W,
}
impl<'a> _HPC_DMAW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: HPC_DMAW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "User mode for the corresponding master"]
    #[inline]
    pub fn hpc_dma_0(self) -> &'a mut W {
        self.variant(HPC_DMAW::HPC_DMA_0)
    }
    #[doc = "Supervisor mode for the corresponding master"]
    #[inline]
    pub fn hpc_dma_1(self) -> &'a mut W {
        self.variant(HPC_DMAW::HPC_DMA_1)
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
#[doc = "Values that can be written to the field `HPC_LCDIF`"]
pub enum HPC_LCDIFW {
    #[doc = "User mode for the corresponding master"]
    HPC_LCDIF_0,
    #[doc = "Supervisor mode for the corresponding master"]
    HPC_LCDIF_1,
}
impl HPC_LCDIFW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            HPC_LCDIFW::HPC_LCDIF_0 => false,
            HPC_LCDIFW::HPC_LCDIF_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _HPC_LCDIFW<'a> {
    w: &'a mut W,
}
impl<'a> _HPC_LCDIFW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: HPC_LCDIFW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "User mode for the corresponding master"]
    #[inline]
    pub fn hpc_lcdif_0(self) -> &'a mut W {
        self.variant(HPC_LCDIFW::HPC_LCDIF_0)
    }
    #[doc = "Supervisor mode for the corresponding master"]
    #[inline]
    pub fn hpc_lcdif_1(self) -> &'a mut W {
        self.variant(HPC_LCDIFW::HPC_LCDIF_1)
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
#[doc = "Values that can be written to the field `HPC_CSI`"]
pub enum HPC_CSIW {
    #[doc = "User mode for the corresponding master"]
    HPC_CSI_0,
    #[doc = "Supervisor mode for the corresponding master"]
    HPC_CSI_1,
}
impl HPC_CSIW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            HPC_CSIW::HPC_CSI_0 => false,
            HPC_CSIW::HPC_CSI_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _HPC_CSIW<'a> {
    w: &'a mut W,
}
impl<'a> _HPC_CSIW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: HPC_CSIW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "User mode for the corresponding master"]
    #[inline]
    pub fn hpc_csi_0(self) -> &'a mut W {
        self.variant(HPC_CSIW::HPC_CSI_0)
    }
    #[doc = "Supervisor mode for the corresponding master"]
    #[inline]
    pub fn hpc_csi_1(self) -> &'a mut W {
        self.variant(HPC_CSIW::HPC_CSI_1)
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
#[doc = "Values that can be written to the field `HPC_PXP`"]
pub enum HPC_PXPW {
    #[doc = "User mode for the corresponding master"]
    HPC_PXP_0,
    #[doc = "Supervisor mode for the corresponding master"]
    HPC_PXP_1,
}
impl HPC_PXPW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            HPC_PXPW::HPC_PXP_0 => false,
            HPC_PXPW::HPC_PXP_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _HPC_PXPW<'a> {
    w: &'a mut W,
}
impl<'a> _HPC_PXPW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: HPC_PXPW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "User mode for the corresponding master"]
    #[inline]
    pub fn hpc_pxp_0(self) -> &'a mut W {
        self.variant(HPC_PXPW::HPC_PXP_0)
    }
    #[doc = "Supervisor mode for the corresponding master"]
    #[inline]
    pub fn hpc_pxp_1(self) -> &'a mut W {
        self.variant(HPC_PXPW::HPC_PXP_1)
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
#[doc = "Values that can be written to the field `HPC_DCP`"]
pub enum HPC_DCPW {
    #[doc = "User mode for the corresponding master"]
    HPC_DCP_0,
    #[doc = "Supervisor mode for the corresponding master"]
    HPC_DCP_1,
}
impl HPC_DCPW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            HPC_DCPW::HPC_DCP_0 => false,
            HPC_DCPW::HPC_DCP_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _HPC_DCPW<'a> {
    w: &'a mut W,
}
impl<'a> _HPC_DCPW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: HPC_DCPW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "User mode for the corresponding master"]
    #[inline]
    pub fn hpc_dcp_0(self) -> &'a mut W {
        self.variant(HPC_DCPW::HPC_DCP_0)
    }
    #[doc = "Supervisor mode for the corresponding master"]
    #[inline]
    pub fn hpc_dcp_1(self) -> &'a mut W {
        self.variant(HPC_DCPW::HPC_DCP_1)
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
#[doc = "Values that can be written to the field `HPC_ENET`"]
pub enum HPC_ENETW {
    #[doc = "User mode for the corresponding master"]
    HPC_ENET_0,
    #[doc = "Supervisor mode for the corresponding master"]
    HPC_ENET_1,
}
impl HPC_ENETW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            HPC_ENETW::HPC_ENET_0 => false,
            HPC_ENETW::HPC_ENET_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _HPC_ENETW<'a> {
    w: &'a mut W,
}
impl<'a> _HPC_ENETW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: HPC_ENETW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "User mode for the corresponding master"]
    #[inline]
    pub fn hpc_enet_0(self) -> &'a mut W {
        self.variant(HPC_ENETW::HPC_ENET_0)
    }
    #[doc = "Supervisor mode for the corresponding master"]
    #[inline]
    pub fn hpc_enet_1(self) -> &'a mut W {
        self.variant(HPC_ENETW::HPC_ENET_1)
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
#[doc = "Values that can be written to the field `HPC_USDHC1`"]
pub enum HPC_USDHC1W {
    #[doc = "User mode for the corresponding master"]
    HPC_USDHC1_0,
    #[doc = "Supervisor mode for the corresponding master"]
    HPC_USDHC1_1,
}
impl HPC_USDHC1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            HPC_USDHC1W::HPC_USDHC1_0 => false,
            HPC_USDHC1W::HPC_USDHC1_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _HPC_USDHC1W<'a> {
    w: &'a mut W,
}
impl<'a> _HPC_USDHC1W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: HPC_USDHC1W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "User mode for the corresponding master"]
    #[inline]
    pub fn hpc_usdhc1_0(self) -> &'a mut W {
        self.variant(HPC_USDHC1W::HPC_USDHC1_0)
    }
    #[doc = "Supervisor mode for the corresponding master"]
    #[inline]
    pub fn hpc_usdhc1_1(self) -> &'a mut W {
        self.variant(HPC_USDHC1W::HPC_USDHC1_1)
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
#[doc = "Values that can be written to the field `HPC_USDHC2`"]
pub enum HPC_USDHC2W {
    #[doc = "User mode for the corresponding master"]
    HPC_USDHC2_0,
    #[doc = "Supervisor mode for the corresponding master"]
    HPC_USDHC2_1,
}
impl HPC_USDHC2W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            HPC_USDHC2W::HPC_USDHC2_0 => false,
            HPC_USDHC2W::HPC_USDHC2_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _HPC_USDHC2W<'a> {
    w: &'a mut W,
}
impl<'a> _HPC_USDHC2W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: HPC_USDHC2W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "User mode for the corresponding master"]
    #[inline]
    pub fn hpc_usdhc2_0(self) -> &'a mut W {
        self.variant(HPC_USDHC2W::HPC_USDHC2_0)
    }
    #[doc = "Supervisor mode for the corresponding master"]
    #[inline]
    pub fn hpc_usdhc2_1(self) -> &'a mut W {
        self.variant(HPC_USDHC2W::HPC_USDHC2_1)
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
#[doc = "Values that can be written to the field `HPC_TPSMP`"]
pub enum HPC_TPSMPW {
    #[doc = "User mode for the corresponding master"]
    HPC_TPSMP_0,
    #[doc = "Supervisor mode for the corresponding master"]
    HPC_TPSMP_1,
}
impl HPC_TPSMPW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            HPC_TPSMPW::HPC_TPSMP_0 => false,
            HPC_TPSMPW::HPC_TPSMP_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _HPC_TPSMPW<'a> {
    w: &'a mut W,
}
impl<'a> _HPC_TPSMPW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: HPC_TPSMPW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "User mode for the corresponding master"]
    #[inline]
    pub fn hpc_tpsmp_0(self) -> &'a mut W {
        self.variant(HPC_TPSMPW::HPC_TPSMP_0)
    }
    #[doc = "Supervisor mode for the corresponding master"]
    #[inline]
    pub fn hpc_tpsmp_1(self) -> &'a mut W {
        self.variant(HPC_TPSMPW::HPC_TPSMP_1)
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
#[doc = "Values that can be written to the field `HPC_USB`"]
pub enum HPC_USBW {
    #[doc = "User mode for the corresponding master"]
    HPC_USB_0,
    #[doc = "Supervisor mode for the corresponding master"]
    HPC_USB_1,
}
impl HPC_USBW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            HPC_USBW::HPC_USB_0 => false,
            HPC_USBW::HPC_USB_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _HPC_USBW<'a> {
    w: &'a mut W,
}
impl<'a> _HPC_USBW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: HPC_USBW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "User mode for the corresponding master"]
    #[inline]
    pub fn hpc_usb_0(self) -> &'a mut W {
        self.variant(HPC_USBW::HPC_USB_0)
    }
    #[doc = "Supervisor mode for the corresponding master"]
    #[inline]
    pub fn hpc_usb_1(self) -> &'a mut W {
        self.variant(HPC_USBW::HPC_USB_1)
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
    #[doc = "Bit 2 - Indicates the privilege/user mode for the eDMA"]
    #[inline]
    pub fn hpc_dma(&self) -> HPC_DMAR {
        HPC_DMAR::_from({
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
    #[doc = "Bit 4 - Indicates the privilege/user mode for the LCDIF"]
    #[inline]
    pub fn hpc_lcdif(&self) -> HPC_LCDIFR {
        HPC_LCDIFR::_from({
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
    #[doc = "Bit 6 - Indicates the privilege/user mode for the CSI"]
    #[inline]
    pub fn hpc_csi(&self) -> HPC_CSIR {
        HPC_CSIR::_from({
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
    #[doc = "Bit 8 - Indicates the privilege/user mode for the PXP"]
    #[inline]
    pub fn hpc_pxp(&self) -> HPC_PXPR {
        HPC_PXPR::_from({
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
    #[doc = "Bit 10 - Indicates the privilege/user mode for the DCP"]
    #[inline]
    pub fn hpc_dcp(&self) -> HPC_DCPR {
        HPC_DCPR::_from({
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
    #[doc = "Bit 14 - Indicates the privilege/user mode for the ENET"]
    #[inline]
    pub fn hpc_enet(&self) -> HPC_ENETR {
        HPC_ENETR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 15 - Lock bit set by the TZ software for the ENET"]
    #[inline]
    pub fn l_enet(&self) -> L_ENETR {
        L_ENETR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 15;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 16 - Indicates the privilege/user mode for the USDHC1"]
    #[inline]
    pub fn hpc_usdhc1(&self) -> HPC_USDHC1R {
        HPC_USDHC1R::_from({
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
    #[doc = "Bit 18 - Indicates the privilege/user mode for the USDHC2"]
    #[inline]
    pub fn hpc_usdhc2(&self) -> HPC_USDHC2R {
        HPC_USDHC2R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 19 - Lock bit set by the TZ software for the USDHC2."]
    #[inline]
    pub fn l_usdhc2(&self) -> L_USDHC2R {
        L_USDHC2R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 19;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 20 - Indicates the privilege/user mode for the TPSMP"]
    #[inline]
    pub fn hpc_tpsmp(&self) -> HPC_TPSMPR {
        HPC_TPSMPR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 21 - Lock bit set by the TZ software for the TPSMP."]
    #[inline]
    pub fn l_tpsmp(&self) -> L_TPSMPR {
        L_TPSMPR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 21;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 22 - Indicates the privilege/user mode for the USB"]
    #[inline]
    pub fn hpc_usb(&self) -> HPC_USBR {
        HPC_USBR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 22;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 23 - Lock bit set by the TZ software for the USB."]
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
    #[doc = "Bit 2 - Indicates the privilege/user mode for the eDMA"]
    #[inline]
    pub fn hpc_dma(&mut self) -> _HPC_DMAW {
        _HPC_DMAW { w: self }
    }
    #[doc = "Bit 3 - Lock bit set by the TZ software for the eDMA"]
    #[inline]
    pub fn l_dma(&mut self) -> _L_DMAW {
        _L_DMAW { w: self }
    }
    #[doc = "Bit 4 - Indicates the privilege/user mode for the LCDIF"]
    #[inline]
    pub fn hpc_lcdif(&mut self) -> _HPC_LCDIFW {
        _HPC_LCDIFW { w: self }
    }
    #[doc = "Bit 5 - Lock bit set by the TZ software for the LCDIF"]
    #[inline]
    pub fn l_lcdif(&mut self) -> _L_LCDIFW {
        _L_LCDIFW { w: self }
    }
    #[doc = "Bit 6 - Indicates the privilege/user mode for the CSI"]
    #[inline]
    pub fn hpc_csi(&mut self) -> _HPC_CSIW {
        _HPC_CSIW { w: self }
    }
    #[doc = "Bit 7 - Lock bit set by the TZ software for the CSI"]
    #[inline]
    pub fn l_csi(&mut self) -> _L_CSIW {
        _L_CSIW { w: self }
    }
    #[doc = "Bit 8 - Indicates the privilege/user mode for the PXP"]
    #[inline]
    pub fn hpc_pxp(&mut self) -> _HPC_PXPW {
        _HPC_PXPW { w: self }
    }
    #[doc = "Bit 9 - Lock bit set by the TZ software for the PXP"]
    #[inline]
    pub fn l_pxp(&mut self) -> _L_PXPW {
        _L_PXPW { w: self }
    }
    #[doc = "Bit 10 - Indicates the privilege/user mode for the DCP"]
    #[inline]
    pub fn hpc_dcp(&mut self) -> _HPC_DCPW {
        _HPC_DCPW { w: self }
    }
    #[doc = "Bit 11 - Lock bit set by the TZ software for the DCP"]
    #[inline]
    pub fn l_dcp(&mut self) -> _L_DCPW {
        _L_DCPW { w: self }
    }
    #[doc = "Bit 14 - Indicates the privilege/user mode for the ENET"]
    #[inline]
    pub fn hpc_enet(&mut self) -> _HPC_ENETW {
        _HPC_ENETW { w: self }
    }
    #[doc = "Bit 15 - Lock bit set by the TZ software for the ENET"]
    #[inline]
    pub fn l_enet(&mut self) -> _L_ENETW {
        _L_ENETW { w: self }
    }
    #[doc = "Bit 16 - Indicates the privilege/user mode for the USDHC1"]
    #[inline]
    pub fn hpc_usdhc1(&mut self) -> _HPC_USDHC1W {
        _HPC_USDHC1W { w: self }
    }
    #[doc = "Bit 17 - Lock bit set by the TZ software for the USDHC1"]
    #[inline]
    pub fn l_usdhc1(&mut self) -> _L_USDHC1W {
        _L_USDHC1W { w: self }
    }
    #[doc = "Bit 18 - Indicates the privilege/user mode for the USDHC2"]
    #[inline]
    pub fn hpc_usdhc2(&mut self) -> _HPC_USDHC2W {
        _HPC_USDHC2W { w: self }
    }
    #[doc = "Bit 19 - Lock bit set by the TZ software for the USDHC2."]
    #[inline]
    pub fn l_usdhc2(&mut self) -> _L_USDHC2W {
        _L_USDHC2W { w: self }
    }
    #[doc = "Bit 20 - Indicates the privilege/user mode for the TPSMP"]
    #[inline]
    pub fn hpc_tpsmp(&mut self) -> _HPC_TPSMPW {
        _HPC_TPSMPW { w: self }
    }
    #[doc = "Bit 21 - Lock bit set by the TZ software for the TPSMP."]
    #[inline]
    pub fn l_tpsmp(&mut self) -> _L_TPSMPW {
        _L_TPSMPW { w: self }
    }
    #[doc = "Bit 22 - Indicates the privilege/user mode for the USB"]
    #[inline]
    pub fn hpc_usb(&mut self) -> _HPC_USBW {
        _HPC_USBW { w: self }
    }
    #[doc = "Bit 23 - Lock bit set by the TZ software for the USB."]
    #[inline]
    pub fn l_usb(&mut self) -> _L_USBW {
        _L_USBW { w: self }
    }
}
