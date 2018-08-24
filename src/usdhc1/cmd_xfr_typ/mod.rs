#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CMD_XFR_TYP {
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
#[doc = "Possible values of the field `RSPTYP`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RSPTYPR {
    #[doc = "No Response"]
    RSPTYP_0,
    #[doc = "Response Length 136"]
    RSPTYP_1,
    #[doc = "Response Length 48"]
    RSPTYP_2,
    #[doc = "Response Length 48, check Busy after response"]
    RSPTYP_3,
}
impl RSPTYPR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            RSPTYPR::RSPTYP_0 => 0,
            RSPTYPR::RSPTYP_1 => 1,
            RSPTYPR::RSPTYP_2 => 2,
            RSPTYPR::RSPTYP_3 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> RSPTYPR {
        match value {
            0 => RSPTYPR::RSPTYP_0,
            1 => RSPTYPR::RSPTYP_1,
            2 => RSPTYPR::RSPTYP_2,
            3 => RSPTYPR::RSPTYP_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `RSPTYP_0`"]
    #[inline]
    pub fn is_rsptyp_0(&self) -> bool {
        *self == RSPTYPR::RSPTYP_0
    }
    #[doc = "Checks if the value of the field is `RSPTYP_1`"]
    #[inline]
    pub fn is_rsptyp_1(&self) -> bool {
        *self == RSPTYPR::RSPTYP_1
    }
    #[doc = "Checks if the value of the field is `RSPTYP_2`"]
    #[inline]
    pub fn is_rsptyp_2(&self) -> bool {
        *self == RSPTYPR::RSPTYP_2
    }
    #[doc = "Checks if the value of the field is `RSPTYP_3`"]
    #[inline]
    pub fn is_rsptyp_3(&self) -> bool {
        *self == RSPTYPR::RSPTYP_3
    }
}
#[doc = "Possible values of the field `CCCEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CCCENR {
    #[doc = "Disable"]
    CCCEN_0,
    #[doc = "Enable"]
    CCCEN_1,
}
impl CCCENR {
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
            CCCENR::CCCEN_0 => false,
            CCCENR::CCCEN_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CCCENR {
        match value {
            false => CCCENR::CCCEN_0,
            true => CCCENR::CCCEN_1,
        }
    }
    #[doc = "Checks if the value of the field is `CCCEN_0`"]
    #[inline]
    pub fn is_cccen_0(&self) -> bool {
        *self == CCCENR::CCCEN_0
    }
    #[doc = "Checks if the value of the field is `CCCEN_1`"]
    #[inline]
    pub fn is_cccen_1(&self) -> bool {
        *self == CCCENR::CCCEN_1
    }
}
#[doc = "Possible values of the field `CICEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CICENR {
    #[doc = "Disable"]
    CICEN_0,
    #[doc = "Enable"]
    CICEN_1,
}
impl CICENR {
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
            CICENR::CICEN_0 => false,
            CICENR::CICEN_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CICENR {
        match value {
            false => CICENR::CICEN_0,
            true => CICENR::CICEN_1,
        }
    }
    #[doc = "Checks if the value of the field is `CICEN_0`"]
    #[inline]
    pub fn is_cicen_0(&self) -> bool {
        *self == CICENR::CICEN_0
    }
    #[doc = "Checks if the value of the field is `CICEN_1`"]
    #[inline]
    pub fn is_cicen_1(&self) -> bool {
        *self == CICENR::CICEN_1
    }
}
#[doc = "Possible values of the field `DPSEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DPSELR {
    #[doc = "No Data Present"]
    DPSEL_0,
    #[doc = "Data Present"]
    DPSEL_1,
}
impl DPSELR {
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
            DPSELR::DPSEL_0 => false,
            DPSELR::DPSEL_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DPSELR {
        match value {
            false => DPSELR::DPSEL_0,
            true => DPSELR::DPSEL_1,
        }
    }
    #[doc = "Checks if the value of the field is `DPSEL_0`"]
    #[inline]
    pub fn is_dpsel_0(&self) -> bool {
        *self == DPSELR::DPSEL_0
    }
    #[doc = "Checks if the value of the field is `DPSEL_1`"]
    #[inline]
    pub fn is_dpsel_1(&self) -> bool {
        *self == DPSELR::DPSEL_1
    }
}
#[doc = "Possible values of the field `CMDTYP`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CMDTYPR {
    #[doc = "Normal Other commands"]
    CMDTYP_0,
    #[doc = "Suspend CMD52 for writing Bus Suspend in CCCR"]
    CMDTYP_1,
    #[doc = "Resume CMD52 for writing Function Select in CCCR"]
    CMDTYP_2,
    #[doc = "Abort CMD12, CMD52 for writing I/O Abort in CCCR"]
    CMDTYP_3,
}
impl CMDTYPR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            CMDTYPR::CMDTYP_0 => 0,
            CMDTYPR::CMDTYP_1 => 1,
            CMDTYPR::CMDTYP_2 => 2,
            CMDTYPR::CMDTYP_3 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> CMDTYPR {
        match value {
            0 => CMDTYPR::CMDTYP_0,
            1 => CMDTYPR::CMDTYP_1,
            2 => CMDTYPR::CMDTYP_2,
            3 => CMDTYPR::CMDTYP_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `CMDTYP_0`"]
    #[inline]
    pub fn is_cmdtyp_0(&self) -> bool {
        *self == CMDTYPR::CMDTYP_0
    }
    #[doc = "Checks if the value of the field is `CMDTYP_1`"]
    #[inline]
    pub fn is_cmdtyp_1(&self) -> bool {
        *self == CMDTYPR::CMDTYP_1
    }
    #[doc = "Checks if the value of the field is `CMDTYP_2`"]
    #[inline]
    pub fn is_cmdtyp_2(&self) -> bool {
        *self == CMDTYPR::CMDTYP_2
    }
    #[doc = "Checks if the value of the field is `CMDTYP_3`"]
    #[inline]
    pub fn is_cmdtyp_3(&self) -> bool {
        *self == CMDTYPR::CMDTYP_3
    }
}
#[doc = r" Value of the field"]
pub struct CMDINXR {
    bits: u8,
}
impl CMDINXR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Values that can be written to the field `RSPTYP`"]
pub enum RSPTYPW {
    #[doc = "No Response"]
    RSPTYP_0,
    #[doc = "Response Length 136"]
    RSPTYP_1,
    #[doc = "Response Length 48"]
    RSPTYP_2,
    #[doc = "Response Length 48, check Busy after response"]
    RSPTYP_3,
}
impl RSPTYPW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            RSPTYPW::RSPTYP_0 => 0,
            RSPTYPW::RSPTYP_1 => 1,
            RSPTYPW::RSPTYP_2 => 2,
            RSPTYPW::RSPTYP_3 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RSPTYPW<'a> {
    w: &'a mut W,
}
impl<'a> _RSPTYPW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RSPTYPW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "No Response"]
    #[inline]
    pub fn rsptyp_0(self) -> &'a mut W {
        self.variant(RSPTYPW::RSPTYP_0)
    }
    #[doc = "Response Length 136"]
    #[inline]
    pub fn rsptyp_1(self) -> &'a mut W {
        self.variant(RSPTYPW::RSPTYP_1)
    }
    #[doc = "Response Length 48"]
    #[inline]
    pub fn rsptyp_2(self) -> &'a mut W {
        self.variant(RSPTYPW::RSPTYP_2)
    }
    #[doc = "Response Length 48, check Busy after response"]
    #[inline]
    pub fn rsptyp_3(self) -> &'a mut W {
        self.variant(RSPTYPW::RSPTYP_3)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 16;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `CCCEN`"]
