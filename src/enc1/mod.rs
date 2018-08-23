#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control Register"]
    pub ctrl: CTRL,
    #[doc = "0x02 - Input Filter Register"]
    pub filt: FILT,
    #[doc = "0x04 - Watchdog Timeout Register"]
    pub wtr: WTR,
    #[doc = "0x06 - Position Difference Counter Register"]
    pub posd: POSD,
    #[doc = "0x08 - Position Difference Hold Register"]
    pub posdh: POSDH,
    #[doc = "0x0a - Revolution Counter Register"]
    pub rev: REV,
    #[doc = "0x0c - Revolution Hold Register"]
    pub revh: REVH,
    #[doc = "0x0e - Upper Position Counter Register"]
    pub upos: UPOS,
    #[doc = "0x10 - Lower Position Counter Register"]
    pub lpos: LPOS,
    #[doc = "0x12 - Upper Position Hold Register"]
    pub uposh: UPOSH,
    #[doc = "0x14 - Lower Position Hold Register"]
    pub lposh: LPOSH,
    #[doc = "0x16 - Upper Initialization Register"]
    pub uinit: UINIT,
    #[doc = "0x18 - Lower Initialization Register"]
    pub linit: LINIT,
    #[doc = "0x1a - Input Monitor Register"]
    pub imr: IMR,
    #[doc = "0x1c - Test Register"]
    pub tst: TST,
    #[doc = "0x1e - Control 2 Register"]
    pub ctrl2: CTRL2,
    #[doc = "0x20 - Upper Modulus Register"]
    pub umod: UMOD,
    #[doc = "0x22 - Lower Modulus Register"]
    pub lmod: LMOD,
    #[doc = "0x24 - Upper Position Compare Register"]
    pub ucomp: UCOMP,
    #[doc = "0x26 - Lower Position Compare Register"]
    pub lcomp: LCOMP,
}
#[doc = "Control Register"]
pub struct CTRL {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Control Register"]
pub mod ctrl;
#[doc = "Input Filter Register"]
pub struct FILT {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Input Filter Register"]
pub mod filt;
#[doc = "Watchdog Timeout Register"]
pub struct WTR {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Watchdog Timeout Register"]
pub mod wtr;
#[doc = "Position Difference Counter Register"]
pub struct POSD {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Position Difference Counter Register"]
pub mod posd;
#[doc = "Position Difference Hold Register"]
pub struct POSDH {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Position Difference Hold Register"]
pub mod posdh;
#[doc = "Revolution Counter Register"]
pub struct REV {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Revolution Counter Register"]
pub mod rev;
#[doc = "Revolution Hold Register"]
pub struct REVH {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Revolution Hold Register"]
pub mod revh;
#[doc = "Upper Position Counter Register"]
pub struct UPOS {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Upper Position Counter Register"]
pub mod upos;
#[doc = "Lower Position Counter Register"]
pub struct LPOS {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Lower Position Counter Register"]
pub mod lpos;
#[doc = "Upper Position Hold Register"]
pub struct UPOSH {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Upper Position Hold Register"]
pub mod uposh;
#[doc = "Lower Position Hold Register"]
pub struct LPOSH {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Lower Position Hold Register"]
pub mod lposh;
#[doc = "Upper Initialization Register"]
pub struct UINIT {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Upper Initialization Register"]
pub mod uinit;
#[doc = "Lower Initialization Register"]
pub struct LINIT {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Lower Initialization Register"]
pub mod linit;
#[doc = "Input Monitor Register"]
pub struct IMR {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Input Monitor Register"]
pub mod imr;
#[doc = "Test Register"]
pub struct TST {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Test Register"]
pub mod tst;
#[doc = "Control 2 Register"]
pub struct CTRL2 {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Control 2 Register"]
pub mod ctrl2;
#[doc = "Upper Modulus Register"]
pub struct UMOD {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Upper Modulus Register"]
pub mod umod;
#[doc = "Lower Modulus Register"]
pub struct LMOD {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Lower Modulus Register"]
pub mod lmod;
#[doc = "Upper Position Compare Register"]
pub struct UCOMP {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Upper Position Compare Register"]
pub mod ucomp;
#[doc = "Lower Position Compare Register"]
pub struct LCOMP {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Lower Position Compare Register"]
pub mod lcomp;
