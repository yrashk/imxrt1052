#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Identification register"]
    pub id: ID,
    #[doc = "0x04 - Hardware General"]
    pub hwgeneral: HWGENERAL,
    #[doc = "0x08 - Host Hardware Parameters"]
    pub hwhost: HWHOST,
    #[doc = "0x0c - Device Hardware Parameters"]
    pub hwdevice: HWDEVICE,
    #[doc = "0x10 - TX Buffer Hardware Parameters"]
    pub hwtxbuf: HWTXBUF,
    #[doc = "0x14 - RX Buffer Hardware Parameters"]
    pub hwrxbuf: HWRXBUF,
    _reserved0: [u8; 104usize],
    #[doc = "0x80 - General Purpose Timer #0 Load"]
    pub gptimer0ld: GPTIMER0LD,
    #[doc = "0x84 - General Purpose Timer #0 Controller"]
    pub gptimer0ctrl: GPTIMER0CTRL,
    #[doc = "0x88 - General Purpose Timer #1 Load"]
    pub gptimer1ld: GPTIMER1LD,
    #[doc = "0x8c - General Purpose Timer #1 Controller"]
    pub gptimer1ctrl: GPTIMER1CTRL,
    #[doc = "0x90 - System Bus Config"]
    pub sbuscfg: SBUSCFG,
    _reserved1: [u8; 108usize],
    #[doc = "0x100 - Capability Registers Length"]
    pub caplength: CAPLENGTH,
    _reserved2: [u8; 1usize],
    #[doc = "0x102 - Host Controller Interface Version"]
    pub hciversion: HCIVERSION,
    #[doc = "0x104 - Host Controller Structural Parameters"]
    pub hcsparams: HCSPARAMS,
    #[doc = "0x108 - Host Controller Capability Parameters"]
    pub hccparams: HCCPARAMS,
    _reserved3: [u8; 20usize],
    #[doc = "0x120 - Device Controller Interface Version"]
    pub dciversion: DCIVERSION,
    _reserved4: [u8; 2usize],
    #[doc = "0x124 - Device Controller Capability Parameters"]
    pub dccparams: DCCPARAMS,
    _reserved5: [u8; 24usize],
    #[doc = "0x140 - USB Command Register"]
    pub usbcmd: USBCMD,
    #[doc = "0x144 - USB Status Register"]
    pub usbsts: USBSTS,
    #[doc = "0x148 - Interrupt Enable Register"]
    pub usbintr: USBINTR,
    #[doc = "0x14c - USB Frame Index"]
    pub frindex: FRINDEX,
    _reserved6: [u8; 4usize],
    #[doc = "0x154 - Device Address"]
    pub deviceaddr: DEVICEADDR,
    #[doc = "0x158 - Next Asynch. Address"]
    pub asynclistaddr: ASYNCLISTADDR,
    _reserved7: [u8; 4usize],
    #[doc = "0x160 - Programmable Burst Size"]
    pub burstsize: BURSTSIZE,
    #[doc = "0x164 - TX FIFO Fill Tuning"]
    pub txfilltuning: TXFILLTUNING,
    _reserved8: [u8; 16usize],
    #[doc = "0x178 - Endpoint NAK"]
    pub endptnak: ENDPTNAK,
    #[doc = "0x17c - Endpoint NAK Enable"]
    pub endptnaken: ENDPTNAKEN,
    #[doc = "0x180 - Configure Flag Register"]
    pub configflag: CONFIGFLAG,
    #[doc = "0x184 - Port Status & Control"]
    pub portsc1: PORTSC1,
    _reserved9: [u8; 28usize],
    #[doc = "0x1a4 - On-The-Go Status & control"]
    pub otgsc: OTGSC,
    #[doc = "0x1a8 - USB Device Mode"]
    pub usbmode: USBMODE,
    #[doc = "0x1ac - Endpoint Setup Status"]
    pub endptsetupstat: ENDPTSETUPSTAT,
    #[doc = "0x1b0 - Endpoint Prime"]
    pub endptprime: ENDPTPRIME,
    #[doc = "0x1b4 - Endpoint Flush"]
    pub endptflush: ENDPTFLUSH,
    #[doc = "0x1b8 - Endpoint Status"]
    pub endptstat: ENDPTSTAT,
    #[doc = "0x1bc - Endpoint Complete"]
    pub endptcomplete: ENDPTCOMPLETE,
    #[doc = "0x1c0 - Endpoint Control0"]
    pub endptctrl0: ENDPTCTRL0,
    #[doc = "0x1c4 - Endpoint Control 1"]
    pub endptctrl1: ENDPTCTRL1,
    #[doc = "0x1c8 - Endpoint Control 2"]
    pub endptctrl2: ENDPTCTRL2,
    #[doc = "0x1cc - Endpoint Control 3"]
    pub endptctrl3: ENDPTCTRL3,
    #[doc = "0x1d0 - Endpoint Control 4"]
    pub endptctrl4: ENDPTCTRL4,
    #[doc = "0x1d4 - Endpoint Control 5"]
    pub endptctrl5: ENDPTCTRL5,
    #[doc = "0x1d8 - Endpoint Control 6"]
    pub endptctrl6: ENDPTCTRL6,
    #[doc = "0x1dc - Endpoint Control 7"]
    pub endptctrl7: ENDPTCTRL7,
}
#[doc = "Identification register"]
pub struct ID {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Identification register"]
pub mod id;
#[doc = "Hardware General"]
pub struct HWGENERAL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Hardware General"]
pub mod hwgeneral;
#[doc = "Host Hardware Parameters"]
pub struct HWHOST {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Host Hardware Parameters"]
pub mod hwhost;
#[doc = "Device Hardware Parameters"]
pub struct HWDEVICE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Device Hardware Parameters"]
pub mod hwdevice;
#[doc = "TX Buffer Hardware Parameters"]
pub struct HWTXBUF {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "TX Buffer Hardware Parameters"]
pub mod hwtxbuf;
#[doc = "RX Buffer Hardware Parameters"]
pub struct HWRXBUF {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "RX Buffer Hardware Parameters"]
pub mod hwrxbuf;
#[doc = "General Purpose Timer #0 Load"]
pub struct GPTIMER0LD {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "General Purpose Timer #0 Load"]
pub mod gptimer0ld;
#[doc = "General Purpose Timer #0 Controller"]
pub struct GPTIMER0CTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "General Purpose Timer #0 Controller"]
pub mod gptimer0ctrl;
#[doc = "General Purpose Timer #1 Load"]
pub struct GPTIMER1LD {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "General Purpose Timer #1 Load"]
pub mod gptimer1ld;
#[doc = "General Purpose Timer #1 Controller"]
pub struct GPTIMER1CTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "General Purpose Timer #1 Controller"]
pub mod gptimer1ctrl;
#[doc = "System Bus Config"]
pub struct SBUSCFG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "System Bus Config"]
pub mod sbuscfg;
#[doc = "Capability Registers Length"]
pub struct CAPLENGTH {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "Capability Registers Length"]
pub mod caplength;
#[doc = "Host Controller Interface Version"]
pub struct HCIVERSION {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Host Controller Interface Version"]
pub mod hciversion;
#[doc = "Host Controller Structural Parameters"]
pub struct HCSPARAMS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Host Controller Structural Parameters"]
pub mod hcsparams;
#[doc = "Host Controller Capability Parameters"]
pub struct HCCPARAMS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Host Controller Capability Parameters"]
pub mod hccparams;
#[doc = "Device Controller Interface Version"]
pub struct DCIVERSION {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Device Controller Interface Version"]
pub mod dciversion;
#[doc = "Device Controller Capability Parameters"]
pub struct DCCPARAMS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Device Controller Capability Parameters"]
pub mod dccparams;
#[doc = "USB Command Register"]
pub struct USBCMD {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "USB Command Register"]
pub mod usbcmd;
#[doc = "USB Status Register"]
pub struct USBSTS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "USB Status Register"]
pub mod usbsts;
#[doc = "Interrupt Enable Register"]
pub struct USBINTR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt Enable Register"]
pub mod usbintr;
#[doc = "USB Frame Index"]
pub struct FRINDEX {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "USB Frame Index"]
pub mod frindex;
#[doc = "Device Address"]
pub struct DEVICEADDR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Device Address"]
pub mod deviceaddr;
#[doc = "Frame List Base Address"]
pub struct PERIODICLISTBASE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Frame List Base Address"]
pub mod periodiclistbase;
#[doc = "Next Asynch. Address"]
pub struct ASYNCLISTADDR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Next Asynch. Address"]
pub mod asynclistaddr;
#[doc = "Endpoint List Address"]
pub struct ENDPTLISTADDR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Endpoint List Address"]
pub mod endptlistaddr;
#[doc = "Programmable Burst Size"]
pub struct BURSTSIZE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Programmable Burst Size"]
pub mod burstsize;
#[doc = "TX FIFO Fill Tuning"]
pub struct TXFILLTUNING {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "TX FIFO Fill Tuning"]
pub mod txfilltuning;
#[doc = "Endpoint NAK"]
pub struct ENDPTNAK {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Endpoint NAK"]
pub mod endptnak;
#[doc = "Endpoint NAK Enable"]
pub struct ENDPTNAKEN {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Endpoint NAK Enable"]
pub mod endptnaken;
#[doc = "Configure Flag Register"]
pub struct CONFIGFLAG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Configure Flag Register"]
pub mod configflag;
#[doc = "Port Status & Control"]
pub struct PORTSC1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Port Status & Control"]
pub mod portsc1;
#[doc = "On-The-Go Status & control"]
pub struct OTGSC {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "On-The-Go Status & control"]
pub mod otgsc;
#[doc = "USB Device Mode"]
pub struct USBMODE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "USB Device Mode"]
pub mod usbmode;
#[doc = "Endpoint Setup Status"]
pub struct ENDPTSETUPSTAT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Endpoint Setup Status"]
pub mod endptsetupstat;
#[doc = "Endpoint Prime"]
pub struct ENDPTPRIME {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Endpoint Prime"]
pub mod endptprime;
#[doc = "Endpoint Flush"]
pub struct ENDPTFLUSH {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Endpoint Flush"]
pub mod endptflush;
#[doc = "Endpoint Status"]
pub struct ENDPTSTAT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Endpoint Status"]
pub mod endptstat;
#[doc = "Endpoint Complete"]
pub struct ENDPTCOMPLETE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Endpoint Complete"]
pub mod endptcomplete;
#[doc = "Endpoint Control0"]
pub struct ENDPTCTRL0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Endpoint Control0"]
pub mod endptctrl0;
#[doc = "Endpoint Control 1"]
pub struct ENDPTCTRL1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Endpoint Control 1"]
pub mod endptctrl1;
#[doc = "Endpoint Control 2"]
pub struct ENDPTCTRL2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Endpoint Control 2"]
pub mod endptctrl2;
#[doc = "Endpoint Control 3"]
pub struct ENDPTCTRL3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Endpoint Control 3"]
pub mod endptctrl3;
#[doc = "Endpoint Control 4"]
pub struct ENDPTCTRL4 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Endpoint Control 4"]
pub mod endptctrl4;
#[doc = "Endpoint Control 5"]
pub struct ENDPTCTRL5 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Endpoint Control 5"]
pub mod endptctrl5;
#[doc = "Endpoint Control 6"]
pub struct ENDPTCTRL6 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Endpoint Control 6"]
pub mod endptctrl6;
#[doc = "Endpoint Control 7"]
pub struct ENDPTCTRL7 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Endpoint Control 7"]
pub mod endptctrl7;