pub enum CCCENW {
    #[doc = "Disable"]
    CCCEN_0,
    #[doc = "Enable"]
    CCCEN_1,
}
impl CCCENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CCCENW::CCCEN_0 => false,
            CCCENW::CCCEN_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CCCENW<'a> {
    w: &'a mut W,
}
impl<'a> _CCCENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CCCENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable"]
    #[inline]
    pub fn cccen_0(self) -> &'a mut W {
        self.variant(CCCENW::CCCEN_0)
    }
    #[doc = "Enable"]
    #[inline]
    pub fn cccen_1(self) -> &'a mut W {
        self.variant(CCCENW::CCCEN_1)
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
        const OFFSET: u8 = 19;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `CICEN`"]
pub enum CICENW {
    #[doc = "Disable"]
    CICEN_0,
    #[doc = "Enable"]
    CICEN_1,
}
impl CICENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CICENW::CICEN_0 => false,
            CICENW::CICEN_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CICENW<'a> {
    w: &'a mut W,
}
impl<'a> _CICENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CICENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable"]
    #[inline]
    pub fn cicen_0(self) -> &'a mut W {
        self.variant(CICENW::CICEN_0)
    }
    #[doc = "Enable"]
    #[inline]
    pub fn cicen_1(self) -> &'a mut W {
        self.variant(CICENW::CICEN_1)
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
        const OFFSET: u8 = 20;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `DPSEL`"]
