#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Boolean Function Term 0 and 1 Configuration Register for EVENTn"]
    pub bfcrt010: BFCRT01,
    #[doc = "0x02 - Boolean Function Term 2 and 3 Configuration Register for EVENTn"]
    pub bfcrt230: BFCRT23,
    #[doc = "0x04 - Boolean Function Term 0 and 1 Configuration Register for EVENTn"]
    pub bfcrt011: BFCRT01,
    #[doc = "0x06 - Boolean Function Term 2 and 3 Configuration Register for EVENTn"]
    pub bfcrt231: BFCRT23,
    #[doc = "0x08 - Boolean Function Term 0 and 1 Configuration Register for EVENTn"]
    pub bfcrt012: BFCRT01,
    #[doc = "0x0a - Boolean Function Term 2 and 3 Configuration Register for EVENTn"]
    pub bfcrt232: BFCRT23,
    #[doc = "0x0c - Boolean Function Term 0 and 1 Configuration Register for EVENTn"]
    pub bfcrt013: BFCRT01,
    #[doc = "0x0e - Boolean Function Term 2 and 3 Configuration Register for EVENTn"]
    pub bfcrt233: BFCRT23,
}
#[doc = "Boolean Function Term 0 and 1 Configuration Register for EVENTn"]
pub struct BFCRT01 {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Boolean Function Term 0 and 1 Configuration Register for EVENTn"]
pub mod bfcrt01;
#[doc = "Boolean Function Term 2 and 3 Configuration Register for EVENTn"]
pub struct BFCRT23 {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Boolean Function Term 2 and 3 Configuration Register for EVENTn"]
pub mod bfcrt23;
