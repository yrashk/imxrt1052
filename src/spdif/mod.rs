#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - SPDIF Configuration Register"]
    pub scr: SCR,
    #[doc = "0x04 - CDText Control Register"]
    pub srcd: SRCD,
    #[doc = "0x08 - PhaseConfig Register"]
    pub srpc: SRPC,
    #[doc = "0x0c - InterruptEn Register"]
    pub sie: SIE,
    #[doc = "0x10 - InterruptClear Register"]
    pub sic: SIC,
    #[doc = "0x14 - SPDIFRxLeft Register"]
    pub srl: SRL,
    #[doc = "0x18 - SPDIFRxRight Register"]
    pub srr: SRR,
    #[doc = "0x1c - SPDIFRxCChannel_h Register"]
    pub srcsh: SRCSH,
    #[doc = "0x20 - SPDIFRxCChannel_l Register"]
    pub srcsl: SRCSL,
    #[doc = "0x24 - UchannelRx Register"]
    pub sru: SRU,
    #[doc = "0x28 - QchannelRx Register"]
    pub srq: SRQ,
    #[doc = "0x2c - SPDIFTxLeft Register"]
    pub stl: STL,
    #[doc = "0x30 - SPDIFTxRight Register"]
    pub str: STR,
    #[doc = "0x34 - SPDIFTxCChannelCons_h Register"]
    pub stcsch: STCSCH,
    #[doc = "0x38 - SPDIFTxCChannelCons_l Register"]
    pub stcscl: STCSCL,
    _reserved0: [u8; 8usize],
    #[doc = "0x44 - FreqMeas Register"]
    pub srfm: SRFM,
    _reserved1: [u8; 8usize],
    #[doc = "0x50 - SPDIFTxClk Register"]
    pub stc: STC,
}
#[doc = "SPDIF Configuration Register"]
pub struct SCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SPDIF Configuration Register"]
pub mod scr;
#[doc = "CDText Control Register"]
pub struct SRCD {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CDText Control Register"]
pub mod srcd;
#[doc = "PhaseConfig Register"]
pub struct SRPC {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PhaseConfig Register"]
pub mod srpc;
#[doc = "InterruptEn Register"]
pub struct SIE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "InterruptEn Register"]
pub mod sie;
#[doc = "InterruptClear Register"]
pub struct SIC {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "InterruptClear Register"]
pub mod sic;
#[doc = "InterruptStat Register"]
pub struct SIS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "InterruptStat Register"]
pub mod sis;
#[doc = "SPDIFRxLeft Register"]
pub struct SRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SPDIFRxLeft Register"]
pub mod srl;
#[doc = "SPDIFRxRight Register"]
pub struct SRR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SPDIFRxRight Register"]
pub mod srr;
#[doc = "SPDIFRxCChannel_h Register"]
pub struct SRCSH {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SPDIFRxCChannel_h Register"]
pub mod srcsh;
#[doc = "SPDIFRxCChannel_l Register"]
pub struct SRCSL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SPDIFRxCChannel_l Register"]
pub mod srcsl;
#[doc = "UchannelRx Register"]
pub struct SRU {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "UchannelRx Register"]
pub mod sru;
#[doc = "QchannelRx Register"]
pub struct SRQ {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "QchannelRx Register"]
pub mod srq;
#[doc = "SPDIFTxLeft Register"]
pub struct STL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SPDIFTxLeft Register"]
pub mod stl;
#[doc = "SPDIFTxRight Register"]
pub struct STR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SPDIFTxRight Register"]
pub mod str;
#[doc = "SPDIFTxCChannelCons_h Register"]
pub struct STCSCH {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SPDIFTxCChannelCons_h Register"]
pub mod stcsch;
#[doc = "SPDIFTxCChannelCons_l Register"]
pub struct STCSCL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SPDIFTxCChannelCons_l Register"]
pub mod stcscl;
#[doc = "FreqMeas Register"]
pub struct SRFM {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "FreqMeas Register"]
pub mod srfm;
#[doc = "SPDIFTxClk Register"]
pub struct STC {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SPDIFTxClk Register"]
pub mod stc;
