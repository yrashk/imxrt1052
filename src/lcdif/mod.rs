#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - eLCDIF General Control Register"]
    pub ctrl: CTRL,
    #[doc = "0x04 - eLCDIF General Control Register"]
    pub ctrl_set: CTRL_SET,
    #[doc = "0x08 - eLCDIF General Control Register"]
    pub ctrl_clr: CTRL_CLR,
    #[doc = "0x0c - eLCDIF General Control Register"]
    pub ctrl_tog: CTRL_TOG,
    #[doc = "0x10 - eLCDIF General Control1 Register"]
    pub ctrl1: CTRL1,
    #[doc = "0x14 - eLCDIF General Control1 Register"]
    pub ctrl1_set: CTRL1_SET,
    #[doc = "0x18 - eLCDIF General Control1 Register"]
    pub ctrl1_clr: CTRL1_CLR,
    #[doc = "0x1c - eLCDIF General Control1 Register"]
    pub ctrl1_tog: CTRL1_TOG,
    #[doc = "0x20 - eLCDIF General Control2 Register"]
    pub ctrl2: CTRL2,
    #[doc = "0x24 - eLCDIF General Control2 Register"]
    pub ctrl2_set: CTRL2_SET,
    #[doc = "0x28 - eLCDIF General Control2 Register"]
    pub ctrl2_clr: CTRL2_CLR,
    #[doc = "0x2c - eLCDIF General Control2 Register"]
    pub ctrl2_tog: CTRL2_TOG,
    #[doc = "0x30 - eLCDIF Horizontal and Vertical Valid Data Count Register"]
    pub transfer_count: TRANSFER_COUNT,
    _reserved0: [u8; 12usize],
    #[doc = "0x40 - LCD Interface Current Buffer Address Register"]
    pub cur_buf: CUR_BUF,
    _reserved1: [u8; 12usize],
    #[doc = "0x50 - LCD Interface Next Buffer Address Register"]
    pub next_buf: NEXT_BUF,
    _reserved2: [u8; 28usize],
    #[doc = "0x70 - eLCDIF VSYNC Mode and Dotclk Mode Control Register0"]
    pub vdctrl0: VDCTRL0,
    #[doc = "0x74 - eLCDIF VSYNC Mode and Dotclk Mode Control Register0"]
    pub vdctrl0_set: VDCTRL0_SET,
    #[doc = "0x78 - eLCDIF VSYNC Mode and Dotclk Mode Control Register0"]
    pub vdctrl0_clr: VDCTRL0_CLR,
    #[doc = "0x7c - eLCDIF VSYNC Mode and Dotclk Mode Control Register0"]
    pub vdctrl0_tog: VDCTRL0_TOG,
    #[doc = "0x80 - eLCDIF VSYNC Mode and Dotclk Mode Control Register1"]
    pub vdctrl1: VDCTRL1,
    _reserved3: [u8; 12usize],
    #[doc = "0x90 - LCDIF VSYNC Mode and Dotclk Mode Control Register2"]
    pub vdctrl2: VDCTRL2,
    _reserved4: [u8; 12usize],
    #[doc = "0xa0 - eLCDIF VSYNC Mode and Dotclk Mode Control Register3"]
    pub vdctrl3: VDCTRL3,
    _reserved5: [u8; 12usize],
    #[doc = "0xb0 - eLCDIF VSYNC Mode and Dotclk Mode Control Register4"]
    pub vdctrl4: VDCTRL4,
    _reserved6: [u8; 220usize],
    #[doc = "0x190 - Bus Master Error Status Register"]
    pub bm_error_stat: BM_ERROR_STAT,
    _reserved7: [u8; 12usize],
    #[doc = "0x1a0 - CRC Status Register"]
    pub crc_stat: CRC_STAT,
    _reserved8: [u8; 12usize],
    #[doc = "0x1b0 - LCD Interface Status Register"]
    pub stat: STAT,
    _reserved9: [u8; 76usize],
    #[doc = "0x200 - eLCDIF Threshold Register"]
    pub thres: THRES,
    _reserved10: [u8; 380usize],
    #[doc = "0x380 - LCDIF Pigeon Mode Control0 Register"]
    pub pigeonctrl0: PIGEONCTRL0,
    #[doc = "0x384 - LCDIF Pigeon Mode Control0 Register"]
    pub pigeonctrl0_set: PIGEONCTRL0_SET,
    #[doc = "0x388 - LCDIF Pigeon Mode Control0 Register"]
    pub pigeonctrl0_clr: PIGEONCTRL0_CLR,
    #[doc = "0x38c - LCDIF Pigeon Mode Control0 Register"]
    pub pigeonctrl0_tog: PIGEONCTRL0_TOG,
    #[doc = "0x390 - LCDIF Pigeon Mode Control1 Register"]
    pub pigeonctrl1: PIGEONCTRL1,
    #[doc = "0x394 - LCDIF Pigeon Mode Control1 Register"]
    pub pigeonctrl1_set: PIGEONCTRL1_SET,
    #[doc = "0x398 - LCDIF Pigeon Mode Control1 Register"]
    pub pigeonctrl1_clr: PIGEONCTRL1_CLR,
    #[doc = "0x39c - LCDIF Pigeon Mode Control1 Register"]
    pub pigeonctrl1_tog: PIGEONCTRL1_TOG,
    #[doc = "0x3a0 - LCDIF Pigeon Mode Control2 Register"]
    pub pigeonctrl2: PIGEONCTRL2,
    #[doc = "0x3a4 - LCDIF Pigeon Mode Control2 Register"]
    pub pigeonctrl2_set: PIGEONCTRL2_SET,
    #[doc = "0x3a8 - LCDIF Pigeon Mode Control2 Register"]
    pub pigeonctrl2_clr: PIGEONCTRL2_CLR,
    #[doc = "0x3ac - LCDIF Pigeon Mode Control2 Register"]
    pub pigeonctrl2_tog: PIGEONCTRL2_TOG,
    _reserved11: [u8; 1104usize],
    #[doc = "0x800 - Panel Interface Signal Generator Register"]
    pub pigeon_0_0: PIGEON_0_0,
    _reserved12: [u8; 12usize],
    #[doc = "0x810 - Panel Interface Signal Generator Register"]
    pub pigeon_0_1: PIGEON_0_1,
    _reserved13: [u8; 12usize],
    #[doc = "0x820 - Panel Interface Signal Generator Register"]
    pub pigeon_0_2: PIGEON_0_2,
    _reserved14: [u8; 28usize],
    #[doc = "0x840 - Panel Interface Signal Generator Register"]
    pub pigeon_1_0: PIGEON_1_0,
    _reserved15: [u8; 12usize],
    #[doc = "0x850 - Panel Interface Signal Generator Register"]
    pub pigeon_1_1: PIGEON_1_1,
    _reserved16: [u8; 12usize],
    #[doc = "0x860 - Panel Interface Signal Generator Register"]
    pub pigeon_1_2: PIGEON_1_2,
    _reserved17: [u8; 28usize],
    #[doc = "0x880 - Panel Interface Signal Generator Register"]
    pub pigeon_2_0: PIGEON_2_0,
    _reserved18: [u8; 12usize],
    #[doc = "0x890 - Panel Interface Signal Generator Register"]
    pub pigeon_2_1: PIGEON_2_1,
    _reserved19: [u8; 12usize],
    #[doc = "0x8a0 - Panel Interface Signal Generator Register"]
    pub pigeon_2_2: PIGEON_2_2,
    _reserved20: [u8; 28usize],
    #[doc = "0x8c0 - Panel Interface Signal Generator Register"]
    pub pigeon_3_0: PIGEON_3_0,
    _reserved21: [u8; 12usize],
    #[doc = "0x8d0 - Panel Interface Signal Generator Register"]
    pub pigeon_3_1: PIGEON_3_1,
    _reserved22: [u8; 12usize],
    #[doc = "0x8e0 - Panel Interface Signal Generator Register"]
    pub pigeon_3_2: PIGEON_3_2,
    _reserved23: [u8; 28usize],
    #[doc = "0x900 - Panel Interface Signal Generator Register"]
    pub pigeon_4_0: PIGEON_4_0,
    _reserved24: [u8; 12usize],
    #[doc = "0x910 - Panel Interface Signal Generator Register"]
    pub pigeon_4_1: PIGEON_4_1,
    _reserved25: [u8; 12usize],
    #[doc = "0x920 - Panel Interface Signal Generator Register"]
    pub pigeon_4_2: PIGEON_4_2,
    _reserved26: [u8; 28usize],
    #[doc = "0x940 - Panel Interface Signal Generator Register"]
    pub pigeon_5_0: PIGEON_5_0,
    _reserved27: [u8; 12usize],
    #[doc = "0x950 - Panel Interface Signal Generator Register"]
    pub pigeon_5_1: PIGEON_5_1,
    _reserved28: [u8; 12usize],
    #[doc = "0x960 - Panel Interface Signal Generator Register"]
    pub pigeon_5_2: PIGEON_5_2,
    _reserved29: [u8; 28usize],
    #[doc = "0x980 - Panel Interface Signal Generator Register"]
    pub pigeon_6_0: PIGEON_6_0,
    _reserved30: [u8; 12usize],
    #[doc = "0x990 - Panel Interface Signal Generator Register"]
    pub pigeon_6_1: PIGEON_6_1,
    _reserved31: [u8; 12usize],
    #[doc = "0x9a0 - Panel Interface Signal Generator Register"]
    pub pigeon_6_2: PIGEON_6_2,
    _reserved32: [u8; 28usize],
    #[doc = "0x9c0 - Panel Interface Signal Generator Register"]
    pub pigeon_7_0: PIGEON_7_0,
    _reserved33: [u8; 12usize],
    #[doc = "0x9d0 - Panel Interface Signal Generator Register"]
    pub pigeon_7_1: PIGEON_7_1,
    _reserved34: [u8; 12usize],
    #[doc = "0x9e0 - Panel Interface Signal Generator Register"]
    pub pigeon_7_2: PIGEON_7_2,
    _reserved35: [u8; 28usize],
    #[doc = "0xa00 - Panel Interface Signal Generator Register"]
    pub pigeon_8_0: PIGEON_8_0,
    _reserved36: [u8; 12usize],
    #[doc = "0xa10 - Panel Interface Signal Generator Register"]
    pub pigeon_8_1: PIGEON_8_1,
    _reserved37: [u8; 12usize],
    #[doc = "0xa20 - Panel Interface Signal Generator Register"]
    pub pigeon_8_2: PIGEON_8_2,
    _reserved38: [u8; 28usize],
    #[doc = "0xa40 - Panel Interface Signal Generator Register"]
    pub pigeon_9_0: PIGEON_9_0,
    _reserved39: [u8; 12usize],
    #[doc = "0xa50 - Panel Interface Signal Generator Register"]
    pub pigeon_9_1: PIGEON_9_1,
    _reserved40: [u8; 12usize],
    #[doc = "0xa60 - Panel Interface Signal Generator Register"]
    pub pigeon_9_2: PIGEON_9_2,
    _reserved41: [u8; 28usize],
    #[doc = "0xa80 - Panel Interface Signal Generator Register"]
    pub pigeon_10_0: PIGEON_10_0,
    _reserved42: [u8; 12usize],
    #[doc = "0xa90 - Panel Interface Signal Generator Register"]
    pub pigeon_10_1: PIGEON_10_1,
    _reserved43: [u8; 12usize],
    #[doc = "0xaa0 - Panel Interface Signal Generator Register"]
    pub pigeon_10_2: PIGEON_10_2,
    _reserved44: [u8; 28usize],
    #[doc = "0xac0 - Panel Interface Signal Generator Register"]
    pub pigeon_11_0: PIGEON_11_0,
    _reserved45: [u8; 12usize],
    #[doc = "0xad0 - Panel Interface Signal Generator Register"]
    pub pigeon_11_1: PIGEON_11_1,
    _reserved46: [u8; 12usize],
    #[doc = "0xae0 - Panel Interface Signal Generator Register"]
    pub pigeon_11_2: PIGEON_11_2,
    _reserved47: [u8; 28usize],
    #[doc = "0xb00 - Lookup Table Data Register."]
    pub lut_ctrl: LUT_CTRL,
    _reserved48: [u8; 12usize],
    #[doc = "0xb10 - Lookup Table Control Register."]
    pub lut0_addr: LUT0_ADDR,
    _reserved49: [u8; 12usize],
    #[doc = "0xb20 - Lookup Table Data Register."]
    pub lut0_data: LUT0_DATA,
    _reserved50: [u8; 12usize],
    #[doc = "0xb30 - Lookup Table Control Register."]
    pub lut1_addr: LUT1_ADDR,
    _reserved51: [u8; 12usize],
    #[doc = "0xb40 - Lookup Table Data Register."]
    pub lut1_data: LUT1_DATA,
}
#[doc = "eLCDIF General Control Register"]
pub struct CTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "eLCDIF General Control Register"]
pub mod ctrl;
#[doc = "eLCDIF General Control Register"]
pub struct CTRL_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "eLCDIF General Control Register"]
pub mod ctrl_set;
#[doc = "eLCDIF General Control Register"]
pub struct CTRL_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "eLCDIF General Control Register"]
pub mod ctrl_clr;
#[doc = "eLCDIF General Control Register"]
pub struct CTRL_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "eLCDIF General Control Register"]
pub mod ctrl_tog;
#[doc = "eLCDIF General Control1 Register"]
pub struct CTRL1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "eLCDIF General Control1 Register"]
pub mod ctrl1;
#[doc = "eLCDIF General Control1 Register"]
pub struct CTRL1_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "eLCDIF General Control1 Register"]
pub mod ctrl1_set;
#[doc = "eLCDIF General Control1 Register"]
pub struct CTRL1_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "eLCDIF General Control1 Register"]
pub mod ctrl1_clr;
#[doc = "eLCDIF General Control1 Register"]
pub struct CTRL1_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "eLCDIF General Control1 Register"]
pub mod ctrl1_tog;
#[doc = "eLCDIF General Control2 Register"]
pub struct CTRL2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "eLCDIF General Control2 Register"]
pub mod ctrl2;
#[doc = "eLCDIF General Control2 Register"]
pub struct CTRL2_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "eLCDIF General Control2 Register"]
pub mod ctrl2_set;
#[doc = "eLCDIF General Control2 Register"]
pub struct CTRL2_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "eLCDIF General Control2 Register"]
pub mod ctrl2_clr;
#[doc = "eLCDIF General Control2 Register"]
pub struct CTRL2_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "eLCDIF General Control2 Register"]
pub mod ctrl2_tog;
#[doc = "eLCDIF Horizontal and Vertical Valid Data Count Register"]
pub struct TRANSFER_COUNT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "eLCDIF Horizontal and Vertical Valid Data Count Register"]
pub mod transfer_count;
#[doc = "LCD Interface Current Buffer Address Register"]
pub struct CUR_BUF {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LCD Interface Current Buffer Address Register"]
pub mod cur_buf;
#[doc = "LCD Interface Next Buffer Address Register"]
pub struct NEXT_BUF {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LCD Interface Next Buffer Address Register"]
pub mod next_buf;
#[doc = "eLCDIF VSYNC Mode and Dotclk Mode Control Register0"]
pub struct VDCTRL0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "eLCDIF VSYNC Mode and Dotclk Mode Control Register0"]
pub mod vdctrl0;
#[doc = "eLCDIF VSYNC Mode and Dotclk Mode Control Register0"]
pub struct VDCTRL0_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "eLCDIF VSYNC Mode and Dotclk Mode Control Register0"]
pub mod vdctrl0_set;
#[doc = "eLCDIF VSYNC Mode and Dotclk Mode Control Register0"]
pub struct VDCTRL0_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "eLCDIF VSYNC Mode and Dotclk Mode Control Register0"]
pub mod vdctrl0_clr;
#[doc = "eLCDIF VSYNC Mode and Dotclk Mode Control Register0"]
pub struct VDCTRL0_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "eLCDIF VSYNC Mode and Dotclk Mode Control Register0"]
pub mod vdctrl0_tog;
#[doc = "eLCDIF VSYNC Mode and Dotclk Mode Control Register1"]
pub struct VDCTRL1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "eLCDIF VSYNC Mode and Dotclk Mode Control Register1"]
pub mod vdctrl1;
#[doc = "LCDIF VSYNC Mode and Dotclk Mode Control Register2"]
pub struct VDCTRL2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LCDIF VSYNC Mode and Dotclk Mode Control Register2"]
pub mod vdctrl2;
#[doc = "eLCDIF VSYNC Mode and Dotclk Mode Control Register3"]
pub struct VDCTRL3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "eLCDIF VSYNC Mode and Dotclk Mode Control Register3"]
pub mod vdctrl3;
#[doc = "eLCDIF VSYNC Mode and Dotclk Mode Control Register4"]
pub struct VDCTRL4 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "eLCDIF VSYNC Mode and Dotclk Mode Control Register4"]
pub mod vdctrl4;
#[doc = "Bus Master Error Status Register"]
pub struct BM_ERROR_STAT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Bus Master Error Status Register"]
pub mod bm_error_stat;
#[doc = "CRC Status Register"]
pub struct CRC_STAT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CRC Status Register"]
pub mod crc_stat;
#[doc = "LCD Interface Status Register"]
pub struct STAT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LCD Interface Status Register"]
pub mod stat;
#[doc = "eLCDIF Threshold Register"]
pub struct THRES {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "eLCDIF Threshold Register"]
pub mod thres;
#[doc = "LCDIF Pigeon Mode Control0 Register"]
pub struct PIGEONCTRL0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LCDIF Pigeon Mode Control0 Register"]
pub mod pigeonctrl0;
#[doc = "LCDIF Pigeon Mode Control0 Register"]
pub struct PIGEONCTRL0_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LCDIF Pigeon Mode Control0 Register"]
pub mod pigeonctrl0_set;
#[doc = "LCDIF Pigeon Mode Control0 Register"]
pub struct PIGEONCTRL0_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LCDIF Pigeon Mode Control0 Register"]
pub mod pigeonctrl0_clr;
#[doc = "LCDIF Pigeon Mode Control0 Register"]
pub struct PIGEONCTRL0_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LCDIF Pigeon Mode Control0 Register"]
pub mod pigeonctrl0_tog;
#[doc = "LCDIF Pigeon Mode Control1 Register"]
pub struct PIGEONCTRL1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LCDIF Pigeon Mode Control1 Register"]
pub mod pigeonctrl1;
#[doc = "LCDIF Pigeon Mode Control1 Register"]
pub struct PIGEONCTRL1_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LCDIF Pigeon Mode Control1 Register"]
pub mod pigeonctrl1_set;
#[doc = "LCDIF Pigeon Mode Control1 Register"]
pub struct PIGEONCTRL1_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LCDIF Pigeon Mode Control1 Register"]
pub mod pigeonctrl1_clr;
#[doc = "LCDIF Pigeon Mode Control1 Register"]
pub struct PIGEONCTRL1_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LCDIF Pigeon Mode Control1 Register"]
pub mod pigeonctrl1_tog;
#[doc = "LCDIF Pigeon Mode Control2 Register"]
pub struct PIGEONCTRL2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LCDIF Pigeon Mode Control2 Register"]
pub mod pigeonctrl2;
#[doc = "LCDIF Pigeon Mode Control2 Register"]
pub struct PIGEONCTRL2_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LCDIF Pigeon Mode Control2 Register"]
pub mod pigeonctrl2_set;
#[doc = "LCDIF Pigeon Mode Control2 Register"]
pub struct PIGEONCTRL2_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LCDIF Pigeon Mode Control2 Register"]
pub mod pigeonctrl2_clr;
#[doc = "LCDIF Pigeon Mode Control2 Register"]
pub struct PIGEONCTRL2_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LCDIF Pigeon Mode Control2 Register"]
pub mod pigeonctrl2_tog;
#[doc = "Panel Interface Signal Generator Register"]
pub struct PIGEON_0_0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Panel Interface Signal Generator Register"]
pub mod pigeon_0_0;
#[doc = "Panel Interface Signal Generator Register"]
pub struct PIGEON_0_1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Panel Interface Signal Generator Register"]
pub mod pigeon_0_1;
#[doc = "Panel Interface Signal Generator Register"]
pub struct PIGEON_0_2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Panel Interface Signal Generator Register"]
pub mod pigeon_0_2;
#[doc = "Panel Interface Signal Generator Register"]
pub struct PIGEON_1_0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Panel Interface Signal Generator Register"]
pub mod pigeon_1_0;
#[doc = "Panel Interface Signal Generator Register"]
pub struct PIGEON_1_1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Panel Interface Signal Generator Register"]
pub mod pigeon_1_1;
#[doc = "Panel Interface Signal Generator Register"]
pub struct PIGEON_1_2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Panel Interface Signal Generator Register"]
pub mod pigeon_1_2;
#[doc = "Panel Interface Signal Generator Register"]
pub struct PIGEON_2_0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Panel Interface Signal Generator Register"]
pub mod pigeon_2_0;
#[doc = "Panel Interface Signal Generator Register"]
pub struct PIGEON_2_1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Panel Interface Signal Generator Register"]
pub mod pigeon_2_1;
#[doc = "Panel Interface Signal Generator Register"]
pub struct PIGEON_2_2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Panel Interface Signal Generator Register"]
pub mod pigeon_2_2;
#[doc = "Panel Interface Signal Generator Register"]
pub struct PIGEON_3_0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Panel Interface Signal Generator Register"]
pub mod pigeon_3_0;
#[doc = "Panel Interface Signal Generator Register"]
pub struct PIGEON_3_1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Panel Interface Signal Generator Register"]
pub mod pigeon_3_1;
#[doc = "Panel Interface Signal Generator Register"]
pub struct PIGEON_3_2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Panel Interface Signal Generator Register"]
pub mod pigeon_3_2;
#[doc = "Panel Interface Signal Generator Register"]
pub struct PIGEON_4_0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Panel Interface Signal Generator Register"]
pub mod pigeon_4_0;
#[doc = "Panel Interface Signal Generator Register"]
pub struct PIGEON_4_1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Panel Interface Signal Generator Register"]
pub mod pigeon_4_1;
#[doc = "Panel Interface Signal Generator Register"]
pub struct PIGEON_4_2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Panel Interface Signal Generator Register"]
pub mod pigeon_4_2;
#[doc = "Panel Interface Signal Generator Register"]
pub struct PIGEON_5_0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Panel Interface Signal Generator Register"]
pub mod pigeon_5_0;
#[doc = "Panel Interface Signal Generator Register"]
pub struct PIGEON_5_1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Panel Interface Signal Generator Register"]
pub mod pigeon_5_1;
#[doc = "Panel Interface Signal Generator Register"]
pub struct PIGEON_5_2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Panel Interface Signal Generator Register"]
pub mod pigeon_5_2;
#[doc = "Panel Interface Signal Generator Register"]
pub struct PIGEON_6_0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Panel Interface Signal Generator Register"]
pub mod pigeon_6_0;
#[doc = "Panel Interface Signal Generator Register"]
pub struct PIGEON_6_1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Panel Interface Signal Generator Register"]
pub mod pigeon_6_1;
#[doc = "Panel Interface Signal Generator Register"]
pub struct PIGEON_6_2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Panel Interface Signal Generator Register"]
pub mod pigeon_6_2;
#[doc = "Panel Interface Signal Generator Register"]
pub struct PIGEON_7_0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Panel Interface Signal Generator Register"]
pub mod pigeon_7_0;
#[doc = "Panel Interface Signal Generator Register"]
pub struct PIGEON_7_1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Panel Interface Signal Generator Register"]
pub mod pigeon_7_1;
#[doc = "Panel Interface Signal Generator Register"]
pub struct PIGEON_7_2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Panel Interface Signal Generator Register"]
pub mod pigeon_7_2;
#[doc = "Panel Interface Signal Generator Register"]
pub struct PIGEON_8_0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Panel Interface Signal Generator Register"]
pub mod pigeon_8_0;
#[doc = "Panel Interface Signal Generator Register"]
pub struct PIGEON_8_1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Panel Interface Signal Generator Register"]
pub mod pigeon_8_1;
#[doc = "Panel Interface Signal Generator Register"]
pub struct PIGEON_8_2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Panel Interface Signal Generator Register"]
pub mod pigeon_8_2;
#[doc = "Panel Interface Signal Generator Register"]
pub struct PIGEON_9_0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Panel Interface Signal Generator Register"]
pub mod pigeon_9_0;
#[doc = "Panel Interface Signal Generator Register"]
pub struct PIGEON_9_1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Panel Interface Signal Generator Register"]
pub mod pigeon_9_1;
#[doc = "Panel Interface Signal Generator Register"]
pub struct PIGEON_9_2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Panel Interface Signal Generator Register"]
pub mod pigeon_9_2;
#[doc = "Panel Interface Signal Generator Register"]
pub struct PIGEON_10_0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Panel Interface Signal Generator Register"]
pub mod pigeon_10_0;
#[doc = "Panel Interface Signal Generator Register"]
pub struct PIGEON_10_1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Panel Interface Signal Generator Register"]
pub mod pigeon_10_1;
#[doc = "Panel Interface Signal Generator Register"]
pub struct PIGEON_10_2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Panel Interface Signal Generator Register"]
pub mod pigeon_10_2;
#[doc = "Panel Interface Signal Generator Register"]
pub struct PIGEON_11_0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Panel Interface Signal Generator Register"]
pub mod pigeon_11_0;
#[doc = "Panel Interface Signal Generator Register"]
pub struct PIGEON_11_1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Panel Interface Signal Generator Register"]
pub mod pigeon_11_1;
#[doc = "Panel Interface Signal Generator Register"]
pub struct PIGEON_11_2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Panel Interface Signal Generator Register"]
pub mod pigeon_11_2;
#[doc = "Lookup Table Data Register."]
pub struct LUT_CTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Lookup Table Data Register."]
pub mod lut_ctrl;
#[doc = "Lookup Table Control Register."]
pub struct LUT0_ADDR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Lookup Table Control Register."]
pub mod lut0_addr;
#[doc = "Lookup Table Data Register."]
pub struct LUT0_DATA {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Lookup Table Data Register."]
pub mod lut0_data;
#[doc = "Lookup Table Control Register."]
pub struct LUT1_ADDR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Lookup Table Control Register."]
pub mod lut1_addr;
#[doc = "Lookup Table Data Register."]
pub struct LUT1_DATA {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Lookup Table Data Register."]
pub mod lut1_data;
