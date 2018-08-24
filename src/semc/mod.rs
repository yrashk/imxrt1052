#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Module Control Register"]
    pub mcr: MCR,
    #[doc = "0x04 - IO Mux Control Register"]
    pub iocr: IOCR,
    #[doc = "0x08 - Master Bus (AXI) Control Register 0"]
    pub bmcr0: BMCR0,
    #[doc = "0x0c - Master Bus (AXI) Control Register 1"]
    pub bmcr1: BMCR1,
    #[doc = "0x10 - Base Register 0 (For SDRAM CS0 device)"]
    pub br0: BR0,
    #[doc = "0x14 - Base Register 1 (For SDRAM CS1 device)"]
    pub br1: BR1,
    #[doc = "0x18 - Base Register 2 (For SDRAM CS2 device)"]
    pub br2: BR2,
    #[doc = "0x1c - Base Register 3 (For SDRAM CS3 device)"]
    pub br3: BR3,
    #[doc = "0x20 - Base Register 4 (For NAND device)"]
    pub br4: BR4,
    #[doc = "0x24 - Base Register 5 (For NOR device)"]
    pub br5: BR5,
    #[doc = "0x28 - Base Register 6 (For PSRAM device)"]
    pub br6: BR6,
    #[doc = "0x2c - Base Register 7 (For DBI-B (MIPI Display Bus Interface Type B) device)"]
    pub br7: BR7,
    #[doc = "0x30 - Base Register 8 (For NAND device)"]
    pub br8: BR8,
    _reserved0: [u8; 4usize],
    #[doc = "0x38 - Interrupt Enable Register"]
    pub inten: INTEN,
    #[doc = "0x3c - Interrupt Enable Register"]
    pub intr: INTR,
    #[doc = "0x40 - SDRAM control register 0"]
    pub sdramcr0: SDRAMCR0,
    #[doc = "0x44 - SDRAM control register 1"]
    pub sdramcr1: SDRAMCR1,
    #[doc = "0x48 - SDRAM control register 2"]
    pub sdramcr2: SDRAMCR2,
    #[doc = "0x4c - SDRAM control register 3"]
    pub sdramcr3: SDRAMCR3,
    #[doc = "0x50 - NAND control register 0"]
    pub nandcr0: NANDCR0,
    #[doc = "0x54 - NAND control register 1"]
    pub nandcr1: NANDCR1,
    #[doc = "0x58 - NAND control register 2"]
    pub nandcr2: NANDCR2,
    #[doc = "0x5c - NAND control register 3"]
    pub nandcr3: NANDCR3,
    #[doc = "0x60 - NOR control register 0"]
    pub norcr0: NORCR0,
    #[doc = "0x64 - NOR control register 1"]
    pub norcr1: NORCR1,
    #[doc = "0x68 - NOR control register 2"]
    pub norcr2: NORCR2,
    #[doc = "0x6c - NOR control register 3"]
    pub norcr3: NORCR3,
    #[doc = "0x70 - SRAM control register 0"]
    pub sramcr0: SRAMCR0,
    #[doc = "0x74 - SRAM control register 1"]
    pub sramcr1: SRAMCR1,
    #[doc = "0x78 - SRAM control register 2"]
    pub sramcr2: SRAMCR2,
    #[doc = "0x7c - SRAM control register 3"]
    pub sramcr3: SRAMCR3,
    #[doc = "0x80 - DBI-B control register 0"]
    pub dbicr0: DBICR0,
    #[doc = "0x84 - DBI-B control register 1"]
    pub dbicr1: DBICR1,
    _reserved1: [u8; 8usize],
    #[doc = "0x90 - IP Command control register 0"]
    pub ipcr0: IPCR0,
    #[doc = "0x94 - IP Command control register 1"]
    pub ipcr1: IPCR1,
    #[doc = "0x98 - IP Command control register 2"]
    pub ipcr2: IPCR2,
    #[doc = "0x9c - IP Command register"]
    pub ipcmd: IPCMD,
    #[doc = "0xa0 - TX DATA register (for IP Command)"]
    pub iptxdat: IPTXDAT,
    _reserved2: [u8; 12usize],
    #[doc = "0xb0 - RX DATA register (for IP Command)"]
    pub iprxdat: IPRXDAT,
    _reserved3: [u8; 12usize],
    #[doc = "0xc0 - Status register 0"]
    pub sts0: STS0,
    #[doc = "0xc4 - Status register 1"]
    pub sts1: STS1,
    #[doc = "0xc8 - Status register 2"]
    pub sts2: STS2,
    #[doc = "0xcc - Status register 3"]
    pub sts3: STS3,
    #[doc = "0xd0 - Status register 4"]
    pub sts4: STS4,
    #[doc = "0xd4 - Status register 5"]
    pub sts5: STS5,
    #[doc = "0xd8 - Status register 6"]
    pub sts6: STS6,
    #[doc = "0xdc - Status register 7"]
    pub sts7: STS7,
    #[doc = "0xe0 - Status register 8"]
    pub sts8: STS8,
    #[doc = "0xe4 - Status register 9"]
    pub sts9: STS9,
    #[doc = "0xe8 - Status register 10"]
    pub sts10: STS10,
    #[doc = "0xec - Status register 11"]
    pub sts11: STS11,
    #[doc = "0xf0 - Status register 12"]
    pub sts12: STS12,
    #[doc = "0xf4 - Status register 13"]
    pub sts13: STS13,
    #[doc = "0xf8 - Status register 14"]
    pub sts14: STS14,
    #[doc = "0xfc - Status register 15"]
    pub sts15: STS15,
}
#[doc = "Module Control Register"]
pub struct MCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Module Control Register"]
pub mod mcr;
#[doc = "IO Mux Control Register"]
pub struct IOCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "IO Mux Control Register"]
pub mod iocr;
#[doc = "Master Bus (AXI) Control Register 0"]
pub struct BMCR0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Master Bus (AXI) Control Register 0"]
pub mod bmcr0;
#[doc = "Master Bus (AXI) Control Register 1"]
pub struct BMCR1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Master Bus (AXI) Control Register 1"]
pub mod bmcr1;
#[doc = "Base Register 0 (For SDRAM CS0 device)"]
pub struct BR0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Base Register 0 (For SDRAM CS0 device)"]
pub mod br0;
#[doc = "Base Register 1 (For SDRAM CS1 device)"]
pub struct BR1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Base Register 1 (For SDRAM CS1 device)"]
pub mod br1;
#[doc = "Base Register 2 (For SDRAM CS2 device)"]
pub struct BR2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Base Register 2 (For SDRAM CS2 device)"]
pub mod br2;
#[doc = "Base Register 3 (For SDRAM CS3 device)"]
pub struct BR3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Base Register 3 (For SDRAM CS3 device)"]
pub mod br3;
#[doc = "Base Register 4 (For NAND device)"]
pub struct BR4 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Base Register 4 (For NAND device)"]
pub mod br4;
#[doc = "Base Register 5 (For NOR device)"]
pub struct BR5 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Base Register 5 (For NOR device)"]
pub mod br5;
#[doc = "Base Register 6 (For PSRAM device)"]
pub struct BR6 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Base Register 6 (For PSRAM device)"]
pub mod br6;
#[doc = "Base Register 7 (For DBI-B (MIPI Display Bus Interface Type B) device)"]
pub struct BR7 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Base Register 7 (For DBI-B (MIPI Display Bus Interface Type B) device)"]
pub mod br7;
#[doc = "Base Register 8 (For NAND device)"]
pub struct BR8 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Base Register 8 (For NAND device)"]
pub mod br8;
#[doc = "Interrupt Enable Register"]
pub struct INTEN {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt Enable Register"]
pub mod inten;
#[doc = "Interrupt Enable Register"]
pub struct INTR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt Enable Register"]
pub mod intr;
#[doc = "SDRAM control register 0"]
pub struct SDRAMCR0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SDRAM control register 0"]
pub mod sdramcr0;
#[doc = "SDRAM control register 1"]
pub struct SDRAMCR1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SDRAM control register 1"]
pub mod sdramcr1;
#[doc = "SDRAM control register 2"]
pub struct SDRAMCR2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SDRAM control register 2"]
pub mod sdramcr2;
#[doc = "SDRAM control register 3"]
pub struct SDRAMCR3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SDRAM control register 3"]
pub mod sdramcr3;
#[doc = "NAND control register 0"]
pub struct NANDCR0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "NAND control register 0"]
pub mod nandcr0;
#[doc = "NAND control register 1"]
pub struct NANDCR1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "NAND control register 1"]
pub mod nandcr1;
#[doc = "NAND control register 2"]
pub struct NANDCR2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "NAND control register 2"]
pub mod nandcr2;
#[doc = "NAND control register 3"]
pub struct NANDCR3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "NAND control register 3"]
pub mod nandcr3;
#[doc = "NOR control register 0"]
pub struct NORCR0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "NOR control register 0"]
pub mod norcr0;
#[doc = "NOR control register 1"]
pub struct NORCR1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "NOR control register 1"]
pub mod norcr1;
#[doc = "NOR control register 2"]
pub struct NORCR2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "NOR control register 2"]
pub mod norcr2;
#[doc = "NOR control register 3"]
pub struct NORCR3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "NOR control register 3"]
pub mod norcr3;
#[doc = "SRAM control register 0"]
pub struct SRAMCR0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SRAM control register 0"]
pub mod sramcr0;
#[doc = "SRAM control register 1"]
pub struct SRAMCR1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SRAM control register 1"]
pub mod sramcr1;
#[doc = "SRAM control register 2"]
pub struct SRAMCR2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SRAM control register 2"]
pub mod sramcr2;
#[doc = "SRAM control register 3"]
pub struct SRAMCR3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SRAM control register 3"]
pub mod sramcr3;
#[doc = "DBI-B control register 0"]
pub struct DBICR0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DBI-B control register 0"]
pub mod dbicr0;
#[doc = "DBI-B control register 1"]
pub struct DBICR1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DBI-B control register 1"]
pub mod dbicr1;
#[doc = "IP Command control register 0"]
pub struct IPCR0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "IP Command control register 0"]
pub mod ipcr0;
#[doc = "IP Command control register 1"]
pub struct IPCR1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "IP Command control register 1"]
pub mod ipcr1;
#[doc = "IP Command control register 2"]
pub struct IPCR2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "IP Command control register 2"]
pub mod ipcr2;
#[doc = "IP Command register"]
pub struct IPCMD {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "IP Command register"]
pub mod ipcmd;
#[doc = "TX DATA register (for IP Command)"]
pub struct IPTXDAT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "TX DATA register (for IP Command)"]
pub mod iptxdat;
#[doc = "RX DATA register (for IP Command)"]
pub struct IPRXDAT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "RX DATA register (for IP Command)"]
pub mod iprxdat;
#[doc = "Status register 0"]
pub struct STS0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Status register 0"]
pub mod sts0;
#[doc = "Status register 1"]
pub struct STS1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Status register 1"]
pub mod sts1;
#[doc = "Status register 2"]
pub struct STS2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Status register 2"]
pub mod sts2;
#[doc = "Status register 3"]
pub struct STS3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Status register 3"]
pub mod sts3;
#[doc = "Status register 4"]
pub struct STS4 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Status register 4"]
pub mod sts4;
#[doc = "Status register 5"]
pub struct STS5 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Status register 5"]
pub mod sts5;
#[doc = "Status register 6"]
pub struct STS6 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Status register 6"]
pub mod sts6;
#[doc = "Status register 7"]
pub struct STS7 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Status register 7"]
pub mod sts7;
#[doc = "Status register 8"]
pub struct STS8 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Status register 8"]
pub mod sts8;
#[doc = "Status register 9"]
pub struct STS9 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Status register 9"]
pub mod sts9;
#[doc = "Status register 10"]
pub struct STS10 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Status register 10"]
pub mod sts10;
#[doc = "Status register 11"]
pub struct STS11 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Status register 11"]
pub mod sts11;
#[doc = "Status register 12"]
pub struct STS12 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Status register 12"]
pub mod sts12;
#[doc = "Status register 13"]
pub struct STS13 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Status register 13"]
pub mod sts13;
#[doc = "Status register 14"]
pub struct STS14 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Status register 14"]
pub mod sts14;
#[doc = "Status register 15"]
pub struct STS15 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Status register 15"]
pub mod sts15;
