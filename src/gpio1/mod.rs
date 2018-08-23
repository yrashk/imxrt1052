#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - GPIO data register"]
    pub dr: DR,
    #[doc = "0x04 - GPIO direction register"]
    pub gdir: GDIR,
    #[doc = "0x08 - GPIO pad status register"]
    pub psr: PSR,
    #[doc = "0x0c - GPIO interrupt configuration register1"]
    pub icr1: ICR1,
    #[doc = "0x10 - GPIO interrupt configuration register2"]
    pub icr2: ICR2,
    #[doc = "0x14 - GPIO interrupt mask register"]
    pub imr: IMR,
    #[doc = "0x18 - GPIO interrupt status register"]
    pub isr: ISR,
    #[doc = "0x1c - GPIO edge select register"]
    pub edge_sel: EDGE_SEL,
}
#[doc = "GPIO data register"]
pub struct DR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "GPIO data register"]
pub mod dr;
#[doc = "GPIO direction register"]
pub struct GDIR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "GPIO direction register"]
pub mod gdir;
#[doc = "GPIO pad status register"]
pub struct PSR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "GPIO pad status register"]
pub mod psr;
#[doc = "GPIO interrupt configuration register1"]
pub struct ICR1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "GPIO interrupt configuration register1"]
pub mod icr1;
#[doc = "GPIO interrupt configuration register2"]
pub struct ICR2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "GPIO interrupt configuration register2"]
pub mod icr2;
#[doc = "GPIO interrupt mask register"]
pub struct IMR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "GPIO interrupt mask register"]
pub mod imr;
#[doc = "GPIO interrupt status register"]
pub struct ISR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "GPIO interrupt status register"]
pub mod isr;
#[doc = "GPIO edge select register"]
pub struct EDGE_SEL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "GPIO edge select register"]
pub mod edge_sel;
