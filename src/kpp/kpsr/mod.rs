#[doc = r" Value read from the register"]
pub struct R {
    bits: u16,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u16,
}
impl super::KPSR {
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
#[doc = "Possible values of the field `KPKD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum KPKDR {
    #[doc = "No key presses detected"]
    KPKD_0,
    #[doc = "A key has been depressed"]
    KPKD_1,
}
impl KPKDR {
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
            KPKDR::KPKD_0 => false,
            KPKDR::KPKD_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> KPKDR {
        match value {
            false => KPKDR::KPKD_0,
            true => KPKDR::KPKD_1,
        }
    }
    #[doc = "Checks if the value of the field is `KPKD_0`"]
    #[inline]
    pub fn is_kpkd_0(&self) -> bool {
        *self == KPKDR::KPKD_0
    }
    #[doc = "Checks if the value of the field is `KPKD_1`"]
    #[inline]
    pub fn is_kpkd_1(&self) -> bool {
        *self == KPKDR::KPKD_1
    }
}
#[doc = "Possible values of the field `KPKR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum KPKRR {
    #[doc = "No key release detected"]
    KPKR_0,
    #[doc = "All keys have been released"]
    KPKR_1,
}
impl KPKRR {
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
            KPKRR::KPKR_0 => false,
            KPKRR::KPKR_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> KPKRR {
        match value {
            false => KPKRR::KPKR_0,
            true => KPKRR::KPKR_1,
        }
    }
    #[doc = "Checks if the value of the field is `KPKR_0`"]
    #[inline]
    pub fn is_kpkr_0(&self) -> bool {
        *self == KPKRR::KPKR_0
    }
    #[doc = "Checks if the value of the field is `KPKR_1`"]
    #[inline]
    pub fn is_kpkr_1(&self) -> bool {
        *self == KPKRR::KPKR_1
    }
}
#[doc = "Possible values of the field `KDIE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum KDIER {
    #[doc = "No interrupt request is generated when KPKD is set."]
    KDIE_0,
    #[doc = "An interrupt request is generated when KPKD is set."]
    KDIE_1,
}
impl KDIER {
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
            KDIER::KDIE_0 => false,
            KDIER::KDIE_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> KDIER {
        match value {
            false => KDIER::KDIE_0,
            true => KDIER::KDIE_1,
        }
    }
    #[doc = "Checks if the value of the field is `KDIE_0`"]
    #[inline]
    pub fn is_kdie_0(&self) -> bool {
        *self == KDIER::KDIE_0
    }
    #[doc = "Checks if the value of the field is `KDIE_1`"]
    #[inline]
    pub fn is_kdie_1(&self) -> bool {
        *self == KDIER::KDIE_1
    }
}
#[doc = "Possible values of the field `KRIE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum KRIER {
    #[doc = "No interrupt request is generated when KPKR is set."]
    KRIE_0,
    #[doc = "An interrupt request is generated when KPKR is set."]
    KRIE_1,
}
impl KRIER {
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
            KRIER::KRIE_0 => false,
            KRIER::KRIE_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> KRIER {
        match value {
            false => KRIER::KRIE_0,
            true => KRIER::KRIE_1,
        }
    }
    #[doc = "Checks if the value of the field is `KRIE_0`"]
    #[inline]
    pub fn is_krie_0(&self) -> bool {
        *self == KRIER::KRIE_0
    }
    #[doc = "Checks if the value of the field is `KRIE_1`"]
    #[inline]
    pub fn is_krie_1(&self) -> bool {
        *self == KRIER::KRIE_1
    }
}
#[doc = "Values that can be written to the field `KPKD`"]
pub enum KPKDW {
    #[doc = "No key presses detected"]
    KPKD_0,
    #[doc = "A key has been depressed"]
    KPKD_1,
}
impl KPKDW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            KPKDW::KPKD_0 => false,
            KPKDW::KPKD_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _KPKDW<'a> {
    w: &'a mut W,
}
impl<'a> _KPKDW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: KPKDW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No key presses detected"]
    #[inline]
    pub fn kpkd_0(self) -> &'a mut W {
        self.variant(KPKDW::KPKD_0)
    }
    #[doc = "A key has been depressed"]
    #[inline]
    pub fn kpkd_1(self) -> &'a mut W {
        self.variant(KPKDW::KPKD_1)
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
        self.w.bits &= !((MASK as u16) << OFFSET);
        self.w.bits |= ((value & MASK) as u16) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `KPKR`"]
pub enum KPKRW {
    #[doc = "No key release detected"]
    KPKR_0,
    #[doc = "All keys have been released"]
    KPKR_1,
}
impl KPKRW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            KPKRW::KPKR_0 => false,
            KPKRW::KPKR_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _KPKRW<'a> {
    w: &'a mut W,
}
impl<'a> _KPKRW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: KPKRW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No key release detected"]
    #[inline]
    pub fn kpkr_0(self) -> &'a mut W {
        self.variant(KPKRW::KPKR_0)
    }
    #[doc = "All keys have been released"]
    #[inline]
    pub fn kpkr_1(self) -> &'a mut W {
        self.variant(KPKRW::KPKR_1)
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
        self.w.bits &= !((MASK as u16) << OFFSET);
        self.w.bits |= ((value & MASK) as u16) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `KDSC`"]
