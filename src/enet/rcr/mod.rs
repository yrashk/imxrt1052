#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::RCR {
    #[doc = r" Modifies the contents of the register"]
    #[inline]
    pub fn modify<F>(&self, f: F)
    where
        for<'w> F: FnOnce(&R, &'w mut W) -> &'w mut W,
    {
        let bits = self.register.get();
        let r = R { bits: bits };
        let mut w = W { bits: bits };
        f(&r, &mut w);
        self.register.set(w.bits);
    }
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
    #[doc = r" Writes to the register"]
    #[inline]
    pub fn write<F>(&self, f: F)
    where
        F: FnOnce(&mut W) -> &mut W,
    {
        let mut w = W::reset_value();
        f(&mut w);
        self.register.set(w.bits);
    }
    #[doc = r" Writes the reset value to the register"]
    #[inline]
    pub fn reset(&self) {
        self.write(|w| w)
    }
}
#[doc = "Possible values of the field `LOOP`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LOOPR {
    #[doc = "Loopback disabled."]
    LOOP_0,
    #[doc = "Transmitted frames are looped back internal to the device and transmit MII output signals are not asserted. DRT must be cleared."]
    LOOP_1,
}
impl LOOPR {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            LOOPR::LOOP_0 => false,
            LOOPR::LOOP_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> LOOPR {
        match value {
            false => LOOPR::LOOP_0,
            true => LOOPR::LOOP_1,
        }
    }
    #[doc = "Checks if the value of the field is `LOOP_0`"]
    #[inline]
    pub fn is_loop_0(&self) -> bool {
        *self == LOOPR::LOOP_0
    }
    #[doc = "Checks if the value of the field is `LOOP_1`"]
    #[inline]
    pub fn is_loop_1(&self) -> bool {
        *self == LOOPR::LOOP_1
    }
}
#[doc = "Possible values of the field `DRT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DRTR {
    #[doc = "Receive path operates independently of transmit (i.e., full-duplex mode). Can also be used to monitor transmit activity in half-duplex mode."]
    DRT_0,
    #[doc = "Disable reception of frames while transmitting. (Normally used for half-duplex mode.)"]
    DRT_1,
}
impl DRTR {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            DRTR::DRT_0 => false,
            DRTR::DRT_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DRTR {
        match value {
            false => DRTR::DRT_0,
            true => DRTR::DRT_1,
        }
    }
    #[doc = "Checks if the value of the field is `DRT_0`"]
    #[inline]
    pub fn is_drt_0(&self) -> bool {
        *self == DRTR::DRT_0
    }
    #[doc = "Checks if the value of the field is `DRT_1`"]
    #[inline]
    pub fn is_drt_1(&self) -> bool {
        *self == DRTR::DRT_1
    }
}
#[doc = "Possible values of the field `MII_MODE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MII_MODER {
    #[doc = "MII or RMII mode, as indicated by the RMII_MODE field."]
    MII_MODE_1,
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl MII_MODER {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            MII_MODER::MII_MODE_1 => true,
            MII_MODER::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> MII_MODER {
        match value {
            true => MII_MODER::MII_MODE_1,
            i => MII_MODER::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `MII_MODE_1`"]
    #[inline]
    pub fn is_mii_mode_1(&self) -> bool {
        *self == MII_MODER::MII_MODE_1
    }
}
#[doc = "Possible values of the field `PROM`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PROMR {
    #[doc = "Disabled."]
    PROM_0,
    #[doc = "Enabled."]
    PROM_1,
}
impl PROMR {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            PROMR::PROM_0 => false,
            PROMR::PROM_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PROMR {
        match value {
            false => PROMR::PROM_0,
            true => PROMR::PROM_1,
        }
    }
    #[doc = "Checks if the value of the field is `PROM_0`"]
    #[inline]
    pub fn is_prom_0(&self) -> bool {
        *self == PROMR::PROM_0
    }
    #[doc = "Checks if the value of the field is `PROM_1`"]
    #[inline]
    pub fn is_prom_1(&self) -> bool {
        *self == PROMR::PROM_1
    }
}
#[doc = r" Value of the field"]
pub struct BC_REJR {
    bits: bool,
}
impl BC_REJR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
#[doc = r" Value of the field"]
pub struct FCER {
    bits: bool,
}
impl FCER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
#[doc = "Possible values of the field `RMII_MODE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RMII_MODER {
    #[doc = "MAC configured for MII mode."]
    RMII_MODE_0,
    #[doc = "MAC configured for RMII operation."]
    RMII_MODE_1,
}
impl RMII_MODER {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            RMII_MODER::RMII_MODE_0 => false,
            RMII_MODER::RMII_MODE_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RMII_MODER {
        match value {
            false => RMII_MODER::RMII_MODE_0,
            true => RMII_MODER::RMII_MODE_1,
        }
    }
    #[doc = "Checks if the value of the field is `RMII_MODE_0`"]
    #[inline]
    pub fn is_rmii_mode_0(&self) -> bool {
        *self == RMII_MODER::RMII_MODE_0
    }
    #[doc = "Checks if the value of the field is `RMII_MODE_1`"]
    #[inline]
    pub fn is_rmii_mode_1(&self) -> bool {
        *self == RMII_MODER::RMII_MODE_1
    }
}
#[doc = "Possible values of the field `RMII_10T`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RMII_10TR {
    #[doc = "100-Mbit/s operation."]
    RMII_10T_0,
    #[doc = "10-Mbit/s operation."]
    RMII_10T_1,
}
impl RMII_10TR {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            RMII_10TR::RMII_10T_0 => false,
            RMII_10TR::RMII_10T_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RMII_10TR {
        match value {
            false => RMII_10TR::RMII_10T_0,
            true => RMII_10TR::RMII_10T_1,
        }
    }
    #[doc = "Checks if the value of the field is `RMII_10T_0`"]
    #[inline]
    pub fn is_rmii_10t_0(&self) -> bool {
        *self == RMII_10TR::RMII_10T_0
    }
    #[doc = "Checks if the value of the field is `RMII_10T_1`"]
    #[inline]
    pub fn is_rmii_10t_1(&self) -> bool {
        *self == RMII_10TR::RMII_10T_1
    }
}
#[doc = "Possible values of the field `PADEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PADENR {
    #[doc = "No padding is removed on receive by the MAC."]
    PADEN_0,
    #[doc = "Padding is removed from received frames."]
    PADEN_1,
}
impl PADENR {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            PADENR::PADEN_0 => false,
            PADENR::PADEN_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PADENR {
        match value {
            false => PADENR::PADEN_0,
            true => PADENR::PADEN_1,
        }
    }
    #[doc = "Checks if the value of the field is `PADEN_0`"]
    #[inline]
    pub fn is_paden_0(&self) -> bool {
        *self == PADENR::PADEN_0
    }
    #[doc = "Checks if the value of the field is `PADEN_1`"]
    #[inline]
    pub fn is_paden_1(&self) -> bool {
        *self == PADENR::PADEN_1
    }
}
#[doc = "Possible values of the field `PAUFWD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAUFWDR {
    #[doc = "Pause frames are terminated and discarded in the MAC."]
    PAUFWD_0,
    #[doc = "Pause frames are forwarded to the user application."]
    PAUFWD_1,
}
impl PAUFWDR {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            PAUFWDR::PAUFWD_0 => false,
            PAUFWDR::PAUFWD_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PAUFWDR {
        match value {
            false => PAUFWDR::PAUFWD_0,
            true => PAUFWDR::PAUFWD_1,
        }
    }
    #[doc = "Checks if the value of the field is `PAUFWD_0`"]
    #[inline]
    pub fn is_paufwd_0(&self) -> bool {
        *self == PAUFWDR::PAUFWD_0
    }
    #[doc = "Checks if the value of the field is `PAUFWD_1`"]
    #[inline]
    pub fn is_paufwd_1(&self) -> bool {
        *self == PAUFWDR::PAUFWD_1
    }
}
#[doc = "Possible values of the field `CRCFWD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CRCFWDR {
    #[doc = "The CRC field of received frames is transmitted to the user application."]
    CRCFWD_0,
    #[doc = "The CRC field is stripped from the frame."]
    CRCFWD_1,
}
impl CRCFWDR {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            CRCFWDR::CRCFWD_0 => false,
            CRCFWDR::CRCFWD_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CRCFWDR {
        match value {
            false => CRCFWDR::CRCFWD_0,
            true => CRCFWDR::CRCFWD_1,
        }
    }
    #[doc = "Checks if the value of the field is `CRCFWD_0`"]
    #[inline]
    pub fn is_crcfwd_0(&self) -> bool {
        *self == CRCFWDR::CRCFWD_0
    }
    #[doc = "Checks if the value of the field is `CRCFWD_1`"]
    #[inline]
    pub fn is_crcfwd_1(&self) -> bool {
        *self == CRCFWDR::CRCFWD_1
    }
}
#[doc = "Possible values of the field `CFEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CFENR {
    #[doc = "MAC control frames with any opcode other than 0x0001 (pause frame) are accepted and forwarded to the client interface."]
    CFEN_0,
    #[doc = "MAC control frames with any opcode other than 0x0001 (pause frame) are silently discarded."]
    CFEN_1,
}
impl CFENR {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            CFENR::CFEN_0 => false,
            CFENR::CFEN_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CFENR {
        match value {
            false => CFENR::CFEN_0,
            true => CFENR::CFEN_1,
        }
    }
    #[doc = "Checks if the value of the field is `CFEN_0`"]
    #[inline]
    pub fn is_cfen_0(&self) -> bool {
        *self == CFENR::CFEN_0
    }
    #[doc = "Checks if the value of the field is `CFEN_1`"]
    #[inline]
    pub fn is_cfen_1(&self) -> bool {
        *self == CFENR::CFEN_1
    }
}
#[doc = r" Value of the field"]
pub struct MAX_FLR {
    bits: u16,
}
impl MAX_FLR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }
}
#[doc = "Possible values of the field `NLC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NLCR {
    #[doc = "The payload length check is disabled."]
    NLC_0,
    #[doc = "The core checks the frame's payload length with the frame length/type field. Errors are indicated in the EIR[PLR] field."]
    NLC_1,
}
impl NLCR {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            NLCR::NLC_0 => false,
            NLCR::NLC_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> NLCR {
        match value {
            false => NLCR::NLC_0,
            true => NLCR::NLC_1,
        }
    }
    #[doc = "Checks if the value of the field is `NLC_0`"]
    #[inline]
    pub fn is_nlc_0(&self) -> bool {
        *self == NLCR::NLC_0
    }
    #[doc = "Checks if the value of the field is `NLC_1`"]
    #[inline]
    pub fn is_nlc_1(&self) -> bool {
        *self == NLCR::NLC_1
    }
}
#[doc = r" Value of the field"]
pub struct GRSR {
    bits: bool,
}
impl GRSR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
#[doc = "Values that can be written to the field `LOOP`"]
pub enum LOOPW {
    #[doc = "Loopback disabled."]
    LOOP_0,
    #[doc = "Transmitted frames are looped back internal to the device and transmit MII output signals are not asserted. DRT must be cleared."]
    LOOP_1,
}
impl LOOPW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            LOOPW::LOOP_0 => false,
            LOOPW::LOOP_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _LOOPW<'a> {
    w: &'a mut W,
}
impl<'a> _LOOPW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: LOOPW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Loopback disabled."]
    #[inline]
    pub fn loop_0(self) -> &'a mut W {
        self.variant(LOOPW::LOOP_0)
    }
    #[doc = "Transmitted frames are looped back internal to the device and transmit MII output signals are not asserted. DRT must be cleared."]
    #[inline]
    pub fn loop_1(self) -> &'a mut W {
        self.variant(LOOPW::LOOP_1)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `DRT`"]
