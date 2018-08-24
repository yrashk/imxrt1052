#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control Register 0"]
    pub ctrl: CTRL,
    #[doc = "0x04 - Control Register 0"]
    pub ctrl_set: CTRL_SET,
    #[doc = "0x08 - Control Register 0"]
    pub ctrl_clr: CTRL_CLR,
    #[doc = "0x0c - Control Register 0"]
    pub ctrl_tog: CTRL_TOG,
    #[doc = "0x10 - Status Register"]
    pub stat: STAT,
    #[doc = "0x14 - Status Register"]
    pub stat_set: STAT_SET,
    #[doc = "0x18 - Status Register"]
    pub stat_clr: STAT_CLR,
    #[doc = "0x1c - Status Register"]
    pub stat_tog: STAT_TOG,
    #[doc = "0x20 - Output Buffer Control Register"]
    pub out_ctrl: OUT_CTRL,
    #[doc = "0x24 - Output Buffer Control Register"]
    pub out_ctrl_set: OUT_CTRL_SET,
    #[doc = "0x28 - Output Buffer Control Register"]
    pub out_ctrl_clr: OUT_CTRL_CLR,
    #[doc = "0x2c - Output Buffer Control Register"]
    pub out_ctrl_tog: OUT_CTRL_TOG,
    #[doc = "0x30 - Output Frame Buffer Pointer"]
    pub out_buf: OUT_BUF,
    _reserved0: [u8; 12usize],
    #[doc = "0x40 - Output Frame Buffer Pointer #2"]
    pub out_buf2: OUT_BUF2,
    _reserved1: [u8; 12usize],
    #[doc = "0x50 - Output Buffer Pitch"]
    pub out_pitch: OUT_PITCH,
    _reserved2: [u8; 12usize],
    #[doc = "0x60 - Output Surface Lower Right Coordinate"]
    pub out_lrc: OUT_LRC,
    _reserved3: [u8; 12usize],
    #[doc = "0x70 - Processed Surface Upper Left Coordinate"]
    pub out_ps_ulc: OUT_PS_ULC,
    _reserved4: [u8; 12usize],
    #[doc = "0x80 - Processed Surface Lower Right Coordinate"]
    pub out_ps_lrc: OUT_PS_LRC,
    _reserved5: [u8; 12usize],
    #[doc = "0x90 - Alpha Surface Upper Left Coordinate"]
    pub out_as_ulc: OUT_AS_ULC,
    _reserved6: [u8; 12usize],
    #[doc = "0xa0 - Alpha Surface Lower Right Coordinate"]
    pub out_as_lrc: OUT_AS_LRC,
    _reserved7: [u8; 12usize],
    #[doc = "0xb0 - Processed Surface (PS) Control Register"]
    pub ps_ctrl: PS_CTRL,
    #[doc = "0xb4 - Processed Surface (PS) Control Register"]
    pub ps_ctrl_set: PS_CTRL_SET,
    #[doc = "0xb8 - Processed Surface (PS) Control Register"]
    pub ps_ctrl_clr: PS_CTRL_CLR,
    #[doc = "0xbc - Processed Surface (PS) Control Register"]
    pub ps_ctrl_tog: PS_CTRL_TOG,
    #[doc = "0xc0 - PS Input Buffer Address"]
    pub ps_buf: PS_BUF,
    _reserved8: [u8; 12usize],
    #[doc = "0xd0 - PS U/Cb or 2 Plane UV Input Buffer Address"]
    pub ps_ubuf: PS_UBUF,
    _reserved9: [u8; 12usize],
    #[doc = "0xe0 - PS V/Cr Input Buffer Address"]
    pub ps_vbuf: PS_VBUF,
    _reserved10: [u8; 12usize],
    #[doc = "0xf0 - Processed Surface Pitch"]
    pub ps_pitch: PS_PITCH,
    _reserved11: [u8; 12usize],
    #[doc = "0x100 - PS Background Color"]
    pub ps_background: PS_BACKGROUND,
    _reserved12: [u8; 12usize],
    #[doc = "0x110 - PS Scale Factor Register"]
    pub ps_scale: PS_SCALE,
    _reserved13: [u8; 12usize],
    #[doc = "0x120 - PS Scale Offset Register"]
    pub ps_offset: PS_OFFSET,
    _reserved14: [u8; 12usize],
    #[doc = "0x130 - PS Color Key Low"]
    pub ps_clrkeylow: PS_CLRKEYLOW,
    _reserved15: [u8; 12usize],
    #[doc = "0x140 - PS Color Key High"]
    pub ps_clrkeyhigh: PS_CLRKEYHIGH,
    _reserved16: [u8; 12usize],
    #[doc = "0x150 - Alpha Surface Control"]
    pub as_ctrl: AS_CTRL,
    _reserved17: [u8; 12usize],
    #[doc = "0x160 - Alpha Surface Buffer Pointer"]
    pub as_buf: AS_BUF,
    _reserved18: [u8; 12usize],
    #[doc = "0x170 - Alpha Surface Pitch"]
    pub as_pitch: AS_PITCH,
    _reserved19: [u8; 12usize],
    #[doc = "0x180 - Overlay Color Key Low"]
    pub as_clrkeylow: AS_CLRKEYLOW,
    _reserved20: [u8; 12usize],
    #[doc = "0x190 - Overlay Color Key High"]
    pub as_clrkeyhigh: AS_CLRKEYHIGH,
    _reserved21: [u8; 12usize],
    #[doc = "0x1a0 - Color Space Conversion Coefficient Register 0"]
    pub csc1_coef0: CSC1_COEF0,
    _reserved22: [u8; 12usize],
    #[doc = "0x1b0 - Color Space Conversion Coefficient Register 1"]
    pub csc1_coef1: CSC1_COEF1,
    _reserved23: [u8; 12usize],
    #[doc = "0x1c0 - Color Space Conversion Coefficient Register 2"]
    pub csc1_coef2: CSC1_COEF2,
    _reserved24: [u8; 348usize],
    #[doc = "0x320 - PXP Power Control Register"]
    pub power: POWER,
    _reserved25: [u8; 220usize],
    #[doc = "0x400 - Next Frame Pointer"]
    pub next: NEXT,
    _reserved26: [u8; 60usize],
    #[doc = "0x440 - PXP Alpha Engine A Control Register."]
    pub porter_duff_ctrl: PORTER_DUFF_CTRL,
}
#[doc = "Control Register 0"]
pub struct CTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Control Register 0"]
pub mod ctrl;
#[doc = "Control Register 0"]
pub struct CTRL_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Control Register 0"]
pub mod ctrl_set;
#[doc = "Control Register 0"]
pub struct CTRL_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Control Register 0"]
pub mod ctrl_clr;
#[doc = "Control Register 0"]
pub struct CTRL_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Control Register 0"]
pub mod ctrl_tog;
#[doc = "Status Register"]
pub struct STAT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Status Register"]
pub mod stat;
#[doc = "Status Register"]
pub struct STAT_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Status Register"]
pub mod stat_set;
#[doc = "Status Register"]
pub struct STAT_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Status Register"]
pub mod stat_clr;
#[doc = "Status Register"]
pub struct STAT_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Status Register"]
pub mod stat_tog;
#[doc = "Output Buffer Control Register"]
pub struct OUT_CTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Output Buffer Control Register"]
pub mod out_ctrl;
#[doc = "Output Buffer Control Register"]
pub struct OUT_CTRL_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Output Buffer Control Register"]
pub mod out_ctrl_set;
#[doc = "Output Buffer Control Register"]
pub struct OUT_CTRL_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Output Buffer Control Register"]
pub mod out_ctrl_clr;
#[doc = "Output Buffer Control Register"]
pub struct OUT_CTRL_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Output Buffer Control Register"]
pub mod out_ctrl_tog;
#[doc = "Output Frame Buffer Pointer"]
pub struct OUT_BUF {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Output Frame Buffer Pointer"]
pub mod out_buf;
#[doc = "Output Frame Buffer Pointer #2"]
pub struct OUT_BUF2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Output Frame Buffer Pointer #2"]
pub mod out_buf2;
#[doc = "Output Buffer Pitch"]
pub struct OUT_PITCH {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Output Buffer Pitch"]
pub mod out_pitch;
#[doc = "Output Surface Lower Right Coordinate"]
pub struct OUT_LRC {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Output Surface Lower Right Coordinate"]
pub mod out_lrc;
#[doc = "Processed Surface Upper Left Coordinate"]
pub struct OUT_PS_ULC {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Processed Surface Upper Left Coordinate"]
pub mod out_ps_ulc;
#[doc = "Processed Surface Lower Right Coordinate"]
pub struct OUT_PS_LRC {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Processed Surface Lower Right Coordinate"]
pub mod out_ps_lrc;
#[doc = "Alpha Surface Upper Left Coordinate"]
pub struct OUT_AS_ULC {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Alpha Surface Upper Left Coordinate"]
pub mod out_as_ulc;
#[doc = "Alpha Surface Lower Right Coordinate"]
pub struct OUT_AS_LRC {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Alpha Surface Lower Right Coordinate"]
pub mod out_as_lrc;
#[doc = "Processed Surface (PS) Control Register"]
pub struct PS_CTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Processed Surface (PS) Control Register"]
pub mod ps_ctrl;
#[doc = "Processed Surface (PS) Control Register"]
pub struct PS_CTRL_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Processed Surface (PS) Control Register"]
pub mod ps_ctrl_set;
#[doc = "Processed Surface (PS) Control Register"]
pub struct PS_CTRL_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Processed Surface (PS) Control Register"]
pub mod ps_ctrl_clr;
#[doc = "Processed Surface (PS) Control Register"]
pub struct PS_CTRL_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Processed Surface (PS) Control Register"]
pub mod ps_ctrl_tog;
#[doc = "PS Input Buffer Address"]
pub struct PS_BUF {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PS Input Buffer Address"]
pub mod ps_buf;
#[doc = "PS U/Cb or 2 Plane UV Input Buffer Address"]
pub struct PS_UBUF {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PS U/Cb or 2 Plane UV Input Buffer Address"]
pub mod ps_ubuf;
#[doc = "PS V/Cr Input Buffer Address"]
pub struct PS_VBUF {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PS V/Cr Input Buffer Address"]
pub mod ps_vbuf;
#[doc = "Processed Surface Pitch"]
pub struct PS_PITCH {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Processed Surface Pitch"]
pub mod ps_pitch;
#[doc = "PS Background Color"]
pub struct PS_BACKGROUND {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PS Background Color"]
pub mod ps_background;
#[doc = "PS Scale Factor Register"]
pub struct PS_SCALE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PS Scale Factor Register"]
pub mod ps_scale;
#[doc = "PS Scale Offset Register"]
pub struct PS_OFFSET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PS Scale Offset Register"]
pub mod ps_offset;
#[doc = "PS Color Key Low"]
pub struct PS_CLRKEYLOW {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PS Color Key Low"]
pub mod ps_clrkeylow;
#[doc = "PS Color Key High"]
pub struct PS_CLRKEYHIGH {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PS Color Key High"]
pub mod ps_clrkeyhigh;
#[doc = "Alpha Surface Control"]
pub struct AS_CTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Alpha Surface Control"]
pub mod as_ctrl;
#[doc = "Alpha Surface Buffer Pointer"]
pub struct AS_BUF {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Alpha Surface Buffer Pointer"]
pub mod as_buf;
#[doc = "Alpha Surface Pitch"]
pub struct AS_PITCH {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Alpha Surface Pitch"]
pub mod as_pitch;
#[doc = "Overlay Color Key Low"]
pub struct AS_CLRKEYLOW {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Overlay Color Key Low"]
pub mod as_clrkeylow;
#[doc = "Overlay Color Key High"]
pub struct AS_CLRKEYHIGH {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Overlay Color Key High"]
pub mod as_clrkeyhigh;
#[doc = "Color Space Conversion Coefficient Register 0"]
pub struct CSC1_COEF0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Color Space Conversion Coefficient Register 0"]
pub mod csc1_coef0;
#[doc = "Color Space Conversion Coefficient Register 1"]
pub struct CSC1_COEF1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Color Space Conversion Coefficient Register 1"]
pub mod csc1_coef1;
#[doc = "Color Space Conversion Coefficient Register 2"]
pub struct CSC1_COEF2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Color Space Conversion Coefficient Register 2"]
pub mod csc1_coef2;
#[doc = "PXP Power Control Register"]
pub struct POWER {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PXP Power Control Register"]
pub mod power;
#[doc = "Next Frame Pointer"]
pub struct NEXT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Next Frame Pointer"]
pub mod next;
#[doc = "PXP Alpha Engine A Control Register."]
pub struct PORTER_DUFF_CTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PXP Alpha Engine A Control Register."]
pub mod porter_duff_ctrl;