pub enum KDSCW {
    #[doc = "No effect"]
    KDSC_0,
    #[doc = "Set bits that clear the keypad depress synchronizer chain"]
    KDSC_1,
}
impl KDSCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            KDSCW::KDSC_0 => false,
            KDSCW::KDSC_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _KDSCW<'a> {
    w: &'a mut W,
}
impl<'a> _KDSCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: KDSCW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No effect"]
    #[inline]
    pub fn kdsc_0(self) -> &'a mut W {
        self.variant(KDSCW::KDSC_0)
    }
    #[doc = "Set bits that clear the keypad depress synchronizer chain"]
    #[inline]
    pub fn kdsc_1(self) -> &'a mut W {
        self.variant(KDSCW::KDSC_1)
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
        self.w.bits &= !((MASK as u16) << OFFSET);
        self.w.bits |= ((value & MASK) as u16) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `KRSS`"]
pub enum KRSSW {
    #[doc = "No effect"]
    KRSS_0,
    #[doc = "Set bits which sets keypad release synchronizer chain"]
    KRSS_1,
}
impl KRSSW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            KRSSW::KRSS_0 => false,
            KRSSW::KRSS_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _KRSSW<'a> {
    w: &'a mut W,
}
impl<'a> _KRSSW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: KRSSW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No effect"]
    #[inline]
    pub fn krss_0(self) -> &'a mut W {
        self.variant(KRSSW::KRSS_0)
    }
    #[doc = "Set bits which sets keypad release synchronizer chain"]
    #[inline]
    pub fn krss_1(self) -> &'a mut W {
        self.variant(KRSSW::KRSS_1)
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
        self.w.bits &= !((MASK as u16) << OFFSET);
        self.w.bits |= ((value & MASK) as u16) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `KDIE`"]
pub enum KDIEW {
    #[doc = "No interrupt request is generated when KPKD is set."]
    KDIE_0,
    #[doc = "An interrupt request is generated when KPKD is set."]
    KDIE_1,
}
impl KDIEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            KDIEW::KDIE_0 => false,
            KDIEW::KDIE_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _KDIEW<'a> {
    w: &'a mut W,
}
impl<'a> _KDIEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: KDIEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No interrupt request is generated when KPKD is set."]
    #[inline]
    pub fn kdie_0(self) -> &'a mut W {
        self.variant(KDIEW::KDIE_0)
    }
    #[doc = "An interrupt request is generated when KPKD is set."]
    #[inline]
    pub fn kdie_1(self) -> &'a mut W {
        self.variant(KDIEW::KDIE_1)
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
        self.w.bits &= !((MASK as u16) << OFFSET);
        self.w.bits |= ((value & MASK) as u16) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `KRIE`"]
pub enum KRIEW {
    #[doc = "No interrupt request is generated when KPKR is set."]
    KRIE_0,
    #[doc = "An interrupt request is generated when KPKR is set."]
    KRIE_1,
}
impl KRIEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            KRIEW::KRIE_0 => false,
            KRIEW::KRIE_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _KRIEW<'a> {
    w: &'a mut W,
}
impl<'a> _KRIEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: KRIEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No interrupt request is generated when KPKR is set."]
    #[inline]
    pub fn krie_0(self) -> &'a mut W {
        self.variant(KRIEW::KRIE_0)
    }
    #[doc = "An interrupt request is generated when KPKR is set."]
    #[inline]
    pub fn krie_1(self) -> &'a mut W {
        self.variant(KRIEW::KRIE_1)
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
        self.w.bits &= !((MASK as u16) << OFFSET);
        self.w.bits |= ((value & MASK) as u16) << OFFSET;
        self.w
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }
    #[doc = "Bit 0 - Keypad Key Depress"]
    #[inline]
    pub fn kpkd(&self) -> KPKDR {
        KPKDR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        })
    }
    #[doc = "Bit 1 - Keypad Key Release"]
    #[inline]
    pub fn kpkr(&self) -> KPKRR {
        KPKRR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        })
    }
    #[doc = "Bit 8 - Keypad Key Depress Interrupt Enable"]
    #[inline]
    pub fn kdie(&self) -> KDIER {
        KDIER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        })
    }
    #[doc = "Bit 9 - Keypad Release Interrupt Enable"]
    #[inline]
    pub fn krie(&self) -> KRIER {
        KRIER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        })
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 1024 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - Keypad Key Depress"]
    #[inline]
    pub fn kpkd(&mut self) -> _KPKDW {
        _KPKDW { w: self }
    }
    #[doc = "Bit 1 - Keypad Key Release"]
    #[inline]
    pub fn kpkr(&mut self) -> _KPKRW {
        _KPKRW { w: self }
    }
    #[doc = "Bit 2 - Key Depress Synchronizer Clear"]
    #[inline]
    pub fn kdsc(&mut self) -> _KDSCW {
        _KDSCW { w: self }
    }
    #[doc = "Bit 3 - Key Release Synchronizer Set"]
    #[inline]
    pub fn krss(&mut self) -> _KRSSW {
        _KRSSW { w: self }
    }
    #[doc = "Bit 8 - Keypad Key Depress Interrupt Enable"]
    #[inline]
    pub fn kdie(&mut self) -> _KDIEW {
        _KDIEW { w: self }
    }
    #[doc = "Bit 9 - Keypad Release Interrupt Enable"]
    #[inline]
    pub fn krie(&mut self) -> _KRIEW {
        _KRIEW { w: self }
    }
}
