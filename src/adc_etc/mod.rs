#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - ADC_ETC Global Control Register"]
    pub ctrl: CTRL,
    #[doc = "0x04 - ETC DONE0 and DONE1 IRQ State Register"]
    pub done0_1_irq: DONE0_1_IRQ,
    #[doc = "0x08 - ETC DONE_2 and DONE_ERR IRQ State Register"]
    pub done2_err_irq: DONE2_ERR_IRQ,
    #[doc = "0x0c - ETC DMA control Register"]
    pub dma_ctrl: DMA_CTRL,
    #[doc = "0x10 - ETC_TRIG0 Control Register"]
    pub trig0_ctrl: TRIG0_CTRL,
    #[doc = "0x14 - ETC_TRIG0 Counter Register"]
    pub trig0_counter: TRIG0_COUNTER,
    #[doc = "0x18 - ETC_TRIG Chain 0/1 Register"]
    pub trig0_chain_1_0: TRIG0_CHAIN_1_0,
    #[doc = "0x1c - ETC_TRIG Chain 2/3 Register"]
    pub trig0_chain_3_2: TRIG0_CHAIN_3_2,
    #[doc = "0x20 - ETC_TRIG Chain 4/5 Register"]
    pub trig0_chain_5_4: TRIG0_CHAIN_5_4,
    #[doc = "0x24 - ETC_TRIG Chain 6/7 Register"]
    pub trig0_chain_7_6: TRIG0_CHAIN_7_6,
    #[doc = "0x28 - ETC_TRIG Result Data 1/0 Register"]
    pub trig0_result_1_0: TRIG0_RESULT_1_0,
    #[doc = "0x2c - ETC_TRIG Result Data 3/2 Register"]
    pub trig0_result_3_2: TRIG0_RESULT_3_2,
    #[doc = "0x30 - ETC_TRIG Result Data 5/4 Register"]
    pub trig0_result_5_4: TRIG0_RESULT_5_4,
    #[doc = "0x34 - ETC_TRIG Result Data 7/6 Register"]
    pub trig0_result_7_6: TRIG0_RESULT_7_6,
    #[doc = "0x38 - ETC_TRIG1 Control Register"]
    pub trig1_ctrl: TRIG1_CTRL,
    #[doc = "0x3c - ETC_TRIG1 Counter Register"]
    pub trig1_counter: TRIG1_COUNTER,
    #[doc = "0x40 - ETC_TRIG Chain 0/1 Register"]
    pub trig1_chain_1_0: TRIG1_CHAIN_1_0,
    #[doc = "0x44 - ETC_TRIG Chain 2/3 Register"]
    pub trig1_chain_3_2: TRIG1_CHAIN_3_2,
    #[doc = "0x48 - ETC_TRIG Chain 4/5 Register"]
    pub trig1_chain_5_4: TRIG1_CHAIN_5_4,
    #[doc = "0x4c - ETC_TRIG Chain 6/7 Register"]
    pub trig1_chain_7_6: TRIG1_CHAIN_7_6,
    #[doc = "0x50 - ETC_TRIG Result Data 1/0 Register"]
    pub trig1_result_1_0: TRIG1_RESULT_1_0,
    #[doc = "0x54 - ETC_TRIG Result Data 3/2 Register"]
    pub trig1_result_3_2: TRIG1_RESULT_3_2,
    #[doc = "0x58 - ETC_TRIG Result Data 5/4 Register"]
    pub trig1_result_5_4: TRIG1_RESULT_5_4,
    #[doc = "0x5c - ETC_TRIG Result Data 7/6 Register"]
    pub trig1_result_7_6: TRIG1_RESULT_7_6,
    #[doc = "0x60 - ETC_TRIG2 Control Register"]
    pub trig2_ctrl: TRIG2_CTRL,
    #[doc = "0x64 - ETC_TRIG2 Counter Register"]
    pub trig2_counter: TRIG2_COUNTER,
    #[doc = "0x68 - ETC_TRIG Chain 0/1 Register"]
    pub trig2_chain_1_0: TRIG2_CHAIN_1_0,
    #[doc = "0x6c - ETC_TRIG Chain 2/3 Register"]
    pub trig2_chain_3_2: TRIG2_CHAIN_3_2,
    #[doc = "0x70 - ETC_TRIG Chain 4/5 Register"]
    pub trig2_chain_5_4: TRIG2_CHAIN_5_4,
    #[doc = "0x74 - ETC_TRIG Chain 6/7 Register"]
    pub trig2_chain_7_6: TRIG2_CHAIN_7_6,
    #[doc = "0x78 - ETC_TRIG Result Data 1/0 Register"]
    pub trig2_result_1_0: TRIG2_RESULT_1_0,
    #[doc = "0x7c - ETC_TRIG Result Data 3/2 Register"]
    pub trig2_result_3_2: TRIG2_RESULT_3_2,
    #[doc = "0x80 - ETC_TRIG Result Data 5/4 Register"]
    pub trig2_result_5_4: TRIG2_RESULT_5_4,
    #[doc = "0x84 - ETC_TRIG Result Data 7/6 Register"]
    pub trig2_result_7_6: TRIG2_RESULT_7_6,
    #[doc = "0x88 - ETC_TRIG3 Control Register"]
    pub trig3_ctrl: TRIG3_CTRL,
    #[doc = "0x8c - ETC_TRIG3 Counter Register"]
    pub trig3_counter: TRIG3_COUNTER,
    #[doc = "0x90 - ETC_TRIG Chain 0/1 Register"]
    pub trig3_chain_1_0: TRIG3_CHAIN_1_0,
    #[doc = "0x94 - ETC_TRIG Chain 2/3 Register"]
    pub trig3_chain_3_2: TRIG3_CHAIN_3_2,
    #[doc = "0x98 - ETC_TRIG Chain 4/5 Register"]
    pub trig3_chain_5_4: TRIG3_CHAIN_5_4,
    #[doc = "0x9c - ETC_TRIG Chain 6/7 Register"]
    pub trig3_chain_7_6: TRIG3_CHAIN_7_6,
    #[doc = "0xa0 - ETC_TRIG Result Data 1/0 Register"]
    pub trig3_result_1_0: TRIG3_RESULT_1_0,
    #[doc = "0xa4 - ETC_TRIG Result Data 3/2 Register"]
    pub trig3_result_3_2: TRIG3_RESULT_3_2,
    #[doc = "0xa8 - ETC_TRIG Result Data 5/4 Register"]
    pub trig3_result_5_4: TRIG3_RESULT_5_4,
    #[doc = "0xac - ETC_TRIG Result Data 7/6 Register"]
    pub trig3_result_7_6: TRIG3_RESULT_7_6,
    #[doc = "0xb0 - ETC_TRIG4 Control Register"]
    pub trig4_ctrl: TRIG4_CTRL,
    #[doc = "0xb4 - ETC_TRIG4 Counter Register"]
    pub trig4_counter: TRIG4_COUNTER,
    #[doc = "0xb8 - ETC_TRIG Chain 0/1 Register"]
    pub trig4_chain_1_0: TRIG4_CHAIN_1_0,
    #[doc = "0xbc - ETC_TRIG Chain 2/3 Register"]
    pub trig4_chain_3_2: TRIG4_CHAIN_3_2,
    #[doc = "0xc0 - ETC_TRIG Chain 4/5 Register"]
    pub trig4_chain_5_4: TRIG4_CHAIN_5_4,
    #[doc = "0xc4 - ETC_TRIG Chain 6/7 Register"]
    pub trig4_chain_7_6: TRIG4_CHAIN_7_6,
    #[doc = "0xc8 - ETC_TRIG Result Data 1/0 Register"]
    pub trig4_result_1_0: TRIG4_RESULT_1_0,
    #[doc = "0xcc - ETC_TRIG Result Data 3/2 Register"]
    pub trig4_result_3_2: TRIG4_RESULT_3_2,
    #[doc = "0xd0 - ETC_TRIG Result Data 5/4 Register"]
    pub trig4_result_5_4: TRIG4_RESULT_5_4,
    #[doc = "0xd4 - ETC_TRIG Result Data 7/6 Register"]
    pub trig4_result_7_6: TRIG4_RESULT_7_6,
    #[doc = "0xd8 - ETC_TRIG5 Control Register"]
    pub trig5_ctrl: TRIG5_CTRL,
    #[doc = "0xdc - ETC_TRIG5 Counter Register"]
    pub trig5_counter: TRIG5_COUNTER,
    #[doc = "0xe0 - ETC_TRIG Chain 0/1 Register"]
    pub trig5_chain_1_0: TRIG5_CHAIN_1_0,
    #[doc = "0xe4 - ETC_TRIG Chain 2/3 Register"]
    pub trig5_chain_3_2: TRIG5_CHAIN_3_2,
    #[doc = "0xe8 - ETC_TRIG Chain 4/5 Register"]
    pub trig5_chain_5_4: TRIG5_CHAIN_5_4,
    #[doc = "0xec - ETC_TRIG Chain 6/7 Register"]
    pub trig5_chain_7_6: TRIG5_CHAIN_7_6,
    #[doc = "0xf0 - ETC_TRIG Result Data 1/0 Register"]
    pub trig5_result_1_0: TRIG5_RESULT_1_0,
    #[doc = "0xf4 - ETC_TRIG Result Data 3/2 Register"]
    pub trig5_result_3_2: TRIG5_RESULT_3_2,
    #[doc = "0xf8 - ETC_TRIG Result Data 5/4 Register"]
    pub trig5_result_5_4: TRIG5_RESULT_5_4,
    #[doc = "0xfc - ETC_TRIG Result Data 7/6 Register"]
    pub trig5_result_7_6: TRIG5_RESULT_7_6,
    #[doc = "0x100 - ETC_TRIG6 Control Register"]
    pub trig6_ctrl: TRIG6_CTRL,
    #[doc = "0x104 - ETC_TRIG6 Counter Register"]
    pub trig6_counter: TRIG6_COUNTER,
    #[doc = "0x108 - ETC_TRIG Chain 0/1 Register"]
    pub trig6_chain_1_0: TRIG6_CHAIN_1_0,
    #[doc = "0x10c - ETC_TRIG Chain 2/3 Register"]
    pub trig6_chain_3_2: TRIG6_CHAIN_3_2,
    #[doc = "0x110 - ETC_TRIG Chain 4/5 Register"]
    pub trig6_chain_5_4: TRIG6_CHAIN_5_4,
    #[doc = "0x114 - ETC_TRIG Chain 6/7 Register"]
    pub trig6_chain_7_6: TRIG6_CHAIN_7_6,
    #[doc = "0x118 - ETC_TRIG Result Data 1/0 Register"]
    pub trig6_result_1_0: TRIG6_RESULT_1_0,
    #[doc = "0x11c - ETC_TRIG Result Data 3/2 Register"]
    pub trig6_result_3_2: TRIG6_RESULT_3_2,
    #[doc = "0x120 - ETC_TRIG Result Data 5/4 Register"]
    pub trig6_result_5_4: TRIG6_RESULT_5_4,
    #[doc = "0x124 - ETC_TRIG Result Data 7/6 Register"]
    pub trig6_result_7_6: TRIG6_RESULT_7_6,
    #[doc = "0x128 - ETC_TRIG7 Control Register"]
    pub trig7_ctrl: TRIG7_CTRL,
    #[doc = "0x12c - ETC_TRIG7 Counter Register"]
    pub trig7_counter: TRIG7_COUNTER,
    #[doc = "0x130 - ETC_TRIG Chain 0/1 Register"]
    pub trig7_chain_1_0: TRIG7_CHAIN_1_0,
    #[doc = "0x134 - ETC_TRIG Chain 2/3 Register"]
    pub trig7_chain_3_2: TRIG7_CHAIN_3_2,
    #[doc = "0x138 - ETC_TRIG Chain 4/5 Register"]
    pub trig7_chain_5_4: TRIG7_CHAIN_5_4,
    #[doc = "0x13c - ETC_TRIG Chain 6/7 Register"]
    pub trig7_chain_7_6: TRIG7_CHAIN_7_6,
    #[doc = "0x140 - ETC_TRIG Result Data 1/0 Register"]
    pub trig7_result_1_0: TRIG7_RESULT_1_0,
    #[doc = "0x144 - ETC_TRIG Result Data 3/2 Register"]
    pub trig7_result_3_2: TRIG7_RESULT_3_2,
    #[doc = "0x148 - ETC_TRIG Result Data 5/4 Register"]
    pub trig7_result_5_4: TRIG7_RESULT_5_4,
    #[doc = "0x14c - ETC_TRIG Result Data 7/6 Register"]
    pub trig7_result_7_6: TRIG7_RESULT_7_6,
}
#[doc = "ADC_ETC Global Control Register"]
pub struct CTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ADC_ETC Global Control Register"]
pub mod ctrl;
#[doc = "ETC DONE0 and DONE1 IRQ State Register"]
pub struct DONE0_1_IRQ {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ETC DONE0 and DONE1 IRQ State Register"]
pub mod done0_1_irq;
#[doc = "ETC DONE_2 and DONE_ERR IRQ State Register"]
pub struct DONE2_ERR_IRQ {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ETC DONE_2 and DONE_ERR IRQ State Register"]
pub mod done2_err_irq;
#[doc = "ETC DMA control Register"]
pub struct DMA_CTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ETC DMA control Register"]
pub mod dma_ctrl;
#[doc = "ETC_TRIG0 Control Register"]
pub struct TRIG0_CTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ETC_TRIG0 Control Register"]
pub mod trig0_ctrl;
#[doc = "ETC_TRIG0 Counter Register"]
pub struct TRIG0_COUNTER {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ETC_TRIG0 Counter Register"]
pub mod trig0_counter;
#[doc = "ETC_TRIG Chain 0/1 Register"]
pub struct TRIG0_CHAIN_1_0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ETC_TRIG Chain 0/1 Register"]
pub mod trig0_chain_1_0;
#[doc = "ETC_TRIG Chain 2/3 Register"]
pub struct TRIG0_CHAIN_3_2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ETC_TRIG Chain 2/3 Register"]
pub mod trig0_chain_3_2;
#[doc = "ETC_TRIG Chain 4/5 Register"]
pub struct TRIG0_CHAIN_5_4 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ETC_TRIG Chain 4/5 Register"]
pub mod trig0_chain_5_4;
#[doc = "ETC_TRIG Chain 6/7 Register"]
pub struct TRIG0_CHAIN_7_6 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ETC_TRIG Chain 6/7 Register"]
pub mod trig0_chain_7_6;
#[doc = "ETC_TRIG Result Data 1/0 Register"]
pub struct TRIG0_RESULT_1_0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ETC_TRIG Result Data 1/0 Register"]
pub mod trig0_result_1_0;
#[doc = "ETC_TRIG Result Data 3/2 Register"]
pub struct TRIG0_RESULT_3_2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ETC_TRIG Result Data 3/2 Register"]
pub mod trig0_result_3_2;
#[doc = "ETC_TRIG Result Data 5/4 Register"]
pub struct TRIG0_RESULT_5_4 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ETC_TRIG Result Data 5/4 Register"]
pub mod trig0_result_5_4;
#[doc = "ETC_TRIG Result Data 7/6 Register"]
pub struct TRIG0_RESULT_7_6 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ETC_TRIG Result Data 7/6 Register"]
pub mod trig0_result_7_6;
#[doc = "ETC_TRIG1 Control Register"]
pub struct TRIG1_CTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ETC_TRIG1 Control Register"]
pub mod trig1_ctrl;
#[doc = "ETC_TRIG1 Counter Register"]
pub struct TRIG1_COUNTER {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ETC_TRIG1 Counter Register"]
pub mod trig1_counter;
#[doc = "ETC_TRIG Chain 0/1 Register"]
pub struct TRIG1_CHAIN_1_0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ETC_TRIG Chain 0/1 Register"]
pub mod trig1_chain_1_0;
#[doc = "ETC_TRIG Chain 2/3 Register"]
pub struct TRIG1_CHAIN_3_2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ETC_TRIG Chain 2/3 Register"]
pub mod trig1_chain_3_2;
#[doc = "ETC_TRIG Chain 4/5 Register"]
pub struct TRIG1_CHAIN_5_4 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ETC_TRIG Chain 4/5 Register"]
pub mod trig1_chain_5_4;
#[doc = "ETC_TRIG Chain 6/7 Register"]
pub struct TRIG1_CHAIN_7_6 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ETC_TRIG Chain 6/7 Register"]
pub mod trig1_chain_7_6;
#[doc = "ETC_TRIG Result Data 1/0 Register"]
pub struct TRIG1_RESULT_1_0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ETC_TRIG Result Data 1/0 Register"]
pub mod trig1_result_1_0;
#[doc = "ETC_TRIG Result Data 3/2 Register"]
pub struct TRIG1_RESULT_3_2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ETC_TRIG Result Data 3/2 Register"]
pub mod trig1_result_3_2;
#[doc = "ETC_TRIG Result Data 5/4 Register"]
pub struct TRIG1_RESULT_5_4 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ETC_TRIG Result Data 5/4 Register"]
pub mod trig1_result_5_4;
#[doc = "ETC_TRIG Result Data 7/6 Register"]
pub struct TRIG1_RESULT_7_6 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ETC_TRIG Result Data 7/6 Register"]
pub mod trig1_result_7_6;
#[doc = "ETC_TRIG2 Control Register"]
pub struct TRIG2_CTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ETC_TRIG2 Control Register"]
pub mod trig2_ctrl;
#[doc = "ETC_TRIG2 Counter Register"]
pub struct TRIG2_COUNTER {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ETC_TRIG2 Counter Register"]
pub mod trig2_counter;
#[doc = "ETC_TRIG Chain 0/1 Register"]
pub struct TRIG2_CHAIN_1_0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ETC_TRIG Chain 0/1 Register"]
pub mod trig2_chain_1_0;
#[doc = "ETC_TRIG Chain 2/3 Register"]
pub struct TRIG2_CHAIN_3_2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ETC_TRIG Chain 2/3 Register"]
pub mod trig2_chain_3_2;
#[doc = "ETC_TRIG Chain 4/5 Register"]
pub struct TRIG2_CHAIN_5_4 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ETC_TRIG Chain 4/5 Register"]
pub mod trig2_chain_5_4;
#[doc = "ETC_TRIG Chain 6/7 Register"]
pub struct TRIG2_CHAIN_7_6 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ETC_TRIG Chain 6/7 Register"]
pub mod trig2_chain_7_6;
#[doc = "ETC_TRIG Result Data 1/0 Register"]
pub struct TRIG2_RESULT_1_0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ETC_TRIG Result Data 1/0 Register"]
pub mod trig2_result_1_0;
#[doc = "ETC_TRIG Result Data 3/2 Register"]
pub struct TRIG2_RESULT_3_2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ETC_TRIG Result Data 3/2 Register"]
pub mod trig2_result_3_2;
#[doc = "ETC_TRIG Result Data 5/4 Register"]
pub struct TRIG2_RESULT_5_4 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ETC_TRIG Result Data 5/4 Register"]
pub mod trig2_result_5_4;
#[doc = "ETC_TRIG Result Data 7/6 Register"]
pub struct TRIG2_RESULT_7_6 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ETC_TRIG Result Data 7/6 Register"]
pub mod trig2_result_7_6;
#[doc = "ETC_TRIG3 Control Register"]
pub struct TRIG3_CTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ETC_TRIG3 Control Register"]
pub mod trig3_ctrl;
#[doc = "ETC_TRIG3 Counter Register"]
pub struct TRIG3_COUNTER {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ETC_TRIG3 Counter Register"]
pub mod trig3_counter;
#[doc = "ETC_TRIG Chain 0/1 Register"]
pub struct TRIG3_CHAIN_1_0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ETC_TRIG Chain 0/1 Register"]
pub mod trig3_chain_1_0;
#[doc = "ETC_TRIG Chain 2/3 Register"]
pub struct TRIG3_CHAIN_3_2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ETC_TRIG Chain 2/3 Register"]
pub mod trig3_chain_3_2;
#[doc = "ETC_TRIG Chain 4/5 Register"]
pub struct TRIG3_CHAIN_5_4 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ETC_TRIG Chain 4/5 Register"]
pub mod trig3_chain_5_4;
#[doc = "ETC_TRIG Chain 6/7 Register"]
pub struct TRIG3_CHAIN_7_6 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ETC_TRIG Chain 6/7 Register"]
pub mod trig3_chain_7_6;
#[doc = "ETC_TRIG Result Data 1/0 Register"]
pub struct TRIG3_RESULT_1_0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ETC_TRIG Result Data 1/0 Register"]
pub mod trig3_result_1_0;
#[doc = "ETC_TRIG Result Data 3/2 Register"]
pub struct TRIG3_RESULT_3_2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ETC_TRIG Result Data 3/2 Register"]
pub mod trig3_result_3_2;
#[doc = "ETC_TRIG Result Data 5/4 Register"]
pub struct TRIG3_RESULT_5_4 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ETC_TRIG Result Data 5/4 Register"]
pub mod trig3_result_5_4;
#[doc = "ETC_TRIG Result Data 7/6 Register"]
pub struct TRIG3_RESULT_7_6 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ETC_TRIG Result Data 7/6 Register"]
pub mod trig3_result_7_6;
#[doc = "ETC_TRIG4 Control Register"]
pub struct TRIG4_CTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ETC_TRIG4 Control Register"]
pub mod trig4_ctrl;
#[doc = "ETC_TRIG4 Counter Register"]
pub struct TRIG4_COUNTER {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ETC_TRIG4 Counter Register"]
pub mod trig4_counter;
#[doc = "ETC_TRIG Chain 0/1 Register"]
pub struct TRIG4_CHAIN_1_0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ETC_TRIG Chain 0/1 Register"]
pub mod trig4_chain_1_0;
#[doc = "ETC_TRIG Chain 2/3 Register"]
pub struct TRIG4_CHAIN_3_2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ETC_TRIG Chain 2/3 Register"]
pub mod trig4_chain_3_2;
#[doc = "ETC_TRIG Chain 4/5 Register"]
pub struct TRIG4_CHAIN_5_4 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ETC_TRIG Chain 4/5 Register"]
pub mod trig4_chain_5_4;
#[doc = "ETC_TRIG Chain 6/7 Register"]
pub struct TRIG4_CHAIN_7_6 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ETC_TRIG Chain 6/7 Register"]
pub mod trig4_chain_7_6;
#[doc = "ETC_TRIG Result Data 1/0 Register"]
pub struct TRIG4_RESULT_1_0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ETC_TRIG Result Data 1/0 Register"]
pub mod trig4_result_1_0;
#[doc = "ETC_TRIG Result Data 3/2 Register"]
pub struct TRIG4_RESULT_3_2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ETC_TRIG Result Data 3/2 Register"]
pub mod trig4_result_3_2;
#[doc = "ETC_TRIG Result Data 5/4 Register"]
pub struct TRIG4_RESULT_5_4 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ETC_TRIG Result Data 5/4 Register"]
pub mod trig4_result_5_4;
#[doc = "ETC_TRIG Result Data 7/6 Register"]
pub struct TRIG4_RESULT_7_6 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ETC_TRIG Result Data 7/6 Register"]
pub mod trig4_result_7_6;
#[doc = "ETC_TRIG5 Control Register"]
pub struct TRIG5_CTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ETC_TRIG5 Control Register"]
pub mod trig5_ctrl;
#[doc = "ETC_TRIG5 Counter Register"]
pub struct TRIG5_COUNTER {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ETC_TRIG5 Counter Register"]
pub mod trig5_counter;
#[doc = "ETC_TRIG Chain 0/1 Register"]
pub struct TRIG5_CHAIN_1_0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ETC_TRIG Chain 0/1 Register"]
pub mod trig5_chain_1_0;
#[doc = "ETC_TRIG Chain 2/3 Register"]
pub struct TRIG5_CHAIN_3_2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ETC_TRIG Chain 2/3 Register"]
pub mod trig5_chain_3_2;
#[doc = "ETC_TRIG Chain 4/5 Register"]
pub struct TRIG5_CHAIN_5_4 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ETC_TRIG Chain 4/5 Register"]
pub mod trig5_chain_5_4;
#[doc = "ETC_TRIG Chain 6/7 Register"]
pub struct TRIG5_CHAIN_7_6 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ETC_TRIG Chain 6/7 Register"]
pub mod trig5_chain_7_6;
#[doc = "ETC_TRIG Result Data 1/0 Register"]
pub struct TRIG5_RESULT_1_0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ETC_TRIG Result Data 1/0 Register"]
pub mod trig5_result_1_0;
#[doc = "ETC_TRIG Result Data 3/2 Register"]
pub struct TRIG5_RESULT_3_2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ETC_TRIG Result Data 3/2 Register"]
pub mod trig5_result_3_2;
#[doc = "ETC_TRIG Result Data 5/4 Register"]
pub struct TRIG5_RESULT_5_4 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ETC_TRIG Result Data 5/4 Register"]
pub mod trig5_result_5_4;
#[doc = "ETC_TRIG Result Data 7/6 Register"]
pub struct TRIG5_RESULT_7_6 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ETC_TRIG Result Data 7/6 Register"]
pub mod trig5_result_7_6;
#[doc = "ETC_TRIG6 Control Register"]
pub struct TRIG6_CTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ETC_TRIG6 Control Register"]
pub mod trig6_ctrl;
#[doc = "ETC_TRIG6 Counter Register"]
pub struct TRIG6_COUNTER {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ETC_TRIG6 Counter Register"]
pub mod trig6_counter;
#[doc = "ETC_TRIG Chain 0/1 Register"]
pub struct TRIG6_CHAIN_1_0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ETC_TRIG Chain 0/1 Register"]
pub mod trig6_chain_1_0;
#[doc = "ETC_TRIG Chain 2/3 Register"]
pub struct TRIG6_CHAIN_3_2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ETC_TRIG Chain 2/3 Register"]
pub mod trig6_chain_3_2;
#[doc = "ETC_TRIG Chain 4/5 Register"]
pub struct TRIG6_CHAIN_5_4 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ETC_TRIG Chain 4/5 Register"]
pub mod trig6_chain_5_4;
#[doc = "ETC_TRIG Chain 6/7 Register"]
pub struct TRIG6_CHAIN_7_6 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ETC_TRIG Chain 6/7 Register"]
pub mod trig6_chain_7_6;
#[doc = "ETC_TRIG Result Data 1/0 Register"]
pub struct TRIG6_RESULT_1_0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ETC_TRIG Result Data 1/0 Register"]
pub mod trig6_result_1_0;
#[doc = "ETC_TRIG Result Data 3/2 Register"]
pub struct TRIG6_RESULT_3_2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ETC_TRIG Result Data 3/2 Register"]
pub mod trig6_result_3_2;
#[doc = "ETC_TRIG Result Data 5/4 Register"]
pub struct TRIG6_RESULT_5_4 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ETC_TRIG Result Data 5/4 Register"]
pub mod trig6_result_5_4;
#[doc = "ETC_TRIG Result Data 7/6 Register"]
pub struct TRIG6_RESULT_7_6 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ETC_TRIG Result Data 7/6 Register"]
pub mod trig6_result_7_6;
#[doc = "ETC_TRIG7 Control Register"]
pub struct TRIG7_CTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ETC_TRIG7 Control Register"]
pub mod trig7_ctrl;
#[doc = "ETC_TRIG7 Counter Register"]
pub struct TRIG7_COUNTER {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ETC_TRIG7 Counter Register"]
pub mod trig7_counter;
#[doc = "ETC_TRIG Chain 0/1 Register"]
pub struct TRIG7_CHAIN_1_0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ETC_TRIG Chain 0/1 Register"]
pub mod trig7_chain_1_0;
#[doc = "ETC_TRIG Chain 2/3 Register"]
pub struct TRIG7_CHAIN_3_2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ETC_TRIG Chain 2/3 Register"]
pub mod trig7_chain_3_2;
#[doc = "ETC_TRIG Chain 4/5 Register"]
pub struct TRIG7_CHAIN_5_4 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ETC_TRIG Chain 4/5 Register"]
pub mod trig7_chain_5_4;
#[doc = "ETC_TRIG Chain 6/7 Register"]
pub struct TRIG7_CHAIN_7_6 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ETC_TRIG Chain 6/7 Register"]
pub mod trig7_chain_7_6;
#[doc = "ETC_TRIG Result Data 1/0 Register"]
pub struct TRIG7_RESULT_1_0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ETC_TRIG Result Data 1/0 Register"]
pub mod trig7_result_1_0;
#[doc = "ETC_TRIG Result Data 3/2 Register"]
pub struct TRIG7_RESULT_3_2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ETC_TRIG Result Data 3/2 Register"]
pub mod trig7_result_3_2;
#[doc = "ETC_TRIG Result Data 5/4 Register"]
pub struct TRIG7_RESULT_5_4 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ETC_TRIG Result Data 5/4 Register"]
pub mod trig7_result_5_4;
#[doc = "ETC_TRIG Result Data 7/6 Register"]
pub struct TRIG7_RESULT_7_6 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ETC_TRIG Result Data 7/6 Register"]
pub mod trig7_result_7_6;
