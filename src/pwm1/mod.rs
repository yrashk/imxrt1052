#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Counter Register"]
    pub sm0cnt: SM0CNT,
    #[doc = "0x02 - Initial Count Register"]
    pub sm0init: SM0INIT,
    #[doc = "0x04 - Control 2 Register"]
    pub sm0ctrl2: SM0CTRL2,
    #[doc = "0x06 - Control Register"]
    pub sm0ctrl: SM0CTRL,
    _reserved0: [u8; 2usize],
    #[doc = "0x0a - Value Register 0"]
    pub sm0val0: SM0VAL0,
    #[doc = "0x0c - Fractional Value Register 1"]
    pub sm0fracval1: SM0FRACVAL1,
    #[doc = "0x0e - Value Register 1"]
    pub sm0val1: SM0VAL1,
    #[doc = "0x10 - Fractional Value Register 2"]
    pub sm0fracval2: SM0FRACVAL2,
    #[doc = "0x12 - Value Register 2"]
    pub sm0val2: SM0VAL2,
    #[doc = "0x14 - Fractional Value Register 3"]
    pub sm0fracval3: SM0FRACVAL3,
    #[doc = "0x16 - Value Register 3"]
    pub sm0val3: SM0VAL3,
    #[doc = "0x18 - Fractional Value Register 4"]
    pub sm0fracval4: SM0FRACVAL4,
    #[doc = "0x1a - Value Register 4"]
    pub sm0val4: SM0VAL4,
    #[doc = "0x1c - Fractional Value Register 5"]
    pub sm0fracval5: SM0FRACVAL5,
    #[doc = "0x1e - Value Register 5"]
    pub sm0val5: SM0VAL5,
    #[doc = "0x20 - Fractional Control Register"]
    pub sm0frctrl: SM0FRCTRL,
    #[doc = "0x22 - Output Control Register"]
    pub sm0octrl: SM0OCTRL,
    #[doc = "0x24 - Status Register"]
    pub sm0sts: SM0STS,
    #[doc = "0x26 - Interrupt Enable Register"]
    pub sm0inten: SM0INTEN,
    #[doc = "0x28 - DMA Enable Register"]
    pub sm0dmaen: SM0DMAEN,
    #[doc = "0x2a - Output Trigger Control Register"]
    pub sm0tctrl: SM0TCTRL,
    #[doc = "0x2c - Fault Disable Mapping Register 0"]
    pub sm0dismap0: SM0DISMAP0,
    #[doc = "0x2e - Fault Disable Mapping Register 1"]
    pub sm0dismap1: SM0DISMAP1,
    #[doc = "0x30 - Deadtime Count Register 0"]
    pub sm0dtcnt0: SM0DTCNT0,
    #[doc = "0x32 - Deadtime Count Register 1"]
    pub sm0dtcnt1: SM0DTCNT1,
    #[doc = "0x34 - Capture Control A Register"]
    pub sm0captctrla: SM0CAPTCTRLA,
    #[doc = "0x36 - Capture Compare A Register"]
    pub sm0captcompa: SM0CAPTCOMPA,
    #[doc = "0x38 - Capture Control B Register"]
    pub sm0captctrlb: SM0CAPTCTRLB,
    #[doc = "0x3a - Capture Compare B Register"]
    pub sm0captcompb: SM0CAPTCOMPB,
    #[doc = "0x3c - Capture Control X Register"]
    pub sm0captctrlx: SM0CAPTCTRLX,
    #[doc = "0x3e - Capture Compare X Register"]
    pub sm0captcompx: SM0CAPTCOMPX,
    #[doc = "0x40 - Capture Value 0 Register"]
    pub sm0cval0: SM0CVAL0,
    #[doc = "0x42 - Capture Value 0 Cycle Register"]
    pub sm0cval0cyc: SM0CVAL0CYC,
    #[doc = "0x44 - Capture Value 1 Register"]
    pub sm0cval1: SM0CVAL1,
    #[doc = "0x46 - Capture Value 1 Cycle Register"]
    pub sm0cval1cyc: SM0CVAL1CYC,
    #[doc = "0x48 - Capture Value 2 Register"]
    pub sm0cval2: SM0CVAL2,
    #[doc = "0x4a - Capture Value 2 Cycle Register"]
    pub sm0cval2cyc: SM0CVAL2CYC,
    #[doc = "0x4c - Capture Value 3 Register"]
    pub sm0cval3: SM0CVAL3,
    #[doc = "0x4e - Capture Value 3 Cycle Register"]
    pub sm0cval3cyc: SM0CVAL3CYC,
    #[doc = "0x50 - Capture Value 4 Register"]
    pub sm0cval4: SM0CVAL4,
    #[doc = "0x52 - Capture Value 4 Cycle Register"]
    pub sm0cval4cyc: SM0CVAL4CYC,
    #[doc = "0x54 - Capture Value 5 Register"]
    pub sm0cval5: SM0CVAL5,
    #[doc = "0x56 - Capture Value 5 Cycle Register"]
    pub sm0cval5cyc: SM0CVAL5CYC,
    _reserved1: [u8; 8usize],
    #[doc = "0x60 - Counter Register"]
    pub sm1cnt: SM1CNT,
    #[doc = "0x62 - Initial Count Register"]
    pub sm1init: SM1INIT,
    #[doc = "0x64 - Control 2 Register"]
    pub sm1ctrl2: SM1CTRL2,
    #[doc = "0x66 - Control Register"]
    pub sm1ctrl: SM1CTRL,
    _reserved2: [u8; 2usize],
    #[doc = "0x6a - Value Register 0"]
    pub sm1val0: SM1VAL0,
    #[doc = "0x6c - Fractional Value Register 1"]
    pub sm1fracval1: SM1FRACVAL1,
    #[doc = "0x6e - Value Register 1"]
    pub sm1val1: SM1VAL1,
    #[doc = "0x70 - Fractional Value Register 2"]
    pub sm1fracval2: SM1FRACVAL2,
    #[doc = "0x72 - Value Register 2"]
    pub sm1val2: SM1VAL2,
    #[doc = "0x74 - Fractional Value Register 3"]
    pub sm1fracval3: SM1FRACVAL3,
    #[doc = "0x76 - Value Register 3"]
    pub sm1val3: SM1VAL3,
    #[doc = "0x78 - Fractional Value Register 4"]
    pub sm1fracval4: SM1FRACVAL4,
    #[doc = "0x7a - Value Register 4"]
    pub sm1val4: SM1VAL4,
    #[doc = "0x7c - Fractional Value Register 5"]
    pub sm1fracval5: SM1FRACVAL5,
    #[doc = "0x7e - Value Register 5"]
    pub sm1val5: SM1VAL5,
    #[doc = "0x80 - Fractional Control Register"]
    pub sm1frctrl: SM1FRCTRL,
    #[doc = "0x82 - Output Control Register"]
    pub sm1octrl: SM1OCTRL,
    #[doc = "0x84 - Status Register"]
    pub sm1sts: SM1STS,
    #[doc = "0x86 - Interrupt Enable Register"]
    pub sm1inten: SM1INTEN,
    #[doc = "0x88 - DMA Enable Register"]
    pub sm1dmaen: SM1DMAEN,
    #[doc = "0x8a - Output Trigger Control Register"]
    pub sm1tctrl: SM1TCTRL,
    #[doc = "0x8c - Fault Disable Mapping Register 0"]
    pub sm1dismap0: SM1DISMAP0,
    #[doc = "0x8e - Fault Disable Mapping Register 1"]
    pub sm1dismap1: SM1DISMAP1,
    #[doc = "0x90 - Deadtime Count Register 0"]
    pub sm1dtcnt0: SM1DTCNT0,
    #[doc = "0x92 - Deadtime Count Register 1"]
    pub sm1dtcnt1: SM1DTCNT1,
    #[doc = "0x94 - Capture Control A Register"]
    pub sm1captctrla: SM1CAPTCTRLA,
    #[doc = "0x96 - Capture Compare A Register"]
    pub sm1captcompa: SM1CAPTCOMPA,
    #[doc = "0x98 - Capture Control B Register"]
    pub sm1captctrlb: SM1CAPTCTRLB,
    #[doc = "0x9a - Capture Compare B Register"]
    pub sm1captcompb: SM1CAPTCOMPB,
    #[doc = "0x9c - Capture Control X Register"]
    pub sm1captctrlx: SM1CAPTCTRLX,
    #[doc = "0x9e - Capture Compare X Register"]
    pub sm1captcompx: SM1CAPTCOMPX,
    #[doc = "0xa0 - Capture Value 0 Register"]
    pub sm1cval0: SM1CVAL0,
    #[doc = "0xa2 - Capture Value 0 Cycle Register"]
    pub sm1cval0cyc: SM1CVAL0CYC,
    #[doc = "0xa4 - Capture Value 1 Register"]
    pub sm1cval1: SM1CVAL1,
    #[doc = "0xa6 - Capture Value 1 Cycle Register"]
    pub sm1cval1cyc: SM1CVAL1CYC,
    #[doc = "0xa8 - Capture Value 2 Register"]
    pub sm1cval2: SM1CVAL2,
    #[doc = "0xaa - Capture Value 2 Cycle Register"]
    pub sm1cval2cyc: SM1CVAL2CYC,
    #[doc = "0xac - Capture Value 3 Register"]
    pub sm1cval3: SM1CVAL3,
    #[doc = "0xae - Capture Value 3 Cycle Register"]
    pub sm1cval3cyc: SM1CVAL3CYC,
    #[doc = "0xb0 - Capture Value 4 Register"]
    pub sm1cval4: SM1CVAL4,
    #[doc = "0xb2 - Capture Value 4 Cycle Register"]
    pub sm1cval4cyc: SM1CVAL4CYC,
    #[doc = "0xb4 - Capture Value 5 Register"]
    pub sm1cval5: SM1CVAL5,
    #[doc = "0xb6 - Capture Value 5 Cycle Register"]
    pub sm1cval5cyc: SM1CVAL5CYC,
    _reserved3: [u8; 8usize],
    #[doc = "0xc0 - Counter Register"]
    pub sm2cnt: SM2CNT,
    #[doc = "0xc2 - Initial Count Register"]
    pub sm2init: SM2INIT,
    #[doc = "0xc4 - Control 2 Register"]
    pub sm2ctrl2: SM2CTRL2,
    #[doc = "0xc6 - Control Register"]
    pub sm2ctrl: SM2CTRL,
    _reserved4: [u8; 2usize],
    #[doc = "0xca - Value Register 0"]
    pub sm2val0: SM2VAL0,
    #[doc = "0xcc - Fractional Value Register 1"]
    pub sm2fracval1: SM2FRACVAL1,
    #[doc = "0xce - Value Register 1"]
    pub sm2val1: SM2VAL1,
    #[doc = "0xd0 - Fractional Value Register 2"]
    pub sm2fracval2: SM2FRACVAL2,
    #[doc = "0xd2 - Value Register 2"]
    pub sm2val2: SM2VAL2,
    #[doc = "0xd4 - Fractional Value Register 3"]
    pub sm2fracval3: SM2FRACVAL3,
    #[doc = "0xd6 - Value Register 3"]
    pub sm2val3: SM2VAL3,
    #[doc = "0xd8 - Fractional Value Register 4"]
    pub sm2fracval4: SM2FRACVAL4,
    #[doc = "0xda - Value Register 4"]
    pub sm2val4: SM2VAL4,
    #[doc = "0xdc - Fractional Value Register 5"]
    pub sm2fracval5: SM2FRACVAL5,
    #[doc = "0xde - Value Register 5"]
    pub sm2val5: SM2VAL5,
    #[doc = "0xe0 - Fractional Control Register"]
    pub sm2frctrl: SM2FRCTRL,
    #[doc = "0xe2 - Output Control Register"]
    pub sm2octrl: SM2OCTRL,
    #[doc = "0xe4 - Status Register"]
    pub sm2sts: SM2STS,
    #[doc = "0xe6 - Interrupt Enable Register"]
    pub sm2inten: SM2INTEN,
    #[doc = "0xe8 - DMA Enable Register"]
    pub sm2dmaen: SM2DMAEN,
    #[doc = "0xea - Output Trigger Control Register"]
    pub sm2tctrl: SM2TCTRL,
    #[doc = "0xec - Fault Disable Mapping Register 0"]
    pub sm2dismap0: SM2DISMAP0,
    #[doc = "0xee - Fault Disable Mapping Register 1"]
    pub sm2dismap1: SM2DISMAP1,
    #[doc = "0xf0 - Deadtime Count Register 0"]
    pub sm2dtcnt0: SM2DTCNT0,
    #[doc = "0xf2 - Deadtime Count Register 1"]
    pub sm2dtcnt1: SM2DTCNT1,
    #[doc = "0xf4 - Capture Control A Register"]
    pub sm2captctrla: SM2CAPTCTRLA,
    #[doc = "0xf6 - Capture Compare A Register"]
    pub sm2captcompa: SM2CAPTCOMPA,
    #[doc = "0xf8 - Capture Control B Register"]
    pub sm2captctrlb: SM2CAPTCTRLB,
    #[doc = "0xfa - Capture Compare B Register"]
    pub sm2captcompb: SM2CAPTCOMPB,
    #[doc = "0xfc - Capture Control X Register"]
    pub sm2captctrlx: SM2CAPTCTRLX,
    #[doc = "0xfe - Capture Compare X Register"]
    pub sm2captcompx: SM2CAPTCOMPX,
    #[doc = "0x100 - Capture Value 0 Register"]
    pub sm2cval0: SM2CVAL0,
    #[doc = "0x102 - Capture Value 0 Cycle Register"]
    pub sm2cval0cyc: SM2CVAL0CYC,
    #[doc = "0x104 - Capture Value 1 Register"]
    pub sm2cval1: SM2CVAL1,
    #[doc = "0x106 - Capture Value 1 Cycle Register"]
    pub sm2cval1cyc: SM2CVAL1CYC,
    #[doc = "0x108 - Capture Value 2 Register"]
    pub sm2cval2: SM2CVAL2,
    #[doc = "0x10a - Capture Value 2 Cycle Register"]
    pub sm2cval2cyc: SM2CVAL2CYC,
    #[doc = "0x10c - Capture Value 3 Register"]
    pub sm2cval3: SM2CVAL3,
    #[doc = "0x10e - Capture Value 3 Cycle Register"]
    pub sm2cval3cyc: SM2CVAL3CYC,
    #[doc = "0x110 - Capture Value 4 Register"]
    pub sm2cval4: SM2CVAL4,
    #[doc = "0x112 - Capture Value 4 Cycle Register"]
    pub sm2cval4cyc: SM2CVAL4CYC,
    #[doc = "0x114 - Capture Value 5 Register"]
    pub sm2cval5: SM2CVAL5,
    #[doc = "0x116 - Capture Value 5 Cycle Register"]
    pub sm2cval5cyc: SM2CVAL5CYC,
    _reserved5: [u8; 8usize],
    #[doc = "0x120 - Counter Register"]
    pub sm3cnt: SM3CNT,
    #[doc = "0x122 - Initial Count Register"]
    pub sm3init: SM3INIT,
    #[doc = "0x124 - Control 2 Register"]
    pub sm3ctrl2: SM3CTRL2,
    #[doc = "0x126 - Control Register"]
    pub sm3ctrl: SM3CTRL,
    _reserved6: [u8; 2usize],
    #[doc = "0x12a - Value Register 0"]
    pub sm3val0: SM3VAL0,
    #[doc = "0x12c - Fractional Value Register 1"]
    pub sm3fracval1: SM3FRACVAL1,
    #[doc = "0x12e - Value Register 1"]
    pub sm3val1: SM3VAL1,
    #[doc = "0x130 - Fractional Value Register 2"]
    pub sm3fracval2: SM3FRACVAL2,
    #[doc = "0x132 - Value Register 2"]
    pub sm3val2: SM3VAL2,
    #[doc = "0x134 - Fractional Value Register 3"]
    pub sm3fracval3: SM3FRACVAL3,
    #[doc = "0x136 - Value Register 3"]
    pub sm3val3: SM3VAL3,
    #[doc = "0x138 - Fractional Value Register 4"]
    pub sm3fracval4: SM3FRACVAL4,
    #[doc = "0x13a - Value Register 4"]
    pub sm3val4: SM3VAL4,
    #[doc = "0x13c - Fractional Value Register 5"]
    pub sm3fracval5: SM3FRACVAL5,
    #[doc = "0x13e - Value Register 5"]
    pub sm3val5: SM3VAL5,
    #[doc = "0x140 - Fractional Control Register"]
    pub sm3frctrl: SM3FRCTRL,
    #[doc = "0x142 - Output Control Register"]
    pub sm3octrl: SM3OCTRL,
    #[doc = "0x144 - Status Register"]
    pub sm3sts: SM3STS,
    #[doc = "0x146 - Interrupt Enable Register"]
    pub sm3inten: SM3INTEN,
    #[doc = "0x148 - DMA Enable Register"]
    pub sm3dmaen: SM3DMAEN,
    #[doc = "0x14a - Output Trigger Control Register"]
    pub sm3tctrl: SM3TCTRL,
    #[doc = "0x14c - Fault Disable Mapping Register 0"]
    pub sm3dismap0: SM3DISMAP0,
    #[doc = "0x14e - Fault Disable Mapping Register 1"]
    pub sm3dismap1: SM3DISMAP1,
    #[doc = "0x150 - Deadtime Count Register 0"]
    pub sm3dtcnt0: SM3DTCNT0,
    #[doc = "0x152 - Deadtime Count Register 1"]
    pub sm3dtcnt1: SM3DTCNT1,
    #[doc = "0x154 - Capture Control A Register"]
    pub sm3captctrla: SM3CAPTCTRLA,
    #[doc = "0x156 - Capture Compare A Register"]
    pub sm3captcompa: SM3CAPTCOMPA,
    #[doc = "0x158 - Capture Control B Register"]
    pub sm3captctrlb: SM3CAPTCTRLB,
    #[doc = "0x15a - Capture Compare B Register"]
    pub sm3captcompb: SM3CAPTCOMPB,
    #[doc = "0x15c - Capture Control X Register"]
    pub sm3captctrlx: SM3CAPTCTRLX,
    #[doc = "0x15e - Capture Compare X Register"]
    pub sm3captcompx: SM3CAPTCOMPX,
    #[doc = "0x160 - Capture Value 0 Register"]
    pub sm3cval0: SM3CVAL0,
    #[doc = "0x162 - Capture Value 0 Cycle Register"]
    pub sm3cval0cyc: SM3CVAL0CYC,
    #[doc = "0x164 - Capture Value 1 Register"]
    pub sm3cval1: SM3CVAL1,
    #[doc = "0x166 - Capture Value 1 Cycle Register"]
    pub sm3cval1cyc: SM3CVAL1CYC,
    #[doc = "0x168 - Capture Value 2 Register"]
    pub sm3cval2: SM3CVAL2,
    #[doc = "0x16a - Capture Value 2 Cycle Register"]
    pub sm3cval2cyc: SM3CVAL2CYC,
    #[doc = "0x16c - Capture Value 3 Register"]
    pub sm3cval3: SM3CVAL3,
    #[doc = "0x16e - Capture Value 3 Cycle Register"]
    pub sm3cval3cyc: SM3CVAL3CYC,
    #[doc = "0x170 - Capture Value 4 Register"]
    pub sm3cval4: SM3CVAL4,
    #[doc = "0x172 - Capture Value 4 Cycle Register"]
    pub sm3cval4cyc: SM3CVAL4CYC,
    #[doc = "0x174 - Capture Value 5 Register"]
    pub sm3cval5: SM3CVAL5,
    #[doc = "0x176 - Capture Value 5 Cycle Register"]
    pub sm3cval5cyc: SM3CVAL5CYC,
    _reserved7: [u8; 8usize],
    #[doc = "0x180 - Output Enable Register"]
    pub outen: OUTEN,
    #[doc = "0x182 - Mask Register"]
    pub mask: MASK,
    #[doc = "0x184 - Software Controlled Output Register"]
    pub swcout: SWCOUT,
    #[doc = "0x186 - PWM Source Select Register"]
    pub dtsrcsel: DTSRCSEL,
    #[doc = "0x188 - Master Control Register"]
    pub mctrl: MCTRL,
    #[doc = "0x18a - Master Control 2 Register"]
    pub mctrl2: MCTRL2,
    #[doc = "0x18c - Fault Control Register"]
    pub fctrl0: FCTRL0,
    #[doc = "0x18e - Fault Status Register"]
    pub fsts0: FSTS0,
    #[doc = "0x190 - Fault Filter Register"]
    pub ffilt0: FFILT0,
    #[doc = "0x192 - Fault Test Register"]
    pub ftst0: FTST0,
    #[doc = "0x194 - Fault Control 2 Register"]
    pub fctrl20: FCTRL20,
}
#[doc = "Counter Register"]
pub struct SM0CNT {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Counter Register"]
pub mod sm0cnt;
#[doc = "Initial Count Register"]
pub struct SM0INIT {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Initial Count Register"]
pub mod sm0init;
#[doc = "Control 2 Register"]
pub struct SM0CTRL2 {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Control 2 Register"]
pub mod sm0ctrl2;
#[doc = "Control Register"]
pub struct SM0CTRL {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Control Register"]
pub mod sm0ctrl;
#[doc = "Value Register 0"]
pub struct SM0VAL0 {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Value Register 0"]
pub mod sm0val0;
#[doc = "Fractional Value Register 1"]
pub struct SM0FRACVAL1 {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Fractional Value Register 1"]
pub mod sm0fracval1;
#[doc = "Value Register 1"]
pub struct SM0VAL1 {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Value Register 1"]
pub mod sm0val1;
#[doc = "Fractional Value Register 2"]
pub struct SM0FRACVAL2 {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Fractional Value Register 2"]
pub mod sm0fracval2;
#[doc = "Value Register 2"]
pub struct SM0VAL2 {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Value Register 2"]
pub mod sm0val2;
#[doc = "Fractional Value Register 3"]
pub struct SM0FRACVAL3 {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Fractional Value Register 3"]
pub mod sm0fracval3;
#[doc = "Value Register 3"]
pub struct SM0VAL3 {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Value Register 3"]
pub mod sm0val3;
#[doc = "Fractional Value Register 4"]
pub struct SM0FRACVAL4 {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Fractional Value Register 4"]
pub mod sm0fracval4;
#[doc = "Value Register 4"]
pub struct SM0VAL4 {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Value Register 4"]
pub mod sm0val4;
#[doc = "Fractional Value Register 5"]
pub struct SM0FRACVAL5 {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Fractional Value Register 5"]
pub mod sm0fracval5;
#[doc = "Value Register 5"]
pub struct SM0VAL5 {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Value Register 5"]
pub mod sm0val5;
#[doc = "Fractional Control Register"]
pub struct SM0FRCTRL {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Fractional Control Register"]
pub mod sm0frctrl;
#[doc = "Output Control Register"]
pub struct SM0OCTRL {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Output Control Register"]
pub mod sm0octrl;
#[doc = "Status Register"]
pub struct SM0STS {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Status Register"]
pub mod sm0sts;
#[doc = "Interrupt Enable Register"]
pub struct SM0INTEN {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Interrupt Enable Register"]
pub mod sm0inten;
#[doc = "DMA Enable Register"]
pub struct SM0DMAEN {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "DMA Enable Register"]
pub mod sm0dmaen;
#[doc = "Output Trigger Control Register"]
pub struct SM0TCTRL {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Output Trigger Control Register"]
pub mod sm0tctrl;
#[doc = "Fault Disable Mapping Register 0"]
pub struct SM0DISMAP0 {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Fault Disable Mapping Register 0"]
pub mod sm0dismap0;
#[doc = "Fault Disable Mapping Register 1"]
pub struct SM0DISMAP1 {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Fault Disable Mapping Register 1"]
pub mod sm0dismap1;
#[doc = "Deadtime Count Register 0"]
pub struct SM0DTCNT0 {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Deadtime Count Register 0"]
pub mod sm0dtcnt0;
#[doc = "Deadtime Count Register 1"]
pub struct SM0DTCNT1 {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Deadtime Count Register 1"]
pub mod sm0dtcnt1;
#[doc = "Capture Control A Register"]
pub struct SM0CAPTCTRLA {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Capture Control A Register"]
pub mod sm0captctrla;
#[doc = "Capture Compare A Register"]
pub struct SM0CAPTCOMPA {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Capture Compare A Register"]
pub mod sm0captcompa;
#[doc = "Capture Control B Register"]
pub struct SM0CAPTCTRLB {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Capture Control B Register"]
pub mod sm0captctrlb;
#[doc = "Capture Compare B Register"]
pub struct SM0CAPTCOMPB {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Capture Compare B Register"]
pub mod sm0captcompb;
#[doc = "Capture Control X Register"]
pub struct SM0CAPTCTRLX {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Capture Control X Register"]
pub mod sm0captctrlx;
#[doc = "Capture Compare X Register"]
pub struct SM0CAPTCOMPX {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Capture Compare X Register"]
pub mod sm0captcompx;
#[doc = "Capture Value 0 Register"]
pub struct SM0CVAL0 {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Capture Value 0 Register"]
pub mod sm0cval0;
#[doc = "Capture Value 0 Cycle Register"]
pub struct SM0CVAL0CYC {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Capture Value 0 Cycle Register"]
pub mod sm0cval0cyc;
#[doc = "Capture Value 1 Register"]
pub struct SM0CVAL1 {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Capture Value 1 Register"]
pub mod sm0cval1;
#[doc = "Capture Value 1 Cycle Register"]
pub struct SM0CVAL1CYC {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Capture Value 1 Cycle Register"]
pub mod sm0cval1cyc;
#[doc = "Capture Value 2 Register"]
pub struct SM0CVAL2 {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Capture Value 2 Register"]
pub mod sm0cval2;
#[doc = "Capture Value 2 Cycle Register"]
pub struct SM0CVAL2CYC {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Capture Value 2 Cycle Register"]
pub mod sm0cval2cyc;
#[doc = "Capture Value 3 Register"]
pub struct SM0CVAL3 {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Capture Value 3 Register"]
pub mod sm0cval3;
#[doc = "Capture Value 3 Cycle Register"]
pub struct SM0CVAL3CYC {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Capture Value 3 Cycle Register"]
pub mod sm0cval3cyc;
#[doc = "Capture Value 4 Register"]
pub struct SM0CVAL4 {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Capture Value 4 Register"]
pub mod sm0cval4;
#[doc = "Capture Value 4 Cycle Register"]
pub struct SM0CVAL4CYC {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Capture Value 4 Cycle Register"]
pub mod sm0cval4cyc;
#[doc = "Capture Value 5 Register"]
pub struct SM0CVAL5 {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Capture Value 5 Register"]
pub mod sm0cval5;
#[doc = "Capture Value 5 Cycle Register"]
pub struct SM0CVAL5CYC {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Capture Value 5 Cycle Register"]
pub mod sm0cval5cyc;
#[doc = "Counter Register"]
pub struct SM1CNT {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Counter Register"]
pub mod sm1cnt;
#[doc = "Initial Count Register"]
pub struct SM1INIT {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Initial Count Register"]
pub mod sm1init;
#[doc = "Control 2 Register"]
pub struct SM1CTRL2 {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Control 2 Register"]
pub mod sm1ctrl2;
#[doc = "Control Register"]
pub struct SM1CTRL {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Control Register"]
pub mod sm1ctrl;
#[doc = "Value Register 0"]
pub struct SM1VAL0 {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Value Register 0"]
pub mod sm1val0;
#[doc = "Fractional Value Register 1"]
pub struct SM1FRACVAL1 {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Fractional Value Register 1"]
pub mod sm1fracval1;
#[doc = "Value Register 1"]
pub struct SM1VAL1 {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Value Register 1"]
pub mod sm1val1;
#[doc = "Fractional Value Register 2"]
pub struct SM1FRACVAL2 {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Fractional Value Register 2"]
pub mod sm1fracval2;
#[doc = "Value Register 2"]
pub struct SM1VAL2 {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Value Register 2"]
pub mod sm1val2;
#[doc = "Fractional Value Register 3"]
pub struct SM1FRACVAL3 {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Fractional Value Register 3"]
pub mod sm1fracval3;
#[doc = "Value Register 3"]
pub struct SM1VAL3 {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Value Register 3"]
pub mod sm1val3;
#[doc = "Fractional Value Register 4"]
pub struct SM1FRACVAL4 {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Fractional Value Register 4"]
pub mod sm1fracval4;
#[doc = "Value Register 4"]
pub struct SM1VAL4 {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Value Register 4"]
pub mod sm1val4;
#[doc = "Fractional Value Register 5"]
pub struct SM1FRACVAL5 {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Fractional Value Register 5"]
pub mod sm1fracval5;
#[doc = "Value Register 5"]
pub struct SM1VAL5 {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Value Register 5"]
pub mod sm1val5;
#[doc = "Fractional Control Register"]
pub struct SM1FRCTRL {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Fractional Control Register"]
pub mod sm1frctrl;
#[doc = "Output Control Register"]
pub struct SM1OCTRL {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Output Control Register"]
pub mod sm1octrl;
#[doc = "Status Register"]
pub struct SM1STS {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Status Register"]
pub mod sm1sts;
#[doc = "Interrupt Enable Register"]
pub struct SM1INTEN {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Interrupt Enable Register"]
pub mod sm1inten;
#[doc = "DMA Enable Register"]
pub struct SM1DMAEN {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "DMA Enable Register"]
pub mod sm1dmaen;
#[doc = "Output Trigger Control Register"]
pub struct SM1TCTRL {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Output Trigger Control Register"]
pub mod sm1tctrl;
#[doc = "Fault Disable Mapping Register 0"]
pub struct SM1DISMAP0 {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Fault Disable Mapping Register 0"]
pub mod sm1dismap0;
#[doc = "Fault Disable Mapping Register 1"]
pub struct SM1DISMAP1 {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Fault Disable Mapping Register 1"]
pub mod sm1dismap1;
#[doc = "Deadtime Count Register 0"]
pub struct SM1DTCNT0 {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Deadtime Count Register 0"]
pub mod sm1dtcnt0;
#[doc = "Deadtime Count Register 1"]
pub struct SM1DTCNT1 {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Deadtime Count Register 1"]
pub mod sm1dtcnt1;
#[doc = "Capture Control A Register"]
pub struct SM1CAPTCTRLA {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Capture Control A Register"]
pub mod sm1captctrla;
#[doc = "Capture Compare A Register"]
pub struct SM1CAPTCOMPA {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Capture Compare A Register"]
pub mod sm1captcompa;
#[doc = "Capture Control B Register"]
pub struct SM1CAPTCTRLB {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Capture Control B Register"]
pub mod sm1captctrlb;
#[doc = "Capture Compare B Register"]
pub struct SM1CAPTCOMPB {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Capture Compare B Register"]
pub mod sm1captcompb;
#[doc = "Capture Control X Register"]
pub struct SM1CAPTCTRLX {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Capture Control X Register"]
pub mod sm1captctrlx;
#[doc = "Capture Compare X Register"]
pub struct SM1CAPTCOMPX {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Capture Compare X Register"]
pub mod sm1captcompx;
#[doc = "Capture Value 0 Register"]
pub struct SM1CVAL0 {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Capture Value 0 Register"]
pub mod sm1cval0;
#[doc = "Capture Value 0 Cycle Register"]
pub struct SM1CVAL0CYC {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Capture Value 0 Cycle Register"]
pub mod sm1cval0cyc;
#[doc = "Capture Value 1 Register"]
pub struct SM1CVAL1 {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Capture Value 1 Register"]
pub mod sm1cval1;
#[doc = "Capture Value 1 Cycle Register"]
pub struct SM1CVAL1CYC {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Capture Value 1 Cycle Register"]
pub mod sm1cval1cyc;
#[doc = "Capture Value 2 Register"]
pub struct SM1CVAL2 {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Capture Value 2 Register"]
pub mod sm1cval2;
#[doc = "Capture Value 2 Cycle Register"]
pub struct SM1CVAL2CYC {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Capture Value 2 Cycle Register"]
pub mod sm1cval2cyc;
#[doc = "Capture Value 3 Register"]
pub struct SM1CVAL3 {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Capture Value 3 Register"]
pub mod sm1cval3;
#[doc = "Capture Value 3 Cycle Register"]
pub struct SM1CVAL3CYC {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Capture Value 3 Cycle Register"]
pub mod sm1cval3cyc;
#[doc = "Capture Value 4 Register"]
pub struct SM1CVAL4 {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Capture Value 4 Register"]
pub mod sm1cval4;
#[doc = "Capture Value 4 Cycle Register"]
pub struct SM1CVAL4CYC {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Capture Value 4 Cycle Register"]
pub mod sm1cval4cyc;
#[doc = "Capture Value 5 Register"]
pub struct SM1CVAL5 {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Capture Value 5 Register"]
pub mod sm1cval5;
#[doc = "Capture Value 5 Cycle Register"]
pub struct SM1CVAL5CYC {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Capture Value 5 Cycle Register"]
pub mod sm1cval5cyc;
#[doc = "Counter Register"]
pub struct SM2CNT {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Counter Register"]
pub mod sm2cnt;
#[doc = "Initial Count Register"]
pub struct SM2INIT {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Initial Count Register"]
pub mod sm2init;
#[doc = "Control 2 Register"]
pub struct SM2CTRL2 {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Control 2 Register"]
pub mod sm2ctrl2;
#[doc = "Control Register"]
pub struct SM2CTRL {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Control Register"]
pub mod sm2ctrl;
#[doc = "Value Register 0"]
pub struct SM2VAL0 {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Value Register 0"]
pub mod sm2val0;
#[doc = "Fractional Value Register 1"]
pub struct SM2FRACVAL1 {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Fractional Value Register 1"]
pub mod sm2fracval1;
#[doc = "Value Register 1"]
pub struct SM2VAL1 {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Value Register 1"]
pub mod sm2val1;
#[doc = "Fractional Value Register 2"]
pub struct SM2FRACVAL2 {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Fractional Value Register 2"]
pub mod sm2fracval2;
#[doc = "Value Register 2"]
pub struct SM2VAL2 {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Value Register 2"]
pub mod sm2val2;
#[doc = "Fractional Value Register 3"]
pub struct SM2FRACVAL3 {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Fractional Value Register 3"]
pub mod sm2fracval3;
#[doc = "Value Register 3"]
pub struct SM2VAL3 {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Value Register 3"]
pub mod sm2val3;
#[doc = "Fractional Value Register 4"]
pub struct SM2FRACVAL4 {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Fractional Value Register 4"]
pub mod sm2fracval4;
#[doc = "Value Register 4"]
pub struct SM2VAL4 {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Value Register 4"]
pub mod sm2val4;
#[doc = "Fractional Value Register 5"]
pub struct SM2FRACVAL5 {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Fractional Value Register 5"]
pub mod sm2fracval5;
#[doc = "Value Register 5"]
pub struct SM2VAL5 {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Value Register 5"]
pub mod sm2val5;
#[doc = "Fractional Control Register"]
pub struct SM2FRCTRL {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Fractional Control Register"]
pub mod sm2frctrl;
#[doc = "Output Control Register"]
pub struct SM2OCTRL {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Output Control Register"]
pub mod sm2octrl;
#[doc = "Status Register"]
pub struct SM2STS {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Status Register"]
pub mod sm2sts;
#[doc = "Interrupt Enable Register"]
pub struct SM2INTEN {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Interrupt Enable Register"]
pub mod sm2inten;
#[doc = "DMA Enable Register"]
pub struct SM2DMAEN {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "DMA Enable Register"]
pub mod sm2dmaen;
#[doc = "Output Trigger Control Register"]
pub struct SM2TCTRL {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Output Trigger Control Register"]
pub mod sm2tctrl;
#[doc = "Fault Disable Mapping Register 0"]
pub struct SM2DISMAP0 {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Fault Disable Mapping Register 0"]
pub mod sm2dismap0;
#[doc = "Fault Disable Mapping Register 1"]
pub struct SM2DISMAP1 {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Fault Disable Mapping Register 1"]
pub mod sm2dismap1;
#[doc = "Deadtime Count Register 0"]
pub struct SM2DTCNT0 {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Deadtime Count Register 0"]
pub mod sm2dtcnt0;
#[doc = "Deadtime Count Register 1"]
pub struct SM2DTCNT1 {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Deadtime Count Register 1"]
pub mod sm2dtcnt1;
#[doc = "Capture Control A Register"]
pub struct SM2CAPTCTRLA {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Capture Control A Register"]
pub mod sm2captctrla;
#[doc = "Capture Compare A Register"]
pub struct SM2CAPTCOMPA {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Capture Compare A Register"]
pub mod sm2captcompa;
#[doc = "Capture Control B Register"]
pub struct SM2CAPTCTRLB {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Capture Control B Register"]
pub mod sm2captctrlb;
#[doc = "Capture Compare B Register"]
pub struct SM2CAPTCOMPB {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Capture Compare B Register"]
pub mod sm2captcompb;
#[doc = "Capture Control X Register"]
pub struct SM2CAPTCTRLX {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Capture Control X Register"]
pub mod sm2captctrlx;
#[doc = "Capture Compare X Register"]
pub struct SM2CAPTCOMPX {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Capture Compare X Register"]
pub mod sm2captcompx;
#[doc = "Capture Value 0 Register"]
pub struct SM2CVAL0 {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Capture Value 0 Register"]
pub mod sm2cval0;
#[doc = "Capture Value 0 Cycle Register"]
pub struct SM2CVAL0CYC {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Capture Value 0 Cycle Register"]
pub mod sm2cval0cyc;
#[doc = "Capture Value 1 Register"]
pub struct SM2CVAL1 {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Capture Value 1 Register"]
pub mod sm2cval1;
#[doc = "Capture Value 1 Cycle Register"]
pub struct SM2CVAL1CYC {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Capture Value 1 Cycle Register"]
pub mod sm2cval1cyc;
#[doc = "Capture Value 2 Register"]
pub struct SM2CVAL2 {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Capture Value 2 Register"]
pub mod sm2cval2;
#[doc = "Capture Value 2 Cycle Register"]
pub struct SM2CVAL2CYC {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Capture Value 2 Cycle Register"]
pub mod sm2cval2cyc;
#[doc = "Capture Value 3 Register"]
pub struct SM2CVAL3 {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Capture Value 3 Register"]
pub mod sm2cval3;
#[doc = "Capture Value 3 Cycle Register"]
pub struct SM2CVAL3CYC {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Capture Value 3 Cycle Register"]
pub mod sm2cval3cyc;
#[doc = "Capture Value 4 Register"]
pub struct SM2CVAL4 {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Capture Value 4 Register"]
pub mod sm2cval4;
#[doc = "Capture Value 4 Cycle Register"]
pub struct SM2CVAL4CYC {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Capture Value 4 Cycle Register"]
pub mod sm2cval4cyc;
#[doc = "Capture Value 5 Register"]
pub struct SM2CVAL5 {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Capture Value 5 Register"]
pub mod sm2cval5;
#[doc = "Capture Value 5 Cycle Register"]
pub struct SM2CVAL5CYC {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Capture Value 5 Cycle Register"]
pub mod sm2cval5cyc;
#[doc = "Counter Register"]
pub struct SM3CNT {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Counter Register"]
pub mod sm3cnt;
#[doc = "Initial Count Register"]
pub struct SM3INIT {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Initial Count Register"]
pub mod sm3init;
#[doc = "Control 2 Register"]
pub struct SM3CTRL2 {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Control 2 Register"]
pub mod sm3ctrl2;
#[doc = "Control Register"]
pub struct SM3CTRL {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Control Register"]
pub mod sm3ctrl;
#[doc = "Value Register 0"]
pub struct SM3VAL0 {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Value Register 0"]
pub mod sm3val0;
#[doc = "Fractional Value Register 1"]
pub struct SM3FRACVAL1 {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Fractional Value Register 1"]
pub mod sm3fracval1;
#[doc = "Value Register 1"]
pub struct SM3VAL1 {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Value Register 1"]
pub mod sm3val1;
#[doc = "Fractional Value Register 2"]
pub struct SM3FRACVAL2 {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Fractional Value Register 2"]
pub mod sm3fracval2;
#[doc = "Value Register 2"]
pub struct SM3VAL2 {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Value Register 2"]
pub mod sm3val2;
#[doc = "Fractional Value Register 3"]
pub struct SM3FRACVAL3 {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Fractional Value Register 3"]
pub mod sm3fracval3;
#[doc = "Value Register 3"]
pub struct SM3VAL3 {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Value Register 3"]
pub mod sm3val3;
#[doc = "Fractional Value Register 4"]
pub struct SM3FRACVAL4 {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Fractional Value Register 4"]
pub mod sm3fracval4;
#[doc = "Value Register 4"]
pub struct SM3VAL4 {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Value Register 4"]
pub mod sm3val4;
#[doc = "Fractional Value Register 5"]
pub struct SM3FRACVAL5 {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Fractional Value Register 5"]
pub mod sm3fracval5;
#[doc = "Value Register 5"]
pub struct SM3VAL5 {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Value Register 5"]
pub mod sm3val5;
#[doc = "Fractional Control Register"]
pub struct SM3FRCTRL {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Fractional Control Register"]
pub mod sm3frctrl;
#[doc = "Output Control Register"]
pub struct SM3OCTRL {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Output Control Register"]
pub mod sm3octrl;
#[doc = "Status Register"]
pub struct SM3STS {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Status Register"]
pub mod sm3sts;
#[doc = "Interrupt Enable Register"]
pub struct SM3INTEN {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Interrupt Enable Register"]
pub mod sm3inten;
#[doc = "DMA Enable Register"]
pub struct SM3DMAEN {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "DMA Enable Register"]
pub mod sm3dmaen;
#[doc = "Output Trigger Control Register"]
pub struct SM3TCTRL {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Output Trigger Control Register"]
pub mod sm3tctrl;
#[doc = "Fault Disable Mapping Register 0"]
pub struct SM3DISMAP0 {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Fault Disable Mapping Register 0"]
pub mod sm3dismap0;
#[doc = "Fault Disable Mapping Register 1"]
pub struct SM3DISMAP1 {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Fault Disable Mapping Register 1"]
pub mod sm3dismap1;
#[doc = "Deadtime Count Register 0"]
pub struct SM3DTCNT0 {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Deadtime Count Register 0"]
pub mod sm3dtcnt0;
#[doc = "Deadtime Count Register 1"]
pub struct SM3DTCNT1 {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Deadtime Count Register 1"]
pub mod sm3dtcnt1;
#[doc = "Capture Control A Register"]
pub struct SM3CAPTCTRLA {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Capture Control A Register"]
pub mod sm3captctrla;
#[doc = "Capture Compare A Register"]
pub struct SM3CAPTCOMPA {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Capture Compare A Register"]
pub mod sm3captcompa;
#[doc = "Capture Control B Register"]
pub struct SM3CAPTCTRLB {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Capture Control B Register"]
pub mod sm3captctrlb;
#[doc = "Capture Compare B Register"]
pub struct SM3CAPTCOMPB {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Capture Compare B Register"]
pub mod sm3captcompb;
#[doc = "Capture Control X Register"]
pub struct SM3CAPTCTRLX {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Capture Control X Register"]
pub mod sm3captctrlx;
#[doc = "Capture Compare X Register"]
pub struct SM3CAPTCOMPX {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Capture Compare X Register"]
pub mod sm3captcompx;
#[doc = "Capture Value 0 Register"]
pub struct SM3CVAL0 {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Capture Value 0 Register"]
pub mod sm3cval0;
#[doc = "Capture Value 0 Cycle Register"]
pub struct SM3CVAL0CYC {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Capture Value 0 Cycle Register"]
pub mod sm3cval0cyc;
#[doc = "Capture Value 1 Register"]
pub struct SM3CVAL1 {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Capture Value 1 Register"]
pub mod sm3cval1;
#[doc = "Capture Value 1 Cycle Register"]
pub struct SM3CVAL1CYC {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Capture Value 1 Cycle Register"]
pub mod sm3cval1cyc;
#[doc = "Capture Value 2 Register"]
pub struct SM3CVAL2 {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Capture Value 2 Register"]
pub mod sm3cval2;
#[doc = "Capture Value 2 Cycle Register"]
pub struct SM3CVAL2CYC {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Capture Value 2 Cycle Register"]
pub mod sm3cval2cyc;
#[doc = "Capture Value 3 Register"]
pub struct SM3CVAL3 {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Capture Value 3 Register"]
pub mod sm3cval3;
#[doc = "Capture Value 3 Cycle Register"]
pub struct SM3CVAL3CYC {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Capture Value 3 Cycle Register"]
pub mod sm3cval3cyc;
#[doc = "Capture Value 4 Register"]
pub struct SM3CVAL4 {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Capture Value 4 Register"]
pub mod sm3cval4;
#[doc = "Capture Value 4 Cycle Register"]
pub struct SM3CVAL4CYC {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Capture Value 4 Cycle Register"]
pub mod sm3cval4cyc;
#[doc = "Capture Value 5 Register"]
pub struct SM3CVAL5 {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Capture Value 5 Register"]
pub mod sm3cval5;
#[doc = "Capture Value 5 Cycle Register"]
pub struct SM3CVAL5CYC {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Capture Value 5 Cycle Register"]
pub mod sm3cval5cyc;
#[doc = "Output Enable Register"]
pub struct OUTEN {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Output Enable Register"]
pub mod outen;
#[doc = "Mask Register"]
pub struct MASK {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Mask Register"]
pub mod mask;
#[doc = "Software Controlled Output Register"]
pub struct SWCOUT {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Software Controlled Output Register"]
pub mod swcout;
#[doc = "PWM Source Select Register"]
pub struct DTSRCSEL {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "PWM Source Select Register"]
pub mod dtsrcsel;
#[doc = "Master Control Register"]
pub struct MCTRL {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Master Control Register"]
pub mod mctrl;
#[doc = "Master Control 2 Register"]
pub struct MCTRL2 {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Master Control 2 Register"]
pub mod mctrl2;
#[doc = "Fault Control Register"]
pub struct FCTRL0 {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Fault Control Register"]
pub mod fctrl0;
#[doc = "Fault Status Register"]
pub struct FSTS0 {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Fault Status Register"]
pub mod fsts0;
#[doc = "Fault Filter Register"]
pub struct FFILT0 {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Fault Filter Register"]
pub mod ffilt0;
#[doc = "Fault Test Register"]
pub struct FTST0 {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Fault Test Register"]
pub mod ftst0;
#[doc = "Fault Control 2 Register"]
pub struct FCTRL20 {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Fault Control 2 Register"]
pub mod fctrl20;
