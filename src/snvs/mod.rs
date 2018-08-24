#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - SNVS_HP Lock Register"]
    pub hplr: HPLR,
    #[doc = "0x04 - SNVS_HP Command Register"]
    pub hpcomr: HPCOMR,
    #[doc = "0x08 - SNVS_HP Control Register"]
    pub hpcr: HPCR,
    #[doc = "0x0c - SNVS_HP Security Interrupt Control Register"]
    pub hpsicr: HPSICR,
    #[doc = "0x10 - SNVS_HP Security Violation Control Register"]
    pub hpsvcr: HPSVCR,
    #[doc = "0x14 - SNVS_HP Status Register"]
    pub hpsr: HPSR,
    #[doc = "0x18 - SNVS_HP Security Violation Status Register"]
    pub hpsvsr: HPSVSR,
    #[doc = "0x1c - SNVS_HP High Assurance Counter IV Register"]
    pub hphacivr: HPHACIVR,
    #[doc = "0x20 - SNVS_HP High Assurance Counter Register"]
    pub hphacr: HPHACR,
    #[doc = "0x24 - SNVS_HP Real Time Counter MSB Register"]
    pub hprtcmr: HPRTCMR,
    #[doc = "0x28 - SNVS_HP Real Time Counter LSB Register"]
    pub hprtclr: HPRTCLR,
    #[doc = "0x2c - SNVS_HP Time Alarm MSB Register"]
    pub hptamr: HPTAMR,
    #[doc = "0x30 - SNVS_HP Time Alarm LSB Register"]
    pub hptalr: HPTALR,
    #[doc = "0x34 - SNVS_LP Lock Register"]
    pub lplr: LPLR,
    #[doc = "0x38 - SNVS_LP Control Register"]
    pub lpcr: LPCR,
    #[doc = "0x3c - SNVS_LP Master Key Control Register"]
    pub lpmkcr: LPMKCR,
    #[doc = "0x40 - SNVS_LP Security Violation Control Register"]
    pub lpsvcr: LPSVCR,
    _reserved0: [u8; 4usize],
    #[doc = "0x48 - SNVS_LP Tamper Detectors Configuration Register"]
    pub lptdcr: LPTDCR,
    #[doc = "0x4c - SNVS_LP Status Register"]
    pub lpsr: LPSR,
    #[doc = "0x50 - SNVS_LP Secure Real Time Counter MSB Register"]
    pub lpsrtcmr: LPSRTCMR,
    #[doc = "0x54 - SNVS_LP Secure Real Time Counter LSB Register"]
    pub lpsrtclr: LPSRTCLR,
    #[doc = "0x58 - SNVS_LP Time Alarm Register"]
    pub lptar: LPTAR,
    #[doc = "0x5c - SNVS_LP Secure Monotonic Counter MSB Register"]
    pub lpsmcmr: LPSMCMR,
    #[doc = "0x60 - SNVS_LP Secure Monotonic Counter LSB Register"]
    pub lpsmclr: LPSMCLR,
    #[doc = "0x64 - SNVS_LP Power Glitch Detector Register"]
    pub lppgdr: LPPGDR,
    #[doc = "0x68 - SNVS_LP General Purpose Register 0 (legacy alias)"]
    pub lpgpr0_legacy_alias: LPGPR0_LEGACY_ALIAS,
    #[doc = "0x6c - SNVS_LP Zeroizable Master Key Register"]
    pub lpzmkr: [LPZMKR; 8],
    _reserved1: [u8; 4usize],
    #[doc = "0x90 - SNVS_LP General Purpose Registers 0 .. 3"]
    pub lpgpr_alias: [LPGPR_ALIAS; 4],
    _reserved2: [u8; 96usize],
    #[doc = "0x100 - SNVS_LP General Purpose Registers 0 .. 3"]
    pub lpgpr: [LPGPR; 4],
    _reserved3: [u8; 2792usize],
    #[doc = "0xbf8 - SNVS_HP Version ID Register 1"]
    pub hpvidr1: HPVIDR1,
    #[doc = "0xbfc - SNVS_HP Version ID Register 2"]
    pub hpvidr2: HPVIDR2,
}
#[doc = "SNVS_HP Lock Register"]
pub struct HPLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SNVS_HP Lock Register"]
pub mod hplr;
#[doc = "SNVS_HP Command Register"]
pub struct HPCOMR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SNVS_HP Command Register"]
pub mod hpcomr;
#[doc = "SNVS_HP Control Register"]
pub struct HPCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SNVS_HP Control Register"]
pub mod hpcr;
#[doc = "SNVS_HP Security Interrupt Control Register"]
pub struct HPSICR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SNVS_HP Security Interrupt Control Register"]
pub mod hpsicr;
#[doc = "SNVS_HP Security Violation Control Register"]
pub struct HPSVCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SNVS_HP Security Violation Control Register"]
pub mod hpsvcr;
#[doc = "SNVS_HP Status Register"]
pub struct HPSR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SNVS_HP Status Register"]
pub mod hpsr;
#[doc = "SNVS_HP Security Violation Status Register"]
pub struct HPSVSR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SNVS_HP Security Violation Status Register"]
pub mod hpsvsr;
#[doc = "SNVS_HP High Assurance Counter IV Register"]
pub struct HPHACIVR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SNVS_HP High Assurance Counter IV Register"]
pub mod hphacivr;
#[doc = "SNVS_HP High Assurance Counter Register"]
pub struct HPHACR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SNVS_HP High Assurance Counter Register"]
pub mod hphacr;
#[doc = "SNVS_HP Real Time Counter MSB Register"]
pub struct HPRTCMR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SNVS_HP Real Time Counter MSB Register"]
pub mod hprtcmr;
#[doc = "SNVS_HP Real Time Counter LSB Register"]
pub struct HPRTCLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SNVS_HP Real Time Counter LSB Register"]
pub mod hprtclr;
#[doc = "SNVS_HP Time Alarm MSB Register"]
pub struct HPTAMR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SNVS_HP Time Alarm MSB Register"]
pub mod hptamr;
#[doc = "SNVS_HP Time Alarm LSB Register"]
pub struct HPTALR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SNVS_HP Time Alarm LSB Register"]
pub mod hptalr;
#[doc = "SNVS_LP Lock Register"]
pub struct LPLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SNVS_LP Lock Register"]
pub mod lplr;
#[doc = "SNVS_LP Control Register"]
pub struct LPCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SNVS_LP Control Register"]
pub mod lpcr;
#[doc = "SNVS_LP Master Key Control Register"]
pub struct LPMKCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SNVS_LP Master Key Control Register"]
pub mod lpmkcr;
#[doc = "SNVS_LP Security Violation Control Register"]
pub struct LPSVCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SNVS_LP Security Violation Control Register"]
pub mod lpsvcr;
#[doc = "SNVS_LP Tamper Detectors Configuration Register"]
pub struct LPTDCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SNVS_LP Tamper Detectors Configuration Register"]
pub mod lptdcr;
#[doc = "SNVS_LP Status Register"]
pub struct LPSR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SNVS_LP Status Register"]
pub mod lpsr;
#[doc = "SNVS_LP Secure Real Time Counter MSB Register"]
pub struct LPSRTCMR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SNVS_LP Secure Real Time Counter MSB Register"]
pub mod lpsrtcmr;
#[doc = "SNVS_LP Secure Real Time Counter LSB Register"]
pub struct LPSRTCLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SNVS_LP Secure Real Time Counter LSB Register"]
pub mod lpsrtclr;
#[doc = "SNVS_LP Time Alarm Register"]
pub struct LPTAR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SNVS_LP Time Alarm Register"]
pub mod lptar;
#[doc = "SNVS_LP Secure Monotonic Counter MSB Register"]
pub struct LPSMCMR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SNVS_LP Secure Monotonic Counter MSB Register"]
pub mod lpsmcmr;
#[doc = "SNVS_LP Secure Monotonic Counter LSB Register"]
pub struct LPSMCLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SNVS_LP Secure Monotonic Counter LSB Register"]
pub mod lpsmclr;
#[doc = "SNVS_LP Power Glitch Detector Register"]
pub struct LPPGDR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SNVS_LP Power Glitch Detector Register"]
pub mod lppgdr;
#[doc = "SNVS_LP General Purpose Register 0 (legacy alias)"]
pub struct LPGPR0_LEGACY_ALIAS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SNVS_LP General Purpose Register 0 (legacy alias)"]
pub mod lpgpr0_legacy_alias;
#[doc = "SNVS_LP Zeroizable Master Key Register"]
pub struct LPZMKR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SNVS_LP Zeroizable Master Key Register"]
pub mod lpzmkr;
#[doc = "SNVS_LP General Purpose Registers 0 .. 3"]
pub struct LPGPR_ALIAS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SNVS_LP General Purpose Registers 0 .. 3"]
pub mod lpgpr_alias;
#[doc = "SNVS_LP General Purpose Registers 0 .. 3"]
pub struct LPGPR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SNVS_LP General Purpose Registers 0 .. 3"]
pub mod lpgpr;
#[doc = "SNVS_HP Version ID Register 1"]
pub struct HPVIDR1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SNVS_HP Version ID Register 1"]
pub mod hpvidr1;
#[doc = "SNVS_HP Version ID Register 2"]
pub struct HPVIDR2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SNVS_HP Version ID Register 2"]
pub mod hpvidr2;
