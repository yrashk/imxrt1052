#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 544usize],
    #[doc = "0x220 - PGC Mega Control Register"]
    pub mega_ctrl: MEGA_CTRL,
    #[doc = "0x224 - PGC Mega Power Up Sequence Control Register"]
    pub mega_pupscr: MEGA_PUPSCR,
    #[doc = "0x228 - PGC Mega Pull Down Sequence Control Register"]
    pub mega_pdnscr: MEGA_PDNSCR,
    #[doc = "0x22c - PGC Mega Power Gating Controller Status Register"]
    pub mega_sr: MEGA_SR,
    _reserved1: [u8; 112usize],
    #[doc = "0x2a0 - PGC CPU Control Register"]
    pub cpu_ctrl: CPU_CTRL,
    #[doc = "0x2a4 - PGC CPU Power Up Sequence Control Register"]
    pub cpu_pupscr: CPU_PUPSCR,
    #[doc = "0x2a8 - PGC CPU Pull Down Sequence Control Register"]
    pub cpu_pdnscr: CPU_PDNSCR,
    #[doc = "0x2ac - PGC CPU Power Gating Controller Status Register"]
    pub cpu_sr: CPU_SR,
}
#[doc = "PGC Mega Control Register"]
pub struct MEGA_CTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PGC Mega Control Register"]
pub mod mega_ctrl;
#[doc = "PGC Mega Power Up Sequence Control Register"]
pub struct MEGA_PUPSCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PGC Mega Power Up Sequence Control Register"]
pub mod mega_pupscr;
#[doc = "PGC Mega Pull Down Sequence Control Register"]
pub struct MEGA_PDNSCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PGC Mega Pull Down Sequence Control Register"]
pub mod mega_pdnscr;
#[doc = "PGC Mega Power Gating Controller Status Register"]
pub struct MEGA_SR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PGC Mega Power Gating Controller Status Register"]
pub mod mega_sr;
#[doc = "PGC CPU Control Register"]
pub struct CPU_CTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PGC CPU Control Register"]
pub mod cpu_ctrl;
#[doc = "PGC CPU Power Up Sequence Control Register"]
pub struct CPU_PUPSCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PGC CPU Power Up Sequence Control Register"]
pub mod cpu_pupscr;
#[doc = "PGC CPU Pull Down Sequence Control Register"]
pub struct CPU_PDNSCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PGC CPU Pull Down Sequence Control Register"]
pub mod cpu_pdnscr;
#[doc = "PGC CPU Power Gating Controller Status Register"]
pub struct CPU_SR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PGC CPU Power Gating Controller Status Register"]
pub mod cpu_sr;
