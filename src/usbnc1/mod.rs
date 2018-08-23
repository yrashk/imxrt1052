#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 2048usize],
    #[doc = "0x800 - USB OTG1 Control Register"]
    pub usb_otg1_ctrl: USB_OTG1_CTRL,
    _reserved1: [u8; 20usize],
    #[doc = "0x818 - OTG1 UTMI PHY Control 0 Register"]
    pub usb_otg1_phy_ctrl_0: USB_OTG1_PHY_CTRL_0,
}
#[doc = "USB OTG1 Control Register"]
pub struct USB_OTG1_CTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "USB OTG1 Control Register"]
pub mod usb_otg1_ctrl;
#[doc = "OTG1 UTMI PHY Control 0 Register"]
pub struct USB_OTG1_PHY_CTRL_0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "OTG1 UTMI PHY Control 0 Register"]
pub mod usb_otg1_phy_ctrl_0;
