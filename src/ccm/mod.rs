#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - CCM Control Register"]
    pub ccr: CCR,
    _reserved0: [u8; 4usize],
    #[doc = "0x08 - CCM Status Register"]
    pub csr: CSR,
    #[doc = "0x0c - CCM Clock Switcher Register"]
    pub ccsr: CCSR,
    #[doc = "0x10 - CCM Arm Clock Root Register"]
    pub cacrr: CACRR,
    #[doc = "0x14 - CCM Bus Clock Divider Register"]
    pub cbcdr: CBCDR,
    #[doc = "0x18 - CCM Bus Clock Multiplexer Register"]
    pub cbcmr: CBCMR,
    #[doc = "0x1c - CCM Serial Clock Multiplexer Register 1"]
    pub cscmr1: CSCMR1,
    #[doc = "0x20 - CCM Serial Clock Multiplexer Register 2"]
    pub cscmr2: CSCMR2,
    #[doc = "0x24 - CCM Serial Clock Divider Register 1"]
    pub cscdr1: CSCDR1,
    #[doc = "0x28 - CCM Clock Divider Register"]
    pub cs1cdr: CS1CDR,
    #[doc = "0x2c - CCM Clock Divider Register"]
    pub cs2cdr: CS2CDR,
    #[doc = "0x30 - CCM D1 Clock Divider Register"]
    pub cdcdr: CDCDR,
    _reserved1: [u8; 4usize],
    #[doc = "0x38 - CCM Serial Clock Divider Register 2"]
    pub cscdr2: CSCDR2,
    #[doc = "0x3c - CCM Serial Clock Divider Register 3"]
    pub cscdr3: CSCDR3,
    _reserved2: [u8; 8usize],
    #[doc = "0x48 - CCM Divider Handshake In-Process Register"]
    pub cdhipr: CDHIPR,
    _reserved3: [u8; 8usize],
    #[doc = "0x54 - CCM Low Power Control Register"]
    pub clpcr: CLPCR,
    #[doc = "0x58 - CCM Interrupt Status Register"]
    pub cisr: CISR,
    #[doc = "0x5c - CCM Interrupt Mask Register"]
    pub cimr: CIMR,
    #[doc = "0x60 - CCM Clock Output Source Register"]
    pub ccosr: CCOSR,
    #[doc = "0x64 - CCM General Purpose Register"]
    pub cgpr: CGPR,
    #[doc = "0x68 - CCM Clock Gating Register 0"]
    pub ccgr0: CCGR0,
    #[doc = "0x6c - CCM Clock Gating Register 1"]
    pub ccgr1: CCGR1,
    #[doc = "0x70 - CCM Clock Gating Register 2"]
    pub ccgr2: CCGR2,
    #[doc = "0x74 - CCM Clock Gating Register 3"]
    pub ccgr3: CCGR3,
    #[doc = "0x78 - CCM Clock Gating Register 4"]
    pub ccgr4: CCGR4,
    #[doc = "0x7c - CCM Clock Gating Register 5"]
    pub ccgr5: CCGR5,
    #[doc = "0x80 - CCM Clock Gating Register 6"]
    pub ccgr6: CCGR6,
    _reserved4: [u8; 4usize],
    #[doc = "0x88 - CCM Module Enable Overide Register"]
    pub cmeor: CMEOR,
}
#[doc = "CCM Control Register"]
pub struct CCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CCM Control Register"]
pub mod ccr;
#[doc = "CCM Status Register"]
pub struct CSR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CCM Status Register"]
pub mod csr;
#[doc = "CCM Clock Switcher Register"]
pub struct CCSR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CCM Clock Switcher Register"]
pub mod ccsr;
#[doc = "CCM Arm Clock Root Register"]
pub struct CACRR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CCM Arm Clock Root Register"]
pub mod cacrr;
#[doc = "CCM Bus Clock Divider Register"]
pub struct CBCDR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CCM Bus Clock Divider Register"]
pub mod cbcdr;
#[doc = "CCM Bus Clock Multiplexer Register"]
pub struct CBCMR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CCM Bus Clock Multiplexer Register"]
pub mod cbcmr;
#[doc = "CCM Serial Clock Multiplexer Register 1"]
pub struct CSCMR1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CCM Serial Clock Multiplexer Register 1"]
pub mod cscmr1;
#[doc = "CCM Serial Clock Multiplexer Register 2"]
pub struct CSCMR2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CCM Serial Clock Multiplexer Register 2"]
pub mod cscmr2;
#[doc = "CCM Serial Clock Divider Register 1"]
pub struct CSCDR1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CCM Serial Clock Divider Register 1"]
pub mod cscdr1;
#[doc = "CCM Clock Divider Register"]
pub struct CS1CDR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CCM Clock Divider Register"]
pub mod cs1cdr;
#[doc = "CCM Clock Divider Register"]
pub struct CS2CDR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CCM Clock Divider Register"]
pub mod cs2cdr;
#[doc = "CCM D1 Clock Divider Register"]
pub struct CDCDR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CCM D1 Clock Divider Register"]
pub mod cdcdr;
#[doc = "CCM Serial Clock Divider Register 2"]
pub struct CSCDR2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CCM Serial Clock Divider Register 2"]
pub mod cscdr2;
#[doc = "CCM Serial Clock Divider Register 3"]
pub struct CSCDR3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CCM Serial Clock Divider Register 3"]
pub mod cscdr3;
#[doc = "CCM Divider Handshake In-Process Register"]
pub struct CDHIPR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CCM Divider Handshake In-Process Register"]
pub mod cdhipr;
#[doc = "CCM Low Power Control Register"]
pub struct CLPCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CCM Low Power Control Register"]
pub mod clpcr;
#[doc = "CCM Interrupt Status Register"]
pub struct CISR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CCM Interrupt Status Register"]
pub mod cisr;
#[doc = "CCM Interrupt Mask Register"]
pub struct CIMR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CCM Interrupt Mask Register"]
pub mod cimr;
#[doc = "CCM Clock Output Source Register"]
pub struct CCOSR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CCM Clock Output Source Register"]
pub mod ccosr;
#[doc = "CCM General Purpose Register"]
pub struct CGPR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CCM General Purpose Register"]
pub mod cgpr;
#[doc = "CCM Clock Gating Register 0"]
pub struct CCGR0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CCM Clock Gating Register 0"]
pub mod ccgr0;
#[doc = "CCM Clock Gating Register 1"]
pub struct CCGR1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CCM Clock Gating Register 1"]
pub mod ccgr1;
#[doc = "CCM Clock Gating Register 2"]
pub struct CCGR2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CCM Clock Gating Register 2"]
pub mod ccgr2;
#[doc = "CCM Clock Gating Register 3"]
pub struct CCGR3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CCM Clock Gating Register 3"]
pub mod ccgr3;
#[doc = "CCM Clock Gating Register 4"]
pub struct CCGR4 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CCM Clock Gating Register 4"]
pub mod ccgr4;
#[doc = "CCM Clock Gating Register 5"]
pub struct CCGR5 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CCM Clock Gating Register 5"]
pub mod ccgr5;
#[doc = "CCM Clock Gating Register 6"]
pub struct CCGR6 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CCM Clock Gating Register 6"]
pub mod ccgr6;
#[doc = "CCM Module Enable Overide Register"]
pub struct CMEOR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CCM Module Enable Overide Register"]
pub mod cmeor;
