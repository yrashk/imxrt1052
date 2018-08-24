#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Master Priviledge Registers"]
    pub mpr: MPR,
    _reserved0: [u8; 60usize],
    #[doc = "0x40 - Off-Platform Peripheral Access Control Registers"]
    pub opacr: OPACR,
    #[doc = "0x44 - Off-Platform Peripheral Access Control Registers"]
    pub opacr1: OPACR1,
    #[doc = "0x48 - Off-Platform Peripheral Access Control Registers"]
    pub opacr2: OPACR2,
    #[doc = "0x4c - Off-Platform Peripheral Access Control Registers"]
    pub opacr3: OPACR3,
    #[doc = "0x50 - Off-Platform Peripheral Access Control Registers"]
    pub opacr4: OPACR4,
}
#[doc = "Master Priviledge Registers"]
pub struct MPR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Master Priviledge Registers"]
pub mod mpr;
#[doc = "Off-Platform Peripheral Access Control Registers"]
pub struct OPACR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Off-Platform Peripheral Access Control Registers"]
pub mod opacr;
#[doc = "Off-Platform Peripheral Access Control Registers"]
pub struct OPACR1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Off-Platform Peripheral Access Control Registers"]
pub mod opacr1;
#[doc = "Off-Platform Peripheral Access Control Registers"]
pub struct OPACR2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Off-Platform Peripheral Access Control Registers"]
pub mod opacr2;
#[doc = "Off-Platform Peripheral Access Control Registers"]
pub struct OPACR3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Off-Platform Peripheral Access Control Registers"]
pub mod opacr3;
#[doc = "Off-Platform Peripheral Access Control Registers"]
pub struct OPACR4 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Off-Platform Peripheral Access Control Registers"]
pub mod opacr4;
