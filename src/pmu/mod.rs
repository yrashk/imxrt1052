#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 272usize],
    #[doc = "0x110 - Regulator 1P1 Register"]
    pub reg_1p1: REG_1P1,
    #[doc = "0x114 - Regulator 1P1 Register"]
    pub reg_1p1_set: REG_1P1_SET,
    #[doc = "0x118 - Regulator 1P1 Register"]
    pub reg_1p1_clr: REG_1P1_CLR,
    #[doc = "0x11c - Regulator 1P1 Register"]
    pub reg_1p1_tog: REG_1P1_TOG,
    #[doc = "0x120 - Regulator 3P0 Register"]
    pub reg_3p0: REG_3P0,
    #[doc = "0x124 - Regulator 3P0 Register"]
    pub reg_3p0_set: REG_3P0_SET,
    #[doc = "0x128 - Regulator 3P0 Register"]
    pub reg_3p0_clr: REG_3P0_CLR,
    #[doc = "0x12c - Regulator 3P0 Register"]
    pub reg_3p0_tog: REG_3P0_TOG,
    #[doc = "0x130 - Regulator 2P5 Register"]
    pub reg_2p5: REG_2P5,
    #[doc = "0x134 - Regulator 2P5 Register"]
    pub reg_2p5_set: REG_2P5_SET,
    #[doc = "0x138 - Regulator 2P5 Register"]
    pub reg_2p5_clr: REG_2P5_CLR,
    #[doc = "0x13c - Regulator 2P5 Register"]
    pub reg_2p5_tog: REG_2P5_TOG,
    #[doc = "0x140 - Digital Regulator Core Register"]
    pub reg_core: REG_CORE,
    #[doc = "0x144 - Digital Regulator Core Register"]
    pub reg_core_set: REG_CORE_SET,
    #[doc = "0x148 - Digital Regulator Core Register"]
    pub reg_core_clr: REG_CORE_CLR,
    #[doc = "0x14c - Digital Regulator Core Register"]
    pub reg_core_tog: REG_CORE_TOG,
    #[doc = "0x150 - Miscellaneous Register 0"]
    pub misc0: MISC0,
    #[doc = "0x154 - Miscellaneous Register 0"]
    pub misc0_set: MISC0_SET,
    #[doc = "0x158 - Miscellaneous Register 0"]
    pub misc0_clr: MISC0_CLR,
    #[doc = "0x15c - Miscellaneous Register 0"]
    pub misc0_tog: MISC0_TOG,
    #[doc = "0x160 - Miscellaneous Register 1"]
    pub misc1: MISC1,
    #[doc = "0x164 - Miscellaneous Register 1"]
    pub misc1_set: MISC1_SET,
    #[doc = "0x168 - Miscellaneous Register 1"]
    pub misc1_clr: MISC1_CLR,
    #[doc = "0x16c - Miscellaneous Register 1"]
    pub misc1_tog: MISC1_TOG,
    #[doc = "0x170 - Miscellaneous Control Register"]
    pub misc2: MISC2,
    #[doc = "0x174 - Miscellaneous Control Register"]
    pub misc2_set: MISC2_SET,
    #[doc = "0x178 - Miscellaneous Control Register"]
    pub misc2_clr: MISC2_CLR,
    #[doc = "0x17c - Miscellaneous Control Register"]
    pub misc2_tog: MISC2_TOG,
}
#[doc = "Regulator 1P1 Register"]
pub struct REG_1P1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Regulator 1P1 Register"]
pub mod reg_1p1;
#[doc = "Regulator 1P1 Register"]
pub struct REG_1P1_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Regulator 1P1 Register"]
pub mod reg_1p1_set;
#[doc = "Regulator 1P1 Register"]
pub struct REG_1P1_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Regulator 1P1 Register"]
pub mod reg_1p1_clr;
#[doc = "Regulator 1P1 Register"]
pub struct REG_1P1_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Regulator 1P1 Register"]
pub mod reg_1p1_tog;
#[doc = "Regulator 3P0 Register"]
pub struct REG_3P0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Regulator 3P0 Register"]
pub mod reg_3p0;
#[doc = "Regulator 3P0 Register"]
pub struct REG_3P0_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Regulator 3P0 Register"]
pub mod reg_3p0_set;
#[doc = "Regulator 3P0 Register"]
pub struct REG_3P0_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Regulator 3P0 Register"]
pub mod reg_3p0_clr;
#[doc = "Regulator 3P0 Register"]
pub struct REG_3P0_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Regulator 3P0 Register"]
pub mod reg_3p0_tog;
#[doc = "Regulator 2P5 Register"]
pub struct REG_2P5 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Regulator 2P5 Register"]
pub mod reg_2p5;
#[doc = "Regulator 2P5 Register"]
pub struct REG_2P5_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Regulator 2P5 Register"]
pub mod reg_2p5_set;
#[doc = "Regulator 2P5 Register"]
pub struct REG_2P5_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Regulator 2P5 Register"]
pub mod reg_2p5_clr;
#[doc = "Regulator 2P5 Register"]
pub struct REG_2P5_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Regulator 2P5 Register"]
pub mod reg_2p5_tog;
#[doc = "Digital Regulator Core Register"]
pub struct REG_CORE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Digital Regulator Core Register"]
pub mod reg_core;
#[doc = "Digital Regulator Core Register"]
pub struct REG_CORE_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Digital Regulator Core Register"]
pub mod reg_core_set;
#[doc = "Digital Regulator Core Register"]
pub struct REG_CORE_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Digital Regulator Core Register"]
pub mod reg_core_clr;
#[doc = "Digital Regulator Core Register"]
pub struct REG_CORE_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Digital Regulator Core Register"]
pub mod reg_core_tog;
#[doc = "Miscellaneous Register 0"]
pub struct MISC0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Miscellaneous Register 0"]
pub mod misc0;
#[doc = "Miscellaneous Register 0"]
pub struct MISC0_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Miscellaneous Register 0"]
pub mod misc0_set;
#[doc = "Miscellaneous Register 0"]
pub struct MISC0_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Miscellaneous Register 0"]
pub mod misc0_clr;
#[doc = "Miscellaneous Register 0"]
pub struct MISC0_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Miscellaneous Register 0"]
pub mod misc0_tog;
#[doc = "Miscellaneous Register 1"]
pub struct MISC1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Miscellaneous Register 1"]
pub mod misc1;
#[doc = "Miscellaneous Register 1"]
pub struct MISC1_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Miscellaneous Register 1"]
pub mod misc1_set;
#[doc = "Miscellaneous Register 1"]
pub struct MISC1_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Miscellaneous Register 1"]
pub mod misc1_clr;
#[doc = "Miscellaneous Register 1"]
pub struct MISC1_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Miscellaneous Register 1"]
pub mod misc1_tog;
#[doc = "Miscellaneous Control Register"]
pub struct MISC2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Miscellaneous Control Register"]
pub mod misc2;
#[doc = "Miscellaneous Control Register"]
pub struct MISC2_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Miscellaneous Control Register"]
pub mod misc2_set;
#[doc = "Miscellaneous Control Register"]
pub struct MISC2_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Miscellaneous Control Register"]
pub mod misc2_clr;
#[doc = "Miscellaneous Control Register"]
pub struct MISC2_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Miscellaneous Control Register"]
pub mod misc2_tog;
