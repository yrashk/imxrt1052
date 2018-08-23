#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Keypad Control Register"]
    pub kpcr: KPCR,
    #[doc = "0x02 - Keypad Status Register"]
    pub kpsr: KPSR,
    #[doc = "0x04 - Keypad Data Direction Register"]
    pub kddr: KDDR,
    #[doc = "0x06 - Keypad Data Register"]
    pub kpdr: KPDR,
}
#[doc = "Keypad Control Register"]
pub struct KPCR {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Keypad Control Register"]
pub mod kpcr;
#[doc = "Keypad Status Register"]
pub struct KPSR {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Keypad Status Register"]
pub mod kpsr;
#[doc = "Keypad Data Direction Register"]
pub struct KDDR {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Keypad Data Direction Register"]
pub mod kddr;
#[doc = "Keypad Data Register"]
pub struct KPDR {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Keypad Data Register"]
pub mod kpdr;
