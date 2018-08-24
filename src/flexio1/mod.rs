#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Version ID Register"]
    pub verid: VERID,
    #[doc = "0x04 - Parameter Register"]
    pub param: PARAM,
    #[doc = "0x08 - FlexIO Control Register"]
    pub ctrl: CTRL,
    #[doc = "0x0c - Pin State Register"]
    pub pin: PIN,
    #[doc = "0x10 - Shifter Status Register"]
    pub shiftstat: SHIFTSTAT,
    #[doc = "0x14 - Shifter Error Register"]
    pub shifterr: SHIFTERR,
    #[doc = "0x18 - Timer Status Register"]
    pub timstat: TIMSTAT,
    _reserved0: [u8; 4usize],
    #[doc = "0x20 - Shifter Status Interrupt Enable"]
    pub shiftsien: SHIFTSIEN,
    #[doc = "0x24 - Shifter Error Interrupt Enable"]
    pub shifteien: SHIFTEIEN,
    #[doc = "0x28 - Timer Interrupt Enable Register"]
    pub timien: TIMIEN,
    _reserved1: [u8; 4usize],
    #[doc = "0x30 - Shifter Status DMA Enable"]
    pub shiftsden: SHIFTSDEN,
    _reserved2: [u8; 12usize],
    #[doc = "0x40 - Shifter State Register"]
    pub shiftstate: SHIFTSTATE,
    _reserved3: [u8; 60usize],
    #[doc = "0x80 - Shifter Control N Register"]
    pub shiftctl: [SHIFTCTL; 4],
    _reserved4: [u8; 112usize],
    #[doc = "0x100 - Shifter Configuration N Register"]
    pub shiftcfg: [SHIFTCFG; 4],
    _reserved5: [u8; 240usize],
    #[doc = "0x200 - Shifter Buffer N Register"]
    pub shiftbuf: [SHIFTBUF; 4],
    _reserved6: [u8; 112usize],
    #[doc = "0x280 - Shifter Buffer N Bit Swapped Register"]
    pub shiftbufbis: [SHIFTBUFBIS; 4],
    _reserved7: [u8; 112usize],
    #[doc = "0x300 - Shifter Buffer N Byte Swapped Register"]
    pub shiftbufbys: [SHIFTBUFBYS; 4],
    _reserved8: [u8; 112usize],
    #[doc = "0x380 - Shifter Buffer N Bit Byte Swapped Register"]
    pub shiftbufbbs: [SHIFTBUFBBS; 4],
    _reserved9: [u8; 112usize],
    #[doc = "0x400 - Timer Control N Register"]
    pub timctl: [TIMCTL; 4],
    _reserved10: [u8; 112usize],
    #[doc = "0x480 - Timer Configuration N Register"]
    pub timcfg: [TIMCFG; 4],
    _reserved11: [u8; 112usize],
    #[doc = "0x500 - Timer Compare N Register"]
    pub timcmp: [TIMCMP; 4],
    _reserved12: [u8; 368usize],
    #[doc = "0x680 - Shifter Buffer N Nibble Byte Swapped Register"]
    pub shiftbufnbs: [SHIFTBUFNBS; 4],
    _reserved13: [u8; 112usize],
    #[doc = "0x700 - Shifter Buffer N Half Word Swapped Register"]
    pub shiftbufhws: [SHIFTBUFHWS; 4],
    _reserved14: [u8; 112usize],
    #[doc = "0x780 - Shifter Buffer N Nibble Swapped Register"]
    pub shiftbufnis: [SHIFTBUFNIS; 4],
}
#[doc = "Version ID Register"]
pub struct VERID {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Version ID Register"]
pub mod verid;
#[doc = "Parameter Register"]
pub struct PARAM {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Parameter Register"]
pub mod param;
#[doc = "FlexIO Control Register"]
pub struct CTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "FlexIO Control Register"]
pub mod ctrl;
#[doc = "Pin State Register"]
pub struct PIN {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pin State Register"]
pub mod pin;
#[doc = "Shifter Status Register"]
pub struct SHIFTSTAT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Shifter Status Register"]
pub mod shiftstat;
#[doc = "Shifter Error Register"]
pub struct SHIFTERR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Shifter Error Register"]
pub mod shifterr;
#[doc = "Timer Status Register"]
pub struct TIMSTAT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Timer Status Register"]
pub mod timstat;
#[doc = "Shifter Status Interrupt Enable"]
pub struct SHIFTSIEN {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Shifter Status Interrupt Enable"]
pub mod shiftsien;
#[doc = "Shifter Error Interrupt Enable"]
pub struct SHIFTEIEN {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Shifter Error Interrupt Enable"]
pub mod shifteien;
#[doc = "Timer Interrupt Enable Register"]
pub struct TIMIEN {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Timer Interrupt Enable Register"]
pub mod timien;
#[doc = "Shifter Status DMA Enable"]
pub struct SHIFTSDEN {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Shifter Status DMA Enable"]
pub mod shiftsden;
#[doc = "Shifter State Register"]
pub struct SHIFTSTATE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Shifter State Register"]
pub mod shiftstate;
#[doc = "Shifter Control N Register"]
pub struct SHIFTCTL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Shifter Control N Register"]
pub mod shiftctl;
#[doc = "Shifter Configuration N Register"]
pub struct SHIFTCFG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Shifter Configuration N Register"]
pub mod shiftcfg;
#[doc = "Shifter Buffer N Register"]
pub struct SHIFTBUF {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Shifter Buffer N Register"]
pub mod shiftbuf;
#[doc = "Shifter Buffer N Bit Swapped Register"]
pub struct SHIFTBUFBIS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Shifter Buffer N Bit Swapped Register"]
pub mod shiftbufbis;
#[doc = "Shifter Buffer N Byte Swapped Register"]
pub struct SHIFTBUFBYS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Shifter Buffer N Byte Swapped Register"]
pub mod shiftbufbys;
#[doc = "Shifter Buffer N Bit Byte Swapped Register"]
pub struct SHIFTBUFBBS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Shifter Buffer N Bit Byte Swapped Register"]
pub mod shiftbufbbs;
#[doc = "Timer Control N Register"]
pub struct TIMCTL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Timer Control N Register"]
pub mod timctl;
#[doc = "Timer Configuration N Register"]
pub struct TIMCFG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Timer Configuration N Register"]
pub mod timcfg;
#[doc = "Timer Compare N Register"]
pub struct TIMCMP {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Timer Compare N Register"]
pub mod timcmp;
#[doc = "Shifter Buffer N Nibble Byte Swapped Register"]
pub struct SHIFTBUFNBS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Shifter Buffer N Nibble Byte Swapped Register"]
pub mod shiftbufnbs;
#[doc = "Shifter Buffer N Half Word Swapped Register"]
pub struct SHIFTBUFHWS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Shifter Buffer N Half Word Swapped Register"]
pub mod shiftbufhws;
#[doc = "Shifter Buffer N Nibble Swapped Register"]
pub struct SHIFTBUFNIS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Shifter Buffer N Nibble Swapped Register"]
pub mod shiftbufnis;
