#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 336usize],
    #[doc = "0x150 - Miscellaneous Register 0"]
    pub misc0: MISC0,
    #[doc = "0x154 - Miscellaneous Register 0"]
    pub misc0_set: MISC0_SET,
    #[doc = "0x158 - Miscellaneous Register 0"]
    pub misc0_clr: MISC0_CLR,
    #[doc = "0x15c - Miscellaneous Register 0"]
    pub misc0_tog: MISC0_TOG,
    _reserved1: [u8; 272usize],
    #[doc = "0x270 - XTAL OSC (LP) Control Register"]
    pub lowpwr_ctrl: LOWPWR_CTRL,
    #[doc = "0x274 - XTAL OSC (LP) Control Register"]
    pub lowpwr_ctrl_set: LOWPWR_CTRL_SET,
    #[doc = "0x278 - XTAL OSC (LP) Control Register"]
    pub lowpwr_ctrl_clr: LOWPWR_CTRL_CLR,
    #[doc = "0x27c - XTAL OSC (LP) Control Register"]
    pub lowpwr_ctrl_tog: LOWPWR_CTRL_TOG,
    _reserved2: [u8; 32usize],
    #[doc = "0x2a0 - XTAL OSC Configuration 0 Register"]
    pub osc_config0: OSC_CONFIG0,
    #[doc = "0x2a4 - XTAL OSC Configuration 0 Register"]
    pub osc_config0_set: OSC_CONFIG0_SET,
    #[doc = "0x2a8 - XTAL OSC Configuration 0 Register"]
    pub osc_config0_clr: OSC_CONFIG0_CLR,
    #[doc = "0x2ac - XTAL OSC Configuration 0 Register"]
    pub osc_config0_tog: OSC_CONFIG0_TOG,
    #[doc = "0x2b0 - XTAL OSC Configuration 1 Register"]
    pub osc_config1: OSC_CONFIG1,
    #[doc = "0x2b4 - XTAL OSC Configuration 1 Register"]
    pub osc_config1_set: OSC_CONFIG1_SET,
    #[doc = "0x2b8 - XTAL OSC Configuration 1 Register"]
    pub osc_config1_clr: OSC_CONFIG1_CLR,
    #[doc = "0x2bc - XTAL OSC Configuration 1 Register"]
    pub osc_config1_tog: OSC_CONFIG1_TOG,
    #[doc = "0x2c0 - XTAL OSC Configuration 2 Register"]
    pub osc_config2: OSC_CONFIG2,
    #[doc = "0x2c4 - XTAL OSC Configuration 2 Register"]
    pub osc_config2_set: OSC_CONFIG2_SET,
    #[doc = "0x2c8 - XTAL OSC Configuration 2 Register"]
    pub osc_config2_clr: OSC_CONFIG2_CLR,
    #[doc = "0x2cc - XTAL OSC Configuration 2 Register"]
    pub osc_config2_tog: OSC_CONFIG2_TOG,
}
#[doc = "Miscellaneous Register 0"]
pub struct MISC0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Miscellaneous Register 0"]
pub mod misc0;
#[doc = "Miscellaneous Register 0"]
pub struct MISC0_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Miscellaneous Register 0"]
pub mod misc0_set;
#[doc = "Miscellaneous Register 0"]
pub struct MISC0_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Miscellaneous Register 0"]
pub mod misc0_clr;
#[doc = "Miscellaneous Register 0"]
pub struct MISC0_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Miscellaneous Register 0"]
pub mod misc0_tog;
#[doc = "XTAL OSC (LP) Control Register"]
pub struct LOWPWR_CTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "XTAL OSC (LP) Control Register"]
pub mod lowpwr_ctrl;
#[doc = "XTAL OSC (LP) Control Register"]
pub struct LOWPWR_CTRL_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "XTAL OSC (LP) Control Register"]
pub mod lowpwr_ctrl_set;
#[doc = "XTAL OSC (LP) Control Register"]
pub struct LOWPWR_CTRL_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "XTAL OSC (LP) Control Register"]
pub mod lowpwr_ctrl_clr;
#[doc = "XTAL OSC (LP) Control Register"]
pub struct LOWPWR_CTRL_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "XTAL OSC (LP) Control Register"]
pub mod lowpwr_ctrl_tog;
#[doc = "XTAL OSC Configuration 0 Register"]
pub struct OSC_CONFIG0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "XTAL OSC Configuration 0 Register"]
pub mod osc_config0;
#[doc = "XTAL OSC Configuration 0 Register"]
pub struct OSC_CONFIG0_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "XTAL OSC Configuration 0 Register"]
pub mod osc_config0_set;
#[doc = "XTAL OSC Configuration 0 Register"]
pub struct OSC_CONFIG0_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "XTAL OSC Configuration 0 Register"]
pub mod osc_config0_clr;
#[doc = "XTAL OSC Configuration 0 Register"]
pub struct OSC_CONFIG0_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "XTAL OSC Configuration 0 Register"]
pub mod osc_config0_tog;
#[doc = "XTAL OSC Configuration 1 Register"]
pub struct OSC_CONFIG1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "XTAL OSC Configuration 1 Register"]
pub mod osc_config1;
#[doc = "XTAL OSC Configuration 1 Register"]
pub struct OSC_CONFIG1_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "XTAL OSC Configuration 1 Register"]
pub mod osc_config1_set;
#[doc = "XTAL OSC Configuration 1 Register"]
pub struct OSC_CONFIG1_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "XTAL OSC Configuration 1 Register"]
pub mod osc_config1_clr;
#[doc = "XTAL OSC Configuration 1 Register"]
pub struct OSC_CONFIG1_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "XTAL OSC Configuration 1 Register"]
pub mod osc_config1_tog;
#[doc = "XTAL OSC Configuration 2 Register"]
pub struct OSC_CONFIG2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "XTAL OSC Configuration 2 Register"]
pub mod osc_config2;
#[doc = "XTAL OSC Configuration 2 Register"]
pub struct OSC_CONFIG2_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "XTAL OSC Configuration 2 Register"]
pub mod osc_config2_set;
#[doc = "XTAL OSC Configuration 2 Register"]
pub struct OSC_CONFIG2_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "XTAL OSC Configuration 2 Register"]
pub mod osc_config2_clr;
#[doc = "XTAL OSC Configuration 2 Register"]
pub struct OSC_CONFIG2_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "XTAL OSC Configuration 2 Register"]
pub mod osc_config2_tog;
