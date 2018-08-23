#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::SCFGR1 {
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
#[doc = "Possible values of the field `ADRSTALL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADRSTALLR {
    #[doc = "Clock stretching is disabled"]
    ADRSTALL_0,
    #[doc = "Clock stretching is enabled"]
    ADRSTALL_1,
}
impl ADRSTALLR {
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
            ADRSTALLR::ADRSTALL_0 => false,
            ADRSTALLR::ADRSTALL_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ADRSTALLR {
        match value {
            false => ADRSTALLR::ADRSTALL_0,
            true => ADRSTALLR::ADRSTALL_1,
        }
    }
    #[doc = "Checks if the value of the field is `ADRSTALL_0`"]
    #[inline]
    pub fn is_adrstall_0(&self) -> bool {
        *self == ADRSTALLR::ADRSTALL_0
    }
    #[doc = "Checks if the value of the field is `ADRSTALL_1`"]
    #[inline]
    pub fn is_adrstall_1(&self) -> bool {
        *self == ADRSTALLR::ADRSTALL_1
    }
}
#[doc = "Possible values of the field `RXSTALL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXSTALLR {
    #[doc = "Clock stretching is disabled"]
    RXSTALL_0,
    #[doc = "Clock stretching is enabled"]
    RXSTALL_1,
}
impl RXSTALLR {
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
            RXSTALLR::RXSTALL_0 => false,
            RXSTALLR::RXSTALL_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RXSTALLR {
        match value {
            false => RXSTALLR::RXSTALL_0,
            true => RXSTALLR::RXSTALL_1,
        }
    }
    #[doc = "Checks if the value of the field is `RXSTALL_0`"]
    #[inline]
    pub fn is_rxstall_0(&self) -> bool {
        *self == RXSTALLR::RXSTALL_0
    }
    #[doc = "Checks if the value of the field is `RXSTALL_1`"]
    #[inline]
    pub fn is_rxstall_1(&self) -> bool {
        *self == RXSTALLR::RXSTALL_1
    }
}
#[doc = "Possible values of the field `TXDSTALL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXDSTALLR {
    #[doc = "Clock stretching is disabled"]
    TXDSTALL_0,
    #[doc = "Clock stretching is enabled"]
    TXDSTALL_1,
}
impl TXDSTALLR {
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
            TXDSTALLR::TXDSTALL_0 => false,
            TXDSTALLR::TXDSTALL_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TXDSTALLR {
        match value {
            false => TXDSTALLR::TXDSTALL_0,
            true => TXDSTALLR::TXDSTALL_1,
        }
    }
    #[doc = "Checks if the value of the field is `TXDSTALL_0`"]
    #[inline]
    pub fn is_txdstall_0(&self) -> bool {
        *self == TXDSTALLR::TXDSTALL_0
    }
    #[doc = "Checks if the value of the field is `TXDSTALL_1`"]
    #[inline]
    pub fn is_txdstall_1(&self) -> bool {
        *self == TXDSTALLR::TXDSTALL_1
    }
}
#[doc = "Possible values of the field `ACKSTALL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ACKSTALLR {
    #[doc = "Clock stretching is disabled"]
    ACKSTALL_0,
    #[doc = "Clock stretching is enabled"]
    ACKSTALL_1,
}
impl ACKSTALLR {
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
            ACKSTALLR::ACKSTALL_0 => false,
            ACKSTALLR::ACKSTALL_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ACKSTALLR {
        match value {
            false => ACKSTALLR::ACKSTALL_0,
            true => ACKSTALLR::ACKSTALL_1,
        }
    }
    #[doc = "Checks if the value of the field is `ACKSTALL_0`"]
    #[inline]
    pub fn is_ackstall_0(&self) -> bool {
        *self == ACKSTALLR::ACKSTALL_0
    }
    #[doc = "Checks if the value of the field is `ACKSTALL_1`"]
    #[inline]
    pub fn is_ackstall_1(&self) -> bool {
        *self == ACKSTALLR::ACKSTALL_1
    }
}
#[doc = "Possible values of the field `GCEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GCENR {
    #[doc = "General Call address is disabled"]
    GCEN_0,
    #[doc = "General Call address is enabled"]
    GCEN_1,
}
impl GCENR {
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
            GCENR::GCEN_0 => false,
            GCENR::GCEN_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> GCENR {
        match value {
            false => GCENR::GCEN_0,
            true => GCENR::GCEN_1,
        }
    }
    #[doc = "Checks if the value of the field is `GCEN_0`"]
    #[inline]
    pub fn is_gcen_0(&self) -> bool {
        *self == GCENR::GCEN_0
    }
    #[doc = "Checks if the value of the field is `GCEN_1`"]
    #[inline]
    pub fn is_gcen_1(&self) -> bool {
        *self == GCENR::GCEN_1
    }
}
#[doc = "Possible values of the field `SAEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SAENR {
    #[doc = "Disables match on SMBus Alert"]
    SAEN_0,
    #[doc = "Enables match on SMBus Alert"]
    SAEN_1,
}
impl SAENR {
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
            SAENR::SAEN_0 => false,
            SAENR::SAEN_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SAENR {
        match value {
            false => SAENR::SAEN_0,
            true => SAENR::SAEN_1,
        }
    }
    #[doc = "Checks if the value of the field is `SAEN_0`"]
    #[inline]
    pub fn is_saen_0(&self) -> bool {
        *self == SAENR::SAEN_0
    }
    #[doc = "Checks if the value of the field is `SAEN_1`"]
    #[inline]
    pub fn is_saen_1(&self) -> bool {
        *self == SAENR::SAEN_1
    }
}
#[doc = "Possible values of the field `TXCFG`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXCFGR {
    #[doc = "Transmit Data Flag will only assert during a slave-transmit transfer when the Transmit Data register is empty"]
    TXCFG_0,
    #[doc = "Transmit Data Flag will assert whenever the Transmit Data register is empty"]
    TXCFG_1,
}
impl TXCFGR {
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
            TXCFGR::TXCFG_0 => false,
            TXCFGR::TXCFG_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TXCFGR {
        match value {
            false => TXCFGR::TXCFG_0,
            true => TXCFGR::TXCFG_1,
        }
    }
    #[doc = "Checks if the value of the field is `TXCFG_0`"]
    #[inline]
    pub fn is_txcfg_0(&self) -> bool {
        *self == TXCFGR::TXCFG_0
    }
    #[doc = "Checks if the value of the field is `TXCFG_1`"]
    #[inline]
    pub fn is_txcfg_1(&self) -> bool {
        *self == TXCFGR::TXCFG_1
    }
}
#[doc = "Possible values of the field `RXCFG`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXCFGR {
    #[doc = "Reading the Receive Data register will return received data and clear the Receive Data flag (MSR[RDF])."]
    RXCFG_0,
    #[doc = "Reading the Receive Data register when the Address Valid flag (SSR[AVF])is set, will return the Address Status register and clear the Address Valid flag. Reading the Receive Data register when the Address Valid flag is clear, will return received data and clear the Receive Data flag (MSR[RDF])."]
    RXCFG_1,
}
impl RXCFGR {
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
            RXCFGR::RXCFG_0 => false,
            RXCFGR::RXCFG_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RXCFGR {
        match value {
            false => RXCFGR::RXCFG_0,
            true => RXCFGR::RXCFG_1,
        }
    }
    #[doc = "Checks if the value of the field is `RXCFG_0`"]
    #[inline]
    pub fn is_rxcfg_0(&self) -> bool {
        *self == RXCFGR::RXCFG_0
    }
    #[doc = "Checks if the value of the field is `RXCFG_1`"]
    #[inline]
    pub fn is_rxcfg_1(&self) -> bool {
        *self == RXCFGR::RXCFG_1
    }
}
#[doc = "Possible values of the field `IGNACK`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IGNACKR {
    #[doc = "Slave will end transfer when NACK is detected"]
    IGNACK_0,
    #[doc = "Slave will not end transfer when NACK detected"]
    IGNACK_1,
}
impl IGNACKR {
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
            IGNACKR::IGNACK_0 => false,
            IGNACKR::IGNACK_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> IGNACKR {
        match value {
            false => IGNACKR::IGNACK_0,
            true => IGNACKR::IGNACK_1,
        }
    }
    #[doc = "Checks if the value of the field is `IGNACK_0`"]
    #[inline]
    pub fn is_ignack_0(&self) -> bool {
        *self == IGNACKR::IGNACK_0
    }
    #[doc = "Checks if the value of the field is `IGNACK_1`"]
    #[inline]
    pub fn is_ignack_1(&self) -> bool {
        *self == IGNACKR::IGNACK_1
    }
}
#[doc = "Possible values of the field `HSMEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HSMENR {
    #[doc = "Disables detection of HS-mode master code"]
    HSMEN_0,
    #[doc = "Enables detection of HS-mode master code"]
    HSMEN_1,
}
impl HSMENR {
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
            HSMENR::HSMEN_0 => false,
            HSMENR::HSMEN_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> HSMENR {
        match value {
            false => HSMENR::HSMEN_0,
            true => HSMENR::HSMEN_1,
        }
    }
    #[doc = "Checks if the value of the field is `HSMEN_0`"]
    #[inline]
    pub fn is_hsmen_0(&self) -> bool {
        *self == HSMENR::HSMEN_0
    }
    #[doc = "Checks if the value of the field is `HSMEN_1`"]
    #[inline]
    pub fn is_hsmen_1(&self) -> bool {
        *self == HSMENR::HSMEN_1
    }
}
#[doc = "Possible values of the field `ADDRCFG`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADDRCFGR {
    #[doc = "Address match 0 (7-bit)"]
    ADDRCFG_0,
    #[doc = "Address match 0 (10-bit)"]
    ADDRCFG_1,
    #[doc = "Address match 0 (7-bit) or Address match 1 (7-bit)"]
    ADDRCFG_2,
    #[doc = "Address match 0 (10-bit) or Address match 1 (10-bit)"]
    ADDRCFG_3,
    #[doc = "Address match 0 (7-bit) or Address match 1 (10-bit)"]
    ADDRCFG_4,
    #[doc = "Address match 0 (10-bit) or Address match 1 (7-bit)"]
    ADDRCFG_5,
    #[doc = "From Address match 0 (7-bit) to Address match 1 (7-bit)"]
    ADDRCFG_6,
    #[doc = "From Address match 0 (10-bit) to Address match 1 (10-bit)"]
    ADDRCFG_7,
}
impl ADDRCFGR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            ADDRCFGR::ADDRCFG_0 => 0,
            ADDRCFGR::ADDRCFG_1 => 1,
            ADDRCFGR::ADDRCFG_2 => 2,
            ADDRCFGR::ADDRCFG_3 => 3,
            ADDRCFGR::ADDRCFG_4 => 4,
            ADDRCFGR::ADDRCFG_5 => 5,
            ADDRCFGR::ADDRCFG_6 => 6,
            ADDRCFGR::ADDRCFG_7 => 7,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> ADDRCFGR {
        match value {
            0 => ADDRCFGR::ADDRCFG_0,
            1 => ADDRCFGR::ADDRCFG_1,
            2 => ADDRCFGR::ADDRCFG_2,
            3 => ADDRCFGR::ADDRCFG_3,
            4 => ADDRCFGR::ADDRCFG_4,
            5 => ADDRCFGR::ADDRCFG_5,
            6 => ADDRCFGR::ADDRCFG_6,
            7 => ADDRCFGR::ADDRCFG_7,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `ADDRCFG_0`"]
    #[inline]
    pub fn is_addrcfg_0(&self) -> bool {
        *self == ADDRCFGR::ADDRCFG_0
    }
    #[doc = "Checks if the value of the field is `ADDRCFG_1`"]
    #[inline]
    pub fn is_addrcfg_1(&self) -> bool {
        *self == ADDRCFGR::ADDRCFG_1
    }
    #[doc = "Checks if the value of the field is `ADDRCFG_2`"]
    #[inline]
    pub fn is_addrcfg_2(&self) -> bool {
        *self == ADDRCFGR::ADDRCFG_2
    }
    #[doc = "Checks if the value of the field is `ADDRCFG_3`"]
    #[inline]
    pub fn is_addrcfg_3(&self) -> bool {
        *self == ADDRCFGR::ADDRCFG_3
    }
    #[doc = "Checks if the value of the field is `ADDRCFG_4`"]
    #[inline]
    pub fn is_addrcfg_4(&self) -> bool {
        *self == ADDRCFGR::ADDRCFG_4
    }
    #[doc = "Checks if the value of the field is `ADDRCFG_5`"]
    #[inline]
    pub fn is_addrcfg_5(&self) -> bool {
        *self == ADDRCFGR::ADDRCFG_5
    }
    #[doc = "Checks if the value of the field is `ADDRCFG_6`"]
    #[inline]
    pub fn is_addrcfg_6(&self) -> bool {
        *self == ADDRCFGR::ADDRCFG_6
    }
    #[doc = "Checks if the value of the field is `ADDRCFG_7`"]
    #[inline]
    pub fn is_addrcfg_7(&self) -> bool {
        *self == ADDRCFGR::ADDRCFG_7
    }
}
#[doc = "Values that can be written to the field `ADRSTALL`"]
pub enum ADRSTALLW {
    #[doc = "Clock stretching is disabled"]
    ADRSTALL_0,
    #[doc = "Clock stretching is enabled"]
    ADRSTALL_1,
}
impl ADRSTALLW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ADRSTALLW::ADRSTALL_0 => false,
            ADRSTALLW::ADRSTALL_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ADRSTALLW<'a> {
    w: &'a mut W,
}
impl<'a> _ADRSTALLW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ADRSTALLW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Clock stretching is disabled"]
    #[inline]
    pub fn adrstall_0(self) -> &'a mut W {
        self.variant(ADRSTALLW::ADRSTALL_0)
    }
    #[doc = "Clock stretching is enabled"]
    #[inline]
    pub fn adrstall_1(self) -> &'a mut W {
        self.variant(ADRSTALLW::ADRSTALL_1)
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
#[doc = "Values that can be written to the field `RXSTALL`"]
pub enum RXSTALLW {
    #[doc = "Clock stretching is disabled"]
    RXSTALL_0,
    #[doc = "Clock stretching is enabled"]
    RXSTALL_1,
}
impl RXSTALLW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RXSTALLW::RXSTALL_0 => false,
            RXSTALLW::RXSTALL_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RXSTALLW<'a> {
    w: &'a mut W,
}
impl<'a> _RXSTALLW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RXSTALLW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Clock stretching is disabled"]
    #[inline]
    pub fn rxstall_0(self) -> &'a mut W {
        self.variant(RXSTALLW::RXSTALL_0)
    }
    #[doc = "Clock stretching is enabled"]
    #[inline]
    pub fn rxstall_1(self) -> &'a mut W {
        self.variant(RXSTALLW::RXSTALL_1)
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
#[doc = "Values that can be written to the field `TXDSTALL`"]
pub enum TXDSTALLW {
    #[doc = "Clock stretching is disabled"]
    TXDSTALL_0,
    #[doc = "Clock stretching is enabled"]
    TXDSTALL_1,
}
impl TXDSTALLW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TXDSTALLW::TXDSTALL_0 => false,
            TXDSTALLW::TXDSTALL_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TXDSTALLW<'a> {
    w: &'a mut W,
}
impl<'a> _TXDSTALLW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TXDSTALLW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Clock stretching is disabled"]
    #[inline]
    pub fn txdstall_0(self) -> &'a mut W {
        self.variant(TXDSTALLW::TXDSTALL_0)
    }
    #[doc = "Clock stretching is enabled"]
    #[inline]
    pub fn txdstall_1(self) -> &'a mut W {
        self.variant(TXDSTALLW::TXDSTALL_1)
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
#[doc = "Values that can be written to the field `ACKSTALL`"]
pub enum ACKSTALLW {
    #[doc = "Clock stretching is disabled"]
    ACKSTALL_0,
    #[doc = "Clock stretching is enabled"]
    ACKSTALL_1,
}
impl ACKSTALLW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ACKSTALLW::ACKSTALL_0 => false,
            ACKSTALLW::ACKSTALL_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ACKSTALLW<'a> {
    w: &'a mut W,
}
impl<'a> _ACKSTALLW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ACKSTALLW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Clock stretching is disabled"]
    #[inline]
    pub fn ackstall_0(self) -> &'a mut W {
        self.variant(ACKSTALLW::ACKSTALL_0)
    }
    #[doc = "Clock stretching is enabled"]
    #[inline]
    pub fn ackstall_1(self) -> &'a mut W {
        self.variant(ACKSTALLW::ACKSTALL_1)
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
#[doc = "Values that can be written to the field `GCEN`"]
pub enum GCENW {
    #[doc = "General Call address is disabled"]
    GCEN_0,
    #[doc = "General Call address is enabled"]
    GCEN_1,
}
impl GCENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            GCENW::GCEN_0 => false,
            GCENW::GCEN_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _GCENW<'a> {
    w: &'a mut W,
}
impl<'a> _GCENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: GCENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "General Call address is disabled"]
    #[inline]
    pub fn gcen_0(self) -> &'a mut W {
        self.variant(GCENW::GCEN_0)
    }
    #[doc = "General Call address is enabled"]
    #[inline]
    pub fn gcen_1(self) -> &'a mut W {
        self.variant(GCENW::GCEN_1)
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
#[doc = "Values that can be written to the field `SAEN`"]
pub enum SAENW {
    #[doc = "Disables match on SMBus Alert"]
    SAEN_0,
    #[doc = "Enables match on SMBus Alert"]
    SAEN_1,
}
impl SAENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SAENW::SAEN_0 => false,
            SAENW::SAEN_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SAENW<'a> {
    w: &'a mut W,
}
impl<'a> _SAENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SAENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disables match on SMBus Alert"]
    #[inline]
    pub fn saen_0(self) -> &'a mut W {
        self.variant(SAENW::SAEN_0)
    }
    #[doc = "Enables match on SMBus Alert"]
    #[inline]
    pub fn saen_1(self) -> &'a mut W {
        self.variant(SAENW::SAEN_1)
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
#[doc = "Values that can be written to the field `TXCFG`"]
pub enum TXCFGW {
    #[doc = "Transmit Data Flag will only assert during a slave-transmit transfer when the Transmit Data register is empty"]
    TXCFG_0,
    #[doc = "Transmit Data Flag will assert whenever the Transmit Data register is empty"]
    TXCFG_1,
}
impl TXCFGW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TXCFGW::TXCFG_0 => false,
            TXCFGW::TXCFG_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TXCFGW<'a> {
    w: &'a mut W,
}
impl<'a> _TXCFGW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TXCFGW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Transmit Data Flag will only assert during a slave-transmit transfer when the Transmit Data register is empty"]
    #[inline]
    pub fn txcfg_0(self) -> &'a mut W {
        self.variant(TXCFGW::TXCFG_0)
    }
    #[doc = "Transmit Data Flag will assert whenever the Transmit Data register is empty"]
    #[inline]
    pub fn txcfg_1(self) -> &'a mut W {
        self.variant(TXCFGW::TXCFG_1)
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
        const OFFSET: u8 = 10;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `RXCFG`"]