pub enum DRTW {
    #[doc = "Receive path operates independently of transmit (i.e., full-duplex mode). Can also be used to monitor transmit activity in half-duplex mode."]
    DRT_0,
    #[doc = "Disable reception of frames while transmitting. (Normally used for half-duplex mode.)"]
    DRT_1,
}
impl DRTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DRTW::DRT_0 => false,
            DRTW::DRT_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DRTW<'a> {
    w: &'a mut W,
}
impl<'a> _DRTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DRTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Receive path operates independently of transmit (i.e., full-duplex mode). Can also be used to monitor transmit activity in half-duplex mode."]
    #[inline]
    pub fn drt_0(self) -> &'a mut W {
        self.variant(DRTW::DRT_0)
    }
    #[doc = "Disable reception of frames while transmitting. (Normally used for half-duplex mode.)"]
    #[inline]
    pub fn drt_1(self) -> &'a mut W {
        self.variant(DRTW::DRT_1)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 1;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `MII_MODE`"]
pub enum MII_MODEW {
    #[doc = "MII or RMII mode, as indicated by the RMII_MODE field."]
    MII_MODE_1,
}
impl MII_MODEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            MII_MODEW::MII_MODE_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MII_MODEW<'a> {
    w: &'a mut W,
}
impl<'a> _MII_MODEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MII_MODEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "MII or RMII mode, as indicated by the RMII_MODE field."]
    #[inline]
    pub fn mii_mode_1(self) -> &'a mut W {
        self.variant(MII_MODEW::MII_MODE_1)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 2;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `PROM`"]
