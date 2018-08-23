#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::HP0 {
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
#[doc = "Possible values of the field `HP_DMA`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HP_DMAR {
    #[doc = "The hprot1 input signal value is routed to the csu_hprot1 output for the corresponding master."]
    HP_DMA_0,
    #[doc = "The HP register bit is routed to the csu_hprot1 output for the corresponding master."]
    HP_DMA_1,
}
impl HP_DMAR {
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
            HP_DMAR::HP_DMA_0 => false,
            HP_DMAR::HP_DMA_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> HP_DMAR {
        match value {
            false => HP_DMAR::HP_DMA_0,
            true => HP_DMAR::HP_DMA_1,
        }
    }
    #[doc = "Checks if the value of the field is `HP_DMA_0`"]
    #[inline]
    pub fn is_hp_dma_0(&self) -> bool {
        *self == HP_DMAR::HP_DMA_0
    }
    #[doc = "Checks if the value of the field is `HP_DMA_1`"]
    #[inline]
    pub fn is_hp_dma_1(&self) -> bool {
        *self == HP_DMAR::HP_DMA_1
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
#[doc = "Possible values of the field `HP_LCDIF`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HP_LCDIFR {
    #[doc = "The hprot1 input signal value is routed to the csu_hprot1 output for the corresponding master."]
    HP_LCDIF_0,
    #[doc = "The HP register bit is routed to the csu_hprot1 output for the corresponding master."]
    HP_LCDIF_1,
}
impl HP_LCDIFR {
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
            HP_LCDIFR::HP_LCDIF_0 => false,
            HP_LCDIFR::HP_LCDIF_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> HP_LCDIFR {
        match value {
            false => HP_LCDIFR::HP_LCDIF_0,
            true => HP_LCDIFR::HP_LCDIF_1,
        }
    }
    #[doc = "Checks if the value of the field is `HP_LCDIF_0`"]
    #[inline]
    pub fn is_hp_lcdif_0(&self) -> bool {
        *self == HP_LCDIFR::HP_LCDIF_0
    }
    #[doc = "Checks if the value of the field is `HP_LCDIF_1`"]
    #[inline]
    pub fn is_hp_lcdif_1(&self) -> bool {
        *self == HP_LCDIFR::HP_LCDIF_1
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
#[doc = "Possible values of the field `HP_CSI`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HP_CSIR {
    #[doc = "The hprot1 input signal value is routed to the csu_hprot1 output for the corresponding master."]
    HP_CSI_0,
    #[doc = "The HP register bit is routed to the csu_hprot1 output for the corresponding master."]
    HP_CSI_1,
}
impl HP_CSIR {
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
            HP_CSIR::HP_CSI_0 => false,
            HP_CSIR::HP_CSI_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> HP_CSIR {
        match value {
            false => HP_CSIR::HP_CSI_0,
            true => HP_CSIR::HP_CSI_1,
        }
    }
    #[doc = "Checks if the value of the field is `HP_CSI_0`"]
    #[inline]
    pub fn is_hp_csi_0(&self) -> bool {
        *self == HP_CSIR::HP_CSI_0
    }
    #[doc = "Checks if the value of the field is `HP_CSI_1`"]
    #[inline]
    pub fn is_hp_csi_1(&self) -> bool {
        *self == HP_CSIR::HP_CSI_1
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
#[doc = "Possible values of the field `HP_PXP`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HP_PXPR {
    #[doc = "The hprot1 input signal value is routed to the csu_hprot1 output for the corresponding master."]
    HP_PXP_0,
    #[doc = "The HP register bit is routed to the csu_hprot1 output for the corresponding master."]
    HP_PXP_1,
}
impl HP_PXPR {
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
            HP_PXPR::HP_PXP_0 => false,
            HP_PXPR::HP_PXP_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> HP_PXPR {
        match value {
            false => HP_PXPR::HP_PXP_0,
            true => HP_PXPR::HP_PXP_1,
        }
    }
    #[doc = "Checks if the value of the field is `HP_PXP_0`"]
    #[inline]
    pub fn is_hp_pxp_0(&self) -> bool {
        *self == HP_PXPR::HP_PXP_0
    }
    #[doc = "Checks if the value of the field is `HP_PXP_1`"]
    #[inline]
    pub fn is_hp_pxp_1(&self) -> bool {
        *self == HP_PXPR::HP_PXP_1
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
#[doc = "Possible values of the field `HP_DCP`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HP_DCPR {
    #[doc = "The hprot1 input signal value is routed to the csu_hprot1 output for the corresponding master."]
    HP_DCP_0,
    #[doc = "The HP register bit is routed to the csu_hprot1 output for the corresponding master."]
    HP_DCP_1,
}
impl HP_DCPR {
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
            HP_DCPR::HP_DCP_0 => false,
            HP_DCPR::HP_DCP_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> HP_DCPR {
        match value {
            false => HP_DCPR::HP_DCP_0,
            true => HP_DCPR::HP_DCP_1,
        }
    }
    #[doc = "Checks if the value of the field is `HP_DCP_0`"]
    #[inline]
    pub fn is_hp_dcp_0(&self) -> bool {
        *self == HP_DCPR::HP_DCP_0
    }
    #[doc = "Checks if the value of the field is `HP_DCP_1`"]
    #[inline]
    pub fn is_hp_dcp_1(&self) -> bool {
        *self == HP_DCPR::HP_DCP_1
    }
}
#[doc = "Possible values of the field `L_DCP`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum L_DCPR {
    #[doc = "No lock-the adjacent (next lower) bit can be written by the software."]
    L_DCP_0,
    #[doc = "Lock-the adjacent (next lower) bit cannot be written by the software."]
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
#[doc = "Possible values of the field `HP_ENET`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HP_ENETR {
    #[doc = "The hprot1 input signal value is routed to the csu_hprot1 output for the corresponding master."]
    HP_ENET_0,
    #[doc = "The HP register bit is routed to the csu_hprot1 output for the corresponding master."]
    HP_ENET_1,
}
impl HP_ENETR {
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
            HP_ENETR::HP_ENET_0 => false,
            HP_ENETR::HP_ENET_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> HP_ENETR {
        match value {
            false => HP_ENETR::HP_ENET_0,
            true => HP_ENETR::HP_ENET_1,
        }
    }
    #[doc = "Checks if the value of the field is `HP_ENET_0`"]
    #[inline]
    pub fn is_hp_enet_0(&self) -> bool {
        *self == HP_ENETR::HP_ENET_0
    }
    #[doc = "Checks if the value of the field is `HP_ENET_1`"]
    #[inline]
    pub fn is_hp_enet_1(&self) -> bool {
        *self == HP_ENETR::HP_ENET_1
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
#[doc = "Possible values of the field `HP_USDHC1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HP_USDHC1R {
    #[doc = "The hprot1 input signal value is routed to the csu_hprot1 output for the corresponding master."]
    HP_USDHC1_0,
    #[doc = "The HP register bit is routed to the csu_hprot1 output for the corresponding master."]
    HP_USDHC1_1,
}
impl HP_USDHC1R {
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
            HP_USDHC1R::HP_USDHC1_0 => false,
            HP_USDHC1R::HP_USDHC1_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> HP_USDHC1R {
        match value {
            false => HP_USDHC1R::HP_USDHC1_0,
            true => HP_USDHC1R::HP_USDHC1_1,
        }
    }
    #[doc = "Checks if the value of the field is `HP_USDHC1_0`"]
    #[inline]
    pub fn is_hp_usdhc1_0(&self) -> bool {
        *self == HP_USDHC1R::HP_USDHC1_0
    }
    #[doc = "Checks if the value of the field is `HP_USDHC1_1`"]
    #[inline]
    pub fn is_hp_usdhc1_1(&self) -> bool {
        *self == HP_USDHC1R::HP_USDHC1_1
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
#[doc = "Possible values of the field `HP_USDHC2`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HP_USDHC2R {
    #[doc = "The hprot1 input signal value is routed to the csu_hprot1 output for the corresponding master."]
    HP_USDHC2_0,
    #[doc = "The HP register bit is routed to the csu_hprot1 output for the corresponding master."]
    HP_USDHC2_1,
}
impl HP_USDHC2R {
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
            HP_USDHC2R::HP_USDHC2_0 => false,
            HP_USDHC2R::HP_USDHC2_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> HP_USDHC2R {
        match value {
            false => HP_USDHC2R::HP_USDHC2_0,
            true => HP_USDHC2R::HP_USDHC2_1,
        }
    }
    #[doc = "Checks if the value of the field is `HP_USDHC2_0`"]
    #[inline]
    pub fn is_hp_usdhc2_0(&self) -> bool {
        *self == HP_USDHC2R::HP_USDHC2_0
    }
    #[doc = "Checks if the value of the field is `HP_USDHC2_1`"]
    #[inline]
    pub fn is_hp_usdhc2_1(&self) -> bool {
        *self == HP_USDHC2R::HP_USDHC2_1
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
#[doc = "Possible values of the field `HP_TPSMP`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HP_TPSMPR {
    #[doc = "The hprot1 input signal value is routed to the csu_hprot1 output for the corresponding master."]
    HP_TPSMP_0,
    #[doc = "The HP register bit is routed to the csu_hprot1 output for the corresponding master."]
    HP_TPSMP_1,
}
impl HP_TPSMPR {
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
            HP_TPSMPR::HP_TPSMP_0 => false,
            HP_TPSMPR::HP_TPSMP_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> HP_TPSMPR {
        match value {
            false => HP_TPSMPR::HP_TPSMP_0,
            true => HP_TPSMPR::HP_TPSMP_1,
        }
    }
    #[doc = "Checks if the value of the field is `HP_TPSMP_0`"]
    #[inline]
    pub fn is_hp_tpsmp_0(&self) -> bool {
        *self == HP_TPSMPR::HP_TPSMP_0
    }
    #[doc = "Checks if the value of the field is `HP_TPSMP_1`"]
    #[inline]
    pub fn is_hp_tpsmp_1(&self) -> bool {
        *self == HP_TPSMPR::HP_TPSMP_1
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
#[doc = "Possible values of the field `HP_USB`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HP_USBR {
    #[doc = "The hprot1 input signal value is routed to the csu_hprot1 output for the corresponding master."]
    HP_USB_0,
    #[doc = "The HP register bit is routed to the csu_hprot1 output for the corresponding master."]
    HP_USB_1,
}
impl HP_USBR {
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
            HP_USBR::HP_USB_0 => false,
            HP_USBR::HP_USB_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> HP_USBR {
        match value {
            false => HP_USBR::HP_USB_0,
            true => HP_USBR::HP_USB_1,
        }
    }
    #[doc = "Checks if the value of the field is `HP_USB_0`"]
    #[inline]
    pub fn is_hp_usb_0(&self) -> bool {
        *self == HP_USBR::HP_USB_0
    }
    #[doc = "Checks if the value of the field is `HP_USB_1`"]
    #[inline]
    pub fn is_hp_usb_1(&self) -> bool {
        *self == HP_USBR::HP_USB_1
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
#[doc = "Values that can be written to the field `HP_DMA`"]
pub enum HP_DMAW {
    #[doc = "The hprot1 input signal value is routed to the csu_hprot1 output for the corresponding master."]
    HP_DMA_0,
    #[doc = "The HP register bit is routed to the csu_hprot1 output for the corresponding master."]
    HP_DMA_1,
}
impl HP_DMAW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            HP_DMAW::HP_DMA_0 => false,
            HP_DMAW::HP_DMA_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _HP_DMAW<'a> {
    w: &'a mut W,
}
impl<'a> _HP_DMAW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: HP_DMAW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The hprot1 input signal value is routed to the csu_hprot1 output for the corresponding master."]
    #[inline]
    pub fn hp_dma_0(self) -> &'a mut W {
        self.variant(HP_DMAW::HP_DMA_0)
    }
    #[doc = "The HP register bit is routed to the csu_hprot1 output for the corresponding master."]
    #[inline]
    pub fn hp_dma_1(self) -> &'a mut W {
        self.variant(HP_DMAW::HP_DMA_1)
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
#[doc = "Values that can be written to the field `HP_LCDIF`"]
pub enum HP_LCDIFW {
    #[doc = "The hprot1 input signal value is routed to the csu_hprot1 output for the corresponding master."]
    HP_LCDIF_0,
    #[doc = "The HP register bit is routed to the csu_hprot1 output for the corresponding master."]
    HP_LCDIF_1,
}
impl HP_LCDIFW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            HP_LCDIFW::HP_LCDIF_0 => false,
            HP_LCDIFW::HP_LCDIF_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _HP_LCDIFW<'a> {
    w: &'a mut W,
}
impl<'a> _HP_LCDIFW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: HP_LCDIFW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The hprot1 input signal value is routed to the csu_hprot1 output for the corresponding master."]
    #[inline]
    pub fn hp_lcdif_0(self) -> &'a mut W {
        self.variant(HP_LCDIFW::HP_LCDIF_0)
    }
    #[doc = "The HP register bit is routed to the csu_hprot1 output for the corresponding master."]
    #[inline]
    pub fn hp_lcdif_1(self) -> &'a mut W {
        self.variant(HP_LCDIFW::HP_LCDIF_1)
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
#[doc = "Values that can be written to the field `HP_CSI`"]
pub enum HP_CSIW {
    #[doc = "The hprot1 input signal value is routed to the csu_hprot1 output for the corresponding master."]
    HP_CSI_0,
    #[doc = "The HP register bit is routed to the csu_hprot1 output for the corresponding master."]
    HP_CSI_1,
}
impl HP_CSIW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            HP_CSIW::HP_CSI_0 => false,
            HP_CSIW::HP_CSI_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _HP_CSIW<'a> {
    w: &'a mut W,
}
impl<'a> _HP_CSIW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: HP_CSIW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The hprot1 input signal value is routed to the csu_hprot1 output for the corresponding master."]
    #[inline]
    pub fn hp_csi_0(self) -> &'a mut W {
        self.variant(HP_CSIW::HP_CSI_0)
    }
    #[doc = "The HP register bit is routed to the csu_hprot1 output for the corresponding master."]
    #[inline]
    pub fn hp_csi_1(self) -> &'a mut W {
        self.variant(HP_CSIW::HP_CSI_1)
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
#[doc = "Values that can be written to the field `HP_PXP`"]
pub enum HP_PXPW {
    #[doc = "The hprot1 input signal value is routed to the csu_hprot1 output for the corresponding master."]
    HP_PXP_0,
    #[doc = "The HP register bit is routed to the csu_hprot1 output for the corresponding master."]
    HP_PXP_1,
}
impl HP_PXPW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            HP_PXPW::HP_PXP_0 => false,
            HP_PXPW::HP_PXP_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _HP_PXPW<'a> {
    w: &'a mut W,
}
impl<'a> _HP_PXPW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: HP_PXPW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The hprot1 input signal value is routed to the csu_hprot1 output for the corresponding master."]
    #[inline]
    pub fn hp_pxp_0(self) -> &'a mut W {
        self.variant(HP_PXPW::HP_PXP_0)
    }
    #[doc = "The HP register bit is routed to the csu_hprot1 output for the corresponding master."]
    #[inline]
    pub fn hp_pxp_1(self) -> &'a mut W {
        self.variant(HP_PXPW::HP_PXP_1)
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
#[doc = "Values that can be written to the field `HP_DCP`"]
pub enum HP_DCPW {
    #[doc = "The hprot1 input signal value is routed to the csu_hprot1 output for the corresponding master."]
    HP_DCP_0,
    #[doc = "The HP register bit is routed to the csu_hprot1 output for the corresponding master."]
    HP_DCP_1,
}
impl HP_DCPW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            HP_DCPW::HP_DCP_0 => false,
            HP_DCPW::HP_DCP_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _HP_DCPW<'a> {
    w: &'a mut W,
}
impl<'a> _HP_DCPW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: HP_DCPW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The hprot1 input signal value is routed to the csu_hprot1 output for the corresponding master."]
    #[inline]
    pub fn hp_dcp_0(self) -> &'a mut W {
        self.variant(HP_DCPW::HP_DCP_0)
    }
    #[doc = "The HP register bit is routed to the csu_hprot1 output for the corresponding master."]
    #[inline]
    pub fn hp_dcp_1(self) -> &'a mut W {
        self.variant(HP_DCPW::HP_DCP_1)
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
    #[doc = "Lock-the adjacent (next lower) bit cannot be written by the software."]
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
    #[doc = "Lock-the adjacent (next lower) bit cannot be written by the software."]
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
#[doc = "Values that can be written to the field `HP_ENET`"]
pub enum HP_ENETW {
    #[doc = "The hprot1 input signal value is routed to the csu_hprot1 output for the corresponding master."]
    HP_ENET_0,
    #[doc = "The HP register bit is routed to the csu_hprot1 output for the corresponding master."]
    HP_ENET_1,
}
impl HP_ENETW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            HP_ENETW::HP_ENET_0 => false,
            HP_ENETW::HP_ENET_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _HP_ENETW<'a> {
    w: &'a mut W,
}
impl<'a> _HP_ENETW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: HP_ENETW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The hprot1 input signal value is routed to the csu_hprot1 output for the corresponding master."]
    #[inline]
    pub fn hp_enet_0(self) -> &'a mut W {
        self.variant(HP_ENETW::HP_ENET_0)
    }
    #[doc = "The HP register bit is routed to the csu_hprot1 output for the corresponding master."]
    #[inline]
    pub fn hp_enet_1(self) -> &'a mut W {
        self.variant(HP_ENETW::HP_ENET_1)
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
#[doc = "Values that can be written to the field `HP_USDHC1`"]
pub enum HP_USDHC1W {
    #[doc = "The hprot1 input signal value is routed to the csu_hprot1 output for the corresponding master."]
    HP_USDHC1_0,
    #[doc = "The HP register bit is routed to the csu_hprot1 output for the corresponding master."]
    HP_USDHC1_1,
}
impl HP_USDHC1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            HP_USDHC1W::HP_USDHC1_0 => false,
            HP_USDHC1W::HP_USDHC1_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _HP_USDHC1W<'a> {
    w: &'a mut W,
}
impl<'a> _HP_USDHC1W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: HP_USDHC1W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The hprot1 input signal value is routed to the csu_hprot1 output for the corresponding master."]
    #[inline]
    pub fn hp_usdhc1_0(self) -> &'a mut W {
        self.variant(HP_USDHC1W::HP_USDHC1_0)
    }
    #[doc = "The HP register bit is routed to the csu_hprot1 output for the corresponding master."]
    #[inline]
    pub fn hp_usdhc1_1(self) -> &'a mut W {
        self.variant(HP_USDHC1W::HP_USDHC1_1)
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
#[doc = "Values that can be written to the field `HP_USDHC2`"]
pub enum HP_USDHC2W {
    #[doc = "The hprot1 input signal value is routed to the csu_hprot1 output for the corresponding master."]
    HP_USDHC2_0,
    #[doc = "The HP register bit is routed to the csu_hprot1 output for the corresponding master."]
    HP_USDHC2_1,
}
impl HP_USDHC2W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            HP_USDHC2W::HP_USDHC2_0 => false,
            HP_USDHC2W::HP_USDHC2_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _HP_USDHC2W<'a> {
    w: &'a mut W,
}
impl<'a> _HP_USDHC2W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: HP_USDHC2W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The hprot1 input signal value is routed to the csu_hprot1 output for the corresponding master."]
    #[inline]
    pub fn hp_usdhc2_0(self) -> &'a mut W {
        self.variant(HP_USDHC2W::HP_USDHC2_0)
    }
    #[doc = "The HP register bit is routed to the csu_hprot1 output for the corresponding master."]
    #[inline]
    pub fn hp_usdhc2_1(self) -> &'a mut W {
        self.variant(HP_USDHC2W::HP_USDHC2_1)
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
#[doc = "Values that can be written to the field `HP_TPSMP`"]
pub enum HP_TPSMPW {
    #[doc = "The hprot1 input signal value is routed to the csu_hprot1 output for the corresponding master."]
    HP_TPSMP_0,
    #[doc = "The HP register bit is routed to the csu_hprot1 output for the corresponding master."]
    HP_TPSMP_1,
}
impl HP_TPSMPW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            HP_TPSMPW::HP_TPSMP_0 => false,
            HP_TPSMPW::HP_TPSMP_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _HP_TPSMPW<'a> {
    w: &'a mut W,
}
impl<'a> _HP_TPSMPW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: HP_TPSMPW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The hprot1 input signal value is routed to the csu_hprot1 output for the corresponding master."]
    #[inline]
    pub fn hp_tpsmp_0(self) -> &'a mut W {
        self.variant(HP_TPSMPW::HP_TPSMP_0)
    }
    #[doc = "The HP register bit is routed to the csu_hprot1 output for the corresponding master."]
    #[inline]
    pub fn hp_tpsmp_1(self) -> &'a mut W {
        self.variant(HP_TPSMPW::HP_TPSMP_1)
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
#[doc = "Values that can be written to the field `HP_USB`"]
pub enum HP_USBW {
    #[doc = "The hprot1 input signal value is routed to the csu_hprot1 output for the corresponding master."]
    HP_USB_0,
    #[doc = "The HP register bit is routed to the csu_hprot1 output for the corresponding master."]
    HP_USB_1,
}
impl HP_USBW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            HP_USBW::HP_USB_0 => false,
            HP_USBW::HP_USB_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _HP_USBW<'a> {
    w: &'a mut W,
}
impl<'a> _HP_USBW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: HP_USBW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The hprot1 input signal value is routed to the csu_hprot1 output for the corresponding master."]
    #[inline]
    pub fn hp_usb_0(self) -> &'a mut W {
        self.variant(HP_USBW::HP_USB_0)
    }
    #[doc = "The HP register bit is routed to the csu_hprot1 output for the corresponding master."]
    #[inline]
    pub fn hp_usb_1(self) -> &'a mut W {
        self.variant(HP_USBW::HP_USB_1)
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
    #[doc = "Bit 2 - Determines whether the register value of the corresponding HP field is passed as the hprot[1] of the eDMA"]
    #[inline]
    pub fn hp_dma(&self) -> HP_DMAR {
        HP_DMAR::_from({
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
    #[doc = "Bit 4 - Determines whether the register value of the corresponding HP field is passed as the hprot[1] of the LCDIF"]
    #[inline]
    pub fn hp_lcdif(&self) -> HP_LCDIFR {
        HP_LCDIFR::_from({
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
    #[doc = "Bit 6 - Determines whether the register value of the corresponding HP field is passed as the hprot[1] of the CSI"]
    #[inline]
    pub fn hp_csi(&self) -> HP_CSIR {
        HP_CSIR::_from({
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
    #[doc = "Bit 8 - Determines whether the register value of the corresponding HP field is passed as the hprot[1] of the PXP"]
    #[inline]
    pub fn hp_pxp(&self) -> HP_PXPR {
        HP_PXPR::_from({
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
    #[doc = "Bit 10 - Determines whether the register value of the corresponding HP field is passed as the hprot[1] of the DCP"]
    #[inline]
    pub fn hp_dcp(&self) -> HP_DCPR {
        HP_DCPR::_from({
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
    #[doc = "Bit 14 - Determines whether the register value of the corresponding HP field is passed as the hprot[1] of the ENET"]
    #[inline]
    pub fn hp_enet(&self) -> HP_ENETR {
        HP_ENETR::_from({
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
    #[doc = "Bit 16 - Determines whether the register value of the corresponding HP field is passed as the hprot[1] of the USDHC1"]
    #[inline]
    pub fn hp_usdhc1(&self) -> HP_USDHC1R {
        HP_USDHC1R::_from({
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
    #[doc = "Bit 18 - Determines whether the register value of the corresponding HP field is passed as the hprot[1] of the USDHC2"]
    #[inline]
    pub fn hp_usdhc2(&self) -> HP_USDHC2R {
        HP_USDHC2R::_from({
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
    #[doc = "Bit 20 - Determines whether the register value of the corresponding HP field is passed as the hprot[1] of the TPSMP"]
    #[inline]
    pub fn hp_tpsmp(&self) -> HP_TPSMPR {
        HP_TPSMPR::_from({
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
    #[doc = "Bit 22 - Determines whether the register value of the corresponding HP field is passed as the hprot[1] of the USB"]
    #[inline]
    pub fn hp_usb(&self) -> HP_USBR {
        HP_USBR::_from({
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
    #[doc = "Bit 2 - Determines whether the register value of the corresponding HP field is passed as the hprot[1] of the eDMA"]
    #[inline]
    pub fn hp_dma(&mut self) -> _HP_DMAW {
        _HP_DMAW { w: self }
    }
    #[doc = "Bit 3 - Lock bit set by the TZ software for the eDMA"]
    #[inline]
    pub fn l_dma(&mut self) -> _L_DMAW {
        _L_DMAW { w: self }
    }
    #[doc = "Bit 4 - Determines whether the register value of the corresponding HP field is passed as the hprot[1] of the LCDIF"]
    #[inline]
    pub fn hp_lcdif(&mut self) -> _HP_LCDIFW {
        _HP_LCDIFW { w: self }
    }
    #[doc = "Bit 5 - Lock bit set by the TZ software for the LCDIF"]
    #[inline]
    pub fn l_lcdif(&mut self) -> _L_LCDIFW {
        _L_LCDIFW { w: self }
    }
    #[doc = "Bit 6 - Determines whether the register value of the corresponding HP field is passed as the hprot[1] of the CSI"]
    #[inline]
    pub fn hp_csi(&mut self) -> _HP_CSIW {
        _HP_CSIW { w: self }
    }
    #[doc = "Bit 7 - Lock bit set by the TZ software for the CSI"]
    #[inline]
    pub fn l_csi(&mut self) -> _L_CSIW {
        _L_CSIW { w: self }
    }
    #[doc = "Bit 8 - Determines whether the register value of the corresponding HP field is passed as the hprot[1] of the PXP"]
    #[inline]
    pub fn hp_pxp(&mut self) -> _HP_PXPW {
        _HP_PXPW { w: self }
    }
    #[doc = "Bit 9 - Lock bit set by the TZ software for the PXP"]
    #[inline]
    pub fn l_pxp(&mut self) -> _L_PXPW {
        _L_PXPW { w: self }
    }
    #[doc = "Bit 10 - Determines whether the register value of the corresponding HP field is passed as the hprot[1] of the DCP"]
    #[inline]
    pub fn hp_dcp(&mut self) -> _HP_DCPW {
        _HP_DCPW { w: self }
    }
    #[doc = "Bit 11 - Lock bit set by the TZ software for the DCP"]
    #[inline]
    pub fn l_dcp(&mut self) -> _L_DCPW {
        _L_DCPW { w: self }
    }
    #[doc = "Bit 14 - Determines whether the register value of the corresponding HP field is passed as the hprot[1] of the ENET"]
    #[inline]
    pub fn hp_enet(&mut self) -> _HP_ENETW {
        _HP_ENETW { w: self }
    }
    #[doc = "Bit 15 - Lock bit set by the TZ software for the ENET"]
    #[inline]
    pub fn l_enet(&mut self) -> _L_ENETW {
        _L_ENETW { w: self }
    }
    #[doc = "Bit 16 - Determines whether the register value of the corresponding HP field is passed as the hprot[1] of the USDHC1"]
    #[inline]
    pub fn hp_usdhc1(&mut self) -> _HP_USDHC1W {
        _HP_USDHC1W { w: self }
    }
    #[doc = "Bit 17 - Lock bit set by the TZ software for the USDHC1"]
    #[inline]
    pub fn l_usdhc1(&mut self) -> _L_USDHC1W {
        _L_USDHC1W { w: self }
    }
    #[doc = "Bit 18 - Determines whether the register value of the corresponding HP field is passed as the hprot[1] of the USDHC2"]
    #[inline]
    pub fn hp_usdhc2(&mut self) -> _HP_USDHC2W {
        _HP_USDHC2W { w: self }
    }
    #[doc = "Bit 19 - Lock bit set by the TZ software for the USDHC2"]
    #[inline]
    pub fn l_usdhc2(&mut self) -> _L_USDHC2W {
        _L_USDHC2W { w: self }
    }
    #[doc = "Bit 20 - Determines whether the register value of the corresponding HP field is passed as the hprot[1] of the TPSMP"]
    #[inline]
    pub fn hp_tpsmp(&mut self) -> _HP_TPSMPW {
        _HP_TPSMPW { w: self }
    }
    #[doc = "Bit 21 - Lock bit set by the TZ software for the TPSMP"]
    #[inline]
    pub fn l_tpsmp(&mut self) -> _L_TPSMPW {
        _L_TPSMPW { w: self }
    }
    #[doc = "Bit 22 - Determines whether the register value of the corresponding HP field is passed as the hprot[1] of the USB"]
    #[inline]
    pub fn hp_usb(&mut self) -> _HP_USBW {
        _HP_USBW { w: self }
    }
    #[doc = "Bit 23 - Lock bit set by the TZ software for the USB"]
    #[inline]
    pub fn l_usb(&mut self) -> _L_USBW {
        _L_USBW { w: self }
    }
}
