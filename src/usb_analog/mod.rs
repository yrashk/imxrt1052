#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 416usize],
    #[doc = "0x1a0 - USB VBUS Detect Register"]
    pub usb1_vbus_detect: USB1_VBUS_DETECT,
    #[doc = "0x1a4 - USB VBUS Detect Register"]
    pub usb1_vbus_detect_set: USB1_VBUS_DETECT_SET,
    #[doc = "0x1a8 - USB VBUS Detect Register"]
    pub usb1_vbus_detect_clr: USB1_VBUS_DETECT_CLR,
    #[doc = "0x1ac - USB VBUS Detect Register"]
    pub usb1_vbus_detect_tog: USB1_VBUS_DETECT_TOG,
    #[doc = "0x1b0 - USB Charger Detect Register"]
    pub usb1_chrg_detect: USB1_CHRG_DETECT,
    #[doc = "0x1b4 - USB Charger Detect Register"]
    pub usb1_chrg_detect_set: USB1_CHRG_DETECT_SET,
    #[doc = "0x1b8 - USB Charger Detect Register"]
    pub usb1_chrg_detect_clr: USB1_CHRG_DETECT_CLR,
    #[doc = "0x1bc - USB Charger Detect Register"]
    pub usb1_chrg_detect_tog: USB1_CHRG_DETECT_TOG,
    #[doc = "0x1c0 - USB VBUS Detect Status Register"]
    pub usb1_vbus_detect_stat: USB1_VBUS_DETECT_STAT,
    _reserved1: [u8; 12usize],
    #[doc = "0x1d0 - USB Charger Detect Status Register"]
    pub usb1_chrg_detect_stat: USB1_CHRG_DETECT_STAT,
    _reserved2: [u8; 28usize],
    #[doc = "0x1f0 - USB Misc Register"]
    pub usb1_misc: USB1_MISC,
    #[doc = "0x1f4 - USB Misc Register"]
    pub usb1_misc_set: USB1_MISC_SET,
    #[doc = "0x1f8 - USB Misc Register"]
    pub usb1_misc_clr: USB1_MISC_CLR,
    #[doc = "0x1fc - USB Misc Register"]
    pub usb1_misc_tog: USB1_MISC_TOG,
    #[doc = "0x200 - USB VBUS Detect Register"]
    pub usb2_vbus_detect: USB2_VBUS_DETECT,
    #[doc = "0x204 - USB VBUS Detect Register"]
    pub usb2_vbus_detect_set: USB2_VBUS_DETECT_SET,
    #[doc = "0x208 - USB VBUS Detect Register"]
    pub usb2_vbus_detect_clr: USB2_VBUS_DETECT_CLR,
    #[doc = "0x20c - USB VBUS Detect Register"]
    pub usb2_vbus_detect_tog: USB2_VBUS_DETECT_TOG,
    #[doc = "0x210 - USB Charger Detect Register"]
    pub usb2_chrg_detect: USB2_CHRG_DETECT,
    #[doc = "0x214 - USB Charger Detect Register"]
    pub usb2_chrg_detect_set: USB2_CHRG_DETECT_SET,
    #[doc = "0x218 - USB Charger Detect Register"]
    pub usb2_chrg_detect_clr: USB2_CHRG_DETECT_CLR,
    #[doc = "0x21c - USB Charger Detect Register"]
    pub usb2_chrg_detect_tog: USB2_CHRG_DETECT_TOG,
    #[doc = "0x220 - USB VBUS Detect Status Register"]
    pub usb2_vbus_detect_stat: USB2_VBUS_DETECT_STAT,
    _reserved3: [u8; 12usize],
    #[doc = "0x230 - USB Charger Detect Status Register"]
    pub usb2_chrg_detect_stat: USB2_CHRG_DETECT_STAT,
    _reserved4: [u8; 28usize],
    #[doc = "0x250 - USB Misc Register"]
    pub usb2_misc: USB2_MISC,
    #[doc = "0x254 - USB Misc Register"]
    pub usb2_misc_set: USB2_MISC_SET,
    #[doc = "0x258 - USB Misc Register"]
    pub usb2_misc_clr: USB2_MISC_CLR,
    #[doc = "0x25c - USB Misc Register"]
    pub usb2_misc_tog: USB2_MISC_TOG,
    #[doc = "0x260 - Chip Silicon Version"]
    pub digprog: DIGPROG,
}
#[doc = "USB VBUS Detect Register"]
pub struct USB1_VBUS_DETECT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "USB VBUS Detect Register"]
pub mod usb1_vbus_detect;
#[doc = "USB VBUS Detect Register"]
pub struct USB1_VBUS_DETECT_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "USB VBUS Detect Register"]
pub mod usb1_vbus_detect_set;
#[doc = "USB VBUS Detect Register"]
pub struct USB1_VBUS_DETECT_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "USB VBUS Detect Register"]
pub mod usb1_vbus_detect_clr;
#[doc = "USB VBUS Detect Register"]
pub struct USB1_VBUS_DETECT_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "USB VBUS Detect Register"]
pub mod usb1_vbus_detect_tog;
#[doc = "USB Charger Detect Register"]
pub struct USB1_CHRG_DETECT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "USB Charger Detect Register"]
pub mod usb1_chrg_detect;
#[doc = "USB Charger Detect Register"]
pub struct USB1_CHRG_DETECT_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "USB Charger Detect Register"]
pub mod usb1_chrg_detect_set;
#[doc = "USB Charger Detect Register"]
pub struct USB1_CHRG_DETECT_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "USB Charger Detect Register"]
pub mod usb1_chrg_detect_clr;
#[doc = "USB Charger Detect Register"]
pub struct USB1_CHRG_DETECT_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "USB Charger Detect Register"]
pub mod usb1_chrg_detect_tog;
#[doc = "USB VBUS Detect Status Register"]
pub struct USB1_VBUS_DETECT_STAT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "USB VBUS Detect Status Register"]
pub mod usb1_vbus_detect_stat;
#[doc = "USB Charger Detect Status Register"]
pub struct USB1_CHRG_DETECT_STAT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "USB Charger Detect Status Register"]
pub mod usb1_chrg_detect_stat;
#[doc = "USB Misc Register"]
pub struct USB1_MISC {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "USB Misc Register"]
pub mod usb1_misc;
#[doc = "USB Misc Register"]
pub struct USB1_MISC_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "USB Misc Register"]
pub mod usb1_misc_set;
#[doc = "USB Misc Register"]
pub struct USB1_MISC_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "USB Misc Register"]
pub mod usb1_misc_clr;
#[doc = "USB Misc Register"]
pub struct USB1_MISC_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "USB Misc Register"]
pub mod usb1_misc_tog;
#[doc = "USB VBUS Detect Register"]
pub struct USB2_VBUS_DETECT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "USB VBUS Detect Register"]
pub mod usb2_vbus_detect;
#[doc = "USB VBUS Detect Register"]
pub struct USB2_VBUS_DETECT_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "USB VBUS Detect Register"]
pub mod usb2_vbus_detect_set;
#[doc = "USB VBUS Detect Register"]
pub struct USB2_VBUS_DETECT_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "USB VBUS Detect Register"]
pub mod usb2_vbus_detect_clr;
#[doc = "USB VBUS Detect Register"]
pub struct USB2_VBUS_DETECT_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "USB VBUS Detect Register"]
pub mod usb2_vbus_detect_tog;
#[doc = "USB Charger Detect Register"]
pub struct USB2_CHRG_DETECT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "USB Charger Detect Register"]
pub mod usb2_chrg_detect;
#[doc = "USB Charger Detect Register"]
pub struct USB2_CHRG_DETECT_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "USB Charger Detect Register"]
pub mod usb2_chrg_detect_set;
#[doc = "USB Charger Detect Register"]
pub struct USB2_CHRG_DETECT_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "USB Charger Detect Register"]
pub mod usb2_chrg_detect_clr;
#[doc = "USB Charger Detect Register"]
pub struct USB2_CHRG_DETECT_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "USB Charger Detect Register"]
pub mod usb2_chrg_detect_tog;
#[doc = "USB VBUS Detect Status Register"]
pub struct USB2_VBUS_DETECT_STAT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "USB VBUS Detect Status Register"]
pub mod usb2_vbus_detect_stat;
#[doc = "USB Charger Detect Status Register"]
pub struct USB2_CHRG_DETECT_STAT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "USB Charger Detect Status Register"]
pub mod usb2_chrg_detect_stat;
#[doc = "USB Misc Register"]
pub struct USB2_MISC {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "USB Misc Register"]
pub mod usb2_misc;
#[doc = "USB Misc Register"]
pub struct USB2_MISC_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "USB Misc Register"]
pub mod usb2_misc_set;
#[doc = "USB Misc Register"]
pub struct USB2_MISC_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "USB Misc Register"]
pub mod usb2_misc_clr;
#[doc = "USB Misc Register"]
pub struct USB2_MISC_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "USB Misc Register"]
pub mod usb2_misc_tog;
#[doc = "Chip Silicon Version"]
pub struct DIGPROG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Chip Silicon Version"]
pub mod digprog;