pub enum PROMW {
    #[doc = "Disabled."]
    PROM_0,
    #[doc = "Enabled."]
    PROM_1,
}
impl PROMW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PROMW::PROM_0 => false,
            PROMW::PROM_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PROMW<'a> {
    w: &'a mut W,
}
impl<'a> _PROMW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PROMW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disabled."]
    #[inline]
    pub fn prom_0(self) -> &'a mut W {
        self.variant(PROMW::PROM_0)
    }
    #[doc = "Enabled."]
    #[inline]
    pub fn prom_1(self) -> &'a mut W {
        self.variant(PROMW::PROM_1)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 3;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _BC_REJW<'a> {
    w: &'a mut W,
}
impl<'a> _BC_REJW<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 4;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _FCEW<'a> {
    w: &'a mut W,
}
impl<'a> _FCEW<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 5;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `RMII_MODE`"]
pub enum RMII_MODEW {
    #[doc = "MAC configured for MII mode."]
    RMII_MODE_0,
    #[doc = "MAC configured for RMII operation."]
    RMII_MODE_1,
}
impl RMII_MODEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RMII_MODEW::RMII_MODE_0 => false,
            RMII_MODEW::RMII_MODE_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RMII_MODEW<'a> {
    w: &'a mut W,
}
impl<'a> _RMII_MODEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RMII_MODEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "MAC configured for MII mode."]
    #[inline]
    pub fn rmii_mode_0(self) -> &'a mut W {
        self.variant(RMII_MODEW::RMII_MODE_0)
    }
    #[doc = "MAC configured for RMII operation."]
    #[inline]
    pub fn rmii_mode_1(self) -> &'a mut W {
        self.variant(RMII_MODEW::RMII_MODE_1)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `RMII_10T`"]
