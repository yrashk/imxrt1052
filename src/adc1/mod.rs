#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control register for hardware triggers"]
    pub hc0: HC0,
    #[doc = "0x04 - Control register for hardware triggers"]
    pub hc1: HC,
    #[doc = "0x08 - Control register for hardware triggers"]
    pub hc2: HC,
    #[doc = "0x0c - Control register for hardware triggers"]
    pub hc3: HC,
    #[doc = "0x10 - Control register for hardware triggers"]
    pub hc4: HC,
    #[doc = "0x14 - Control register for hardware triggers"]
    pub hc5: HC,
    #[doc = "0x18 - Control register for hardware triggers"]
    pub hc6: HC,
    #[doc = "0x1c - Control register for hardware triggers"]
    pub hc7: HC,
    #[doc = "0x20 - Status register for HW triggers"]
    pub hs: HS,
    #[doc = "0x24 - Data result register for HW triggers"]
    pub r0: R0,
    #[doc = "0x28 - Data result register for HW triggers"]
    pub r1: R,
    #[doc = "0x2c - Data result register for HW triggers"]
    pub r2: R,
    #[doc = "0x30 - Data result register for HW triggers"]
    pub r3: R,
    #[doc = "0x34 - Data result register for HW triggers"]
    pub r4: R,
    #[doc = "0x38 - Data result register for HW triggers"]
    pub r5: R,
    #[doc = "0x3c - Data result register for HW triggers"]
    pub r6: R,
    #[doc = "0x40 - Data result register for HW triggers"]
    pub r7: R,
    #[doc = "0x44 - Configuration register"]
    pub cfg: CFG,
    #[doc = "0x48 - General control register"]
    pub gc: GC,
    #[doc = "0x4c - General status register"]
    pub gs: GS,
    #[doc = "0x50 - Compare value register"]
    pub cv: CV,
    #[doc = "0x54 - Offset correction value register"]
    pub ofs: OFS,
    #[doc = "0x58 - Calibration value register"]
    pub cal: CAL,
}
#[doc = "Control register for hardware triggers"]
pub struct HC0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Control register for hardware triggers"]
pub mod hc0;
#[doc = "Control register for hardware triggers"]
pub struct HC {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Control register for hardware triggers"]
pub mod hc;
#[doc = "Status register for HW triggers"]
pub struct HS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Status register for HW triggers"]
pub mod hs;
#[doc = "Data result register for HW triggers"]
pub struct R0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Data result register for HW triggers"]
pub mod r0;
#[doc = "Data result register for HW triggers"]
pub struct R {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Data result register for HW triggers"]
pub mod r;
#[doc = "Configuration register"]
pub struct CFG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Configuration register"]
pub mod cfg;
#[doc = "General control register"]
pub struct GC {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "General control register"]
pub mod gc;
#[doc = "General status register"]
pub struct GS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "General status register"]
pub mod gs;
#[doc = "Compare value register"]
pub struct CV {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Compare value register"]
pub mod cv;
#[doc = "Offset correction value register"]
pub struct OFS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Offset correction value register"]
pub mod ofs;
#[doc = "Calibration value register"]
pub struct CAL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Calibration value register"]
pub mod cal;
