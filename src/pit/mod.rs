#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - PIT Module Control Register"]
    pub mcr: MCR,
    _reserved0: [u8; 220usize],
    #[doc = "0xe0 - PIT Upper Lifetime Timer Register"]
    pub ltmr64h: LTMR64H,
    #[doc = "0xe4 - PIT Lower Lifetime Timer Register"]
    pub ltmr64l: LTMR64L,
    _reserved1: [u8; 24usize],
    #[doc = "0x100 - no description available"]
    pub timer: [TIMER; 4],
}
#[doc = r" Register block"]
#[repr(C)]
pub struct TIMER {
    #[doc = "0x00 - Timer Load Value Register"]
    pub ldval: self::timer::LDVAL,
    #[doc = "0x04 - Current Timer Value Register"]
    pub cval: self::timer::CVAL,
    #[doc = "0x08 - Timer Control Register"]
    pub tctrl: self::timer::TCTRL,
    #[doc = "0x0c - Timer Flag Register"]
    pub tflg: self::timer::TFLG,
}
#[doc = r" Register block"]
#[doc = "no description available"]
pub mod timer;
#[doc = "PIT Module Control Register"]
pub struct MCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PIT Module Control Register"]
pub mod mcr;
#[doc = "PIT Upper Lifetime Timer Register"]
pub struct LTMR64H {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PIT Upper Lifetime Timer Register"]
pub mod ltmr64h;
#[doc = "PIT Lower Lifetime Timer Register"]
pub struct LTMR64L {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PIT Lower Lifetime Timer Register"]
pub mod ltmr64l;
