#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - BEE Control Register"]
    pub ctrl: CTRL,
    #[doc = "0x04 - no description available"]
    pub addr_offset0: ADDR_OFFSET0,
    #[doc = "0x08 - no description available"]
    pub addr_offset1: ADDR_OFFSET1,
    #[doc = "0x0c - no description available"]
    pub aes_key0_w0: AES_KEY0_W0,
    #[doc = "0x10 - no description available"]
    pub aes_key0_w1: AES_KEY0_W1,
    #[doc = "0x14 - no description available"]
    pub aes_key0_w2: AES_KEY0_W2,
    #[doc = "0x18 - no description available"]
    pub aes_key0_w3: AES_KEY0_W3,
    #[doc = "0x1c - no description available"]
    pub status: STATUS,
    #[doc = "0x20 - no description available"]
    pub ctr_nonce0_w0: CTR_NONCE0_W0,
    #[doc = "0x24 - no description available"]
    pub ctr_nonce0_w1: CTR_NONCE0_W1,
    #[doc = "0x28 - no description available"]
    pub ctr_nonce0_w2: CTR_NONCE0_W2,
    #[doc = "0x2c - no description available"]
    pub ctr_nonce0_w3: CTR_NONCE0_W3,
    #[doc = "0x30 - no description available"]
    pub ctr_nonce1_w0: CTR_NONCE1_W0,
    #[doc = "0x34 - no description available"]
    pub ctr_nonce1_w1: CTR_NONCE1_W1,
    #[doc = "0x38 - no description available"]
    pub ctr_nonce1_w2: CTR_NONCE1_W2,
    #[doc = "0x3c - no description available"]
    pub ctr_nonce1_w3: CTR_NONCE1_W3,
    #[doc = "0x40 - no description available"]
    pub region1_top: REGION1_TOP,
    #[doc = "0x44 - no description available"]
    pub region1_bot: REGION1_BOT,
}
#[doc = "BEE Control Register"]
pub struct CTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "BEE Control Register"]
pub mod ctrl;
#[doc = "no description available"]
pub struct ADDR_OFFSET0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "no description available"]
pub mod addr_offset0;
#[doc = "no description available"]
pub struct ADDR_OFFSET1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "no description available"]
pub mod addr_offset1;
#[doc = "no description available"]
pub struct AES_KEY0_W0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "no description available"]
pub mod aes_key0_w0;
#[doc = "no description available"]
pub struct AES_KEY0_W1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "no description available"]
pub mod aes_key0_w1;
#[doc = "no description available"]
pub struct AES_KEY0_W2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "no description available"]
pub mod aes_key0_w2;
#[doc = "no description available"]
pub struct AES_KEY0_W3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "no description available"]
pub mod aes_key0_w3;
#[doc = "no description available"]
pub struct STATUS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "no description available"]
pub mod status;
#[doc = "no description available"]
pub struct CTR_NONCE0_W0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "no description available"]
pub mod ctr_nonce0_w0;
#[doc = "no description available"]
pub struct CTR_NONCE0_W1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "no description available"]
pub mod ctr_nonce0_w1;
#[doc = "no description available"]
pub struct CTR_NONCE0_W2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "no description available"]
pub mod ctr_nonce0_w2;
#[doc = "no description available"]
pub struct CTR_NONCE0_W3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "no description available"]
pub mod ctr_nonce0_w3;
#[doc = "no description available"]
pub struct CTR_NONCE1_W0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "no description available"]
pub mod ctr_nonce1_w0;
#[doc = "no description available"]
pub struct CTR_NONCE1_W1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "no description available"]
pub mod ctr_nonce1_w1;
#[doc = "no description available"]
pub struct CTR_NONCE1_W2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "no description available"]
pub mod ctr_nonce1_w2;
#[doc = "no description available"]
pub struct CTR_NONCE1_W3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "no description available"]
pub mod ctr_nonce1_w3;
#[doc = "no description available"]
pub struct REGION1_TOP {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "no description available"]
pub mod region1_top;
#[doc = "no description available"]
pub struct REGION1_BOT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "no description available"]
pub mod region1_bot;
