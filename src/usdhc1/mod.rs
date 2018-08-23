#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - DMA System Address"]
    pub ds_addr: DS_ADDR,
    #[doc = "0x04 - Block Attributes"]
    pub blk_att: BLK_ATT,
    #[doc = "0x08 - Command Argument"]
    pub cmd_arg: CMD_ARG,
    #[doc = "0x0c - Command Transfer Type"]
    pub cmd_xfr_typ: CMD_XFR_TYP,
    #[doc = "0x10 - Command Response0"]
    pub cmd_rsp0: CMD_RSP0,
    #[doc = "0x14 - Command Response1"]
    pub cmd_rsp1: CMD_RSP1,
    #[doc = "0x18 - Command Response2"]
    pub cmd_rsp2: CMD_RSP2,
    #[doc = "0x1c - Command Response3"]
    pub cmd_rsp3: CMD_RSP3,
    #[doc = "0x20 - Data Buffer Access Port"]
    pub data_buff_acc_port: DATA_BUFF_ACC_PORT,
    #[doc = "0x24 - Present State"]
    pub pres_state: PRES_STATE,
    #[doc = "0x28 - Protocol Control"]
    pub prot_ctrl: PROT_CTRL,
    #[doc = "0x2c - System Control"]
    pub sys_ctrl: SYS_CTRL,
    #[doc = "0x30 - Interrupt Status"]
    pub int_status: INT_STATUS,
    #[doc = "0x34 - Interrupt Status Enable"]
    pub int_status_en: INT_STATUS_EN,
    #[doc = "0x38 - Interrupt Signal Enable"]
    pub int_signal_en: INT_SIGNAL_EN,
    #[doc = "0x3c - Auto CMD12 Error Status"]
    pub autocmd12_err_status: AUTOCMD12_ERR_STATUS,
    #[doc = "0x40 - Host Controller Capabilities"]
    pub host_ctrl_cap: HOST_CTRL_CAP,
    #[doc = "0x44 - Watermark Level"]
    pub wtmk_lvl: WTMK_LVL,
    #[doc = "0x48 - Mixer Control"]
    pub mix_ctrl: MIX_CTRL,
    _reserved0: [u8; 4usize],
    #[doc = "0x50 - Force Event"]
    pub force_event: FORCE_EVENT,
    #[doc = "0x54 - ADMA Error Status Register"]
    pub adma_err_status: ADMA_ERR_STATUS,
    #[doc = "0x58 - ADMA System Address"]
    pub adma_sys_addr: ADMA_SYS_ADDR,
    _reserved1: [u8; 4usize],
    #[doc = "0x60 - DLL (Delay Line) Control"]
    pub dll_ctrl: DLL_CTRL,
    #[doc = "0x64 - DLL Status"]
    pub dll_status: DLL_STATUS,
    #[doc = "0x68 - CLK Tuning Control and Status"]
    pub clk_tune_ctrl_status: CLK_TUNE_CTRL_STATUS,
    _reserved2: [u8; 84usize],
    #[doc = "0xc0 - Vendor Specific Register"]
    pub vend_spec: VEND_SPEC,
    #[doc = "0xc4 - MMC Boot Register"]
    pub mmc_boot: MMC_BOOT,
    #[doc = "0xc8 - Vendor Specific 2 Register"]
    pub vend_spec2: VEND_SPEC2,
    #[doc = "0xcc - Tuning Control Register"]
    pub tuning_ctrl: TUNING_CTRL,
}
#[doc = "DMA System Address"]
pub struct DS_ADDR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DMA System Address"]
pub mod ds_addr;
#[doc = "Block Attributes"]
pub struct BLK_ATT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Block Attributes"]
pub mod blk_att;
#[doc = "Command Argument"]
pub struct CMD_ARG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Command Argument"]
pub mod cmd_arg;
#[doc = "Command Transfer Type"]
pub struct CMD_XFR_TYP {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Command Transfer Type"]
pub mod cmd_xfr_typ;
#[doc = "Command Response0"]
pub struct CMD_RSP0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Command Response0"]
pub mod cmd_rsp0;
#[doc = "Command Response1"]
pub struct CMD_RSP1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Command Response1"]
pub mod cmd_rsp1;
#[doc = "Command Response2"]
pub struct CMD_RSP2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Command Response2"]
pub mod cmd_rsp2;
#[doc = "Command Response3"]
pub struct CMD_RSP3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Command Response3"]
pub mod cmd_rsp3;
#[doc = "Data Buffer Access Port"]
pub struct DATA_BUFF_ACC_PORT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Data Buffer Access Port"]
pub mod data_buff_acc_port;
#[doc = "Present State"]
pub struct PRES_STATE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Present State"]
pub mod pres_state;
#[doc = "Protocol Control"]
pub struct PROT_CTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Protocol Control"]
pub mod prot_ctrl;
#[doc = "System Control"]
pub struct SYS_CTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "System Control"]
pub mod sys_ctrl;
#[doc = "Interrupt Status"]
pub struct INT_STATUS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt Status"]
pub mod int_status;
#[doc = "Interrupt Status Enable"]
pub struct INT_STATUS_EN {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt Status Enable"]
pub mod int_status_en;
#[doc = "Interrupt Signal Enable"]
pub struct INT_SIGNAL_EN {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt Signal Enable"]
pub mod int_signal_en;
#[doc = "Auto CMD12 Error Status"]
pub struct AUTOCMD12_ERR_STATUS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Auto CMD12 Error Status"]
pub mod autocmd12_err_status;
#[doc = "Host Controller Capabilities"]
pub struct HOST_CTRL_CAP {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Host Controller Capabilities"]
pub mod host_ctrl_cap;
#[doc = "Watermark Level"]
pub struct WTMK_LVL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Watermark Level"]
pub mod wtmk_lvl;
#[doc = "Mixer Control"]
pub struct MIX_CTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Mixer Control"]
pub mod mix_ctrl;
#[doc = "Force Event"]
pub struct FORCE_EVENT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Force Event"]
pub mod force_event;
#[doc = "ADMA Error Status Register"]
pub struct ADMA_ERR_STATUS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ADMA Error Status Register"]
pub mod adma_err_status;
#[doc = "ADMA System Address"]
pub struct ADMA_SYS_ADDR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ADMA System Address"]
pub mod adma_sys_addr;
#[doc = "DLL (Delay Line) Control"]
pub struct DLL_CTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DLL (Delay Line) Control"]
pub mod dll_ctrl;
#[doc = "DLL Status"]
pub struct DLL_STATUS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DLL Status"]
pub mod dll_status;
#[doc = "CLK Tuning Control and Status"]
pub struct CLK_TUNE_CTRL_STATUS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CLK Tuning Control and Status"]
pub mod clk_tune_ctrl_status;
#[doc = "Vendor Specific Register"]
pub struct VEND_SPEC {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Vendor Specific Register"]
pub mod vend_spec;
#[doc = "MMC Boot Register"]
pub struct MMC_BOOT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "MMC Boot Register"]
pub mod mmc_boot;
#[doc = "Vendor Specific 2 Register"]
pub struct VEND_SPEC2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Vendor Specific 2 Register"]
pub mod vend_spec2;
#[doc = "Tuning Control Register"]
pub struct TUNING_CTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Tuning Control Register"]
pub mod tuning_ctrl;
