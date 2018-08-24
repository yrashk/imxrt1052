#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::BAUD {
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
#[doc = r" Value of the field"]
pub struct SBRR {
    bits: u16,
}
impl SBRR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }
}
#[doc = "Possible values of the field `SBNS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SBNSR {
    #[doc = "One stop bit."]
    SBNS_0,
    #[doc = "Two stop bits."]
    SBNS_1,
}
impl SBNSR {
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
            SBNSR::SBNS_0 => false,
            SBNSR::SBNS_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SBNSR {
        match value {
            false => SBNSR::SBNS_0,
            true => SBNSR::SBNS_1,
        }
    }
    #[doc = "Checks if the value of the field is `SBNS_0`"]
    #[inline]
    pub fn is_sbns_0(&self) -> bool {
        *self == SBNSR::SBNS_0
    }
    #[doc = "Checks if the value of the field is `SBNS_1`"]
    #[inline]
    pub fn is_sbns_1(&self) -> bool {
        *self == SBNSR::SBNS_1
    }
}
#[doc = "Possible values of the field `RXEDGIE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXEDGIER {
    #[doc = "Hardware interrupts from LPUART_STAT[RXEDGIF] disabled."]
    RXEDGIE_0,
    #[doc = "Hardware interrupt requested when LPUART_STAT[RXEDGIF] flag is 1."]
    RXEDGIE_1,
}
impl RXEDGIER {
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
            RXEDGIER::RXEDGIE_0 => false,
            RXEDGIER::RXEDGIE_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RXEDGIER {
        match value {
            false => RXEDGIER::RXEDGIE_0,
            true => RXEDGIER::RXEDGIE_1,
        }
    }
    #[doc = "Checks if the value of the field is `RXEDGIE_0`"]
    #[inline]
    pub fn is_rxedgie_0(&self) -> bool {
        *self == RXEDGIER::RXEDGIE_0
    }
    #[doc = "Checks if the value of the field is `RXEDGIE_1`"]
    #[inline]
    pub fn is_rxedgie_1(&self) -> bool {
        *self == RXEDGIER::RXEDGIE_1
    }
}
#[doc = "Possible values of the field `LBKDIE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LBKDIER {
    #[doc = "Hardware interrupts from LPUART_STAT[LBKDIF] disabled (use polling)."]
    LBKDIE_0,
    #[doc = "Hardware interrupt requested when LPUART_STAT[LBKDIF] flag is 1."]
    LBKDIE_1,
}
impl LBKDIER {
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
            LBKDIER::LBKDIE_0 => false,
            LBKDIER::LBKDIE_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> LBKDIER {
        match value {
            false => LBKDIER::LBKDIE_0,
            true => LBKDIER::LBKDIE_1,
        }
    }
    #[doc = "Checks if the value of the field is `LBKDIE_0`"]
    #[inline]
    pub fn is_lbkdie_0(&self) -> bool {
        *self == LBKDIER::LBKDIE_0
    }
    #[doc = "Checks if the value of the field is `LBKDIE_1`"]
    #[inline]
    pub fn is_lbkdie_1(&self) -> bool {
        *self == LBKDIER::LBKDIE_1
    }
}
#[doc = "Possible values of the field `RESYNCDIS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RESYNCDISR {
    #[doc = "Resynchronization during received data word is supported"]
    RESYNCDIS_0,
    #[doc = "Resynchronization during received data word is disabled"]
    RESYNCDIS_1,
}
impl RESYNCDISR {
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
            RESYNCDISR::RESYNCDIS_0 => false,
            RESYNCDISR::RESYNCDIS_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RESYNCDISR {
        match value {
            false => RESYNCDISR::RESYNCDIS_0,
            true => RESYNCDISR::RESYNCDIS_1,
        }
    }
    #[doc = "Checks if the value of the field is `RESYNCDIS_0`"]
    #[inline]
    pub fn is_resyncdis_0(&self) -> bool {
        *self == RESYNCDISR::RESYNCDIS_0
    }
    #[doc = "Checks if the value of the field is `RESYNCDIS_1`"]
    #[inline]
    pub fn is_resyncdis_1(&self) -> bool {
        *self == RESYNCDISR::RESYNCDIS_1
    }
}
#[doc = "Possible values of the field `BOTHEDGE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BOTHEDGER {
    #[doc = "Receiver samples input data using the rising edge of the baud rate clock."]
    BOTHEDGE_0,
    #[doc = "Receiver samples input data using the rising and falling edge of the baud rate clock."]
    BOTHEDGE_1,
}
impl BOTHEDGER {
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
            BOTHEDGER::BOTHEDGE_0 => false,
            BOTHEDGER::BOTHEDGE_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> BOTHEDGER {
        match value {
            false => BOTHEDGER::BOTHEDGE_0,
            true => BOTHEDGER::BOTHEDGE_1,
        }
    }
    #[doc = "Checks if the value of the field is `BOTHEDGE_0`"]
    #[inline]
    pub fn is_bothedge_0(&self) -> bool {
        *self == BOTHEDGER::BOTHEDGE_0
    }
    #[doc = "Checks if the value of the field is `BOTHEDGE_1`"]
    #[inline]
    pub fn is_bothedge_1(&self) -> bool {
        *self == BOTHEDGER::BOTHEDGE_1
    }
}
#[doc = "Possible values of the field `MATCFG`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MATCFGR {
    #[doc = "Address Match Wakeup"]
    MATCFG_0,
    #[doc = "Idle Match Wakeup"]
    MATCFG_1,
    #[doc = "Match On and Match Off"]
    MATCFG_2,
    #[doc = "no description available"]
    MATCFG_3,
}
impl MATCFGR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            MATCFGR::MATCFG_0 => 0,
            MATCFGR::MATCFG_1 => 1,
            MATCFGR::MATCFG_2 => 2,
            MATCFGR::MATCFG_3 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> MATCFGR {
        match value {
            0 => MATCFGR::MATCFG_0,
            1 => MATCFGR::MATCFG_1,
            2 => MATCFGR::MATCFG_2,
            3 => MATCFGR::MATCFG_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `MATCFG_0`"]
    #[inline]
    pub fn is_matcfg_0(&self) -> bool {
        *self == MATCFGR::MATCFG_0
    }
    #[doc = "Checks if the value of the field is `MATCFG_1`"]
    #[inline]
    pub fn is_matcfg_1(&self) -> bool {
        *self == MATCFGR::MATCFG_1
    }
    #[doc = "Checks if the value of the field is `MATCFG_2`"]
    #[inline]
    pub fn is_matcfg_2(&self) -> bool {
        *self == MATCFGR::MATCFG_2
    }
    #[doc = "Checks if the value of the field is `MATCFG_3`"]
    #[inline]
    pub fn is_matcfg_3(&self) -> bool {
        *self == MATCFGR::MATCFG_3
    }
}
#[doc = "Possible values of the field `RDMAE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RDMAER {
    #[doc = "DMA request disabled."]
    RDMAE_0,
    #[doc = "DMA request enabled."]
    RDMAE_1,
}
impl RDMAER {
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
            RDMAER::RDMAE_0 => false,
            RDMAER::RDMAE_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RDMAER {
        match value {
            false => RDMAER::RDMAE_0,
            true => RDMAER::RDMAE_1,
        }
    }
    #[doc = "Checks if the value of the field is `RDMAE_0`"]
    #[inline]
    pub fn is_rdmae_0(&self) -> bool {
        *self == RDMAER::RDMAE_0
    }
    #[doc = "Checks if the value of the field is `RDMAE_1`"]
    #[inline]
    pub fn is_rdmae_1(&self) -> bool {
        *self == RDMAER::RDMAE_1
    }
}
#[doc = "Possible values of the field `TDMAE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TDMAER {
    #[doc = "DMA request disabled."]
    TDMAE_0,
    #[doc = "DMA request enabled."]
    TDMAE_1,
}
impl TDMAER {
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
            TDMAER::TDMAE_0 => false,
            TDMAER::TDMAE_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TDMAER {
        match value {
            false => TDMAER::TDMAE_0,
            true => TDMAER::TDMAE_1,
        }
    }
    #[doc = "Checks if the value of the field is `TDMAE_0`"]
    #[inline]
    pub fn is_tdmae_0(&self) -> bool {
        *self == TDMAER::TDMAE_0
    }
    #[doc = "Checks if the value of the field is `TDMAE_1`"]
    #[inline]
    pub fn is_tdmae_1(&self) -> bool {
        *self == TDMAER::TDMAE_1
    }
}
#[doc = "Possible values of the field `OSR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OSRR {
    #[doc = "Writing 0 to this field will result in an oversampling ratio of 16"]
    OSR_0,
    #[doc = "Oversampling ratio of 4, requires BOTHEDGE to be set."]
    OSR_3,
    #[doc = "Oversampling ratio of 5, requires BOTHEDGE to be set."]
    OSR_4,
    #[doc = "Oversampling ratio of 6, requires BOTHEDGE to be set."]
    OSR_5,
    #[doc = "Oversampling ratio of 7, requires BOTHEDGE to be set."]
    OSR_6,
    #[doc = "Oversampling ratio of 8."]
    OSR_7,
    #[doc = "Oversampling ratio of 9."]
    OSR_8,
    #[doc = "Oversampling ratio of 10."]
    OSR_9,
    #[doc = "Oversampling ratio of 11."]
    OSR_10,
    #[doc = "Oversampling ratio of 12."]
    OSR_11,
    #[doc = "Oversampling ratio of 13."]
    OSR_12,
    #[doc = "Oversampling ratio of 14."]
    OSR_13,
    #[doc = "Oversampling ratio of 15."]
    OSR_14,
    #[doc = "Oversampling ratio of 16."]
    OSR_15,
    #[doc = "Oversampling ratio of 17."]
    OSR_16,
    #[doc = "Oversampling ratio of 18."]
    OSR_17,
    #[doc = "Oversampling ratio of 19."]
    OSR_18,
    #[doc = "Oversampling ratio of 20."]
    OSR_19,
    #[doc = "Oversampling ratio of 21."]
    OSR_20,
    #[doc = "Oversampling ratio of 22."]
    OSR_21,
    #[doc = "Oversampling ratio of 23."]
    OSR_22,
    #[doc = "Oversampling ratio of 24."]
    OSR_23,
    #[doc = "Oversampling ratio of 25."]
    OSR_24,
    #[doc = "Oversampling ratio of 26."]
    OSR_25,
    #[doc = "Oversampling ratio of 27."]
    OSR_26,
    #[doc = "Oversampling ratio of 28."]
    OSR_27,
    #[doc = "Oversampling ratio of 29."]
    OSR_28,
    #[doc = "Oversampling ratio of 30."]
    OSR_29,
    #[doc = "Oversampling ratio of 31."]
    OSR_30,
    #[doc = "Oversampling ratio of 32."]
    OSR_31,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl OSRR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            OSRR::OSR_0 => 0,
            OSRR::OSR_3 => 3,
            OSRR::OSR_4 => 4,
            OSRR::OSR_5 => 5,
            OSRR::OSR_6 => 6,
            OSRR::OSR_7 => 7,
            OSRR::OSR_8 => 8,
            OSRR::OSR_9 => 9,
            OSRR::OSR_10 => 10,
            OSRR::OSR_11 => 11,
            OSRR::OSR_12 => 12,
            OSRR::OSR_13 => 13,
            OSRR::OSR_14 => 14,
            OSRR::OSR_15 => 15,
            OSRR::OSR_16 => 16,
            OSRR::OSR_17 => 17,
            OSRR::OSR_18 => 18,
            OSRR::OSR_19 => 19,
            OSRR::OSR_20 => 20,
            OSRR::OSR_21 => 21,
            OSRR::OSR_22 => 22,
            OSRR::OSR_23 => 23,
            OSRR::OSR_24 => 24,
            OSRR::OSR_25 => 25,
            OSRR::OSR_26 => 26,
            OSRR::OSR_27 => 27,
            OSRR::OSR_28 => 28,
            OSRR::OSR_29 => 29,
            OSRR::OSR_30 => 30,
            OSRR::OSR_31 => 31,
            OSRR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> OSRR {
        match value {
            0 => OSRR::OSR_0,
            3 => OSRR::OSR_3,
            4 => OSRR::OSR_4,
            5 => OSRR::OSR_5,
            6 => OSRR::OSR_6,
            7 => OSRR::OSR_7,
            8 => OSRR::OSR_8,
            9 => OSRR::OSR_9,
            10 => OSRR::OSR_10,
            11 => OSRR::OSR_11,
            12 => OSRR::OSR_12,
            13 => OSRR::OSR_13,
            14 => OSRR::OSR_14,
            15 => OSRR::OSR_15,
            16 => OSRR::OSR_16,
            17 => OSRR::OSR_17,
            18 => OSRR::OSR_18,
            19 => OSRR::OSR_19,
            20 => OSRR::OSR_20,
            21 => OSRR::OSR_21,
            22 => OSRR::OSR_22,
            23 => OSRR::OSR_23,
            24 => OSRR::OSR_24,
            25 => OSRR::OSR_25,
            26 => OSRR::OSR_26,
            27 => OSRR::OSR_27,
            28 => OSRR::OSR_28,
            29 => OSRR::OSR_29,
            30 => OSRR::OSR_30,
            31 => OSRR::OSR_31,
            i => OSRR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `OSR_0`"]
    #[inline]
    pub fn is_osr_0(&self) -> bool {
        *self == OSRR::OSR_0
    }
    #[doc = "Checks if the value of the field is `OSR_3`"]
    #[inline]
    pub fn is_osr_3(&self) -> bool {
        *self == OSRR::OSR_3
    }
    #[doc = "Checks if the value of the field is `OSR_4`"]
    #[inline]
    pub fn is_osr_4(&self) -> bool {
        *self == OSRR::OSR_4
    }
    #[doc = "Checks if the value of the field is `OSR_5`"]
    #[inline]
    pub fn is_osr_5(&self) -> bool {
        *self == OSRR::OSR_5
    }
    #[doc = "Checks if the value of the field is `OSR_6`"]
    #[inline]
    pub fn is_osr_6(&self) -> bool {
        *self == OSRR::OSR_6
    }
    #[doc = "Checks if the value of the field is `OSR_7`"]
    #[inline]
    pub fn is_osr_7(&self) -> bool {
        *self == OSRR::OSR_7
    }
    #[doc = "Checks if the value of the field is `OSR_8`"]
    #[inline]
    pub fn is_osr_8(&self) -> bool {
        *self == OSRR::OSR_8
    }
    #[doc = "Checks if the value of the field is `OSR_9`"]
    #[inline]
    pub fn is_osr_9(&self) -> bool {
        *self == OSRR::OSR_9
    }
    #[doc = "Checks if the value of the field is `OSR_10`"]
    #[inline]
    pub fn is_osr_10(&self) -> bool {
        *self == OSRR::OSR_10
    }
    #[doc = "Checks if the value of the field is `OSR_11`"]
    #[inline]
    pub fn is_osr_11(&self) -> bool {
        *self == OSRR::OSR_11
    }
    #[doc = "Checks if the value of the field is `OSR_12`"]
    #[inline]
    pub fn is_osr_12(&self) -> bool {
        *self == OSRR::OSR_12
    }
    #[doc = "Checks if the value of the field is `OSR_13`"]
    #[inline]
    pub fn is_osr_13(&self) -> bool {
        *self == OSRR::OSR_13
    }
    #[doc = "Checks if the value of the field is `OSR_14`"]
    #[inline]
    pub fn is_osr_14(&self) -> bool {
        *self == OSRR::OSR_14
    }
    #[doc = "Checks if the value of the field is `OSR_15`"]
    #[inline]
    pub fn is_osr_15(&self) -> bool {
        *self == OSRR::OSR_15
    }
    #[doc = "Checks if the value of the field is `OSR_16`"]
    #[inline]
    pub fn is_osr_16(&self) -> bool {
        *self == OSRR::OSR_16
    }
    #[doc = "Checks if the value of the field is `OSR_17`"]
    #[inline]
    pub fn is_osr_17(&self) -> bool {
        *self == OSRR::OSR_17
    }
    #[doc = "Checks if the value of the field is `OSR_18`"]
    #[inline]
    pub fn is_osr_18(&self) -> bool {
        *self == OSRR::OSR_18
    }
    #[doc = "Checks if the value of the field is `OSR_19`"]
    #[inline]
    pub fn is_osr_19(&self) -> bool {
        *self == OSRR::OSR_19
    }
    #[doc = "Checks if the value of the field is `OSR_20`"]
    #[inline]
    pub fn is_osr_20(&self) -> bool {
        *self == OSRR::OSR_20
    }
    #[doc = "Checks if the value of the field is `OSR_21`"]
    #[inline]
    pub fn is_osr_21(&self) -> bool {
        *self == OSRR::OSR_21
    }
    #[doc = "Checks if the value of the field is `OSR_22`"]
    #[inline]
    pub fn is_osr_22(&self) -> bool {
        *self == OSRR::OSR_22
    }
    #[doc = "Checks if the value of the field is `OSR_23`"]
    #[inline]
    pub fn is_osr_23(&self) -> bool {
        *self == OSRR::OSR_23
    }
    #[doc = "Checks if the value of the field is `OSR_24`"]
    #[inline]
    pub fn is_osr_24(&self) -> bool {
        *self == OSRR::OSR_24
    }
    #[doc = "Checks if the value of the field is `OSR_25`"]
    #[inline]
    pub fn is_osr_25(&self) -> bool {
        *self == OSRR::OSR_25
    }
    #[doc = "Checks if the value of the field is `OSR_26`"]
    #[inline]
    pub fn is_osr_26(&self) -> bool {
        *self == OSRR::OSR_26
    }
    #[doc = "Checks if the value of the field is `OSR_27`"]
    #[inline]
    pub fn is_osr_27(&self) -> bool {
        *self == OSRR::OSR_27
    }
    #[doc = "Checks if the value of the field is `OSR_28`"]
    #[inline]
    pub fn is_osr_28(&self) -> bool {
        *self == OSRR::OSR_28
    }
    #[doc = "Checks if the value of the field is `OSR_29`"]
    #[inline]
    pub fn is_osr_29(&self) -> bool {
        *self == OSRR::OSR_29
    }
    #[doc = "Checks if the value of the field is `OSR_30`"]
    #[inline]
    pub fn is_osr_30(&self) -> bool {
        *self == OSRR::OSR_30
    }
    #[doc = "Checks if the value of the field is `OSR_31`"]
    #[inline]
    pub fn is_osr_31(&self) -> bool {
        *self == OSRR::OSR_31
    }
}
#[doc = "Possible values of the field `M10`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum M10R {
    #[doc = "Receiver and transmitter use 7-bit to 9-bit data characters."]
    M10_0,
    #[doc = "Receiver and transmitter use 10-bit data characters."]
    M10_1,
}
impl M10R {
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
            M10R::M10_0 => false,
            M10R::M10_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> M10R {
        match value {
            false => M10R::M10_0,
            true => M10R::M10_1,
        }
    }
    #[doc = "Checks if the value of the field is `M10_0`"]
    #[inline]
    pub fn is_m10_0(&self) -> bool {
        *self == M10R::M10_0
    }
    #[doc = "Checks if the value of the field is `M10_1`"]
    #[inline]
    pub fn is_m10_1(&self) -> bool {
        *self == M10R::M10_1
    }
}
#[doc = "Possible values of the field `MAEN2`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MAEN2R {
    #[doc = "Normal operation."]
    MAEN2_0,
    #[doc = "Enables automatic address matching or data matching mode for MATCH[MA2]."]
    MAEN2_1,
}
impl MAEN2R {
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
            MAEN2R::MAEN2_0 => false,
            MAEN2R::MAEN2_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> MAEN2R {
        match value {
            false => MAEN2R::MAEN2_0,
            true => MAEN2R::MAEN2_1,
        }
    }
    #[doc = "Checks if the value of the field is `MAEN2_0`"]
    #[inline]
    pub fn is_maen2_0(&self) -> bool {
        *self == MAEN2R::MAEN2_0
    }
    #[doc = "Checks if the value of the field is `MAEN2_1`"]
    #[inline]
    pub fn is_maen2_1(&self) -> bool {
        *self == MAEN2R::MAEN2_1
    }
}
#[doc = "Possible values of the field `MAEN1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MAEN1R {
    #[doc = "Normal operation."]
    MAEN1_0,
    #[doc = "Enables automatic address matching or data matching mode for MATCH[MA1]."]
    MAEN1_1,
}
impl MAEN1R {
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
            MAEN1R::MAEN1_0 => false,
            MAEN1R::MAEN1_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> MAEN1R {
        match value {
            false => MAEN1R::MAEN1_0,
            true => MAEN1R::MAEN1_1,
        }
    }
    #[doc = "Checks if the value of the field is `MAEN1_0`"]
    #[inline]
    pub fn is_maen1_0(&self) -> bool {
        *self == MAEN1R::MAEN1_0
    }
    #[doc = "Checks if the value of the field is `MAEN1_1`"]
    #[inline]
    pub fn is_maen1_1(&self) -> bool {
        *self == MAEN1R::MAEN1_1
    }
}
#[doc = r" Proxy"]
pub struct _SBRW<'a> {
    w: &'a mut W,
}
impl<'a> _SBRW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        const MASK: u16 = 8191;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `SBNS`"]
pub enum SBNSW {
    #[doc = "One stop bit."]
    SBNS_0,
    #[doc = "Two stop bits."]
    SBNS_1,
}
impl SBNSW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SBNSW::SBNS_0 => false,
            SBNSW::SBNS_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SBNSW<'a> {
    w: &'a mut W,
}
impl<'a> _SBNSW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SBNSW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "One stop bit."]
    #[inline]
    pub fn sbns_0(self) -> &'a mut W {
        self.variant(SBNSW::SBNS_0)
    }
    #[doc = "Two stop bits."]
    #[inline]
    pub fn sbns_1(self) -> &'a mut W {
        self.variant(SBNSW::SBNS_1)
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
#[doc = "Values that can be written to the field `RXEDGIE`"]
pub enum RXEDGIEW {
    #[doc = "Hardware interrupts from LPUART_STAT[RXEDGIF] disabled."]
    RXEDGIE_0,
    #[doc = "Hardware interrupt requested when LPUART_STAT[RXEDGIF] flag is 1."]
    RXEDGIE_1,
}
impl RXEDGIEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RXEDGIEW::RXEDGIE_0 => false,
            RXEDGIEW::RXEDGIE_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RXEDGIEW<'a> {
    w: &'a mut W,
}
impl<'a> _RXEDGIEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RXEDGIEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Hardware interrupts from LPUART_STAT[RXEDGIF] disabled."]
    #[inline]
    pub fn rxedgie_0(self) -> &'a mut W {
        self.variant(RXEDGIEW::RXEDGIE_0)
    }
    #[doc = "Hardware interrupt requested when LPUART_STAT[RXEDGIF] flag is 1."]
    #[inline]
    pub fn rxedgie_1(self) -> &'a mut W {
        self.variant(RXEDGIEW::RXEDGIE_1)
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
#[doc = "Values that can be written to the field `LBKDIE`"]
pub enum LBKDIEW {
    #[doc = "Hardware interrupts from LPUART_STAT[LBKDIF] disabled (use polling)."]
    LBKDIE_0,
    #[doc = "Hardware interrupt requested when LPUART_STAT[LBKDIF] flag is 1."]
    LBKDIE_1,
}
impl LBKDIEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            LBKDIEW::LBKDIE_0 => false,
            LBKDIEW::LBKDIE_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _LBKDIEW<'a> {
    w: &'a mut W,
}
impl<'a> _LBKDIEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: LBKDIEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Hardware interrupts from LPUART_STAT[LBKDIF] disabled (use polling)."]
    #[inline]
    pub fn lbkdie_0(self) -> &'a mut W {
        self.variant(LBKDIEW::LBKDIE_0)
    }
    #[doc = "Hardware interrupt requested when LPUART_STAT[LBKDIF] flag is 1."]
    #[inline]
    pub fn lbkdie_1(self) -> &'a mut W {
        self.variant(LBKDIEW::LBKDIE_1)
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
#[doc = "Values that can be written to the field `RESYNCDIS`"]
pub enum RESYNCDISW {
    #[doc = "Resynchronization during received data word is supported"]
    RESYNCDIS_0,
    #[doc = "Resynchronization during received data word is disabled"]
    RESYNCDIS_1,
}
impl RESYNCDISW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RESYNCDISW::RESYNCDIS_0 => false,
            RESYNCDISW::RESYNCDIS_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RESYNCDISW<'a> {
    w: &'a mut W,
}
impl<'a> _RESYNCDISW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RESYNCDISW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Resynchronization during received data word is supported"]
    #[inline]
    pub fn resyncdis_0(self) -> &'a mut W {
        self.variant(RESYNCDISW::RESYNCDIS_0)
    }
    #[doc = "Resynchronization during received data word is disabled"]
    #[inline]
    pub fn resyncdis_1(self) -> &'a mut W {
        self.variant(RESYNCDISW::RESYNCDIS_1)
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
        const OFFSET: u8 = 16;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `BOTHEDGE`"]
