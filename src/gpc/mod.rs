#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - GPC Interface control register"]
    pub cntr: CNTR,
    _reserved0: [u8; 4usize],
    #[doc = "0x08 - IRQ masking register 1"]
    pub imr1: IMR1,
    #[doc = "0x0c - IRQ masking register 2"]
    pub imr2: IMR2,
    #[doc = "0x10 - IRQ masking register 3"]
    pub imr3: IMR3,
    #[doc = "0x14 - IRQ masking register 4"]
    pub imr4: IMR4,
    #[doc = "0x18 - IRQ status resister 1"]
    pub isr1: ISR1,
    #[doc = "0x1c - IRQ status resister 2"]
    pub isr2: ISR2,
    #[doc = "0x20 - IRQ status resister 3"]
    pub isr3: ISR3,
    #[doc = "0x24 - IRQ status resister 4"]
    pub isr4: ISR4,
    _reserved1: [u8; 12usize],
    #[doc = "0x34 - IRQ masking register 5"]
    pub imr5: IMR5,
    #[doc = "0x38 - IRQ status resister 5"]
    pub isr5: ISR5,
}
#[doc = "GPC Interface control register"]
pub struct CNTR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "GPC Interface control register"]
pub mod cntr;
#[doc = "IRQ masking register 1"]
pub struct IMR1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "IRQ masking register 1"]
pub mod imr1;
#[doc = "IRQ masking register 2"]
pub struct IMR2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "IRQ masking register 2"]
pub mod imr2;
#[doc = "IRQ masking register 3"]
pub struct IMR3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "IRQ masking register 3"]
pub mod imr3;
#[doc = "IRQ masking register 4"]
pub struct IMR4 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "IRQ masking register 4"]
pub mod imr4;
#[doc = "IRQ status resister 1"]
pub struct ISR1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "IRQ status resister 1"]
pub mod isr1;
#[doc = "IRQ status resister 2"]
pub struct ISR2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "IRQ status resister 2"]
pub mod isr2;
#[doc = "IRQ status resister 3"]
pub struct ISR3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "IRQ status resister 3"]
pub mod isr3;
#[doc = "IRQ status resister 4"]
pub struct ISR4 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "IRQ status resister 4"]
pub mod isr4;
#[doc = "IRQ masking register 5"]
pub struct IMR5 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "IRQ masking register 5"]
pub mod imr5;
#[doc = "IRQ status resister 5"]
pub struct ISR5 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "IRQ status resister 5"]
pub mod isr5;
