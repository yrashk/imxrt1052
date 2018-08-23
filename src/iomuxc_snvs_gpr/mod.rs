#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - GPR0 General Purpose Register"]
    pub gpr0: GPR0,
    #[doc = "0x04 - GPR1 General Purpose Register"]
    pub gpr1: GPR1,
    #[doc = "0x08 - GPR2 General Purpose Register"]
    pub gpr2: GPR2,
    #[doc = "0x0c - GPR3 General Purpose Register"]
    pub gpr3: GPR3,
}
#[doc = "GPR0 General Purpose Register"]
pub struct GPR0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "GPR0 General Purpose Register"]
pub mod gpr0;
#[doc = "GPR1 General Purpose Register"]
pub struct GPR1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "GPR1 General Purpose Register"]
pub mod gpr1;
#[doc = "GPR2 General Purpose Register"]
pub struct GPR2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "GPR2 General Purpose Register"]
pub mod gpr2;
#[doc = "GPR3 General Purpose Register"]
pub struct GPR3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "GPR3 General Purpose Register"]
pub mod gpr3;
