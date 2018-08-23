#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Timer Channel Compare Register 1"]
    pub comp10: COMP1,
    #[doc = "0x02 - Timer Channel Compare Register 2"]
    pub comp20: COMP2,
    #[doc = "0x04 - Timer Channel Capture Register"]
    pub capt0: CAPT,
    #[doc = "0x06 - Timer Channel Load Register"]
    pub load0: LOAD,
    #[doc = "0x08 - Timer Channel Hold Register"]
    pub hold0: HOLD,
    #[doc = "0x0a - Timer Channel Counter Register"]
    pub cntr0: CNTR,
    #[doc = "0x0c - Timer Channel Control Register"]
    pub ctrl0: CTRL,
    #[doc = "0x0e - Timer Channel Status and Control Register"]
    pub sctrl0: SCTRL,
    #[doc = "0x10 - Timer Channel Comparator Load Register 1"]
    pub cmpld10: CMPLD1,
    #[doc = "0x12 - Timer Channel Comparator Load Register 2"]
    pub cmpld20: CMPLD2,
    #[doc = "0x14 - Timer Channel Comparator Status and Control Register"]
    pub csctrl0: CSCTRL,
    #[doc = "0x16 - Timer Channel Input Filter Register"]
    pub filt0: FILT,
    #[doc = "0x18 - Timer Channel DMA Enable Register"]
    pub dma0: DMA,
    _reserved0: [u8; 4usize],
    #[doc = "0x1e - Timer Channel Enable Register"]
    pub enbl: ENBL,
    #[doc = "0x20 - Timer Channel Compare Register 1"]
    pub comp11: COMP1,
    #[doc = "0x22 - Timer Channel Compare Register 2"]
    pub comp21: COMP2,
    #[doc = "0x24 - Timer Channel Capture Register"]
    pub capt1: CAPT,
    #[doc = "0x26 - Timer Channel Load Register"]
    pub load1: LOAD,
    #[doc = "0x28 - Timer Channel Hold Register"]
    pub hold1: HOLD,
    #[doc = "0x2a - Timer Channel Counter Register"]
    pub cntr1: CNTR,
    #[doc = "0x2c - Timer Channel Control Register"]
    pub ctrl1: CTRL,
    #[doc = "0x2e - Timer Channel Status and Control Register"]
    pub sctrl1: SCTRL,
    #[doc = "0x30 - Timer Channel Comparator Load Register 1"]
    pub cmpld11: CMPLD1,
    #[doc = "0x32 - Timer Channel Comparator Load Register 2"]
    pub cmpld21: CMPLD2,
    #[doc = "0x34 - Timer Channel Comparator Status and Control Register"]
    pub csctrl1: CSCTRL,
    #[doc = "0x36 - Timer Channel Input Filter Register"]
    pub filt1: FILT,
    #[doc = "0x38 - Timer Channel DMA Enable Register"]
    pub dma1: DMA,
    _reserved1: [u8; 6usize],
    #[doc = "0x40 - Timer Channel Compare Register 1"]
    pub comp12: COMP1,
    #[doc = "0x42 - Timer Channel Compare Register 2"]
    pub comp22: COMP2,
    #[doc = "0x44 - Timer Channel Capture Register"]
    pub capt2: CAPT,
    #[doc = "0x46 - Timer Channel Load Register"]
    pub load2: LOAD,
    #[doc = "0x48 - Timer Channel Hold Register"]
    pub hold2: HOLD,
    #[doc = "0x4a - Timer Channel Counter Register"]
    pub cntr2: CNTR,
    #[doc = "0x4c - Timer Channel Control Register"]
    pub ctrl2: CTRL,
    #[doc = "0x4e - Timer Channel Status and Control Register"]
    pub sctrl2: SCTRL,
    #[doc = "0x50 - Timer Channel Comparator Load Register 1"]
    pub cmpld12: CMPLD1,
    #[doc = "0x52 - Timer Channel Comparator Load Register 2"]
    pub cmpld22: CMPLD2,
    #[doc = "0x54 - Timer Channel Comparator Status and Control Register"]
    pub csctrl2: CSCTRL,
    #[doc = "0x56 - Timer Channel Input Filter Register"]
    pub filt2: FILT,
    #[doc = "0x58 - Timer Channel DMA Enable Register"]
    pub dma2: DMA,
    _reserved2: [u8; 6usize],
    #[doc = "0x60 - Timer Channel Compare Register 1"]
    pub comp13: COMP1,
    #[doc = "0x62 - Timer Channel Compare Register 2"]
    pub comp23: COMP2,
    #[doc = "0x64 - Timer Channel Capture Register"]
    pub capt3: CAPT,
    #[doc = "0x66 - Timer Channel Load Register"]
    pub load3: LOAD,
    #[doc = "0x68 - Timer Channel Hold Register"]
    pub hold3: HOLD,
    #[doc = "0x6a - Timer Channel Counter Register"]
    pub cntr3: CNTR,
    #[doc = "0x6c - Timer Channel Control Register"]
    pub ctrl3: CTRL,
    #[doc = "0x6e - Timer Channel Status and Control Register"]
    pub sctrl3: SCTRL,
    #[doc = "0x70 - Timer Channel Comparator Load Register 1"]
    pub cmpld13: CMPLD1,
    #[doc = "0x72 - Timer Channel Comparator Load Register 2"]
    pub cmpld23: CMPLD2,
    #[doc = "0x74 - Timer Channel Comparator Status and Control Register"]
    pub csctrl3: CSCTRL,
    #[doc = "0x76 - Timer Channel Input Filter Register"]
    pub filt3: FILT,
    #[doc = "0x78 - Timer Channel DMA Enable Register"]
    pub dma3: DMA,
}
#[doc = "Timer Channel Compare Register 1"]
pub struct COMP1 {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Timer Channel Compare Register 1"]
pub mod comp1;
#[doc = "Timer Channel Compare Register 2"]
pub struct COMP2 {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Timer Channel Compare Register 2"]
pub mod comp2;
#[doc = "Timer Channel Capture Register"]
pub struct CAPT {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Timer Channel Capture Register"]
pub mod capt;
#[doc = "Timer Channel Load Register"]
pub struct LOAD {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Timer Channel Load Register"]
pub mod load;
#[doc = "Timer Channel Hold Register"]
pub struct HOLD {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Timer Channel Hold Register"]
pub mod hold;
#[doc = "Timer Channel Counter Register"]
pub struct CNTR {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Timer Channel Counter Register"]
pub mod cntr;
#[doc = "Timer Channel Control Register"]
pub struct CTRL {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Timer Channel Control Register"]
pub mod ctrl;
#[doc = "Timer Channel Status and Control Register"]
pub struct SCTRL {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Timer Channel Status and Control Register"]
pub mod sctrl;
#[doc = "Timer Channel Comparator Load Register 1"]
pub struct CMPLD1 {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Timer Channel Comparator Load Register 1"]
pub mod cmpld1;
#[doc = "Timer Channel Comparator Load Register 2"]
pub struct CMPLD2 {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Timer Channel Comparator Load Register 2"]
pub mod cmpld2;
#[doc = "Timer Channel Comparator Status and Control Register"]
pub struct CSCTRL {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Timer Channel Comparator Status and Control Register"]
pub mod csctrl;
#[doc = "Timer Channel Input Filter Register"]
pub struct FILT {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Timer Channel Input Filter Register"]
pub mod filt;
#[doc = "Timer Channel DMA Enable Register"]
pub struct DMA {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Timer Channel DMA Enable Register"]
pub mod dma;
#[doc = "Timer Channel Enable Register"]
pub struct ENBL {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Timer Channel Enable Register"]
pub mod enbl;
