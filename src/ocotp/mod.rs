#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - OTP Controller Control Register"]
    pub hw_ocotp_ctrl: HW_OCOTP_CTRL,
    #[doc = "0x04 - OTP Controller Control Register"]
    pub hw_ocotp_ctrl_set: HW_OCOTP_CTRL_SET,
    #[doc = "0x08 - OTP Controller Control Register"]
    pub hw_ocotp_ctrl_clr: HW_OCOTP_CTRL_CLR,
    #[doc = "0x0c - OTP Controller Control Register"]
    pub hw_ocotp_ctrl_tog: HW_OCOTP_CTRL_TOG,
    #[doc = "0x10 - OTP Controller Timing Register"]
    pub hw_ocotp_timing: HW_OCOTP_TIMING,
    _reserved0: [u8; 12usize],
    #[doc = "0x20 - OTP Controller Write Data Register"]
    pub hw_ocotp_data: HW_OCOTP_DATA,
    _reserved1: [u8; 12usize],
    #[doc = "0x30 - OTP Controller Write Data Register"]
    pub hw_ocotp_read_ctrl: HW_OCOTP_READ_CTRL,
    _reserved2: [u8; 12usize],
    #[doc = "0x40 - OTP Controller Read Data Register"]
    pub hw_ocotp_read_fuse_data: HW_OCOTP_READ_FUSE_DATA,
    _reserved3: [u8; 12usize],
    #[doc = "0x50 - Sticky bit Register"]
    pub hw_ocotp_sw_sticky: HW_OCOTP_SW_STICKY,
    _reserved4: [u8; 12usize],
    #[doc = "0x60 - Software Controllable Signals Register"]
    pub hw_ocotp_scs: HW_OCOTP_SCS,
    #[doc = "0x64 - Software Controllable Signals Register"]
    pub hw_ocotp_scs_set: HW_OCOTP_SCS_SET,
    #[doc = "0x68 - Software Controllable Signals Register"]
    pub hw_ocotp_scs_clr: HW_OCOTP_SCS_CLR,
    #[doc = "0x6c - Software Controllable Signals Register"]
    pub hw_ocotp_scs_tog: HW_OCOTP_SCS_TOG,
    _reserved5: [u8; 32usize],
    #[doc = "0x90 - OTP Controller Version Register"]
    pub hw_ocotp_version: HW_OCOTP_VERSION,
    _reserved6: [u8; 108usize],
    #[doc = "0x100 - OTP Controller Timing Register 2"]
    pub hw_ocotp_timing2: HW_OCOTP_TIMING2,
    _reserved7: [u8; 764usize],
    #[doc = "0x400 - Value of OTP Bank0 Word0 (Lock controls)"]
    pub hw_ocotp_lock: HW_OCOTP_LOCK,
    _reserved8: [u8; 12usize],
    #[doc = "0x410 - Value of OTP Bank0 Word1 (Configuration and Manufacturing Info.)"]
    pub hw_ocotp_cfg0: HW_OCOTP_CFG0,
    _reserved9: [u8; 12usize],
    #[doc = "0x420 - Value of OTP Bank0 Word2 (Configuration and Manufacturing Info.)"]
    pub hw_ocotp_cfg1: HW_OCOTP_CFG1,
    _reserved10: [u8; 12usize],
    #[doc = "0x430 - Value of OTP Bank0 Word3 (Configuration and Manufacturing Info.)"]
    pub hw_ocotp_cfg2: HW_OCOTP_CFG2,
    _reserved11: [u8; 12usize],
    #[doc = "0x440 - Value of OTP Bank0 Word4 (Configuration and Manufacturing Info.)"]
    pub hw_ocotp_cfg3: HW_OCOTP_CFG3,
    _reserved12: [u8; 12usize],
    #[doc = "0x450 - Value of OTP Bank0 Word5 (Configuration and Manufacturing Info.)"]
    pub hw_ocotp_cfg4: HW_OCOTP_CFG4,
    _reserved13: [u8; 12usize],
    #[doc = "0x460 - Value of OTP Bank0 Word6 (Configuration and Manufacturing Info.)"]
    pub hw_ocotp_cfg5: HW_OCOTP_CFG5,
    _reserved14: [u8; 12usize],
    #[doc = "0x470 - Value of OTP Bank0 Word7 (Configuration and Manufacturing Info.)"]
    pub hw_ocotp_cfg6: HW_OCOTP_CFG6,
    _reserved15: [u8; 12usize],
    #[doc = "0x480 - Value of OTP Bank1 Word0 (Memory Related Info.)"]
    pub hw_ocotp_mem0: HW_OCOTP_MEM0,
    _reserved16: [u8; 12usize],
    #[doc = "0x490 - Value of OTP Bank1 Word1 (Memory Related Info.)"]
    pub hw_ocotp_mem1: HW_OCOTP_MEM1,
    _reserved17: [u8; 12usize],
    #[doc = "0x4a0 - Value of OTP Bank1 Word2 (Memory Related Info.)"]
    pub hw_ocotp_mem2: HW_OCOTP_MEM2,
    _reserved18: [u8; 12usize],
    #[doc = "0x4b0 - Value of OTP Bank1 Word3 (Memory Related Info.)"]
    pub hw_ocotp_mem3: HW_OCOTP_MEM3,
    _reserved19: [u8; 12usize],
    #[doc = "0x4c0 - Value of OTP Bank1 Word4 (Memory Related Info.)"]
    pub hw_ocotp_mem4: HW_OCOTP_MEM4,
    _reserved20: [u8; 12usize],
    #[doc = "0x4d0 - Value of OTP Bank1 Word5 (Analog Info.)"]
    pub hw_ocotp_ana0: HW_OCOTP_ANA0,
    _reserved21: [u8; 12usize],
    #[doc = "0x4e0 - Value of OTP Bank1 Word6 (Analog Info.)"]
    pub hw_ocotp_ana1: HW_OCOTP_ANA1,
    _reserved22: [u8; 12usize],
    #[doc = "0x4f0 - Value of OTP Bank1 Word7 (Analog Info.)"]
    pub hw_ocotp_ana2: HW_OCOTP_ANA2,
    _reserved23: [u8; 140usize],
    #[doc = "0x580 - Shadow Register for OTP Bank3 Word0 (SRK Hash)"]
    pub hw_ocotp_srk0: HW_OCOTP_SRK0,
    _reserved24: [u8; 12usize],
    #[doc = "0x590 - Shadow Register for OTP Bank3 Word1 (SRK Hash)"]
    pub hw_ocotp_srk1: HW_OCOTP_SRK1,
    _reserved25: [u8; 12usize],
    #[doc = "0x5a0 - Shadow Register for OTP Bank3 Word2 (SRK Hash)"]
    pub hw_ocotp_srk2: HW_OCOTP_SRK2,
    _reserved26: [u8; 12usize],
    #[doc = "0x5b0 - Shadow Register for OTP Bank3 Word3 (SRK Hash)"]
    pub hw_ocotp_srk3: HW_OCOTP_SRK3,
    _reserved27: [u8; 12usize],
    #[doc = "0x5c0 - Shadow Register for OTP Bank3 Word4 (SRK Hash)"]
    pub hw_ocotp_srk4: HW_OCOTP_SRK4,
    _reserved28: [u8; 12usize],
    #[doc = "0x5d0 - Shadow Register for OTP Bank3 Word5 (SRK Hash)"]
    pub hw_ocotp_srk5: HW_OCOTP_SRK5,
    _reserved29: [u8; 12usize],
    #[doc = "0x5e0 - Shadow Register for OTP Bank3 Word6 (SRK Hash)"]
    pub hw_ocotp_srk6: HW_OCOTP_SRK6,
    _reserved30: [u8; 12usize],
    #[doc = "0x5f0 - Shadow Register for OTP Bank3 Word7 (SRK Hash)"]
    pub hw_ocotp_srk7: HW_OCOTP_SRK7,
    _reserved31: [u8; 12usize],
    #[doc = "0x600 - Value of OTP Bank4 Word0 (Secure JTAG Response Field)"]
    pub hw_ocotp_sjc_resp0: HW_OCOTP_SJC_RESP0,
    _reserved32: [u8; 12usize],
    #[doc = "0x610 - Value of OTP Bank4 Word1 (Secure JTAG Response Field)"]
    pub hw_ocotp_sjc_resp1: HW_OCOTP_SJC_RESP1,
    _reserved33: [u8; 12usize],
    #[doc = "0x620 - Value of OTP Bank4 Word2 (MAC Address)"]
    pub hw_ocotp_mac0: HW_OCOTP_MAC0,
    _reserved34: [u8; 12usize],
    #[doc = "0x630 - Value of OTP Bank4 Word3 (MAC Address)"]
    pub hw_ocotp_mac1: HW_OCOTP_MAC1,
    _reserved35: [u8; 12usize],
    #[doc = "0x640 - Value of OTP Bank4 Word4 (MAC Address)"]
    pub hw_ocotp_gp3: HW_OCOTP_GP3,
    _reserved36: [u8; 28usize],
    #[doc = "0x660 - Value of OTP Bank4 Word6 (General Purpose Customer Defined Info)"]
    pub hw_ocotp_gp1: HW_OCOTP_GP1,
    _reserved37: [u8; 12usize],
    #[doc = "0x670 - Value of OTP Bank4 Word7 (General Purpose Customer Defined Info)"]
    pub hw_ocotp_gp2: HW_OCOTP_GP2,
    _reserved38: [u8; 12usize],
    #[doc = "0x680 - Value of OTP Bank5 Word0 (SW GP1)"]
    pub hw_ocotp_sw_gp1: HW_OCOTP_SW_GP1,
    _reserved39: [u8; 12usize],
    #[doc = "0x690 - Value of OTP Bank5 Word1 (SW GP2)"]
    pub hw_ocotp_sw_gp20: HW_OCOTP_SW_GP20,
    _reserved40: [u8; 12usize],
    #[doc = "0x6a0 - Value of OTP Bank5 Word2 (SW GP2)"]
    pub hw_ocotp_sw_gp21: HW_OCOTP_SW_GP21,
    _reserved41: [u8; 12usize],
    #[doc = "0x6b0 - Value of OTP Bank5 Word3 (SW GP2)"]
    pub hw_ocotp_sw_gp22: HW_OCOTP_SW_GP22,
    _reserved42: [u8; 12usize],
    #[doc = "0x6c0 - Value of OTP Bank5 Word4 (SW GP2)"]
    pub hw_ocotp_sw_gp23: HW_OCOTP_SW_GP23,
    _reserved43: [u8; 12usize],
    #[doc = "0x6d0 - Value of OTP Bank5 Word5 (Misc Conf)"]
    pub hw_ocotp_misc_conf0: HW_OCOTP_MISC_CONF0,
    _reserved44: [u8; 12usize],
    #[doc = "0x6e0 - Value of OTP Bank5 Word6 (Misc Conf)"]
    pub hw_ocotp_misc_conf1: HW_OCOTP_MISC_CONF1,
    _reserved45: [u8; 12usize],
    #[doc = "0x6f0 - Value of OTP Bank5 Word7 (SRK Revoke)"]
    pub hw_ocotp_srk_revoke: HW_OCOTP_SRK_REVOKE,
}
#[doc = "OTP Controller Control Register"]
pub struct HW_OCOTP_CTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "OTP Controller Control Register"]
pub mod hw_ocotp_ctrl;
#[doc = "OTP Controller Control Register"]
pub struct HW_OCOTP_CTRL_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "OTP Controller Control Register"]
pub mod hw_ocotp_ctrl_set;
#[doc = "OTP Controller Control Register"]
pub struct HW_OCOTP_CTRL_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "OTP Controller Control Register"]
pub mod hw_ocotp_ctrl_clr;
#[doc = "OTP Controller Control Register"]
pub struct HW_OCOTP_CTRL_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "OTP Controller Control Register"]
pub mod hw_ocotp_ctrl_tog;
#[doc = "OTP Controller Timing Register"]
pub struct HW_OCOTP_TIMING {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "OTP Controller Timing Register"]
pub mod hw_ocotp_timing;
#[doc = "OTP Controller Write Data Register"]
pub struct HW_OCOTP_DATA {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "OTP Controller Write Data Register"]
pub mod hw_ocotp_data;
#[doc = "OTP Controller Write Data Register"]
pub struct HW_OCOTP_READ_CTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "OTP Controller Write Data Register"]
pub mod hw_ocotp_read_ctrl;
#[doc = "OTP Controller Read Data Register"]
pub struct HW_OCOTP_READ_FUSE_DATA {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "OTP Controller Read Data Register"]
pub mod hw_ocotp_read_fuse_data;
#[doc = "Sticky bit Register"]
pub struct HW_OCOTP_SW_STICKY {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Sticky bit Register"]
pub mod hw_ocotp_sw_sticky;
#[doc = "Software Controllable Signals Register"]
pub struct HW_OCOTP_SCS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Software Controllable Signals Register"]
pub mod hw_ocotp_scs;
#[doc = "Software Controllable Signals Register"]
pub struct HW_OCOTP_SCS_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Software Controllable Signals Register"]
pub mod hw_ocotp_scs_set;
#[doc = "Software Controllable Signals Register"]
pub struct HW_OCOTP_SCS_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Software Controllable Signals Register"]
pub mod hw_ocotp_scs_clr;
#[doc = "Software Controllable Signals Register"]
pub struct HW_OCOTP_SCS_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Software Controllable Signals Register"]
pub mod hw_ocotp_scs_tog;
#[doc = "OTP Controller Version Register"]
pub struct HW_OCOTP_VERSION {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "OTP Controller Version Register"]
pub mod hw_ocotp_version;
#[doc = "OTP Controller Timing Register 2"]
pub struct HW_OCOTP_TIMING2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "OTP Controller Timing Register 2"]
pub mod hw_ocotp_timing2;
#[doc = "Value of OTP Bank0 Word0 (Lock controls)"]
pub struct HW_OCOTP_LOCK {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Value of OTP Bank0 Word0 (Lock controls)"]
pub mod hw_ocotp_lock;
#[doc = "Value of OTP Bank0 Word1 (Configuration and Manufacturing Info.)"]
pub struct HW_OCOTP_CFG0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Value of OTP Bank0 Word1 (Configuration and Manufacturing Info.)"]
pub mod hw_ocotp_cfg0;
#[doc = "Value of OTP Bank0 Word2 (Configuration and Manufacturing Info.)"]
pub struct HW_OCOTP_CFG1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Value of OTP Bank0 Word2 (Configuration and Manufacturing Info.)"]
pub mod hw_ocotp_cfg1;
#[doc = "Value of OTP Bank0 Word3 (Configuration and Manufacturing Info.)"]
pub struct HW_OCOTP_CFG2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Value of OTP Bank0 Word3 (Configuration and Manufacturing Info.)"]
pub mod hw_ocotp_cfg2;
#[doc = "Value of OTP Bank0 Word4 (Configuration and Manufacturing Info.)"]
pub struct HW_OCOTP_CFG3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Value of OTP Bank0 Word4 (Configuration and Manufacturing Info.)"]
pub mod hw_ocotp_cfg3;
#[doc = "Value of OTP Bank0 Word5 (Configuration and Manufacturing Info.)"]
pub struct HW_OCOTP_CFG4 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Value of OTP Bank0 Word5 (Configuration and Manufacturing Info.)"]
pub mod hw_ocotp_cfg4;
#[doc = "Value of OTP Bank0 Word6 (Configuration and Manufacturing Info.)"]
pub struct HW_OCOTP_CFG5 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Value of OTP Bank0 Word6 (Configuration and Manufacturing Info.)"]
pub mod hw_ocotp_cfg5;
#[doc = "Value of OTP Bank0 Word7 (Configuration and Manufacturing Info.)"]
pub struct HW_OCOTP_CFG6 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Value of OTP Bank0 Word7 (Configuration and Manufacturing Info.)"]
pub mod hw_ocotp_cfg6;
#[doc = "Value of OTP Bank1 Word0 (Memory Related Info.)"]
pub struct HW_OCOTP_MEM0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Value of OTP Bank1 Word0 (Memory Related Info.)"]
pub mod hw_ocotp_mem0;
#[doc = "Value of OTP Bank1 Word1 (Memory Related Info.)"]
pub struct HW_OCOTP_MEM1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Value of OTP Bank1 Word1 (Memory Related Info.)"]
pub mod hw_ocotp_mem1;
#[doc = "Value of OTP Bank1 Word2 (Memory Related Info.)"]
pub struct HW_OCOTP_MEM2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Value of OTP Bank1 Word2 (Memory Related Info.)"]
pub mod hw_ocotp_mem2;
#[doc = "Value of OTP Bank1 Word3 (Memory Related Info.)"]
pub struct HW_OCOTP_MEM3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Value of OTP Bank1 Word3 (Memory Related Info.)"]
pub mod hw_ocotp_mem3;
#[doc = "Value of OTP Bank1 Word4 (Memory Related Info.)"]
pub struct HW_OCOTP_MEM4 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Value of OTP Bank1 Word4 (Memory Related Info.)"]
pub mod hw_ocotp_mem4;
#[doc = "Value of OTP Bank1 Word5 (Analog Info.)"]
pub struct HW_OCOTP_ANA0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Value of OTP Bank1 Word5 (Analog Info.)"]
pub mod hw_ocotp_ana0;
#[doc = "Value of OTP Bank1 Word6 (Analog Info.)"]
pub struct HW_OCOTP_ANA1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Value of OTP Bank1 Word6 (Analog Info.)"]
pub mod hw_ocotp_ana1;
#[doc = "Value of OTP Bank1 Word7 (Analog Info.)"]
pub struct HW_OCOTP_ANA2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Value of OTP Bank1 Word7 (Analog Info.)"]
pub mod hw_ocotp_ana2;
#[doc = "Shadow Register for OTP Bank3 Word0 (SRK Hash)"]
pub struct HW_OCOTP_SRK0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Shadow Register for OTP Bank3 Word0 (SRK Hash)"]
pub mod hw_ocotp_srk0;
#[doc = "Shadow Register for OTP Bank3 Word1 (SRK Hash)"]
pub struct HW_OCOTP_SRK1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Shadow Register for OTP Bank3 Word1 (SRK Hash)"]
pub mod hw_ocotp_srk1;
#[doc = "Shadow Register for OTP Bank3 Word2 (SRK Hash)"]
pub struct HW_OCOTP_SRK2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Shadow Register for OTP Bank3 Word2 (SRK Hash)"]
pub mod hw_ocotp_srk2;
#[doc = "Shadow Register for OTP Bank3 Word3 (SRK Hash)"]
pub struct HW_OCOTP_SRK3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Shadow Register for OTP Bank3 Word3 (SRK Hash)"]
pub mod hw_ocotp_srk3;
#[doc = "Shadow Register for OTP Bank3 Word4 (SRK Hash)"]
pub struct HW_OCOTP_SRK4 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Shadow Register for OTP Bank3 Word4 (SRK Hash)"]
pub mod hw_ocotp_srk4;
#[doc = "Shadow Register for OTP Bank3 Word5 (SRK Hash)"]
pub struct HW_OCOTP_SRK5 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Shadow Register for OTP Bank3 Word5 (SRK Hash)"]
pub mod hw_ocotp_srk5;
#[doc = "Shadow Register for OTP Bank3 Word6 (SRK Hash)"]
pub struct HW_OCOTP_SRK6 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Shadow Register for OTP Bank3 Word6 (SRK Hash)"]
pub mod hw_ocotp_srk6;
#[doc = "Shadow Register for OTP Bank3 Word7 (SRK Hash)"]
pub struct HW_OCOTP_SRK7 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Shadow Register for OTP Bank3 Word7 (SRK Hash)"]
pub mod hw_ocotp_srk7;
#[doc = "Value of OTP Bank4 Word0 (Secure JTAG Response Field)"]
pub struct HW_OCOTP_SJC_RESP0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Value of OTP Bank4 Word0 (Secure JTAG Response Field)"]
pub mod hw_ocotp_sjc_resp0;
#[doc = "Value of OTP Bank4 Word1 (Secure JTAG Response Field)"]
pub struct HW_OCOTP_SJC_RESP1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Value of OTP Bank4 Word1 (Secure JTAG Response Field)"]
pub mod hw_ocotp_sjc_resp1;
#[doc = "Value of OTP Bank4 Word2 (MAC Address)"]
pub struct HW_OCOTP_MAC0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Value of OTP Bank4 Word2 (MAC Address)"]
pub mod hw_ocotp_mac0;
#[doc = "Value of OTP Bank4 Word3 (MAC Address)"]
pub struct HW_OCOTP_MAC1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Value of OTP Bank4 Word3 (MAC Address)"]
pub mod hw_ocotp_mac1;
#[doc = "Value of OTP Bank4 Word4 (MAC Address)"]
pub struct HW_OCOTP_GP3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Value of OTP Bank4 Word4 (MAC Address)"]
pub mod hw_ocotp_gp3;
#[doc = "Value of OTP Bank4 Word6 (General Purpose Customer Defined Info)"]
pub struct HW_OCOTP_GP1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Value of OTP Bank4 Word6 (General Purpose Customer Defined Info)"]
pub mod hw_ocotp_gp1;
#[doc = "Value of OTP Bank4 Word7 (General Purpose Customer Defined Info)"]
pub struct HW_OCOTP_GP2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Value of OTP Bank4 Word7 (General Purpose Customer Defined Info)"]
pub mod hw_ocotp_gp2;
#[doc = "Value of OTP Bank5 Word0 (SW GP1)"]
pub struct HW_OCOTP_SW_GP1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Value of OTP Bank5 Word0 (SW GP1)"]
pub mod hw_ocotp_sw_gp1;
#[doc = "Value of OTP Bank5 Word1 (SW GP2)"]
pub struct HW_OCOTP_SW_GP20 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Value of OTP Bank5 Word1 (SW GP2)"]
pub mod hw_ocotp_sw_gp20;
#[doc = "Value of OTP Bank5 Word2 (SW GP2)"]
pub struct HW_OCOTP_SW_GP21 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Value of OTP Bank5 Word2 (SW GP2)"]
pub mod hw_ocotp_sw_gp21;
#[doc = "Value of OTP Bank5 Word3 (SW GP2)"]
pub struct HW_OCOTP_SW_GP22 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Value of OTP Bank5 Word3 (SW GP2)"]
pub mod hw_ocotp_sw_gp22;
#[doc = "Value of OTP Bank5 Word4 (SW GP2)"]
pub struct HW_OCOTP_SW_GP23 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Value of OTP Bank5 Word4 (SW GP2)"]
pub mod hw_ocotp_sw_gp23;
#[doc = "Value of OTP Bank5 Word5 (Misc Conf)"]
pub struct HW_OCOTP_MISC_CONF0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Value of OTP Bank5 Word5 (Misc Conf)"]
pub mod hw_ocotp_misc_conf0;
#[doc = "Value of OTP Bank5 Word6 (Misc Conf)"]
pub struct HW_OCOTP_MISC_CONF1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Value of OTP Bank5 Word6 (Misc Conf)"]
pub mod hw_ocotp_misc_conf1;
#[doc = "Value of OTP Bank5 Word7 (SRK Revoke)"]
pub struct HW_OCOTP_SRK_REVOKE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Value of OTP Bank5 Word7 (SRK Revoke)"]
pub mod hw_ocotp_srk_revoke;
