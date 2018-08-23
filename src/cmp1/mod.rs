#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - CMP Control Register 0"]
    pub cr0: CR0,
    #[doc = "0x01 - CMP Control Register 1"]
    pub cr1: CR1,
    #[doc = "0x02 - CMP Filter Period Register"]
    pub fpr: FPR,
    #[doc = "0x03 - CMP Status and Control Register"]
    pub scr: SCR,
    #[doc = "0x04 - DAC Control Register"]
    pub daccr: DACCR,
    #[doc = "0x05 - MUX Control Register"]
    pub muxcr: MUXCR,
}
#[doc = "CMP Control Register 0"]
pub struct CR0 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "CMP Control Register 0"]
pub mod cr0;
#[doc = "CMP Control Register 1"]
pub struct CR1 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "CMP Control Register 1"]
pub mod cr1;
#[doc = "CMP Filter Period Register"]
pub struct FPR {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "CMP Filter Period Register"]
pub mod fpr;
#[doc = "CMP Status and Control Register"]
pub struct SCR {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "CMP Status and Control Register"]
pub mod scr;
#[doc = "DAC Control Register"]
pub struct DACCR {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "DAC Control Register"]
pub mod daccr;
#[doc = "MUX Control Register"]
pub struct MUXCR {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "MUX Control Register"]
pub mod muxcr;