pub enum RMII_10TW {
    #[doc = "100-Mbit/s operation."]
    RMII_10T_0,
    #[doc = "10-Mbit/s operation."]
    RMII_10T_1,
}
impl RMII_10TW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RMII_10TW::RMII_10T_0 => false,
            RMII_10TW::RMII_10T_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RMII_10TW<'a> {
    w: &'a mut W,
}
impl<'a> _RMII_10TW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RMII_10TW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "100-Mbit/s operation."]
    #[inline]
    pub fn rmii_10t_0(self) -> &'a mut W {
        self.variant(RMII_10TW::RMII_10T_0)
    }
    #[doc = "10-Mbit/s operation."]
    #[inline]
    pub fn rmii_10t_1(self) -> &'a mut W {
        self.variant(RMII_10TW::RMII_10T_1)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 9;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `PADEN`"]
pub enum PADENW {
    #[doc = "No padding is removed on receive by the MAC."]
    PADEN_0,
    #[doc = "Padding is removed from received frames."]
    PADEN_1,
}
impl PADENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PADENW::PADEN_0 => false,
            PADENW::PADEN_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PADENW<'a> {
    w: &'a mut W,
}
impl<'a> _PADENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PADENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No padding is removed on receive by the MAC."]
    #[inline]
    pub fn paden_0(self) -> &'a mut W {
        self.variant(PADENW::PADEN_0)
    }
    #[doc = "Padding is removed from received frames."]
    #[inline]
    pub fn paden_1(self) -> &'a mut W {
        self.variant(PADENW::PADEN_1)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 12;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `PAUFWD`"]