pub enum BOTHEDGEW {
    #[doc = "Receiver samples input data using the rising edge of the baud rate clock."]
    BOTHEDGE_0,
    #[doc = "Receiver samples input data using the rising and falling edge of the baud rate clock."]
    BOTHEDGE_1,
}
impl BOTHEDGEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            BOTHEDGEW::BOTHEDGE_0 => false,
            BOTHEDGEW::BOTHEDGE_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _BOTHEDGEW<'a> {
    w: &'a mut W,
}
impl<'a> _BOTHEDGEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: BOTHEDGEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Receiver samples input data using the rising edge of the baud rate clock."]
    #[inline]
    pub fn bothedge_0(self) -> &'a mut W {
        self.variant(BOTHEDGEW::BOTHEDGE_0)
    }
    #[doc = "Receiver samples input data using the rising and falling edge of the baud rate clock."]
    #[inline]
    pub fn bothedge_1(self) -> &'a mut W {
        self.variant(BOTHEDGEW::BOTHEDGE_1)
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
        const OFFSET: u8 = 17;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `MATCFG`"]
pub enum MATCFGW {
    #[doc = "Address Match Wakeup"]
    MATCFG_0,
    #[doc = "Idle Match Wakeup"]
    MATCFG_1,
    #[doc = "Match On and Match Off"]
    MATCFG_2,
    #[doc = "no description available"]
    MATCFG_3,
}
impl MATCFGW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            MATCFGW::MATCFG_0 => 0,
            MATCFGW::MATCFG_1 => 1,
            MATCFGW::MATCFG_2 => 2,
            MATCFGW::MATCFG_3 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MATCFGW<'a> {
    w: &'a mut W,
}
impl<'a> _MATCFGW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MATCFGW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Address Match Wakeup"]
    #[inline]
    pub fn matcfg_0(self) -> &'a mut W {
        self.variant(MATCFGW::MATCFG_0)
    }
    #[doc = "Idle Match Wakeup"]
    #[inline]
    pub fn matcfg_1(self) -> &'a mut W {
        self.variant(MATCFGW::MATCFG_1)
    }
    #[doc = "Match On and Match Off"]
    #[inline]
    pub fn matcfg_2(self) -> &'a mut W {
        self.variant(MATCFGW::MATCFG_2)
    }
    #[doc = "no description available"]
    #[inline]
    pub fn matcfg_3(self) -> &'a mut W {
        self.variant(MATCFGW::MATCFG_3)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 18;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `RDMAE`"]
