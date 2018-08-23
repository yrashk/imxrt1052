#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - DCDC Register 0"]
    pub reg0: REG0,
    #[doc = "0x04 - DCDC Register 1"]
    pub reg1: REG1,
    #[doc = "0x08 - DCDC Register 2"]
    pub reg2: REG2,
    #[doc = "0x0c - DCDC Register 3"]
    pub reg3: REG3,
}
#[doc = "DCDC Register 0"]
pub struct REG0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DCDC Register 0"]
pub mod reg0;
#[doc = "DCDC Register 1"]
pub struct REG1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DCDC Register 1"]
pub mod reg1;
#[doc = "DCDC Register 2"]
pub struct REG2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DCDC Register 2"]
pub mod reg2;
#[doc = "DCDC Register 3"]
pub struct REG3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DCDC Register 3"]
pub mod reg3;