pub enum PAUFWDW {
    #[doc = "Pause frames are terminated and discarded in the MAC."]
    PAUFWD_0,
    #[doc = "Pause frames are forwarded to the user application."]
    PAUFWD_1,
}
impl PAUFWDW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PAUFWDW::PAUFWD_0 => false,
            PAUFWDW::PAUFWD_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PAUFWDW<'a> {
    w: &'a mut W,
}
impl<'a> _PAUFWDW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PAUFWDW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Pause frames are terminated and discarded in the MAC."]
    #[inline]
    pub fn paufwd_0(self) -> &'a mut W {
        self.variant(PAUFWDW::PAUFWD_0)
    }
    #[doc = "Pause frames are forwarded to the user application."]
    #[inline]
    pub fn paufwd_1(self) -> &'a mut W {
        self.variant(PAUFWDW::PAUFWD_1)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 13;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `CRCFWD`"]
pub enum CRCFWDW {
    #[doc = "The CRC field of received frames is transmitted to the user application."]
    CRCFWD_0,
    #[doc = "The CRC field is stripped from the frame."]
    CRCFWD_1,
}
impl CRCFWDW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CRCFWDW::CRCFWD_0 => false,
            CRCFWDW::CRCFWD_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CRCFWDW<'a> {
    w: &'a mut W,
}
impl<'a> _CRCFWDW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CRCFWDW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The CRC field of received frames is transmitted to the user application."]
    #[inline]
    pub fn crcfwd_0(self) -> &'a mut W {
        self.variant(CRCFWDW::CRCFWD_0)
    }
    #[doc = "The CRC field is stripped from the frame."]
    #[inline]
    pub fn crcfwd_1(self) -> &'a mut W {
        self.variant(CRCFWDW::CRCFWD_1)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 14;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `CFEN`"]
pub enum CFENW {
    #[doc = "MAC control frames with any opcode other than 0x0001 (pause frame) are accepted and forwarded to the client interface."]
    CFEN_0,
    #[doc = "MAC control frames with any opcode other than 0x0001 (pause frame) are silently discarded."]
    CFEN_1,
}
impl CFENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CFENW::CFEN_0 => false,
            CFENW::CFEN_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CFENW<'a> {
    w: &'a mut W,
}
impl<'a> _CFENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CFENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "MAC control frames with any opcode other than 0x0001 (pause frame) are accepted and forwarded to the client interface."]
    #[inline]
    pub fn cfen_0(self) -> &'a mut W {
        self.variant(CFENW::CFEN_0)
    }
    #[doc = "MAC control frames with any opcode other than 0x0001 (pause frame) are silently discarded."]
    #[inline]
    pub fn cfen_1(self) -> &'a mut W {
        self.variant(CFENW::CFEN_1)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 15;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _MAX_FLW<'a> {
    w: &'a mut W,
}
impl<'a> _MAX_FLW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        const MASK: u16 = 16383;
        const OFFSET: u8 = 16;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `NLC`"]