pub enum RDMAEW {
    #[doc = "DMA request disabled."]
    RDMAE_0,
    #[doc = "DMA request enabled."]
    RDMAE_1,
}
impl RDMAEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RDMAEW::RDMAE_0 => false,
            RDMAEW::RDMAE_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RDMAEW<'a> {
    w: &'a mut W,
}
impl<'a> _RDMAEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RDMAEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "DMA request disabled."]
    #[inline]
    pub fn rdmae_0(self) -> &'a mut W {
        self.variant(RDMAEW::RDMAE_0)
    }
    #[doc = "DMA request enabled."]
    #[inline]
    pub fn rdmae_1(self) -> &'a mut W {
        self.variant(RDMAEW::RDMAE_1)
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
        const OFFSET: u8 = 21;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `TDMAE`"]
pub enum TDMAEW {
    #[doc = "DMA request disabled."]
    TDMAE_0,
    #[doc = "DMA request enabled."]
    TDMAE_1,
}
impl TDMAEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TDMAEW::TDMAE_0 => false,
            TDMAEW::TDMAE_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TDMAEW<'a> {
    w: &'a mut W,
}
impl<'a> _TDMAEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TDMAEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "DMA request disabled."]
    #[inline]
    pub fn tdmae_0(self) -> &'a mut W {
        self.variant(TDMAEW::TDMAE_0)
    }
    #[doc = "DMA request enabled."]
    #[inline]
    pub fn tdmae_1(self) -> &'a mut W {
        self.variant(TDMAEW::TDMAE_1)
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
        const OFFSET: u8 = 23;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `OSR`"]
