#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - GPT Control Register"]
    pub cr: CR,
    #[doc = "0x04 - GPT Prescaler Register"]
    pub pr: PR,
    #[doc = "0x08 - GPT Status Register"]
    pub sr: SR,
    #[doc = "0x0c - GPT Interrupt Register"]
    pub ir: IR,
    #[doc = "0x10 - GPT Output Compare Register 1"]
    pub ocr1: OCR1,
    #[doc = "0x14 - GPT Output Compare Register 2"]
    pub ocr2: OCR2,
    #[doc = "0x18 - GPT Output Compare Register 3"]
    pub ocr3: OCR3,
    #[doc = "0x1c - GPT Input Capture Register 1"]
    pub icr1: ICR1,
    #[doc = "0x20 - GPT Input Capture Register 2"]
    pub icr2: ICR2,
    #[doc = "0x24 - GPT Counter Register"]
    pub cnt: CNT,
}
#[doc = "GPT Control Register"]
pub struct CR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "GPT Control Register"]
pub mod cr;
#[doc = "GPT Prescaler Register"]
pub struct PR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "GPT Prescaler Register"]
pub mod pr;
#[doc = "GPT Status Register"]
pub struct SR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "GPT Status Register"]
pub mod sr;
#[doc = "GPT Interrupt Register"]
pub struct IR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "GPT Interrupt Register"]
pub mod ir;
#[doc = "GPT Output Compare Register 1"]
pub struct OCR1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "GPT Output Compare Register 1"]
pub mod ocr1;
#[doc = "GPT Output Compare Register 2"]
pub struct OCR2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "GPT Output Compare Register 2"]
pub mod ocr2;
#[doc = "GPT Output Compare Register 3"]
pub struct OCR3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "GPT Output Compare Register 3"]
pub mod ocr3;
#[doc = "GPT Input Capture Register 1"]
pub struct ICR1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "GPT Input Capture Register 1"]
pub mod icr1;
#[doc = "GPT Input Capture Register 2"]
pub struct ICR2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "GPT Input Capture Register 2"]
pub mod icr2;
#[doc = "GPT Counter Register"]
pub struct CNT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "GPT Counter Register"]
pub mod cnt;
