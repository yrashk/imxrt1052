#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Crossbar B Select Register 0"]
    pub sel0: SEL0,
    #[doc = "0x02 - Crossbar B Select Register 1"]
    pub sel1: SEL1,
    #[doc = "0x04 - Crossbar B Select Register 2"]
    pub sel2: SEL2,
    #[doc = "0x06 - Crossbar B Select Register 3"]
    pub sel3: SEL3,
    #[doc = "0x08 - Crossbar B Select Register 4"]
    pub sel4: SEL4,
    #[doc = "0x0a - Crossbar B Select Register 5"]
    pub sel5: SEL5,
    #[doc = "0x0c - Crossbar B Select Register 6"]
    pub sel6: SEL6,
    #[doc = "0x0e - Crossbar B Select Register 7"]
    pub sel7: SEL7,
}
#[doc = "Crossbar B Select Register 0"]
pub struct SEL0 {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Crossbar B Select Register 0"]
pub mod sel0;
#[doc = "Crossbar B Select Register 1"]
pub struct SEL1 {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Crossbar B Select Register 1"]
pub mod sel1;
#[doc = "Crossbar B Select Register 2"]
pub struct SEL2 {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Crossbar B Select Register 2"]
pub mod sel2;
#[doc = "Crossbar B Select Register 3"]
pub struct SEL3 {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Crossbar B Select Register 3"]
pub mod sel3;
#[doc = "Crossbar B Select Register 4"]
pub struct SEL4 {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Crossbar B Select Register 4"]
pub mod sel4;
#[doc = "Crossbar B Select Register 5"]
pub struct SEL5 {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Crossbar B Select Register 5"]
pub mod sel5;
#[doc = "Crossbar B Select Register 6"]
pub struct SEL6 {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Crossbar B Select Register 6"]
pub mod sel6;
#[doc = "Crossbar B Select Register 7"]
pub struct SEL7 {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Crossbar B Select Register 7"]
pub mod sel7;
