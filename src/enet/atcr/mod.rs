#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::ATCR {
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
#[doc = "Possible values of the field `EN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ENR {
    #[doc = "The timer stops at the current value."]
    EN_0,
    #[doc = "The timer starts incrementing."]
    EN_1,
}
impl ENR {
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
            ENR::EN_0 => false,
            ENR::EN_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ENR {
        match value {
            false => ENR::EN_0,
            true => ENR::EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `EN_0`"]
    #[inline]
    pub fn is_en_0(&self) -> bool {
        *self == ENR::EN_0
    }
    #[doc = "Checks if the value of the field is `EN_1`"]
    #[inline]
    pub fn is_en_1(&self) -> bool {
        *self == ENR::EN_1
    }
}
#[doc = "Possible values of the field `OFFEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OFFENR {
    #[doc = "Disable."]
    OFFEN_0,
    #[doc = "The timer can be reset to zero when the given offset time is reached (offset event). The field is cleared when the offset event is reached, so no further event occurs until the field is set again. The timer offset value must be set before setting this field."]
    OFFEN_1,
}
impl OFFENR {
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
            OFFENR::OFFEN_0 => false,
            OFFENR::OFFEN_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> OFFENR {
        match value {
            false => OFFENR::OFFEN_0,
            true => OFFENR::OFFEN_1,
        }
    }
    #[doc = "Checks if the value of the field is `OFFEN_0`"]
    #[inline]
    pub fn is_offen_0(&self) -> bool {
        *self == OFFENR::OFFEN_0
    }
    #[doc = "Checks if the value of the field is `OFFEN_1`"]
    #[inline]
    pub fn is_offen_1(&self) -> bool {
        *self == OFFENR::OFFEN_1
    }
}
#[doc = "Possible values of the field `OFFRST`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OFFRSTR {
    #[doc = "The timer is not affected and no action occurs, besides clearing OFFEN, when the offset is reached."]
    OFFRST_0,
    #[doc = "If OFFEN is set, the timer resets to zero when the offset setting is reached. The offset event does not cause a timer interrupt."]
    OFFRST_1,
}
impl OFFRSTR {
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
            OFFRSTR::OFFRST_0 => false,
            OFFRSTR::OFFRST_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> OFFRSTR {
        match value {
            false => OFFRSTR::OFFRST_0,
            true => OFFRSTR::OFFRST_1,
        }
    }
    #[doc = "Checks if the value of the field is `OFFRST_0`"]
    #[inline]
    pub fn is_offrst_0(&self) -> bool {
        *self == OFFRSTR::OFFRST_0
    }
    #[doc = "Checks if the value of the field is `OFFRST_1`"]
    #[inline]
    pub fn is_offrst_1(&self) -> bool {
        *self == OFFRSTR::OFFRST_1
    }
}
#[doc = "Possible values of the field `PEREN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PERENR {
    #[doc = "Disable."]
    PEREN_0,
    #[doc = "A period event interrupt can be generated (EIR[TS_TIMER]) and the event signal output is asserted when the timer wraps around according to the periodic setting ATPER. The timer period value must be set before setting this bit. Not all devices contain the event signal output. See the chip configuration details."]
    PEREN_1,
}
impl PERENR {
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
            PERENR::PEREN_0 => false,
            PERENR::PEREN_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PERENR {
        match value {
            false => PERENR::PEREN_0,
            true => PERENR::PEREN_1,
        }
    }
    #[doc = "Checks if the value of the field is `PEREN_0`"]
    #[inline]
    pub fn is_peren_0(&self) -> bool {
        *self == PERENR::PEREN_0
    }
    #[doc = "Checks if the value of the field is `PEREN_1`"]
    #[inline]
    pub fn is_peren_1(&self) -> bool {
        *self == PERENR::PEREN_1
    }
}
#[doc = "Possible values of the field `PINPER`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PINPERR {
    #[doc = "Disable."]
    PINPER_0,
    #[doc = "Enable."]
    PINPER_1,
}
impl PINPERR {
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
            PINPERR::PINPER_0 => false,
            PINPERR::PINPER_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PINPERR {
        match value {
            false => PINPERR::PINPER_0,
            true => PINPERR::PINPER_1,
        }
    }
    #[doc = "Checks if the value of the field is `PINPER_0`"]
    #[inline]
    pub fn is_pinper_0(&self) -> bool {
        *self == PINPERR::PINPER_0
    }
    #[doc = "Checks if the value of the field is `PINPER_1`"]
    #[inline]
    pub fn is_pinper_1(&self) -> bool {
        *self == PINPERR::PINPER_1
    }
}
#[doc = r" Value of the field"]
pub struct RESTARTR {
    bits: bool,
}
impl RESTARTR {
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
#[doc = "Possible values of the field `CAPTURE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CAPTURER {
    #[doc = "No effect."]
    CAPTURE_0,
    #[doc = "The current time is captured and can be read from the ATVR register."]
    CAPTURE_1,
}
impl CAPTURER {
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
            CAPTURER::CAPTURE_0 => false,
            CAPTURER::CAPTURE_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CAPTURER {
        match value {
            false => CAPTURER::CAPTURE_0,
            true => CAPTURER::CAPTURE_1,
        }
    }
    #[doc = "Checks if the value of the field is `CAPTURE_0`"]
    #[inline]
    pub fn is_capture_0(&self) -> bool {
        *self == CAPTURER::CAPTURE_0
    }
    #[doc = "Checks if the value of the field is `CAPTURE_1`"]
    #[inline]
    pub fn is_capture_1(&self) -> bool {
        *self == CAPTURER::CAPTURE_1
    }
}
#[doc = "Possible values of the field `SLAVE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SLAVER {
    #[doc = "The timer is active and all configuration fields in this register are relevant."]
    SLAVE_0,
    #[doc = "The internal timer is disabled and the externally provided timer value is used. All other fields, except CAPTURE, in this register have no effect. CAPTURE can still be used to capture the current timer value."]
    SLAVE_1,
}
impl SLAVER {
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
            SLAVER::SLAVE_0 => false,
            SLAVER::SLAVE_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SLAVER {
        match value {
            false => SLAVER::SLAVE_0,
            true => SLAVER::SLAVE_1,
        }
    }
    #[doc = "Checks if the value of the field is `SLAVE_0`"]
    #[inline]
    pub fn is_slave_0(&self) -> bool {
        *self == SLAVER::SLAVE_0
    }
    #[doc = "Checks if the value of the field is `SLAVE_1`"]
    #[inline]
    pub fn is_slave_1(&self) -> bool {
        *self == SLAVER::SLAVE_1
    }
}
#[doc = "Values that can be written to the field `EN`"]
pub enum ENW {
    #[doc = "The timer stops at the current value."]
    EN_0,
    #[doc = "The timer starts incrementing."]
    EN_1,
}
impl ENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ENW::EN_0 => false,
            ENW::EN_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ENW<'a> {
    w: &'a mut W,
}
impl<'a> _ENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The timer stops at the current value."]
    #[inline]
    pub fn en_0(self) -> &'a mut W {
        self.variant(ENW::EN_0)
    }
    #[doc = "The timer starts incrementing."]
    #[inline]
    pub fn en_1(self) -> &'a mut W {
        self.variant(ENW::EN_1)
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
#[doc = "Values that can be written to the field `OFFEN`"]
pub enum OFFENW {
    #[doc = "Disable."]
    OFFEN_0,
    #[doc = "The timer can be reset to zero when the given offset time is reached (offset event). The field is cleared when the offset event is reached, so no further event occurs until the field is set again. The timer offset value must be set before setting this field."]
    OFFEN_1,
}
impl OFFENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            OFFENW::OFFEN_0 => false,
            OFFENW::OFFEN_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _OFFENW<'a> {
    w: &'a mut W,
}
impl<'a> _OFFENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: OFFENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable."]
    #[inline]
    pub fn offen_0(self) -> &'a mut W {
        self.variant(OFFENW::OFFEN_0)
    }
    #[doc = "The timer can be reset to zero when the given offset time is reached (offset event). The field is cleared when the offset event is reached, so no further event occurs until the field is set again. The timer offset value must be set before setting this field."]
    #[inline]
    pub fn offen_1(self) -> &'a mut W {
        self.variant(OFFENW::OFFEN_1)
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
#[doc = "Values that can be written to the field `OFFRST`"]
pub enum OFFRSTW {
    #[doc = "The timer is not affected and no action occurs, besides clearing OFFEN, when the offset is reached."]
    OFFRST_0,
    #[doc = "If OFFEN is set, the timer resets to zero when the offset setting is reached. The offset event does not cause a timer interrupt."]
    OFFRST_1,
}
impl OFFRSTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            OFFRSTW::OFFRST_0 => false,
            OFFRSTW::OFFRST_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _OFFRSTW<'a> {
    w: &'a mut W,
}
impl<'a> _OFFRSTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: OFFRSTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The timer is not affected and no action occurs, besides clearing OFFEN, when the offset is reached."]
    #[inline]
    pub fn offrst_0(self) -> &'a mut W {
        self.variant(OFFRSTW::OFFRST_0)
    }
    #[doc = "If OFFEN is set, the timer resets to zero when the offset setting is reached. The offset event does not cause a timer interrupt."]
    #[inline]
    pub fn offrst_1(self) -> &'a mut W {
        self.variant(OFFRSTW::OFFRST_1)
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
#[doc = "Values that can be written to the field `PEREN`"]
pub enum PERENW {
    #[doc = "Disable."]
    PEREN_0,
    #[doc = "A period event interrupt can be generated (EIR[TS_TIMER]) and the event signal output is asserted when the timer wraps around according to the periodic setting ATPER. The timer period value must be set before setting this bit. Not all devices contain the event signal output. See the chip configuration details."]
    PEREN_1,
}
impl PERENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PERENW::PEREN_0 => false,
            PERENW::PEREN_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PERENW<'a> {
    w: &'a mut W,
}
impl<'a> _PERENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PERENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable."]
    #[inline]
    pub fn peren_0(self) -> &'a mut W {
        self.variant(PERENW::PEREN_0)
    }
    #[doc = "A period event interrupt can be generated (EIR[TS_TIMER]) and the event signal output is asserted when the timer wraps around according to the periodic setting ATPER. The timer period value must be set before setting this bit. Not all devices contain the event signal output. See the chip configuration details."]
    #[inline]
    pub fn peren_1(self) -> &'a mut W {
        self.variant(PERENW::PEREN_1)
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
        const OFFSET: u8 = 4;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `PINPER`"]
