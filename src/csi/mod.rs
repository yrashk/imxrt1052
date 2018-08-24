#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - CSI Control Register 1"]
    pub csicr1: CSICR1,
    #[doc = "0x04 - CSI Control Register 2"]
    pub csicr2: CSICR2,
    #[doc = "0x08 - CSI Control Register 3"]
    pub csicr3: CSICR3,
    #[doc = "0x0c - CSI Statistic FIFO Register"]
    pub csistatfifo: CSISTATFIFO,
    #[doc = "0x10 - CSI RX FIFO Register"]
    pub csirfifo: CSIRFIFO,
    #[doc = "0x14 - CSI RX Count Register"]
    pub csirxcnt: CSIRXCNT,
    #[doc = "0x18 - CSI Status Register"]
    pub csisr: CSISR,
    _reserved0: [u8; 4usize],
    #[doc = "0x20 - CSI DMA Start Address Register - for STATFIFO"]
    pub csidmasa_statfifo: CSIDMASA_STATFIFO,
    #[doc = "0x24 - CSI DMA Transfer Size Register - for STATFIFO"]
    pub csidmats_statfifo: CSIDMATS_STATFIFO,
    #[doc = "0x28 - CSI DMA Start Address Register - for Frame Buffer1"]
    pub csidmasa_fb1: CSIDMASA_FB1,
    #[doc = "0x2c - CSI DMA Transfer Size Register - for Frame Buffer2"]
    pub csidmasa_fb2: CSIDMASA_FB2,
    #[doc = "0x30 - CSI Frame Buffer Parameter Register"]
    pub csifbuf_para: CSIFBUF_PARA,
    #[doc = "0x34 - CSI Image Parameter Register"]
    pub csiimag_para: CSIIMAG_PARA,
    _reserved1: [u8; 16usize],
    #[doc = "0x48 - CSI Control Register 18"]
    pub csicr18: CSICR18,
    #[doc = "0x4c - CSI Control Register 19"]
    pub csicr19: CSICR19,
}
#[doc = "CSI Control Register 1"]
pub struct CSICR1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CSI Control Register 1"]
pub mod csicr1;
#[doc = "CSI Control Register 2"]
pub struct CSICR2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CSI Control Register 2"]
pub mod csicr2;
#[doc = "CSI Control Register 3"]
pub struct CSICR3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CSI Control Register 3"]
pub mod csicr3;
#[doc = "CSI Statistic FIFO Register"]
pub struct CSISTATFIFO {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CSI Statistic FIFO Register"]
pub mod csistatfifo;
#[doc = "CSI RX FIFO Register"]
pub struct CSIRFIFO {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CSI RX FIFO Register"]
pub mod csirfifo;
#[doc = "CSI RX Count Register"]
pub struct CSIRXCNT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CSI RX Count Register"]
pub mod csirxcnt;
#[doc = "CSI Status Register"]
pub struct CSISR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CSI Status Register"]
pub mod csisr;
#[doc = "CSI DMA Start Address Register - for STATFIFO"]
pub struct CSIDMASA_STATFIFO {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CSI DMA Start Address Register - for STATFIFO"]
pub mod csidmasa_statfifo;
#[doc = "CSI DMA Transfer Size Register - for STATFIFO"]
pub struct CSIDMATS_STATFIFO {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CSI DMA Transfer Size Register - for STATFIFO"]
pub mod csidmats_statfifo;
#[doc = "CSI DMA Start Address Register - for Frame Buffer1"]
pub struct CSIDMASA_FB1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CSI DMA Start Address Register - for Frame Buffer1"]
pub mod csidmasa_fb1;
#[doc = "CSI DMA Transfer Size Register - for Frame Buffer2"]
pub struct CSIDMASA_FB2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CSI DMA Transfer Size Register - for Frame Buffer2"]
pub mod csidmasa_fb2;
#[doc = "CSI Frame Buffer Parameter Register"]
pub struct CSIFBUF_PARA {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CSI Frame Buffer Parameter Register"]
pub mod csifbuf_para;
#[doc = "CSI Image Parameter Register"]
pub struct CSIIMAG_PARA {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CSI Image Parameter Register"]
pub mod csiimag_para;
#[doc = "CSI Control Register 18"]
pub struct CSICR18 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CSI Control Register 18"]
pub mod csicr18;
#[doc = "CSI Control Register 19"]
pub struct CSICR19 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CSI Control Register 19"]
pub mod csicr19;
