#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - SRC Control Register"]
    pub scr: SCR,
    #[doc = "0x04 - SRC Boot Mode Register 1"]
    pub sbmr1: SBMR1,
    #[doc = "0x08 - SRC Reset Status Register"]
    pub srsr: SRSR,
    _reserved0: [u8; 16usize],
    #[doc = "0x1c - SRC Boot Mode Register 2"]
    pub sbmr2: SBMR2,
    #[doc = "0x20 - SRC General Purpose Register 1"]
    pub gpr1: GPR1,
    #[doc = "0x24 - SRC General Purpose Register 2"]
    pub gpr2: GPR2,
    #[doc = "0x28 - SRC General Purpose Register 3"]
    pub gpr3: GPR3,
    #[doc = "0x2c - SRC General Purpose Register 4"]
    pub gpr4: GPR4,
    #[doc = "0x30 - SRC General Purpose Register 5"]
    pub gpr5: GPR5,
    #[doc = "0x34 - SRC General Purpose Register 6"]
    pub gpr6: GPR6,
    #[doc = "0x38 - SRC General Purpose Register 7"]
    pub gpr7: GPR7,
    #[doc = "0x3c - SRC General Purpose Register 8"]
    pub gpr8: GPR8,
    #[doc = "0x40 - SRC General Purpose Register 9"]
    pub gpr9: GPR9,
    #[doc = "0x44 - SRC General Purpose Register 10"]
    pub gpr10: GPR10,
}
#[doc = "SRC Control Register"]
pub struct SCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SRC Control Register"]
pub mod scr;
#[doc = "SRC Boot Mode Register 1"]
pub struct SBMR1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SRC Boot Mode Register 1"]
pub mod sbmr1;
#[doc = "SRC Reset Status Register"]
pub struct SRSR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SRC Reset Status Register"]
pub mod srsr;
#[doc = "SRC Boot Mode Register 2"]
pub struct SBMR2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SRC Boot Mode Register 2"]
pub mod sbmr2;
#[doc = "SRC General Purpose Register 1"]
pub struct GPR1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SRC General Purpose Register 1"]
pub mod gpr1;
#[doc = "SRC General Purpose Register 2"]
pub struct GPR2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SRC General Purpose Register 2"]
pub mod gpr2;
#[doc = "SRC General Purpose Register 3"]
pub struct GPR3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SRC General Purpose Register 3"]
pub mod gpr3;
#[doc = "SRC General Purpose Register 4"]
pub struct GPR4 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SRC General Purpose Register 4"]
pub mod gpr4;
#[doc = "SRC General Purpose Register 5"]
pub struct GPR5 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SRC General Purpose Register 5"]
pub mod gpr5;
#[doc = "SRC General Purpose Register 6"]
pub struct GPR6 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SRC General Purpose Register 6"]
pub mod gpr6;
#[doc = "SRC General Purpose Register 7"]
pub struct GPR7 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SRC General Purpose Register 7"]
pub mod gpr7;
#[doc = "SRC General Purpose Register 8"]
pub struct GPR8 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SRC General Purpose Register 8"]
pub mod gpr8;
#[doc = "SRC General Purpose Register 9"]
pub struct GPR9 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SRC General Purpose Register 9"]
pub mod gpr9;
#[doc = "SRC General Purpose Register 10"]
pub struct GPR10 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SRC General Purpose Register 10"]
pub mod gpr10;
