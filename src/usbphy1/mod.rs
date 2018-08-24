#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - USB PHY Power-Down Register"]
    pub pwd: PWD,
    #[doc = "0x04 - USB PHY Power-Down Register"]
    pub pwd_set: PWD_SET,
    #[doc = "0x08 - USB PHY Power-Down Register"]
    pub pwd_clr: PWD_CLR,
    #[doc = "0x0c - USB PHY Power-Down Register"]
    pub pwd_tog: PWD_TOG,
    #[doc = "0x10 - USB PHY Transmitter Control Register"]
    pub tx: TX,
    #[doc = "0x14 - USB PHY Transmitter Control Register"]
    pub tx_set: TX_SET,
    #[doc = "0x18 - USB PHY Transmitter Control Register"]
    pub tx_clr: TX_CLR,
    #[doc = "0x1c - USB PHY Transmitter Control Register"]
    pub tx_tog: TX_TOG,
    #[doc = "0x20 - USB PHY Receiver Control Register"]
    pub rx: RX,
    #[doc = "0x24 - USB PHY Receiver Control Register"]
    pub rx_set: RX_SET,
    #[doc = "0x28 - USB PHY Receiver Control Register"]
    pub rx_clr: RX_CLR,
    #[doc = "0x2c - USB PHY Receiver Control Register"]
    pub rx_tog: RX_TOG,
    #[doc = "0x30 - USB PHY General Control Register"]
    pub ctrl: CTRL,
    #[doc = "0x34 - USB PHY General Control Register"]
    pub ctrl_set: CTRL_SET,
    #[doc = "0x38 - USB PHY General Control Register"]
    pub ctrl_clr: CTRL_CLR,
    #[doc = "0x3c - USB PHY General Control Register"]
    pub ctrl_tog: CTRL_TOG,
    #[doc = "0x40 - USB PHY Status Register"]
    pub status: STATUS,
    _reserved0: [u8; 12usize],
    #[doc = "0x50 - USB PHY Debug Register"]
    pub debug: DEBUG,
    #[doc = "0x54 - USB PHY Debug Register"]
    pub debug_set: DEBUG_SET,
    #[doc = "0x58 - USB PHY Debug Register"]
    pub debug_clr: DEBUG_CLR,
    #[doc = "0x5c - USB PHY Debug Register"]
    pub debug_tog: DEBUG_TOG,
    #[doc = "0x60 - UTMI Debug Status Register 0"]
    pub debug0_status: DEBUG0_STATUS,
    _reserved1: [u8; 12usize],
    #[doc = "0x70 - UTMI Debug Status Register 1"]
    pub debug1: DEBUG1,
    #[doc = "0x74 - UTMI Debug Status Register 1"]
    pub debug1_set: DEBUG1_SET,
    #[doc = "0x78 - UTMI Debug Status Register 1"]
    pub debug1_clr: DEBUG1_CLR,
    #[doc = "0x7c - UTMI Debug Status Register 1"]
    pub debug1_tog: DEBUG1_TOG,
    #[doc = "0x80 - UTMI RTL Version"]
    pub version: VERSION,
}
#[doc = "USB PHY Power-Down Register"]
pub struct PWD {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "USB PHY Power-Down Register"]
pub mod pwd;
#[doc = "USB PHY Power-Down Register"]
pub struct PWD_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "USB PHY Power-Down Register"]
pub mod pwd_set;
#[doc = "USB PHY Power-Down Register"]
pub struct PWD_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "USB PHY Power-Down Register"]
pub mod pwd_clr;
#[doc = "USB PHY Power-Down Register"]
pub struct PWD_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "USB PHY Power-Down Register"]
pub mod pwd_tog;
#[doc = "USB PHY Transmitter Control Register"]
pub struct TX {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "USB PHY Transmitter Control Register"]
pub mod tx;
#[doc = "USB PHY Transmitter Control Register"]
pub struct TX_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "USB PHY Transmitter Control Register"]
pub mod tx_set;
#[doc = "USB PHY Transmitter Control Register"]
pub struct TX_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "USB PHY Transmitter Control Register"]
pub mod tx_clr;
#[doc = "USB PHY Transmitter Control Register"]
pub struct TX_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "USB PHY Transmitter Control Register"]
pub mod tx_tog;
#[doc = "USB PHY Receiver Control Register"]
pub struct RX {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "USB PHY Receiver Control Register"]
pub mod rx;
#[doc = "USB PHY Receiver Control Register"]
pub struct RX_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "USB PHY Receiver Control Register"]
pub mod rx_set;
#[doc = "USB PHY Receiver Control Register"]
pub struct RX_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "USB PHY Receiver Control Register"]
pub mod rx_clr;
#[doc = "USB PHY Receiver Control Register"]
pub struct RX_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "USB PHY Receiver Control Register"]
pub mod rx_tog;
#[doc = "USB PHY General Control Register"]
pub struct CTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "USB PHY General Control Register"]
pub mod ctrl;
#[doc = "USB PHY General Control Register"]
pub struct CTRL_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "USB PHY General Control Register"]
pub mod ctrl_set;
#[doc = "USB PHY General Control Register"]
pub struct CTRL_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "USB PHY General Control Register"]
pub mod ctrl_clr;
#[doc = "USB PHY General Control Register"]
pub struct CTRL_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "USB PHY General Control Register"]
pub mod ctrl_tog;
#[doc = "USB PHY Status Register"]
pub struct STATUS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "USB PHY Status Register"]
pub mod status;
#[doc = "USB PHY Debug Register"]
pub struct DEBUG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "USB PHY Debug Register"]
pub mod debug;
#[doc = "USB PHY Debug Register"]
pub struct DEBUG_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "USB PHY Debug Register"]
pub mod debug_set;
#[doc = "USB PHY Debug Register"]
pub struct DEBUG_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "USB PHY Debug Register"]
pub mod debug_clr;
#[doc = "USB PHY Debug Register"]
pub struct DEBUG_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "USB PHY Debug Register"]
pub mod debug_tog;
#[doc = "UTMI Debug Status Register 0"]
pub struct DEBUG0_STATUS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "UTMI Debug Status Register 0"]
pub mod debug0_status;
#[doc = "UTMI Debug Status Register 1"]
pub struct DEBUG1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "UTMI Debug Status Register 1"]
pub mod debug1;
#[doc = "UTMI Debug Status Register 1"]
pub struct DEBUG1_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "UTMI Debug Status Register 1"]
pub mod debug1_set;
#[doc = "UTMI Debug Status Register 1"]
pub struct DEBUG1_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "UTMI Debug Status Register 1"]
pub mod debug1_clr;
#[doc = "UTMI Debug Status Register 1"]
pub struct DEBUG1_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "UTMI Debug Status Register 1"]
pub mod debug1_tog;
#[doc = "UTMI RTL Version"]
pub struct VERSION {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "UTMI RTL Version"]
pub mod version;