pub enum OSRW {
    #[doc = "Writing 0 to this field will result in an oversampling ratio of 16"]
    OSR_0,
    #[doc = "Oversampling ratio of 4, requires BOTHEDGE to be set."]
    OSR_3,
    #[doc = "Oversampling ratio of 5, requires BOTHEDGE to be set."]
    OSR_4,
    #[doc = "Oversampling ratio of 6, requires BOTHEDGE to be set."]
    OSR_5,
    #[doc = "Oversampling ratio of 7, requires BOTHEDGE to be set."]
    OSR_6,
    #[doc = "Oversampling ratio of 8."]
    OSR_7,
    #[doc = "Oversampling ratio of 9."]
    OSR_8,
    #[doc = "Oversampling ratio of 10."]
    OSR_9,
    #[doc = "Oversampling ratio of 11."]
    OSR_10,
    #[doc = "Oversampling ratio of 12."]
    OSR_11,
    #[doc = "Oversampling ratio of 13."]
    OSR_12,
    #[doc = "Oversampling ratio of 14."]
    OSR_13,
    #[doc = "Oversampling ratio of 15."]
    OSR_14,
    #[doc = "Oversampling ratio of 16."]
    OSR_15,
    #[doc = "Oversampling ratio of 17."]
    OSR_16,
    #[doc = "Oversampling ratio of 18."]
    OSR_17,
    #[doc = "Oversampling ratio of 19."]
    OSR_18,
    #[doc = "Oversampling ratio of 20."]
    OSR_19,
    #[doc = "Oversampling ratio of 21."]
    OSR_20,
    #[doc = "Oversampling ratio of 22."]
    OSR_21,
    #[doc = "Oversampling ratio of 23."]
    OSR_22,
    #[doc = "Oversampling ratio of 24."]
    OSR_23,
    #[doc = "Oversampling ratio of 25."]
    OSR_24,
    #[doc = "Oversampling ratio of 26."]
    OSR_25,
    #[doc = "Oversampling ratio of 27."]
    OSR_26,
    #[doc = "Oversampling ratio of 28."]
    OSR_27,
    #[doc = "Oversampling ratio of 29."]
    OSR_28,
    #[doc = "Oversampling ratio of 30."]
    OSR_29,
    #[doc = "Oversampling ratio of 31."]
    OSR_30,
    #[doc = "Oversampling ratio of 32."]
    OSR_31,
}
impl OSRW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            OSRW::OSR_0 => 0,
            OSRW::OSR_3 => 3,
            OSRW::OSR_4 => 4,
            OSRW::OSR_5 => 5,
            OSRW::OSR_6 => 6,
            OSRW::OSR_7 => 7,
            OSRW::OSR_8 => 8,
            OSRW::OSR_9 => 9,
            OSRW::OSR_10 => 10,
            OSRW::OSR_11 => 11,
            OSRW::OSR_12 => 12,
            OSRW::OSR_13 => 13,
            OSRW::OSR_14 => 14,
            OSRW::OSR_15 => 15,
            OSRW::OSR_16 => 16,
            OSRW::OSR_17 => 17,
            OSRW::OSR_18 => 18,
            OSRW::OSR_19 => 19,
            OSRW::OSR_20 => 20,
            OSRW::OSR_21 => 21,
            OSRW::OSR_22 => 22,
            OSRW::OSR_23 => 23,
            OSRW::OSR_24 => 24,
            OSRW::OSR_25 => 25,
            OSRW::OSR_26 => 26,
            OSRW::OSR_27 => 27,
            OSRW::OSR_28 => 28,
            OSRW::OSR_29 => 29,
            OSRW::OSR_30 => 30,
            OSRW::OSR_31 => 31,
        }
    }
}
#[doc = r" Proxy"]
pub struct _OSRW<'a> {
    w: &'a mut W,
}
impl<'a> _OSRW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: OSRW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Writing 0 to this field will result in an oversampling ratio of 16"]
    #[inline]
    pub fn osr_0(self) -> &'a mut W {
        self.variant(OSRW::OSR_0)
    }
    #[doc = "Oversampling ratio of 4, requires BOTHEDGE to be set."]
    #[inline]
    pub fn osr_3(self) -> &'a mut W {
        self.variant(OSRW::OSR_3)
    }
    #[doc = "Oversampling ratio of 5, requires BOTHEDGE to be set."]
    #[inline]
    pub fn osr_4(self) -> &'a mut W {
        self.variant(OSRW::OSR_4)
    }
    #[doc = "Oversampling ratio of 6, requires BOTHEDGE to be set."]
    #[inline]
    pub fn osr_5(self) -> &'a mut W {
        self.variant(OSRW::OSR_5)
    }
    #[doc = "Oversampling ratio of 7, requires BOTHEDGE to be set."]
    #[inline]
    pub fn osr_6(self) -> &'a mut W {
        self.variant(OSRW::OSR_6)
    }
    #[doc = "Oversampling ratio of 8."]
    #[inline]
    pub fn osr_7(self) -> &'a mut W {
        self.variant(OSRW::OSR_7)
    }
    #[doc = "Oversampling ratio of 9."]
    #[inline]
    pub fn osr_8(self) -> &'a mut W {
        self.variant(OSRW::OSR_8)
    }
    #[doc = "Oversampling ratio of 10."]
    #[inline]
    pub fn osr_9(self) -> &'a mut W {
        self.variant(OSRW::OSR_9)
    }
    #[doc = "Oversampling ratio of 11."]
    #[inline]
    pub fn osr_10(self) -> &'a mut W {
        self.variant(OSRW::OSR_10)
    }
    #[doc = "Oversampling ratio of 12."]
    #[inline]
    pub fn osr_11(self) -> &'a mut W {
        self.variant(OSRW::OSR_11)
    }
    #[doc = "Oversampling ratio of 13."]
    #[inline]
    pub fn osr_12(self) -> &'a mut W {
        self.variant(OSRW::OSR_12)
    }
    #[doc = "Oversampling ratio of 14."]
    #[inline]
    pub fn osr_13(self) -> &'a mut W {
        self.variant(OSRW::OSR_13)
    }
    #[doc = "Oversampling ratio of 15."]
    #[inline]
    pub fn osr_14(self) -> &'a mut W {
        self.variant(OSRW::OSR_14)
    }
    #[doc = "Oversampling ratio of 16."]
    #[inline]
    pub fn osr_15(self) -> &'a mut W {
        self.variant(OSRW::OSR_15)
    }
    #[doc = "Oversampling ratio of 17."]
    #[inline]
    pub fn osr_16(self) -> &'a mut W {
        self.variant(OSRW::OSR_16)
    }
    #[doc = "Oversampling ratio of 18."]
    #[inline]
    pub fn osr_17(self) -> &'a mut W {
        self.variant(OSRW::OSR_17)
    }
    #[doc = "Oversampling ratio of 19."]
    #[inline]
    pub fn osr_18(self) -> &'a mut W {
        self.variant(OSRW::OSR_18)
    }
    #[doc = "Oversampling ratio of 20."]
    #[inline]
    pub fn osr_19(self) -> &'a mut W {
        self.variant(OSRW::OSR_19)
    }
    #[doc = "Oversampling ratio of 21."]
    #[inline]
    pub fn osr_20(self) -> &'a mut W {
        self.variant(OSRW::OSR_20)
    }
    #[doc = "Oversampling ratio of 22."]
    #[inline]
    pub fn osr_21(self) -> &'a mut W {
        self.variant(OSRW::OSR_21)
    }
    #[doc = "Oversampling ratio of 23."]
    #[inline]
    pub fn osr_22(self) -> &'a mut W {
        self.variant(OSRW::OSR_22)
    }
    #[doc = "Oversampling ratio of 24."]
    #[inline]
    pub fn osr_23(self) -> &'a mut W {
        self.variant(OSRW::OSR_23)
    }
    #[doc = "Oversampling ratio of 25."]
    #[inline]
    pub fn osr_24(self) -> &'a mut W {
        self.variant(OSRW::OSR_24)
    }
    #[doc = "Oversampling ratio of 26."]
    #[inline]
    pub fn osr_25(self) -> &'a mut W {
        self.variant(OSRW::OSR_25)
    }
    #[doc = "Oversampling ratio of 27."]
    #[inline]
    pub fn osr_26(self) -> &'a mut W {
        self.variant(OSRW::OSR_26)
    }
    #[doc = "Oversampling ratio of 28."]
    #[inline]
    pub fn osr_27(self) -> &'a mut W {
        self.variant(OSRW::OSR_27)
    }
    #[doc = "Oversampling ratio of 29."]
    #[inline]
    pub fn osr_28(self) -> &'a mut W {
        self.variant(OSRW::OSR_28)
    }
    #[doc = "Oversampling ratio of 30."]
    #[inline]
    pub fn osr_29(self) -> &'a mut W {
        self.variant(OSRW::OSR_29)
    }
    #[doc = "Oversampling ratio of 31."]
    #[inline]
    pub fn osr_30(self) -> &'a mut W {
        self.variant(OSRW::OSR_30)
    }
    #[doc = "Oversampling ratio of 32."]
    #[inline]
    pub fn osr_31(self) -> &'a mut W {
        self.variant(OSRW::OSR_31)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 31;
        const OFFSET: u8 = 24;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `M10`"]
