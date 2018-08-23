#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Analog ARM PLL control Register"]
    pub pll_arm: PLL_ARM,
    #[doc = "0x04 - Analog ARM PLL control Register"]
    pub pll_arm_set: PLL_ARM_SET,
    #[doc = "0x08 - Analog ARM PLL control Register"]
    pub pll_arm_clr: PLL_ARM_CLR,
    #[doc = "0x0c - Analog ARM PLL control Register"]
    pub pll_arm_tog: PLL_ARM_TOG,
    #[doc = "0x10 - Analog USB1 480MHz PLL Control Register"]
    pub pll_usb1: PLL_USB1,
    #[doc = "0x14 - Analog USB1 480MHz PLL Control Register"]
    pub pll_usb1_set: PLL_USB1_SET,
    #[doc = "0x18 - Analog USB1 480MHz PLL Control Register"]
    pub pll_usb1_clr: PLL_USB1_CLR,
    #[doc = "0x1c - Analog USB1 480MHz PLL Control Register"]
    pub pll_usb1_tog: PLL_USB1_TOG,
    #[doc = "0x20 - Analog USB2 480MHz PLL Control Register"]
    pub pll_usb2: PLL_USB2,
    #[doc = "0x24 - Analog USB2 480MHz PLL Control Register"]
    pub pll_usb2_set: PLL_USB2_SET,
    #[doc = "0x28 - Analog USB2 480MHz PLL Control Register"]
    pub pll_usb2_clr: PLL_USB2_CLR,
    #[doc = "0x2c - Analog USB2 480MHz PLL Control Register"]
    pub pll_usb2_tog: PLL_USB2_TOG,
    #[doc = "0x30 - Analog System PLL Control Register"]
    pub pll_sys: PLL_SYS,
    #[doc = "0x34 - Analog System PLL Control Register"]
    pub pll_sys_set: PLL_SYS_SET,
    #[doc = "0x38 - Analog System PLL Control Register"]
    pub pll_sys_clr: PLL_SYS_CLR,
    #[doc = "0x3c - Analog System PLL Control Register"]
    pub pll_sys_tog: PLL_SYS_TOG,
    #[doc = "0x40 - 528MHz System PLL Spread Spectrum Register"]
    pub pll_sys_ss: PLL_SYS_SS,
    _reserved0: [u8; 12usize],
    #[doc = "0x50 - Numerator of 528MHz System PLL Fractional Loop Divider Register"]
    pub pll_sys_num: PLL_SYS_NUM,
    _reserved1: [u8; 12usize],
    #[doc = "0x60 - Denominator of 528MHz System PLL Fractional Loop Divider Register"]
    pub pll_sys_denom: PLL_SYS_DENOM,
    _reserved2: [u8; 12usize],
    #[doc = "0x70 - Analog Audio PLL control Register"]
    pub pll_audio: PLL_AUDIO,
    #[doc = "0x74 - Analog Audio PLL control Register"]
    pub pll_audio_set: PLL_AUDIO_SET,
    #[doc = "0x78 - Analog Audio PLL control Register"]
    pub pll_audio_clr: PLL_AUDIO_CLR,
    #[doc = "0x7c - Analog Audio PLL control Register"]
    pub pll_audio_tog: PLL_AUDIO_TOG,
    #[doc = "0x80 - Numerator of Audio PLL Fractional Loop Divider Register"]
    pub pll_audio_num: PLL_AUDIO_NUM,
    _reserved3: [u8; 12usize],
    #[doc = "0x90 - Denominator of Audio PLL Fractional Loop Divider Register"]
    pub pll_audio_denom: PLL_AUDIO_DENOM,
    _reserved4: [u8; 12usize],
    #[doc = "0xa0 - Analog Video PLL control Register"]
    pub pll_video: PLL_VIDEO,
    #[doc = "0xa4 - Analog Video PLL control Register"]
    pub pll_video_set: PLL_VIDEO_SET,
    #[doc = "0xa8 - Analog Video PLL control Register"]
    pub pll_video_clr: PLL_VIDEO_CLR,
    #[doc = "0xac - Analog Video PLL control Register"]
    pub pll_video_tog: PLL_VIDEO_TOG,
    #[doc = "0xb0 - Numerator of Video PLL Fractional Loop Divider Register"]
    pub pll_video_num: PLL_VIDEO_NUM,
    _reserved5: [u8; 12usize],
    #[doc = "0xc0 - Denominator of Video PLL Fractional Loop Divider Register"]
    pub pll_video_denom: PLL_VIDEO_DENOM,
    _reserved6: [u8; 28usize],
    #[doc = "0xe0 - Analog ENET PLL Control Register"]
    pub pll_enet: PLL_ENET,
    #[doc = "0xe4 - Analog ENET PLL Control Register"]
    pub pll_enet_set: PLL_ENET_SET,
    #[doc = "0xe8 - Analog ENET PLL Control Register"]
    pub pll_enet_clr: PLL_ENET_CLR,
    #[doc = "0xec - Analog ENET PLL Control Register"]
    pub pll_enet_tog: PLL_ENET_TOG,
    #[doc = "0xf0 - 480MHz Clock (PLL3) Phase Fractional Divider Control Register"]
    pub pfd_480: PFD_480,
    #[doc = "0xf4 - 480MHz Clock (PLL3) Phase Fractional Divider Control Register"]
    pub pfd_480_set: PFD_480_SET,
    #[doc = "0xf8 - 480MHz Clock (PLL3) Phase Fractional Divider Control Register"]
    pub pfd_480_clr: PFD_480_CLR,
    #[doc = "0xfc - 480MHz Clock (PLL3) Phase Fractional Divider Control Register"]
    pub pfd_480_tog: PFD_480_TOG,
    #[doc = "0x100 - 528MHz Clock (PLL2) Phase Fractional Divider Control Register"]
    pub pfd_528: PFD_528,
    #[doc = "0x104 - 528MHz Clock (PLL2) Phase Fractional Divider Control Register"]
    pub pfd_528_set: PFD_528_SET,
    #[doc = "0x108 - 528MHz Clock (PLL2) Phase Fractional Divider Control Register"]
    pub pfd_528_clr: PFD_528_CLR,
    #[doc = "0x10c - 528MHz Clock (PLL2) Phase Fractional Divider Control Register"]
    pub pfd_528_tog: PFD_528_TOG,
    _reserved7: [u8; 64usize],
    #[doc = "0x150 - Miscellaneous Register 0"]
    pub misc0: MISC0,
    #[doc = "0x154 - Miscellaneous Register 0"]
    pub misc0_set: MISC0_SET,
    #[doc = "0x158 - Miscellaneous Register 0"]
    pub misc0_clr: MISC0_CLR,
    #[doc = "0x15c - Miscellaneous Register 0"]
    pub misc0_tog: MISC0_TOG,
    #[doc = "0x160 - Miscellaneous Register 1"]
    pub misc1: MISC1,
    #[doc = "0x164 - Miscellaneous Register 1"]
    pub misc1_set: MISC1_SET,
    #[doc = "0x168 - Miscellaneous Register 1"]
    pub misc1_clr: MISC1_CLR,
    #[doc = "0x16c - Miscellaneous Register 1"]
    pub misc1_tog: MISC1_TOG,
    #[doc = "0x170 - Miscellaneous Register 2"]
    pub misc2: MISC2,
    #[doc = "0x174 - Miscellaneous Register 2"]
    pub misc2_set: MISC2_SET,
    #[doc = "0x178 - Miscellaneous Register 2"]
    pub misc2_clr: MISC2_CLR,
    #[doc = "0x17c - Miscellaneous Register 2"]
    pub misc2_tog: MISC2_TOG,
}
#[doc = "Analog ARM PLL control Register"]
pub struct PLL_ARM {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Analog ARM PLL control Register"]
pub mod pll_arm;
#[doc = "Analog ARM PLL control Register"]
pub struct PLL_ARM_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Analog ARM PLL control Register"]
pub mod pll_arm_set;
#[doc = "Analog ARM PLL control Register"]
pub struct PLL_ARM_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Analog ARM PLL control Register"]
pub mod pll_arm_clr;
#[doc = "Analog ARM PLL control Register"]
pub struct PLL_ARM_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Analog ARM PLL control Register"]
pub mod pll_arm_tog;
#[doc = "Analog USB1 480MHz PLL Control Register"]
pub struct PLL_USB1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Analog USB1 480MHz PLL Control Register"]
pub mod pll_usb1;
#[doc = "Analog USB1 480MHz PLL Control Register"]
pub struct PLL_USB1_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Analog USB1 480MHz PLL Control Register"]
pub mod pll_usb1_set;
#[doc = "Analog USB1 480MHz PLL Control Register"]
pub struct PLL_USB1_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Analog USB1 480MHz PLL Control Register"]
pub mod pll_usb1_clr;
#[doc = "Analog USB1 480MHz PLL Control Register"]
pub struct PLL_USB1_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Analog USB1 480MHz PLL Control Register"]
pub mod pll_usb1_tog;
#[doc = "Analog USB2 480MHz PLL Control Register"]
pub struct PLL_USB2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Analog USB2 480MHz PLL Control Register"]
pub mod pll_usb2;
#[doc = "Analog USB2 480MHz PLL Control Register"]
pub struct PLL_USB2_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Analog USB2 480MHz PLL Control Register"]
pub mod pll_usb2_set;
#[doc = "Analog USB2 480MHz PLL Control Register"]
pub struct PLL_USB2_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Analog USB2 480MHz PLL Control Register"]
pub mod pll_usb2_clr;
#[doc = "Analog USB2 480MHz PLL Control Register"]
pub struct PLL_USB2_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Analog USB2 480MHz PLL Control Register"]
pub mod pll_usb2_tog;
#[doc = "Analog System PLL Control Register"]
pub struct PLL_SYS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Analog System PLL Control Register"]
pub mod pll_sys;
#[doc = "Analog System PLL Control Register"]
pub struct PLL_SYS_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Analog System PLL Control Register"]
pub mod pll_sys_set;
#[doc = "Analog System PLL Control Register"]
pub struct PLL_SYS_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Analog System PLL Control Register"]
pub mod pll_sys_clr;
#[doc = "Analog System PLL Control Register"]
pub struct PLL_SYS_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Analog System PLL Control Register"]
pub mod pll_sys_tog;
#[doc = "528MHz System PLL Spread Spectrum Register"]
pub struct PLL_SYS_SS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "528MHz System PLL Spread Spectrum Register"]
pub mod pll_sys_ss;
#[doc = "Numerator of 528MHz System PLL Fractional Loop Divider Register"]
pub struct PLL_SYS_NUM {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Numerator of 528MHz System PLL Fractional Loop Divider Register"]
pub mod pll_sys_num;
#[doc = "Denominator of 528MHz System PLL Fractional Loop Divider Register"]
pub struct PLL_SYS_DENOM {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Denominator of 528MHz System PLL Fractional Loop Divider Register"]
pub mod pll_sys_denom;
#[doc = "Analog Audio PLL control Register"]
pub struct PLL_AUDIO {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Analog Audio PLL control Register"]
pub mod pll_audio;
#[doc = "Analog Audio PLL control Register"]
pub struct PLL_AUDIO_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Analog Audio PLL control Register"]
pub mod pll_audio_set;
#[doc = "Analog Audio PLL control Register"]
pub struct PLL_AUDIO_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Analog Audio PLL control Register"]
pub mod pll_audio_clr;
#[doc = "Analog Audio PLL control Register"]
pub struct PLL_AUDIO_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Analog Audio PLL control Register"]
pub mod pll_audio_tog;
#[doc = "Numerator of Audio PLL Fractional Loop Divider Register"]
pub struct PLL_AUDIO_NUM {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Numerator of Audio PLL Fractional Loop Divider Register"]
pub mod pll_audio_num;
#[doc = "Denominator of Audio PLL Fractional Loop Divider Register"]
pub struct PLL_AUDIO_DENOM {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Denominator of Audio PLL Fractional Loop Divider Register"]
pub mod pll_audio_denom;
#[doc = "Analog Video PLL control Register"]
pub struct PLL_VIDEO {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Analog Video PLL control Register"]
pub mod pll_video;
#[doc = "Analog Video PLL control Register"]
pub struct PLL_VIDEO_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Analog Video PLL control Register"]
pub mod pll_video_set;
#[doc = "Analog Video PLL control Register"]
pub struct PLL_VIDEO_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Analog Video PLL control Register"]
pub mod pll_video_clr;
#[doc = "Analog Video PLL control Register"]
pub struct PLL_VIDEO_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Analog Video PLL control Register"]
pub mod pll_video_tog;
#[doc = "Numerator of Video PLL Fractional Loop Divider Register"]
pub struct PLL_VIDEO_NUM {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Numerator of Video PLL Fractional Loop Divider Register"]
pub mod pll_video_num;
#[doc = "Denominator of Video PLL Fractional Loop Divider Register"]
pub struct PLL_VIDEO_DENOM {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Denominator of Video PLL Fractional Loop Divider Register"]
pub mod pll_video_denom;
#[doc = "Analog ENET PLL Control Register"]
pub struct PLL_ENET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Analog ENET PLL Control Register"]
pub mod pll_enet;
#[doc = "Analog ENET PLL Control Register"]
pub struct PLL_ENET_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Analog ENET PLL Control Register"]
pub mod pll_enet_set;
#[doc = "Analog ENET PLL Control Register"]
pub struct PLL_ENET_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Analog ENET PLL Control Register"]
pub mod pll_enet_clr;
#[doc = "Analog ENET PLL Control Register"]
pub struct PLL_ENET_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Analog ENET PLL Control Register"]
pub mod pll_enet_tog;
#[doc = "480MHz Clock (PLL3) Phase Fractional Divider Control Register"]
pub struct PFD_480 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "480MHz Clock (PLL3) Phase Fractional Divider Control Register"]
pub mod pfd_480;
#[doc = "480MHz Clock (PLL3) Phase Fractional Divider Control Register"]
pub struct PFD_480_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "480MHz Clock (PLL3) Phase Fractional Divider Control Register"]
pub mod pfd_480_set;
#[doc = "480MHz Clock (PLL3) Phase Fractional Divider Control Register"]
pub struct PFD_480_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "480MHz Clock (PLL3) Phase Fractional Divider Control Register"]
pub mod pfd_480_clr;
#[doc = "480MHz Clock (PLL3) Phase Fractional Divider Control Register"]
pub struct PFD_480_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "480MHz Clock (PLL3) Phase Fractional Divider Control Register"]
pub mod pfd_480_tog;
#[doc = "528MHz Clock (PLL2) Phase Fractional Divider Control Register"]
pub struct PFD_528 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "528MHz Clock (PLL2) Phase Fractional Divider Control Register"]
pub mod pfd_528;
#[doc = "528MHz Clock (PLL2) Phase Fractional Divider Control Register"]
pub struct PFD_528_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "528MHz Clock (PLL2) Phase Fractional Divider Control Register"]
pub mod pfd_528_set;
#[doc = "528MHz Clock (PLL2) Phase Fractional Divider Control Register"]
pub struct PFD_528_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "528MHz Clock (PLL2) Phase Fractional Divider Control Register"]
pub mod pfd_528_clr;
#[doc = "528MHz Clock (PLL2) Phase Fractional Divider Control Register"]
pub struct PFD_528_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "528MHz Clock (PLL2) Phase Fractional Divider Control Register"]
pub mod pfd_528_tog;
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
#[doc = "Miscellaneous Register 1"]
pub struct MISC1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Miscellaneous Register 1"]
pub mod misc1;
#[doc = "Miscellaneous Register 1"]
pub struct MISC1_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Miscellaneous Register 1"]
pub mod misc1_set;
#[doc = "Miscellaneous Register 1"]
pub struct MISC1_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Miscellaneous Register 1"]
pub mod misc1_clr;
#[doc = "Miscellaneous Register 1"]
pub struct MISC1_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Miscellaneous Register 1"]
pub mod misc1_tog;
#[doc = "Miscellaneous Register 2"]
pub struct MISC2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Miscellaneous Register 2"]
pub mod misc2;
#[doc = "Miscellaneous Register 2"]
pub struct MISC2_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Miscellaneous Register 2"]
pub mod misc2_set;
#[doc = "Miscellaneous Register 2"]
pub struct MISC2_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Miscellaneous Register 2"]
pub mod misc2_clr;
#[doc = "Miscellaneous Register 2"]
pub struct MISC2_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Miscellaneous Register 2"]
pub mod misc2_tog;