pub enum NLCW {
    #[doc = "The payload length check is disabled."]
    NLC_0,
    #[doc = "The core checks the frame's payload length with the frame length/type field. Errors are indicated in the EIR[PLR] field."]
    NLC_1,
}
impl NLCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            NLCW::NLC_0 => false,
            NLCW::NLC_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _NLCW<'a> {
    w: &'a mut W,
}
impl<'a> _NLCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: NLCW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The payload length check is disabled."]
    #[inline]
    pub fn nlc_0(self) -> &'a mut W {
        self.variant(NLCW::NLC_0)
    }
    #[doc = "The core checks the frame's payload length with the frame length/type field. Errors are indicated in the EIR[PLR] field."]
    #[inline]
    pub fn nlc_1(self) -> &'a mut W {
        self.variant(NLCW::NLC_1)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 30;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - Internal Loopback"]
    #[inline]
    pub fn loop_(&self) -> LOOPR {
        LOOPR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - Disable Receive On Transmit"]
    #[inline]
    pub fn drt(&self) -> DRTR {
        DRTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - Media Independent Interface Mode"]
    #[inline]
    pub fn mii_mode(&self) -> MII_MODER {
        MII_MODER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - Promiscuous Mode"]
    #[inline]
    pub fn prom(&self) -> PROMR {
        PROMR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 4 - Broadcast Frame Reject"]
    #[inline]
    pub fn bc_rej(&self) -> BC_REJR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        BC_REJR { bits }
    }
    #[doc = "Bit 5 - Flow Control Enable"]
    #[inline]
    pub fn fce(&self) -> FCER {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        FCER { bits }
    }
    #[doc = "Bit 8 - RMII Mode Enable"]
    #[inline]
    pub fn rmii_mode(&self) -> RMII_MODER {
        RMII_MODER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 9 - Enables 10-Mbit/s mode of the RMII ."]
    #[inline]
    pub fn rmii_10t(&self) -> RMII_10TR {
        RMII_10TR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 12 - Enable Frame Padding Remove On Receive"]
    #[inline]
    pub fn paden(&self) -> PADENR {
        PADENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 13 - Terminate/Forward Pause Frames"]
    #[inline]
    pub fn paufwd(&self) -> PAUFWDR {
        PAUFWDR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 14 - Terminate/Forward Received CRC"]
    #[inline]
    pub fn crcfwd(&self) -> CRCFWDR {
        CRCFWDR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 15 - MAC Control Frame Enable"]
    #[inline]
    pub fn cfen(&self) -> CFENR {
        CFENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 15;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 16:29 - Maximum Frame Length"]
    #[inline]
    pub fn max_fl(&self) -> MAX_FLR {
        let bits = {
            const MASK: u16 = 16383;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        };
        MAX_FLR { bits }
    }
    #[doc = "Bit 30 - Payload Length Check Disable"]
    #[inline]
    pub fn nlc(&self) -> NLCR {
        NLCR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 30;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 31 - Graceful Receive Stopped"]
    #[inline]
    pub fn grs(&self) -> GRSR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 31;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        GRSR { bits }
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 99483649 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - Internal Loopback"]
    #[inline]
    pub fn loop_(&mut self) -> _LOOPW {
        _LOOPW { w: self }
    }
    #[doc = "Bit 1 - Disable Receive On Transmit"]
    #[inline]
    pub fn drt(&mut self) -> _DRTW {
        _DRTW { w: self }
    }
    #[doc = "Bit 2 - Media Independent Interface Mode"]
    #[inline]
    pub fn mii_mode(&mut self) -> _MII_MODEW {
        _MII_MODEW { w: self }
    }
    #[doc = "Bit 3 - Promiscuous Mode"]
    #[inline]
    pub fn prom(&mut self) -> _PROMW {
        _PROMW { w: self }
    }
    #[doc = "Bit 4 - Broadcast Frame Reject"]
    #[inline]
    pub fn bc_rej(&mut self) -> _BC_REJW {
        _BC_REJW { w: self }
    }
    #[doc = "Bit 5 - Flow Control Enable"]
    #[inline]
    pub fn fce(&mut self) -> _FCEW {
        _FCEW { w: self }
    }
    #[doc = "Bit 8 - RMII Mode Enable"]
    #[inline]
    pub fn rmii_mode(&mut self) -> _RMII_MODEW {
        _RMII_MODEW { w: self }
    }
    #[doc = "Bit 9 - Enables 10-Mbit/s mode of the RMII ."]
    #[inline]
    pub fn rmii_10t(&mut self) -> _RMII_10TW {
        _RMII_10TW { w: self }
    }
    #[doc = "Bit 12 - Enable Frame Padding Remove On Receive"]
    #[inline]
    pub fn paden(&mut self) -> _PADENW {
        _PADENW { w: self }
    }
    #[doc = "Bit 13 - Terminate/Forward Pause Frames"]
    #[inline]
    pub fn paufwd(&mut self) -> _PAUFWDW {
        _PAUFWDW { w: self }
    }
    #[doc = "Bit 14 - Terminate/Forward Received CRC"]
    #[inline]
    pub fn crcfwd(&mut self) -> _CRCFWDW {
        _CRCFWDW { w: self }
    }
    #[doc = "Bit 15 - MAC Control Frame Enable"]
    #[inline]
    pub fn cfen(&mut self) -> _CFENW {
        _CFENW { w: self }
    }
    #[doc = "Bits 16:29 - Maximum Frame Length"]
    #[inline]
    pub fn max_fl(&mut self) -> _MAX_FLW {
        _MAX_FLW { w: self }
    }
    #[doc = "Bit 30 - Payload Length Check Disable"]
    #[inline]
    pub fn nlc(&mut self) -> _NLCW {
        _NLCW { w: self }
    }
}
