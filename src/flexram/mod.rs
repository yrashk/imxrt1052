#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - TCM CRTL Register"]
    pub tcm_ctrl: TCM_CTRL,
    #[doc = "0x04 - OCRAM Magic Address Register"]
    pub ocram_magic_addr: OCRAM_MAGIC_ADDR,
    #[doc = "0x08 - DTCM Magic Address Register"]
    pub dtcm_magic_addr: DTCM_MAGIC_ADDR,
    #[doc = "0x0c - ITCM Magic Address Register"]
    pub itcm_magic_addr: ITCM_MAGIC_ADDR,
    #[doc = "0x10 - Interrupt Status Register"]
    pub int_status: INT_STATUS,
    #[doc = "0x14 - Interrupt Status Enable Register"]
    pub int_stat_en: INT_STAT_EN,
    #[doc = "0x18 - Interrupt Enable Register"]
    pub int_sig_en: INT_SIG_EN,
}
#[doc = "TCM CRTL Register"]
pub struct TCM_CTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "TCM CRTL Register"]
pub mod tcm_ctrl;
#[doc = "OCRAM Magic Address Register"]
pub struct OCRAM_MAGIC_ADDR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "OCRAM Magic Address Register"]
pub mod ocram_magic_addr;
#[doc = "DTCM Magic Address Register"]
pub struct DTCM_MAGIC_ADDR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DTCM Magic Address Register"]
pub mod dtcm_magic_addr;
#[doc = "ITCM Magic Address Register"]
pub struct ITCM_MAGIC_ADDR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ITCM Magic Address Register"]
pub mod itcm_magic_addr;
#[doc = "Interrupt Status Register"]
pub struct INT_STATUS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt Status Register"]
pub mod int_status;
#[doc = "Interrupt Status Enable Register"]
pub struct INT_STAT_EN {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt Status Enable Register"]
pub mod int_stat_en;
#[doc = "Interrupt Enable Register"]
pub struct INT_SIG_EN {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt Enable Register"]
pub mod int_sig_en;
