#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 384usize],
    #[doc = "0x180 - Tempsensor Control Register 0"]
    pub tempsense0: TEMPSENSE0,
    #[doc = "0x184 - Tempsensor Control Register 0"]
    pub tempsense0_set: TEMPSENSE0_SET,
    #[doc = "0x188 - Tempsensor Control Register 0"]
    pub tempsense0_clr: TEMPSENSE0_CLR,
    #[doc = "0x18c - Tempsensor Control Register 0"]
    pub tempsense0_tog: TEMPSENSE0_TOG,
    #[doc = "0x190 - Tempsensor Control Register 1"]
    pub tempsense1: TEMPSENSE1,
    #[doc = "0x194 - Tempsensor Control Register 1"]
    pub tempsense1_set: TEMPSENSE1_SET,
    #[doc = "0x198 - Tempsensor Control Register 1"]
    pub tempsense1_clr: TEMPSENSE1_CLR,
    #[doc = "0x19c - Tempsensor Control Register 1"]
    pub tempsense1_tog: TEMPSENSE1_TOG,
    _reserved1: [u8; 240usize],
    #[doc = "0x290 - Tempsensor Control Register 2"]
    pub tempsense2: TEMPSENSE2,
    #[doc = "0x294 - Tempsensor Control Register 2"]
    pub tempsense2_set: TEMPSENSE2_SET,
    #[doc = "0x298 - Tempsensor Control Register 2"]
    pub tempsense2_clr: TEMPSENSE2_CLR,
    #[doc = "0x29c - Tempsensor Control Register 2"]
    pub tempsense2_tog: TEMPSENSE2_TOG,
}
#[doc = "Tempsensor Control Register 0"]
pub struct TEMPSENSE0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Tempsensor Control Register 0"]
pub mod tempsense0;
#[doc = "Tempsensor Control Register 0"]
pub struct TEMPSENSE0_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Tempsensor Control Register 0"]
pub mod tempsense0_set;
#[doc = "Tempsensor Control Register 0"]
pub struct TEMPSENSE0_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Tempsensor Control Register 0"]
pub mod tempsense0_clr;
#[doc = "Tempsensor Control Register 0"]
pub struct TEMPSENSE0_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Tempsensor Control Register 0"]
pub mod tempsense0_tog;
#[doc = "Tempsensor Control Register 1"]
pub struct TEMPSENSE1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Tempsensor Control Register 1"]
pub mod tempsense1;
#[doc = "Tempsensor Control Register 1"]
pub struct TEMPSENSE1_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Tempsensor Control Register 1"]
pub mod tempsense1_set;
#[doc = "Tempsensor Control Register 1"]
pub struct TEMPSENSE1_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Tempsensor Control Register 1"]
pub mod tempsense1_clr;
#[doc = "Tempsensor Control Register 1"]
pub struct TEMPSENSE1_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Tempsensor Control Register 1"]
pub mod tempsense1_tog;
#[doc = "Tempsensor Control Register 2"]
pub struct TEMPSENSE2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Tempsensor Control Register 2"]
pub mod tempsense2;
#[doc = "Tempsensor Control Register 2"]
pub struct TEMPSENSE2_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Tempsensor Control Register 2"]
pub mod tempsense2_set;
#[doc = "Tempsensor Control Register 2"]
pub struct TEMPSENSE2_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Tempsensor Control Register 2"]
pub mod tempsense2_clr;
#[doc = "Tempsensor Control Register 2"]
pub struct TEMPSENSE2_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Tempsensor Control Register 2"]
pub mod tempsense2_tog;
