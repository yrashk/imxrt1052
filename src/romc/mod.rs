#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 212usize],
    #[doc = "0xd4 - ROMC Data Registers"]
    pub rompatch7d: ROMPATCHD,
    #[doc = "0xd8 - ROMC Data Registers"]
    pub rompatch6d: ROMPATCHD,
    #[doc = "0xdc - ROMC Data Registers"]
    pub rompatch5d: ROMPATCHD,
    #[doc = "0xe0 - ROMC Data Registers"]
    pub rompatch4d: ROMPATCHD,
    #[doc = "0xe4 - ROMC Data Registers"]
    pub rompatch3d: ROMPATCHD,
    #[doc = "0xe8 - ROMC Data Registers"]
    pub rompatch2d: ROMPATCHD,
    #[doc = "0xec - ROMC Data Registers"]
    pub rompatch1d: ROMPATCHD,
    #[doc = "0xf0 - ROMC Data Registers"]
    pub rompatch0d: ROMPATCHD,
    #[doc = "0xf4 - ROMC Control Register"]
    pub rompatchcntl: ROMPATCHCNTL,
    #[doc = "0xf8 - ROMC Enable Register High"]
    pub rompatchenh: ROMPATCHENH,
    #[doc = "0xfc - ROMC Enable Register Low"]
    pub rompatchenl: ROMPATCHENL,
    #[doc = "0x100 - ROMC Address Registers"]
    pub rompatcha: [ROMPATCHA; 16],
    _reserved1: [u8; 200usize],
    #[doc = "0x208 - ROMC Status Register"]
    pub rompatchsr: ROMPATCHSR,
}
#[doc = "ROMC Data Registers"]
pub struct ROMPATCHD {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ROMC Data Registers"]
pub mod rompatchd;
#[doc = "ROMC Control Register"]
pub struct ROMPATCHCNTL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ROMC Control Register"]
pub mod rompatchcntl;
#[doc = "ROMC Enable Register High"]
pub struct ROMPATCHENH {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ROMC Enable Register High"]
pub mod rompatchenh;
#[doc = "ROMC Enable Register Low"]
pub struct ROMPATCHENL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ROMC Enable Register Low"]
pub mod rompatchenl;
#[doc = "ROMC Address Registers"]
pub struct ROMPATCHA {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ROMC Address Registers"]
pub mod rompatcha;
#[doc = "ROMC Status Register"]
pub struct ROMPATCHSR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ROMC Status Register"]
pub mod rompatchsr;