pub enum DPSELW {
    #[doc = "No Data Present"]
    DPSEL_0,
    #[doc = "Data Present"]
    DPSEL_1,
}
impl DPSELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DPSELW::DPSEL_0 => false,
            DPSELW::DPSEL_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DPSELW<'a> {
    w: &'a mut W,
}
impl<'a> _DPSELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DPSELW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No Data Present"]
    #[inline]
    pub fn dpsel_0(self) -> &'a mut W {
        self.variant(DPSELW::DPSEL_0)
    }
    #[doc = "Data Present"]
    #[inline]
    pub fn dpsel_1(self) -> &'a mut W {
        self.variant(DPSELW::DPSEL_1)
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
#[doc = "Values that can be written to the field `CMDTYP`"]
pub enum CMDTYPW {
    #[doc = "Normal Other commands"]
    CMDTYP_0,
    #[doc = "Suspend CMD52 for writing Bus Suspend in CCCR"]
    CMDTYP_1,
    #[doc = "Resume CMD52 for writing Function Select in CCCR"]
    CMDTYP_2,
    #[doc = "Abort CMD12, CMD52 for writing I/O Abort in CCCR"]
    CMDTYP_3,
}
impl CMDTYPW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            CMDTYPW::CMDTYP_0 => 0,
            CMDTYPW::CMDTYP_1 => 1,
            CMDTYPW::CMDTYP_2 => 2,
            CMDTYPW::CMDTYP_3 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CMDTYPW<'a> {
    w: &'a mut W,
}
impl<'a> _CMDTYPW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CMDTYPW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Normal Other commands"]
    #[inline]
    pub fn cmdtyp_0(self) -> &'a mut W {
        self.variant(CMDTYPW::CMDTYP_0)
    }
    #[doc = "Suspend CMD52 for writing Bus Suspend in CCCR"]
    #[inline]
    pub fn cmdtyp_1(self) -> &'a mut W {
        self.variant(CMDTYPW::CMDTYP_1)
    }
    #[doc = "Resume CMD52 for writing Function Select in CCCR"]
    #[inline]
    pub fn cmdtyp_2(self) -> &'a mut W {
        self.variant(CMDTYPW::CMDTYP_2)
    }
    #[doc = "Abort CMD12, CMD52 for writing I/O Abort in CCCR"]
    #[inline]
    pub fn cmdtyp_3(self) -> &'a mut W {
        self.variant(CMDTYPW::CMDTYP_3)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 22;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _CMDINXW<'a> {
    w: &'a mut W,
}
impl<'a> _CMDINXW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 63;
        const OFFSET: u8 = 24;
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
    #[doc = "Bits 16:17 - Response Type Select"]
    #[inline]
    pub fn rsptyp(&self) -> RSPTYPR {
        RSPTYPR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 19 - Command CRC Check Enable"]
    #[inline]
    pub fn cccen(&self) -> CCCENR {
        CCCENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 19;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 20 - Command Index Check Enable"]
    #[inline]
    pub fn cicen(&self) -> CICENR {
        CICENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 21 - Data Present Select"]
    #[inline]
    pub fn dpsel(&self) -> DPSELR {
        DPSELR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 21;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 22:23 - Command Type"]
    #[inline]
    pub fn cmdtyp(&self) -> CMDTYPR {
        CMDTYPR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 22;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 24:29 - Command Index"]
    #[inline]
    pub fn cmdinx(&self) -> CMDINXR {
        let bits = {
            const MASK: u8 = 63;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        CMDINXR { bits }
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
    #[doc = "Bits 16:17 - Response Type Select"]
    #[inline]
    pub fn rsptyp(&mut self) -> _RSPTYPW {
        _RSPTYPW { w: self }
    }
    #[doc = "Bit 19 - Command CRC Check Enable"]
    #[inline]
    pub fn cccen(&mut self) -> _CCCENW {
        _CCCENW { w: self }
    }
    #[doc = "Bit 20 - Command Index Check Enable"]
    #[inline]
    pub fn cicen(&mut self) -> _CICENW {
        _CICENW { w: self }
    }
    #[doc = "Bit 21 - Data Present Select"]
    #[inline]
    pub fn dpsel(&mut self) -> _DPSELW {
        _DPSELW { w: self }
    }
    #[doc = "Bits 22:23 - Command Type"]
    #[inline]
    pub fn cmdtyp(&mut self) -> _CMDTYPW {
        _CMDTYPW { w: self }
    }
    #[doc = "Bits 24:29 - Command Index"]
    #[inline]
    pub fn cmdinx(&mut self) -> _CMDINXW {
        _CMDINXW { w: self }
    }
}
