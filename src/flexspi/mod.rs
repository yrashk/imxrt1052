#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Module Control Register 0"]
    pub mcr0: MCR0,
    #[doc = "0x04 - Module Control Register 1"]
    pub mcr1: MCR1,
    #[doc = "0x08 - Module Control Register 2"]
    pub mcr2: MCR2,
    #[doc = "0x0c - AHB Bus Control Register"]
    pub ahbcr: AHBCR,
    #[doc = "0x10 - Interrupt Enable Register"]
    pub inten: INTEN,
    #[doc = "0x14 - Interrupt Register"]
    pub intr: INTR,
    #[doc = "0x18 - LUT Key Register"]
    pub lutkey: LUTKEY,
    #[doc = "0x1c - LUT Control Register"]
    pub lutcr: LUTCR,
    #[doc = "0x20 - AHB RX Buffer 0 Control Register 0"]
    pub ahbrxbufcr0: [AHBRXBUFCR0; 4],
    _reserved0: [u8; 48usize],
    #[doc = "0x60 - Flash A1 Control Register 0"]
    pub flshcr0a1: FLSHCR0,
    #[doc = "0x64 - Flash A1 Control Register 0"]
    pub flshcr0a2: FLSHCR0,
    #[doc = "0x68 - Flash A1 Control Register 0"]
    pub flshcr0b1: FLSHCR0,
    #[doc = "0x6c - Flash A1 Control Register 0"]
    pub flshcr0b2: FLSHCR0,
    #[doc = "0x70 - Flash A1 Control Register 1"]
    pub flshcr1a1: FLSHCR1,
    #[doc = "0x74 - Flash A1 Control Register 1"]
    pub flshcr1a2: FLSHCR1,
    #[doc = "0x78 - Flash A1 Control Register 1"]
    pub flshcr1b1: FLSHCR1,
    #[doc = "0x7c - Flash A1 Control Register 1"]
    pub flshcr1b2: FLSHCR1,
    #[doc = "0x80 - Flash A1 Control Register 2"]
    pub flshcr2a1: FLSHCR2,
    #[doc = "0x84 - Flash A1 Control Register 2"]
    pub flshcr2a2: FLSHCR2,
    #[doc = "0x88 - Flash A1 Control Register 2"]
    pub flshcr2b1: FLSHCR2,
    #[doc = "0x8c - Flash A1 Control Register 2"]
    pub flshcr2b2: FLSHCR2,
    _reserved1: [u8; 4usize],
    #[doc = "0x94 - Flash Control Register 4"]
    pub flshcr4: FLSHCR4,
    _reserved2: [u8; 8usize],
    #[doc = "0xa0 - IP Control Register 0"]
    pub ipcr0: IPCR0,
    #[doc = "0xa4 - IP Control Register 1"]
    pub ipcr1: IPCR1,
    _reserved3: [u8; 8usize],
    #[doc = "0xb0 - IP Command Register"]
    pub ipcmd: IPCMD,
    _reserved4: [u8; 4usize],
    #[doc = "0xb8 - IP RX FIFO Control Register"]
    pub iprxfcr: IPRXFCR,
    #[doc = "0xbc - IP TX FIFO Control Register"]
    pub iptxfcr: IPTXFCR,
    #[doc = "0xc0 - DLL Control Register 0"]
    pub dllcra: DLLCR,
    #[doc = "0xc4 - DLL Control Register 0"]
    pub dllcrb: DLLCR,
    _reserved5: [u8; 24usize],
    #[doc = "0xe0 - Status Register 0"]
    pub sts0: STS0,
    #[doc = "0xe4 - Status Register 1"]
    pub sts1: STS1,
    #[doc = "0xe8 - Status Register 2"]
    pub sts2: STS2,
    #[doc = "0xec - AHB Suspend Status Register"]
    pub ahbspndsts: AHBSPNDSTS,
    #[doc = "0xf0 - IP RX FIFO Status Register"]
    pub iprxfsts: IPRXFSTS,
    #[doc = "0xf4 - IP TX FIFO Status Register"]
    pub iptxfsts: IPTXFSTS,
    _reserved6: [u8; 8usize],
    #[doc = "0x100 - IP RX FIFO Data Register 0"]
    pub rfdr: [RFDR; 32],
    #[doc = "0x180 - IP TX FIFO Data Register 0"]
    pub tfdr: [TFDR; 32],
    #[doc = "0x200 - LUT 0"]
    pub lut: [LUT; 64],
}
#[doc = "Module Control Register 0"]
pub struct MCR0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Module Control Register 0"]
pub mod mcr0;
#[doc = "Module Control Register 1"]
pub struct MCR1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Module Control Register 1"]
pub mod mcr1;
#[doc = "Module Control Register 2"]
pub struct MCR2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Module Control Register 2"]
pub mod mcr2;
#[doc = "AHB Bus Control Register"]
pub struct AHBCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "AHB Bus Control Register"]
pub mod ahbcr;
#[doc = "Interrupt Enable Register"]
pub struct INTEN {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt Enable Register"]
pub mod inten;
#[doc = "Interrupt Register"]
pub struct INTR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt Register"]
pub mod intr;
#[doc = "LUT Key Register"]
pub struct LUTKEY {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LUT Key Register"]
pub mod lutkey;
#[doc = "LUT Control Register"]
pub struct LUTCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LUT Control Register"]
pub mod lutcr;
#[doc = "AHB RX Buffer 0 Control Register 0"]
pub struct AHBRXBUFCR0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "AHB RX Buffer 0 Control Register 0"]
pub mod ahbrxbufcr0;
#[doc = "Flash A1 Control Register 0"]
pub struct FLSHCR0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Flash A1 Control Register 0"]
pub mod flshcr0;
#[doc = "Flash A1 Control Register 1"]
pub struct FLSHCR1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Flash A1 Control Register 1"]
pub mod flshcr1;
#[doc = "Flash A1 Control Register 2"]
pub struct FLSHCR2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Flash A1 Control Register 2"]
pub mod flshcr2;
#[doc = "Flash Control Register 4"]
pub struct FLSHCR4 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Flash Control Register 4"]
pub mod flshcr4;
#[doc = "IP Control Register 0"]
pub struct IPCR0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "IP Control Register 0"]
pub mod ipcr0;
#[doc = "IP Control Register 1"]
pub struct IPCR1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "IP Control Register 1"]
pub mod ipcr1;
#[doc = "IP Command Register"]
pub struct IPCMD {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "IP Command Register"]
pub mod ipcmd;
#[doc = "IP RX FIFO Control Register"]
pub struct IPRXFCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "IP RX FIFO Control Register"]
pub mod iprxfcr;
#[doc = "IP TX FIFO Control Register"]
pub struct IPTXFCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "IP TX FIFO Control Register"]
pub mod iptxfcr;
#[doc = "DLL Control Register 0"]
pub struct DLLCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DLL Control Register 0"]
pub mod dllcr;
#[doc = "Status Register 0"]
pub struct STS0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Status Register 0"]
pub mod sts0;
#[doc = "Status Register 1"]
pub struct STS1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Status Register 1"]
pub mod sts1;
#[doc = "Status Register 2"]
pub struct STS2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Status Register 2"]
pub mod sts2;
#[doc = "AHB Suspend Status Register"]
pub struct AHBSPNDSTS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "AHB Suspend Status Register"]
pub mod ahbspndsts;
#[doc = "IP RX FIFO Status Register"]
pub struct IPRXFSTS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "IP RX FIFO Status Register"]
pub mod iprxfsts;
#[doc = "IP TX FIFO Status Register"]
pub struct IPTXFSTS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "IP TX FIFO Status Register"]
pub mod iptxfsts;
#[doc = "IP RX FIFO Data Register 0"]
pub struct RFDR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "IP RX FIFO Data Register 0"]
pub mod rfdr;
#[doc = "IP TX FIFO Data Register 0"]
pub struct TFDR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "IP TX FIFO Data Register 0"]
pub mod tfdr;
#[doc = "LUT 0"]
pub struct LUT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LUT 0"]
pub mod lut;
