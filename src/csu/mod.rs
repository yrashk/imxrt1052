#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Config security level register"]
    pub csl: [CSL; 32],
    _reserved0: [u8; 384usize],
    #[doc = "0x200 - HP0 register"]
    pub hp0: HP0,
    _reserved1: [u8; 20usize],
    #[doc = "0x218 - Secure access register"]
    pub sa: SA,
    _reserved2: [u8; 316usize],
    #[doc = "0x358 - HPCONTROL0 register"]
    pub hpcontrol0: HPCONTROL0,
}
#[doc = "Config security level register"]
pub struct CSL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Config security level register"]
pub mod csl;
#[doc = "HP0 register"]
pub struct HP0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "HP0 register"]
pub mod hp0;
#[doc = "Secure access register"]
pub struct SA {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Secure access register"]
pub mod sa;
#[doc = "HPCONTROL0 register"]
pub struct HPCONTROL0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "HPCONTROL0 register"]
pub mod hpcontrol0;
