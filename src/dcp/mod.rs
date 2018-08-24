#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - DCP control register 0"]
    pub ctrl: CTRL,
    _reserved0: [u8; 12usize],
    #[doc = "0x10 - DCP status register"]
    pub stat: STAT,
    _reserved1: [u8; 12usize],
    #[doc = "0x20 - DCP channel control register"]
    pub channelctrl: CHANNELCTRL,
    _reserved2: [u8; 12usize],
    #[doc = "0x30 - DCP capability 0 register"]
    pub capability0: CAPABILITY0,
    _reserved3: [u8; 12usize],
    #[doc = "0x40 - DCP capability 1 register"]
    pub capability1: CAPABILITY1,
    _reserved4: [u8; 12usize],
    #[doc = "0x50 - DCP context buffer pointer"]
    pub context: CONTEXT,
    _reserved5: [u8; 12usize],
    #[doc = "0x60 - DCP key index"]
    pub key: KEY,
    _reserved6: [u8; 12usize],
    #[doc = "0x70 - DCP key data"]
    pub keydata: KEYDATA,
    _reserved7: [u8; 12usize],
    #[doc = "0x80 - DCP work packet 0 status register"]
    pub packet0: PACKET0,
    _reserved8: [u8; 12usize],
    #[doc = "0x90 - DCP work packet 1 status register"]
    pub packet1: PACKET1,
    _reserved9: [u8; 12usize],
    #[doc = "0xa0 - DCP work packet 2 status register"]
    pub packet2: PACKET2,
    _reserved10: [u8; 12usize],
    #[doc = "0xb0 - DCP work packet 3 status register"]
    pub packet3: PACKET3,
    _reserved11: [u8; 12usize],
    #[doc = "0xc0 - DCP work packet 4 status register"]
    pub packet4: PACKET4,
    _reserved12: [u8; 12usize],
    #[doc = "0xd0 - DCP work packet 5 status register"]
    pub packet5: PACKET5,
    _reserved13: [u8; 12usize],
    #[doc = "0xe0 - DCP work packet 6 status register"]
    pub packet6: PACKET6,
    _reserved14: [u8; 28usize],
    #[doc = "0x100 - DCP channel 0 command pointer address register"]
    pub ch0cmdptr: CH0CMDPTR,
    _reserved15: [u8; 12usize],
    #[doc = "0x110 - DCP channel 0 semaphore register"]
    pub ch0sema: CH0SEMA,
    _reserved16: [u8; 12usize],
    #[doc = "0x120 - DCP channel 0 status register"]
    pub ch0stat: CH0STAT,
    _reserved17: [u8; 12usize],
    #[doc = "0x130 - DCP channel 0 options register"]
    pub ch0opts: CH0OPTS,
    _reserved18: [u8; 12usize],
    #[doc = "0x140 - DCP channel 1 command pointer address register"]
    pub ch1cmdptr: CH1CMDPTR,
    _reserved19: [u8; 12usize],
    #[doc = "0x150 - DCP channel 1 semaphore register"]
    pub ch1sema: CH1SEMA,
    _reserved20: [u8; 12usize],
    #[doc = "0x160 - DCP channel 1 status register"]
    pub ch1stat: CH1STAT,
    _reserved21: [u8; 12usize],
    #[doc = "0x170 - DCP channel 1 options register"]
    pub ch1opts: CH1OPTS,
    _reserved22: [u8; 12usize],
    #[doc = "0x180 - DCP channel 2 command pointer address register"]
    pub ch2cmdptr: CH2CMDPTR,
    _reserved23: [u8; 12usize],
    #[doc = "0x190 - DCP channel 2 semaphore register"]
    pub ch2sema: CH2SEMA,
    _reserved24: [u8; 12usize],
    #[doc = "0x1a0 - DCP channel 2 status register"]
    pub ch2stat: CH2STAT,
    _reserved25: [u8; 12usize],
    #[doc = "0x1b0 - DCP channel 2 options register"]
    pub ch2opts: CH2OPTS,
    _reserved26: [u8; 12usize],
    #[doc = "0x1c0 - DCP channel 3 command pointer address register"]
    pub ch3cmdptr: CH3CMDPTR,
    _reserved27: [u8; 12usize],
    #[doc = "0x1d0 - DCP channel 3 semaphore register"]
    pub ch3sema: CH3SEMA,
    _reserved28: [u8; 12usize],
    #[doc = "0x1e0 - DCP channel 3 status register"]
    pub ch3stat: CH3STAT,
    _reserved29: [u8; 12usize],
    #[doc = "0x1f0 - DCP channel 3 options register"]
    pub ch3opts: CH3OPTS,
    _reserved30: [u8; 524usize],
    #[doc = "0x400 - DCP debug select register"]
    pub dbgselect: DBGSELECT,
    _reserved31: [u8; 12usize],
    #[doc = "0x410 - DCP debug data register"]
    pub dbgdata: DBGDATA,
    _reserved32: [u8; 12usize],
    #[doc = "0x420 - DCP page table register"]
    pub pagetable: PAGETABLE,
    _reserved33: [u8; 12usize],
    #[doc = "0x430 - DCP version register"]
    pub version: VERSION,
}
#[doc = "DCP control register 0"]
pub struct CTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DCP control register 0"]
pub mod ctrl;
#[doc = "DCP status register"]
pub struct STAT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DCP status register"]
pub mod stat;
#[doc = "DCP channel control register"]
pub struct CHANNELCTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DCP channel control register"]
pub mod channelctrl;
#[doc = "DCP capability 0 register"]
pub struct CAPABILITY0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DCP capability 0 register"]
pub mod capability0;
#[doc = "DCP capability 1 register"]
pub struct CAPABILITY1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DCP capability 1 register"]
pub mod capability1;
#[doc = "DCP context buffer pointer"]
pub struct CONTEXT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DCP context buffer pointer"]
pub mod context;
#[doc = "DCP key index"]
pub struct KEY {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DCP key index"]
pub mod key;
#[doc = "DCP key data"]
pub struct KEYDATA {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DCP key data"]
pub mod keydata;
#[doc = "DCP work packet 0 status register"]
pub struct PACKET0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DCP work packet 0 status register"]
pub mod packet0;
#[doc = "DCP work packet 1 status register"]
pub struct PACKET1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DCP work packet 1 status register"]
pub mod packet1;
#[doc = "DCP work packet 2 status register"]
pub struct PACKET2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DCP work packet 2 status register"]
pub mod packet2;
#[doc = "DCP work packet 3 status register"]
pub struct PACKET3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DCP work packet 3 status register"]
pub mod packet3;
#[doc = "DCP work packet 4 status register"]
pub struct PACKET4 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DCP work packet 4 status register"]
pub mod packet4;
#[doc = "DCP work packet 5 status register"]
pub struct PACKET5 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DCP work packet 5 status register"]
pub mod packet5;
#[doc = "DCP work packet 6 status register"]
pub struct PACKET6 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DCP work packet 6 status register"]
pub mod packet6;
#[doc = "DCP channel 0 command pointer address register"]
pub struct CH0CMDPTR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DCP channel 0 command pointer address register"]
pub mod ch0cmdptr;
#[doc = "DCP channel 0 semaphore register"]
pub struct CH0SEMA {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DCP channel 0 semaphore register"]
pub mod ch0sema;
#[doc = "DCP channel 0 status register"]
pub struct CH0STAT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DCP channel 0 status register"]
pub mod ch0stat;
#[doc = "DCP channel 0 options register"]
pub struct CH0OPTS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DCP channel 0 options register"]
pub mod ch0opts;
#[doc = "DCP channel 1 command pointer address register"]
pub struct CH1CMDPTR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DCP channel 1 command pointer address register"]
pub mod ch1cmdptr;
#[doc = "DCP channel 1 semaphore register"]
pub struct CH1SEMA {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DCP channel 1 semaphore register"]
pub mod ch1sema;
#[doc = "DCP channel 1 status register"]
pub struct CH1STAT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DCP channel 1 status register"]
pub mod ch1stat;
#[doc = "DCP channel 1 options register"]
pub struct CH1OPTS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DCP channel 1 options register"]
pub mod ch1opts;
#[doc = "DCP channel 2 command pointer address register"]
pub struct CH2CMDPTR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DCP channel 2 command pointer address register"]
pub mod ch2cmdptr;
#[doc = "DCP channel 2 semaphore register"]
pub struct CH2SEMA {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DCP channel 2 semaphore register"]
pub mod ch2sema;
#[doc = "DCP channel 2 status register"]
pub struct CH2STAT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DCP channel 2 status register"]
pub mod ch2stat;
#[doc = "DCP channel 2 options register"]
pub struct CH2OPTS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DCP channel 2 options register"]
pub mod ch2opts;
#[doc = "DCP channel 3 command pointer address register"]
pub struct CH3CMDPTR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DCP channel 3 command pointer address register"]
pub mod ch3cmdptr;
#[doc = "DCP channel 3 semaphore register"]
pub struct CH3SEMA {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DCP channel 3 semaphore register"]
pub mod ch3sema;
#[doc = "DCP channel 3 status register"]
pub struct CH3STAT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DCP channel 3 status register"]
pub mod ch3stat;
#[doc = "DCP channel 3 options register"]
pub struct CH3OPTS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DCP channel 3 options register"]
pub mod ch3opts;
#[doc = "DCP debug select register"]
pub struct DBGSELECT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DCP debug select register"]
pub mod dbgselect;
#[doc = "DCP debug data register"]
pub struct DBGDATA {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DCP debug data register"]
pub mod dbgdata;
#[doc = "DCP page table register"]
pub struct PAGETABLE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DCP page table register"]
pub mod pagetable;
#[doc = "DCP version register"]
pub struct VERSION {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DCP version register"]
pub mod version;
