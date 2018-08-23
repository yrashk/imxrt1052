#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - PS Input Buffer Address"]
    pub basic_setting: BASIC_SETTING,
    _reserved0: [u8; 12usize],
    #[doc = "0x10 - PS Input Buffer Address"]
    pub ps_input_buffer_addr: PS_INPUT_BUFFER_ADDR,
    _reserved1: [u8; 12usize],
    #[doc = "0x20 - Flow Control"]
    pub flow_control: FLOW_CONTROL,
    _reserved2: [u8; 12usize],
    #[doc = "0x30 - Measure Value"]
    pub measeure_value: MEASEURE_VALUE,
    _reserved3: [u8; 12usize],
    #[doc = "0x40 - Interrupt Enable"]
    pub int_en: INT_EN,
    _reserved4: [u8; 12usize],
    #[doc = "0x50 - Interrupt Signal Enable"]
    pub int_sig_en: INT_SIG_EN,
    _reserved5: [u8; 12usize],
    #[doc = "0x60 - Intterrupt Status"]
    pub int_status: INT_STATUS,
    _reserved6: [u8; 12usize],
    #[doc = "0x70 - no description available"]
    pub debug_mode: DEBUG_MODE,
    _reserved7: [u8; 12usize],
    #[doc = "0x80 - no description available"]
    pub debug_mode2: DEBUG_MODE2,
}
#[doc = "PS Input Buffer Address"]
pub struct BASIC_SETTING {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PS Input Buffer Address"]
pub mod basic_setting;
#[doc = "PS Input Buffer Address"]
pub struct PS_INPUT_BUFFER_ADDR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PS Input Buffer Address"]
pub mod ps_input_buffer_addr;
#[doc = "Flow Control"]
pub struct FLOW_CONTROL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Flow Control"]
pub mod flow_control;
#[doc = "Measure Value"]
pub struct MEASEURE_VALUE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Measure Value"]
pub mod measeure_value;
#[doc = "Interrupt Enable"]
pub struct INT_EN {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt Enable"]
pub mod int_en;
#[doc = "Interrupt Signal Enable"]
pub struct INT_SIG_EN {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt Signal Enable"]
pub mod int_sig_en;
#[doc = "Intterrupt Status"]
pub struct INT_STATUS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Intterrupt Status"]
pub mod int_status;
#[doc = "no description available"]
pub struct DEBUG_MODE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "no description available"]
pub mod debug_mode;
#[doc = "no description available"]
pub struct DEBUG_MODE2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "no description available"]
pub mod debug_mode2;
