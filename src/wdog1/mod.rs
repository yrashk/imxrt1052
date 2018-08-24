#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Watchdog Control Register"]
    pub wcr: WCR,
    #[doc = "0x02 - Watchdog Service Register"]
    pub wsr: WSR,
    #[doc = "0x04 - Watchdog Reset Status Register"]
    pub wrsr: WRSR,
    #[doc = "0x06 - Watchdog Interrupt Control Register"]
    pub wicr: WICR,
    #[doc = "0x08 - Watchdog Miscellaneous Control Register"]
    pub wmcr: WMCR,
}
#[doc = "Watchdog Control Register"]
pub struct WCR {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Watchdog Control Register"]
pub mod wcr;
#[doc = "Watchdog Service Register"]
pub struct WSR {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Watchdog Service Register"]
pub mod wsr;
#[doc = "Watchdog Reset Status Register"]
pub struct WRSR {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Watchdog Reset Status Register"]
pub mod wrsr;
#[doc = "Watchdog Interrupt Control Register"]
pub struct WICR {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Watchdog Interrupt Control Register"]
pub mod wicr;
#[doc = "Watchdog Miscellaneous Control Register"]
pub struct WMCR {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Watchdog Miscellaneous Control Register"]
pub mod wmcr;
