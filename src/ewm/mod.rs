#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control Register"]
    pub ctrl: CTRL,
    #[doc = "0x01 - Service Register"]
    pub serv: SERV,
    #[doc = "0x02 - Compare Low Register"]
    pub cmpl: CMPL,
    #[doc = "0x03 - Compare High Register"]
    pub cmph: CMPH,
    #[doc = "0x04 - Clock Control Register"]
    pub clkctrl: CLKCTRL,
    #[doc = "0x05 - Clock Prescaler Register"]
    pub clkprescaler: CLKPRESCALER,
}
#[doc = "Control Register"]
pub struct CTRL {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "Control Register"]
pub mod ctrl;
#[doc = "Service Register"]
pub struct SERV {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "Service Register"]
pub mod serv;
#[doc = "Compare Low Register"]
pub struct CMPL {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "Compare Low Register"]
pub mod cmpl;
#[doc = "Compare High Register"]
pub struct CMPH {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "Compare High Register"]
pub mod cmph;
#[doc = "Clock Control Register"]
pub struct CLKCTRL {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "Clock Control Register"]
pub mod clkctrl;
#[doc = "Clock Prescaler Register"]
pub struct CLKPRESCALER {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "Clock Prescaler Register"]
pub mod clkprescaler;
