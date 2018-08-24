#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Miscellaneous Control Register"]
    pub mctl: MCTL,
    #[doc = "0x04 - Statistical Check Miscellaneous Register"]
    pub scmisc: SCMISC,
    #[doc = "0x08 - Poker Range Register"]
    pub pkrrng: PKRRNG,
    #[doc = "0x0c - Poker Maximum Limit Register"]
    pub pkrmax: PKRMAX,
    #[doc = "0x10 - Seed Control Register"]
    pub sdctl: SDCTL,
    #[doc = "0x14 - Sparse Bit Limit Register"]
    pub sblim: SBLIM,
    #[doc = "0x18 - Frequency Count Minimum Limit Register"]
    pub frqmin: FRQMIN,
    #[doc = "0x1c - Frequency Count Register"]
    pub frqcnt: FRQCNT,
    #[doc = "0x20 - Statistical Check Monobit Count Register"]
    pub scmc: SCMC,
    #[doc = "0x24 - Statistical Check Run Length 1 Count Register"]
    pub scr1c: SCR1C,
    #[doc = "0x28 - Statistical Check Run Length 2 Count Register"]
    pub scr2c: SCR2C,
    #[doc = "0x2c - Statistical Check Run Length 3 Count Register"]
    pub scr3c: SCR3C,
    #[doc = "0x30 - Statistical Check Run Length 4 Count Register"]
    pub scr4c: SCR4C,
    #[doc = "0x34 - Statistical Check Run Length 5 Count Register"]
    pub scr5c: SCR5C,
    #[doc = "0x38 - Statistical Check Run Length 6+ Count Register"]
    pub scr6pc: SCR6PC,
    #[doc = "0x3c - Status Register"]
    pub status: STATUS,
    #[doc = "0x40 - Entropy Read Register"]
    pub ent: [ENT; 16],
    #[doc = "0x80 - Statistical Check Poker Count 1 and 0 Register"]
    pub pkrcnt10: PKRCNT10,
    #[doc = "0x84 - Statistical Check Poker Count 3 and 2 Register"]
    pub pkrcnt32: PKRCNT32,
    #[doc = "0x88 - Statistical Check Poker Count 5 and 4 Register"]
    pub pkrcnt54: PKRCNT54,
    #[doc = "0x8c - Statistical Check Poker Count 7 and 6 Register"]
    pub pkrcnt76: PKRCNT76,
    #[doc = "0x90 - Statistical Check Poker Count 9 and 8 Register"]
    pub pkrcnt98: PKRCNT98,
    #[doc = "0x94 - Statistical Check Poker Count B and A Register"]
    pub pkrcntba: PKRCNTBA,
    #[doc = "0x98 - Statistical Check Poker Count D and C Register"]
    pub pkrcntdc: PKRCNTDC,
    #[doc = "0x9c - Statistical Check Poker Count F and E Register"]
    pub pkrcntfe: PKRCNTFE,
    #[doc = "0xa0 - Security Configuration Register"]
    pub sec_cfg: SEC_CFG,
    #[doc = "0xa4 - Interrupt Control Register"]
    pub int_ctrl: INT_CTRL,
    #[doc = "0xa8 - Mask Register"]
    pub int_mask: INT_MASK,
    #[doc = "0xac - Interrupt Status Register"]
    pub int_status: INT_STATUS,
    _reserved0: [u8; 64usize],
    #[doc = "0xf0 - Version ID Register (MS)"]
    pub vid1: VID1,
    #[doc = "0xf4 - Version ID Register (LS)"]
    pub vid2: VID2,
}
#[doc = "Miscellaneous Control Register"]
pub struct MCTL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Miscellaneous Control Register"]
pub mod mctl;
#[doc = "Statistical Check Miscellaneous Register"]
pub struct SCMISC {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Statistical Check Miscellaneous Register"]
pub mod scmisc;
#[doc = "Poker Range Register"]
pub struct PKRRNG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Poker Range Register"]
pub mod pkrrng;
#[doc = "Poker Maximum Limit Register"]
pub struct PKRMAX {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Poker Maximum Limit Register"]
pub mod pkrmax;
#[doc = "Poker Square Calculation Result Register"]
pub struct PKRSQ {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Poker Square Calculation Result Register"]
pub mod pkrsq;
#[doc = "Seed Control Register"]
pub struct SDCTL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Seed Control Register"]
pub mod sdctl;
#[doc = "Sparse Bit Limit Register"]
pub struct SBLIM {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Sparse Bit Limit Register"]
pub mod sblim;
#[doc = "Total Samples Register"]
pub struct TOTSAM {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Total Samples Register"]
pub mod totsam;
#[doc = "Frequency Count Minimum Limit Register"]
pub struct FRQMIN {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Frequency Count Minimum Limit Register"]
pub mod frqmin;
#[doc = "Frequency Count Register"]
pub struct FRQCNT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Frequency Count Register"]
pub mod frqcnt;
#[doc = "Frequency Count Maximum Limit Register"]
pub struct FRQMAX {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Frequency Count Maximum Limit Register"]
pub mod frqmax;
#[doc = "Statistical Check Monobit Count Register"]
pub struct SCMC {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Statistical Check Monobit Count Register"]
pub mod scmc;
#[doc = "Statistical Check Monobit Limit Register"]
pub struct SCML {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Statistical Check Monobit Limit Register"]
pub mod scml;
#[doc = "Statistical Check Run Length 1 Count Register"]
pub struct SCR1C {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Statistical Check Run Length 1 Count Register"]
pub mod scr1c;
#[doc = "Statistical Check Run Length 1 Limit Register"]
pub struct SCR1L {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Statistical Check Run Length 1 Limit Register"]
pub mod scr1l;
#[doc = "Statistical Check Run Length 2 Count Register"]
pub struct SCR2C {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Statistical Check Run Length 2 Count Register"]
pub mod scr2c;
#[doc = "Statistical Check Run Length 2 Limit Register"]
pub struct SCR2L {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Statistical Check Run Length 2 Limit Register"]
pub mod scr2l;
#[doc = "Statistical Check Run Length 3 Count Register"]
pub struct SCR3C {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Statistical Check Run Length 3 Count Register"]
pub mod scr3c;
#[doc = "Statistical Check Run Length 3 Limit Register"]
pub struct SCR3L {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Statistical Check Run Length 3 Limit Register"]
pub mod scr3l;
#[doc = "Statistical Check Run Length 4 Count Register"]
pub struct SCR4C {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Statistical Check Run Length 4 Count Register"]
pub mod scr4c;
#[doc = "Statistical Check Run Length 4 Limit Register"]
pub struct SCR4L {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Statistical Check Run Length 4 Limit Register"]
pub mod scr4l;
#[doc = "Statistical Check Run Length 5 Count Register"]
pub struct SCR5C {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Statistical Check Run Length 5 Count Register"]
pub mod scr5c;
#[doc = "Statistical Check Run Length 5 Limit Register"]
pub struct SCR5L {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Statistical Check Run Length 5 Limit Register"]
pub mod scr5l;
#[doc = "Statistical Check Run Length 6+ Count Register"]
pub struct SCR6PC {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Statistical Check Run Length 6+ Count Register"]
pub mod scr6pc;
#[doc = "Statistical Check Run Length 6+ Limit Register"]
pub struct SCR6PL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Statistical Check Run Length 6+ Limit Register"]
pub mod scr6pl;
#[doc = "Status Register"]
pub struct STATUS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Status Register"]
pub mod status;
#[doc = "Entropy Read Register"]
pub struct ENT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Entropy Read Register"]
pub mod ent;
#[doc = "Statistical Check Poker Count 1 and 0 Register"]
pub struct PKRCNT10 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Statistical Check Poker Count 1 and 0 Register"]
pub mod pkrcnt10;
#[doc = "Statistical Check Poker Count 3 and 2 Register"]
pub struct PKRCNT32 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Statistical Check Poker Count 3 and 2 Register"]
pub mod pkrcnt32;
#[doc = "Statistical Check Poker Count 5 and 4 Register"]
pub struct PKRCNT54 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Statistical Check Poker Count 5 and 4 Register"]
pub mod pkrcnt54;
#[doc = "Statistical Check Poker Count 7 and 6 Register"]
pub struct PKRCNT76 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Statistical Check Poker Count 7 and 6 Register"]
pub mod pkrcnt76;
#[doc = "Statistical Check Poker Count 9 and 8 Register"]
pub struct PKRCNT98 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Statistical Check Poker Count 9 and 8 Register"]
pub mod pkrcnt98;
#[doc = "Statistical Check Poker Count B and A Register"]
pub struct PKRCNTBA {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Statistical Check Poker Count B and A Register"]
pub mod pkrcntba;
#[doc = "Statistical Check Poker Count D and C Register"]
pub struct PKRCNTDC {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Statistical Check Poker Count D and C Register"]
pub mod pkrcntdc;
#[doc = "Statistical Check Poker Count F and E Register"]
pub struct PKRCNTFE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Statistical Check Poker Count F and E Register"]
pub mod pkrcntfe;
#[doc = "Security Configuration Register"]
pub struct SEC_CFG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Security Configuration Register"]
pub mod sec_cfg;
#[doc = "Interrupt Control Register"]
pub struct INT_CTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt Control Register"]
pub mod int_ctrl;
#[doc = "Mask Register"]
pub struct INT_MASK {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Mask Register"]
pub mod int_mask;
#[doc = "Interrupt Status Register"]
pub struct INT_STATUS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt Status Register"]
pub mod int_status;
#[doc = "Version ID Register (MS)"]
pub struct VID1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Version ID Register (MS)"]
pub mod vid1;
#[doc = "Version ID Register (LS)"]
pub struct VID2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Version ID Register (LS)"]
pub mod vid2;