pub enum PINPERW {
    #[doc = "Disable."]
    PINPER_0,
    #[doc = "Enable."]
    PINPER_1,
}
impl PINPERW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PINPERW::PINPER_0 => false,
            PINPERW::PINPER_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PINPERW<'a> {
    w: &'a mut W,
}
impl<'a> _PINPERW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PINPERW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable."]
    #[inline]
    pub fn pinper_0(self) -> &'a mut W {
        self.variant(PINPERW::PINPER_0)
    }
    #[doc = "Enable."]
    #[inline]
    pub fn pinper_1(self) -> &'a mut W {
        self.variant(PINPERW::PINPER_1)
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
        const OFFSET: u8 = 7;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _RESTARTW<'a> {
    w: &'a mut W,
}
impl<'a> _RESTARTW<'a> {
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
#[doc = "Values that can be written to the field `CAPTURE`"]
pub enum CAPTUREW {
    #[doc = "No effect."]
    CAPTURE_0,
    #[doc = "The current time is captured and can be read from the ATVR register."]
    CAPTURE_1,
}
impl CAPTUREW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CAPTUREW::CAPTURE_0 => false,
            CAPTUREW::CAPTURE_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CAPTUREW<'a> {
    w: &'a mut W,
}
impl<'a> _CAPTUREW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CAPTUREW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No effect."]
    #[inline]
    pub fn capture_0(self) -> &'a mut W {
        self.variant(CAPTUREW::CAPTURE_0)
    }
    #[doc = "The current time is captured and can be read from the ATVR register."]
    #[inline]
    pub fn capture_1(self) -> &'a mut W {
        self.variant(CAPTUREW::CAPTURE_1)
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
#[doc = "Values that can be written to the field `SLAVE`"]
pub enum SLAVEW {
    #[doc = "The timer is active and all configuration fields in this register are relevant."]
    SLAVE_0,
    #[doc = "The internal timer is disabled and the externally provided timer value is used. All other fields, except CAPTURE, in this register have no effect. CAPTURE can still be used to capture the current timer value."]
    SLAVE_1,
}
impl SLAVEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SLAVEW::SLAVE_0 => false,
            SLAVEW::SLAVE_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SLAVEW<'a> {
    w: &'a mut W,
}
impl<'a> _SLAVEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SLAVEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The timer is active and all configuration fields in this register are relevant."]
    #[inline]
    pub fn slave_0(self) -> &'a mut W {
        self.variant(SLAVEW::SLAVE_0)
    }
    #[doc = "The internal timer is disabled and the externally provided timer value is used. All other fields, except CAPTURE, in this register have no effect. CAPTURE can still be used to capture the current timer value."]
    #[inline]
    pub fn slave_1(self) -> &'a mut W {
        self.variant(SLAVEW::SLAVE_1)
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
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - Enable Timer"]
    #[inline]
    pub fn en(&self) -> ENR {
        ENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - Enable One-Shot Offset Event"]
    #[inline]
    pub fn offen(&self) -> OFFENR {
        OFFENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - Reset Timer On Offset Event"]
    #[inline]
    pub fn offrst(&self) -> OFFRSTR {
        OFFRSTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 4 - Enable Periodical Event"]
    #[inline]
    pub fn peren(&self) -> PERENR {
        PERENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 7 - Enables event signal output assertion on period event"]
    #[inline]
    pub fn pinper(&self) -> PINPERR {
        PINPERR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 9 - Reset Timer"]
    #[inline]
    pub fn restart(&self) -> RESTARTR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        RESTARTR { bits }
    }
    #[doc = "Bit 11 - Capture Timer Value"]
    #[inline]
    pub fn capture(&self) -> CAPTURER {
        CAPTURER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 13 - Enable Timer Slave Mode"]
    #[inline]
    pub fn slave(&self) -> SLAVER {
        SLAVER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u32) != 0
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
    #[doc = "Bit 0 - Enable Timer"]
    #[inline]
    pub fn en(&mut self) -> _ENW {
        _ENW { w: self }
    }
    #[doc = "Bit 2 - Enable One-Shot Offset Event"]
    #[inline]
    pub fn offen(&mut self) -> _OFFENW {
        _OFFENW { w: self }
    }
    #[doc = "Bit 3 - Reset Timer On Offset Event"]
    #[inline]
    pub fn offrst(&mut self) -> _OFFRSTW {
        _OFFRSTW { w: self }
    }
    #[doc = "Bit 4 - Enable Periodical Event"]
    #[inline]
    pub fn peren(&mut self) -> _PERENW {
        _PERENW { w: self }
    }
    #[doc = "Bit 7 - Enables event signal output assertion on period event"]
    #[inline]
    pub fn pinper(&mut self) -> _PINPERW {
        _PINPERW { w: self }
    }
    #[doc = "Bit 9 - Reset Timer"]
    #[inline]
    pub fn restart(&mut self) -> _RESTARTW {
        _RESTARTW { w: self }
    }
    #[doc = "Bit 11 - Capture Timer Value"]
    #[inline]
    pub fn capture(&mut self) -> _CAPTUREW {
        _CAPTUREW { w: self }
    }
    #[doc = "Bit 13 - Enable Timer Slave Mode"]
    #[inline]
    pub fn slave(&mut self) -> _SLAVEW {
        _SLAVEW { w: self }
    }
}