pub enum M10W {
    #[doc = "Receiver and transmitter use 7-bit to 9-bit data characters."]
    M10_0,
    #[doc = "Receiver and transmitter use 10-bit data characters."]
    M10_1,
}
impl M10W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            M10W::M10_0 => false,
            M10W::M10_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _M10W<'a> {
    w: &'a mut W,
}
impl<'a> _M10W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: M10W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Receiver and transmitter use 7-bit to 9-bit data characters."]
    #[inline]
    pub fn m10_0(self) -> &'a mut W {
        self.variant(M10W::M10_0)
    }
    #[doc = "Receiver and transmitter use 10-bit data characters."]
    #[inline]
    pub fn m10_1(self) -> &'a mut W {
        self.variant(M10W::M10_1)
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
        const OFFSET: u8 = 29;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `MAEN2`"]
pub enum MAEN2W {
    #[doc = "Normal operation."]
    MAEN2_0,
    #[doc = "Enables automatic address matching or data matching mode for MATCH[MA2]."]
    MAEN2_1,
}
impl MAEN2W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            MAEN2W::MAEN2_0 => false,
            MAEN2W::MAEN2_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MAEN2W<'a> {
    w: &'a mut W,
}
impl<'a> _MAEN2W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MAEN2W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Normal operation."]
    #[inline]
    pub fn maen2_0(self) -> &'a mut W {
        self.variant(MAEN2W::MAEN2_0)
    }
    #[doc = "Enables automatic address matching or data matching mode for MATCH[MA2]."]
    #[inline]
    pub fn maen2_1(self) -> &'a mut W {
        self.variant(MAEN2W::MAEN2_1)
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
#[doc = "Values that can be written to the field `MAEN1`"]
pub enum MAEN1W {
    #[doc = "Normal operation."]
    MAEN1_0,
    #[doc = "Enables automatic address matching or data matching mode for MATCH[MA1]."]
    MAEN1_1,
}
impl MAEN1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            MAEN1W::MAEN1_0 => false,
            MAEN1W::MAEN1_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MAEN1W<'a> {
    w: &'a mut W,
}
impl<'a> _MAEN1W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MAEN1W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Normal operation."]
    #[inline]
    pub fn maen1_0(self) -> &'a mut W {
        self.variant(MAEN1W::MAEN1_0)
    }
    #[doc = "Enables automatic address matching or data matching mode for MATCH[MA1]."]
    #[inline]
    pub fn maen1_1(self) -> &'a mut W {
        self.variant(MAEN1W::MAEN1_1)
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
        const OFFSET: u8 = 31;
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
    #[doc = "Bits 0:12 - Baud Rate Modulo Divisor."]
    #[inline]
    pub fn sbr(&self) -> SBRR {
        let bits = {
            const MASK: u16 = 8191;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        };
        SBRR { bits }
    }
    #[doc = "Bit 13 - Stop Bit Number Select"]
    #[inline]
    pub fn sbns(&self) -> SBNSR {
        SBNSR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 14 - RX Input Active Edge Interrupt Enable"]
    #[inline]
    pub fn rxedgie(&self) -> RXEDGIER {
        RXEDGIER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 15 - LIN Break Detect Interrupt Enable"]
    #[inline]
    pub fn lbkdie(&self) -> LBKDIER {
        LBKDIER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 15;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 16 - Resynchronization Disable"]
    #[inline]
    pub fn resyncdis(&self) -> RESYNCDISR {
        RESYNCDISR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 17 - Both Edge Sampling"]
    #[inline]
    pub fn bothedge(&self) -> BOTHEDGER {
        BOTHEDGER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 17;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 18:19 - Match Configuration"]
    #[inline]
    pub fn matcfg(&self) -> MATCFGR {
        MATCFGR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 21 - Receiver Full DMA Enable"]
    #[inline]
    pub fn rdmae(&self) -> RDMAER {
        RDMAER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 21;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 23 - Transmitter DMA Enable"]
    #[inline]
    pub fn tdmae(&self) -> TDMAER {
        TDMAER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 23;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 24:28 - Oversampling Ratio"]
    #[inline]
    pub fn osr(&self) -> OSRR {
        OSRR::_from({
            const MASK: u8 = 31;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 29 - 10-bit Mode select"]
    #[inline]
    pub fn m10(&self) -> M10R {
        M10R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 29;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 30 - Match Address Mode Enable 2"]
    #[inline]
    pub fn maen2(&self) -> MAEN2R {
        MAEN2R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 30;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 31 - Match Address Mode Enable 1"]
    #[inline]
    pub fn maen1(&self) -> MAEN1R {
        MAEN1R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 31;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 251658244 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:12 - Baud Rate Modulo Divisor."]
    #[inline]
    pub fn sbr(&mut self) -> _SBRW {
        _SBRW { w: self }
    }
    #[doc = "Bit 13 - Stop Bit Number Select"]
    #[inline]
    pub fn sbns(&mut self) -> _SBNSW {
        _SBNSW { w: self }
    }
    #[doc = "Bit 14 - RX Input Active Edge Interrupt Enable"]
    #[inline]
    pub fn rxedgie(&mut self) -> _RXEDGIEW {
        _RXEDGIEW { w: self }
    }
    #[doc = "Bit 15 - LIN Break Detect Interrupt Enable"]
    #[inline]
    pub fn lbkdie(&mut self) -> _LBKDIEW {
        _LBKDIEW { w: self }
    }
    #[doc = "Bit 16 - Resynchronization Disable"]
    #[inline]
    pub fn resyncdis(&mut self) -> _RESYNCDISW {
        _RESYNCDISW { w: self }
    }
    #[doc = "Bit 17 - Both Edge Sampling"]
    #[inline]
    pub fn bothedge(&mut self) -> _BOTHEDGEW {
        _BOTHEDGEW { w: self }
    }
    #[doc = "Bits 18:19 - Match Configuration"]
    #[inline]
    pub fn matcfg(&mut self) -> _MATCFGW {
        _MATCFGW { w: self }
    }
    #[doc = "Bit 21 - Receiver Full DMA Enable"]
    #[inline]
    pub fn rdmae(&mut self) -> _RDMAEW {
        _RDMAEW { w: self }
    }
    #[doc = "Bit 23 - Transmitter DMA Enable"]
    #[inline]
    pub fn tdmae(&mut self) -> _TDMAEW {
        _TDMAEW { w: self }
    }
    #[doc = "Bits 24:28 - Oversampling Ratio"]
    #[inline]
    pub fn osr(&mut self) -> _OSRW {
        _OSRW { w: self }
    }
    #[doc = "Bit 29 - 10-bit Mode select"]
    #[inline]
    pub fn m10(&mut self) -> _M10W {
        _M10W { w: self }
    }
    #[doc = "Bit 30 - Match Address Mode Enable 2"]
    #[inline]
    pub fn maen2(&mut self) -> _MAEN2W {
        _MAEN2W { w: self }
    }
    #[doc = "Bit 31 - Match Address Mode Enable 1"]
    #[inline]
    pub fn maen1(&mut self) -> _MAEN1W {
        _MAEN1W { w: self }
    }
}
