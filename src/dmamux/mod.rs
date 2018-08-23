#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Channel 0 Configuration Register"]
    pub chcfg: [CHCFG; 32],
}
#[doc = "Channel 0 Configuration Register"]
pub struct CHCFG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Channel 0 Configuration Register"]
pub mod chcfg;