pub enum RXCFGW {
    #[doc = "Reading the Receive Data register will return received data and clear the Receive Data flag (MSR[RDF])."]
    RXCFG_0,
    #[doc = "Reading the Receive Data register when the Address Valid flag (SSR[AVF])is set, will return the Address Status register and clear the Address Valid flag. Reading the Receive Data register when the Address Valid flag is clear, will return received data and clear the Receive Data flag (MSR[RDF])."]
    RXCFG_1,
}
impl RXCFGW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RXCFGW::RXCFG_0 => false,
            RXCFGW::RXCFG_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RXCFGW<'a> {
    w: &'a mut W,
}
impl<'a> _RXCFGW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RXCFGW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Reading the Receive Data register will return received data and clear the Receive Data flag (MSR[RDF])."]
    #[inline]
    pub fn rxcfg_0(self) -> &'a mut W {
        self.variant(RXCFGW::RXCFG_0)
    }
    #[doc = "Reading the Receive Data register when the Address Valid flag (SSR[AVF])is set, will return the Address Status register and clear the Address Valid flag. Reading the Receive Data register when the Address Valid flag is clear, will return received data and clear the Receive Data flag (MSR[RDF])."]
    #[inline]
    pub fn rxcfg_1(self) -> &'a mut W {
        self.variant(RXCFGW::RXCFG_1)
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
        const OFFSET: u8 = 11;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `IGNACK`"]
pub enum IGNACKW {
    #[doc = "Slave will end transfer when NACK is detected"]
    IGNACK_0,
    #[doc = "Slave will not end transfer when NACK detected"]
    IGNACK_1,
}
impl IGNACKW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            IGNACKW::IGNACK_0 => false,
            IGNACKW::IGNACK_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _IGNACKW<'a> {
    w: &'a mut W,
}
impl<'a> _IGNACKW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: IGNACKW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Slave will end transfer when NACK is detected"]
    #[inline]
    pub fn ignack_0(self) -> &'a mut W {
        self.variant(IGNACKW::IGNACK_0)
    }
    #[doc = "Slave will not end transfer when NACK detected"]
    #[inline]
    pub fn ignack_1(self) -> &'a mut W {
        self.variant(IGNACKW::IGNACK_1)
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
#[doc = "Values that can be written to the field `HSMEN`"]
pub enum HSMENW {
    #[doc = "Disables detection of HS-mode master code"]
    HSMEN_0,
    #[doc = "Enables detection of HS-mode master code"]
    HSMEN_1,
}
impl HSMENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            HSMENW::HSMEN_0 => false,
            HSMENW::HSMEN_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _HSMENW<'a> {
    w: &'a mut W,
}
impl<'a> _HSMENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: HSMENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disables detection of HS-mode master code"]
    #[inline]
    pub fn hsmen_0(self) -> &'a mut W {
        self.variant(HSMENW::HSMEN_0)
    }
    #[doc = "Enables detection of HS-mode master code"]
    #[inline]
    pub fn hsmen_1(self) -> &'a mut W {
        self.variant(HSMENW::HSMEN_1)
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
#[doc = "Values that can be written to the field `ADDRCFG`"]
pub enum ADDRCFGW {
    #[doc = "Address match 0 (7-bit)"]
    ADDRCFG_0,
    #[doc = "Address match 0 (10-bit)"]
    ADDRCFG_1,
    #[doc = "Address match 0 (7-bit) or Address match 1 (7-bit)"]
    ADDRCFG_2,
    #[doc = "Address match 0 (10-bit) or Address match 1 (10-bit)"]
    ADDRCFG_3,
    #[doc = "Address match 0 (7-bit) or Address match 1 (10-bit)"]
    ADDRCFG_4,
    #[doc = "Address match 0 (10-bit) or Address match 1 (7-bit)"]
    ADDRCFG_5,
    #[doc = "From Address match 0 (7-bit) to Address match 1 (7-bit)"]
    ADDRCFG_6,
    #[doc = "From Address match 0 (10-bit) to Address match 1 (10-bit)"]
    ADDRCFG_7,
}
impl ADDRCFGW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            ADDRCFGW::ADDRCFG_0 => 0,
            ADDRCFGW::ADDRCFG_1 => 1,
            ADDRCFGW::ADDRCFG_2 => 2,
            ADDRCFGW::ADDRCFG_3 => 3,
            ADDRCFGW::ADDRCFG_4 => 4,
            ADDRCFGW::ADDRCFG_5 => 5,
            ADDRCFGW::ADDRCFG_6 => 6,
            ADDRCFGW::ADDRCFG_7 => 7,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ADDRCFGW<'a> {
    w: &'a mut W,
}
impl<'a> _ADDRCFGW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ADDRCFGW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Address match 0 (7-bit)"]
    #[inline]
    pub fn addrcfg_0(self) -> &'a mut W {
        self.variant(ADDRCFGW::ADDRCFG_0)
    }
    #[doc = "Address match 0 (10-bit)"]
    #[inline]
    pub fn addrcfg_1(self) -> &'a mut W {
        self.variant(ADDRCFGW::ADDRCFG_1)
    }
    #[doc = "Address match 0 (7-bit) or Address match 1 (7-bit)"]
    #[inline]
    pub fn addrcfg_2(self) -> &'a mut W {
        self.variant(ADDRCFGW::ADDRCFG_2)
    }
    #[doc = "Address match 0 (10-bit) or Address match 1 (10-bit)"]
    #[inline]
    pub fn addrcfg_3(self) -> &'a mut W {
        self.variant(ADDRCFGW::ADDRCFG_3)
    }
    #[doc = "Address match 0 (7-bit) or Address match 1 (10-bit)"]
    #[inline]
    pub fn addrcfg_4(self) -> &'a mut W {
        self.variant(ADDRCFGW::ADDRCFG_4)
    }
    #[doc = "Address match 0 (10-bit) or Address match 1 (7-bit)"]
    #[inline]
    pub fn addrcfg_5(self) -> &'a mut W {
        self.variant(ADDRCFGW::ADDRCFG_5)
    }
    #[doc = "From Address match 0 (7-bit) to Address match 1 (7-bit)"]
    #[inline]
    pub fn addrcfg_6(self) -> &'a mut W {
        self.variant(ADDRCFGW::ADDRCFG_6)
    }
    #[doc = "From Address match 0 (10-bit) to Address match 1 (10-bit)"]
    #[inline]
    pub fn addrcfg_7(self) -> &'a mut W {
        self.variant(ADDRCFGW::ADDRCFG_7)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 16;
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
    #[doc = "Bit 0 - Address SCL Stall"]
    #[inline]
    pub fn adrstall(&self) -> ADRSTALLR {
        ADRSTALLR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - RX SCL Stall"]
    #[inline]
    pub fn rxstall(&self) -> RXSTALLR {
        RXSTALLR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - TX Data SCL Stall"]
    #[inline]
    pub fn txdstall(&self) -> TXDSTALLR {
        TXDSTALLR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - ACK SCL Stall"]
    #[inline]
    pub fn ackstall(&self) -> ACKSTALLR {
        ACKSTALLR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 8 - General Call Enable"]
    #[inline]
    pub fn gcen(&self) -> GCENR {
        GCENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 9 - SMBus Alert Enable"]
    #[inline]
    pub fn saen(&self) -> SAENR {
        SAENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 10 - Transmit Flag Configuration"]
    #[inline]
    pub fn txcfg(&self) -> TXCFGR {
        TXCFGR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 11 - Receive Data Configuration"]
    #[inline]
    pub fn rxcfg(&self) -> RXCFGR {
        RXCFGR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 12 - Ignore NACK"]
    #[inline]
    pub fn ignack(&self) -> IGNACKR {
        IGNACKR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 13 - High Speed Mode Enable"]
    #[inline]
    pub fn hsmen(&self) -> HSMENR {
        HSMENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 16:18 - Address Configuration"]
    #[inline]
    pub fn addrcfg(&self) -> ADDRCFGR {
        ADDRCFGR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 0 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - Address SCL Stall"]
    #[inline]
    pub fn adrstall(&mut self) -> _ADRSTALLW {
        _ADRSTALLW { w: self }
    }
    #[doc = "Bit 1 - RX SCL Stall"]
    #[inline]
    pub fn rxstall(&mut self) -> _RXSTALLW {
        _RXSTALLW { w: self }
    }
    #[doc = "Bit 2 - TX Data SCL Stall"]
    #[inline]
    pub fn txdstall(&mut self) -> _TXDSTALLW {
        _TXDSTALLW { w: self }
    }
    #[doc = "Bit 3 - ACK SCL Stall"]
    #[inline]
    pub fn ackstall(&mut self) -> _ACKSTALLW {
        _ACKSTALLW { w: self }
    }
    #[doc = "Bit 8 - General Call Enable"]
    #[inline]
    pub fn gcen(&mut self) -> _GCENW {
        _GCENW { w: self }
    }
    #[doc = "Bit 9 - SMBus Alert Enable"]
    #[inline]
    pub fn saen(&mut self) -> _SAENW {
        _SAENW { w: self }
    }
    #[doc = "Bit 10 - Transmit Flag Configuration"]
    #[inline]
    pub fn txcfg(&mut self) -> _TXCFGW {
        _TXCFGW { w: self }
    }
    #[doc = "Bit 11 - Receive Data Configuration"]
    #[inline]
    pub fn rxcfg(&mut self) -> _RXCFGW {
        _RXCFGW { w: self }
    }
    #[doc = "Bit 12 - Ignore NACK"]
    #[inline]
    pub fn ignack(&mut self) -> _IGNACKW {
        _IGNACKW { w: self }
    }
    #[doc = "Bit 13 - High Speed Mode Enable"]
    #[inline]
    pub fn hsmen(&mut self) -> _HSMENW {
        _HSMENW { w: self }
    }
    #[doc = "Bits 16:18 - Address Configuration"]
    #[inline]
    pub fn addrcfg(&mut self) -> _ADDRCFGW {
        _ADDRCFGW { w: self }
    }
}
