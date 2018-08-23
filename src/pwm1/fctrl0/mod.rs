#[doc = r" Value read from the register"]
pub struct R {
    bits: u16,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u16,
}
impl super::FCTRL0 {
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
#[doc = "Possible values of the field `FIE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FIER {
    #[doc = "FAULTx CPU interrupt requests disabled."]
    FIE_0,
    #[doc = "FAULTx CPU interrupt requests enabled."]
    FIE_1,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl FIER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            FIER::FIE_0 => 0,
            FIER::FIE_1 => 1,
            FIER::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> FIER {
        match value {
            0 => FIER::FIE_0,
            1 => FIER::FIE_1,
            i => FIER::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `FIE_0`"]
    #[inline]
    pub fn is_fie_0(&self) -> bool {
        *self == FIER::FIE_0
    }
    #[doc = "Checks if the value of the field is `FIE_1`"]
    #[inline]
    pub fn is_fie_1(&self) -> bool {
        *self == FIER::FIE_1
    }
}
#[doc = "Possible values of the field `FSAFE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FSAFER {
    #[doc = "Normal mode. PWM outputs disabled by this fault are not enabled until FSTS[FFLAGx] is clear at the start of a half cycle or full cycle depending on the state of FSTS[FFULL] without regard to the state of FSTS[FFPINx]. The PWM outputs disabled by this fault input will not be re-enabled until the actual FAULTx input signal de-asserts since the fault input will combinationally disable the PWM outputs (as programmed in DISMAPn)."]
    FSAFE_0,
    #[doc = "Safe mode. PWM outputs disabled by this fault are not enabled until FSTS[FFLAGx] is clear and FSTS[FFPINx] is clear at the start of a half cycle or full cycle depending on the state of FSTS[FFULL]."]
    FSAFE_1,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl FSAFER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            FSAFER::FSAFE_0 => 0,
            FSAFER::FSAFE_1 => 1,
            FSAFER::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> FSAFER {
        match value {
            0 => FSAFER::FSAFE_0,
            1 => FSAFER::FSAFE_1,
            i => FSAFER::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `FSAFE_0`"]
    #[inline]
    pub fn is_fsafe_0(&self) -> bool {
        *self == FSAFER::FSAFE_0
    }
    #[doc = "Checks if the value of the field is `FSAFE_1`"]
    #[inline]
    pub fn is_fsafe_1(&self) -> bool {
        *self == FSAFER::FSAFE_1
    }
}
#[doc = "Possible values of the field `FAUTO`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FAUTOR {
    #[doc = "Manual fault clearing. PWM outputs disabled by this fault are not enabled until FSTS[FFLAGx] is clear at the start of a half cycle or full cycle depending the state of FSTS[FFULL]. This is further controlled by FCTRL[FSAFE]."]
    FAUTO_0,
    #[doc = "Automatic fault clearing. PWM outputs disabled by this fault are enabled when FSTS[FFPINx] is clear at the start of a half cycle or full cycle depending on the state of FSTS[FFULL] without regard to the state of FSTS[FFLAGx]."]
    FAUTO_1,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl FAUTOR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            FAUTOR::FAUTO_0 => 0,
            FAUTOR::FAUTO_1 => 1,
            FAUTOR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> FAUTOR {
        match value {
            0 => FAUTOR::FAUTO_0,
            1 => FAUTOR::FAUTO_1,
            i => FAUTOR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `FAUTO_0`"]
    #[inline]
    pub fn is_fauto_0(&self) -> bool {
        *self == FAUTOR::FAUTO_0
    }
    #[doc = "Checks if the value of the field is `FAUTO_1`"]
    #[inline]
    pub fn is_fauto_1(&self) -> bool {
        *self == FAUTOR::FAUTO_1
    }
}
#[doc = "Possible values of the field `FLVL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FLVLR {
    #[doc = "A logic 0 on the fault input indicates a fault condition."]
    FLVL_0,
    #[doc = "A logic 1 on the fault input indicates a fault condition."]
    FLVL_1,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl FLVLR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            FLVLR::FLVL_0 => 0,
            FLVLR::FLVL_1 => 1,
            FLVLR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> FLVLR {
        match value {
            0 => FLVLR::FLVL_0,
            1 => FLVLR::FLVL_1,
            i => FLVLR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `FLVL_0`"]
    #[inline]
    pub fn is_flvl_0(&self) -> bool {
        *self == FLVLR::FLVL_0
    }
    #[doc = "Checks if the value of the field is `FLVL_1`"]
    #[inline]
    pub fn is_flvl_1(&self) -> bool {
        *self == FLVLR::FLVL_1
    }
}
#[doc = "Values that can be written to the field `FIE`"]
pub enum FIEW {
    #[doc = "FAULTx CPU interrupt requests disabled."]
    FIE_0,
    #[doc = "FAULTx CPU interrupt requests enabled."]
    FIE_1,
}
impl FIEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            FIEW::FIE_0 => 0,
            FIEW::FIE_1 => 1,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FIEW<'a> {
    w: &'a mut W,
}
impl<'a> _FIEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FIEW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "FAULTx CPU interrupt requests disabled."]
    #[inline]
    pub fn fie_0(self) -> &'a mut W {
        self.variant(FIEW::FIE_0)
    }
    #[doc = "FAULTx CPU interrupt requests enabled."]
    #[inline]
    pub fn fie_1(self) -> &'a mut W {
        self.variant(FIEW::FIE_1)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u16) << OFFSET);
        self.w.bits |= ((value & MASK) as u16) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `FSAFE`"]
pub enum FSAFEW {
    #[doc = "Normal mode. PWM outputs disabled by this fault are not enabled until FSTS[FFLAGx] is clear at the start of a half cycle or full cycle depending on the state of FSTS[FFULL] without regard to the state of FSTS[FFPINx]. The PWM outputs disabled by this fault input will not be re-enabled until the actual FAULTx input signal de-asserts since the fault input will combinationally disable the PWM outputs (as programmed in DISMAPn)."]
    FSAFE_0,
    #[doc = "Safe mode. PWM outputs disabled by this fault are not enabled until FSTS[FFLAGx] is clear and FSTS[FFPINx] is clear at the start of a half cycle or full cycle depending on the state of FSTS[FFULL]."]
    FSAFE_1,
}
impl FSAFEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            FSAFEW::FSAFE_0 => 0,
            FSAFEW::FSAFE_1 => 1,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FSAFEW<'a> {
    w: &'a mut W,
}
impl<'a> _FSAFEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FSAFEW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Normal mode. PWM outputs disabled by this fault are not enabled until FSTS[FFLAGx] is clear at the start of a half cycle or full cycle depending on the state of FSTS[FFULL] without regard to the state of FSTS[FFPINx]. The PWM outputs disabled by this fault input will not be re-enabled until the actual FAULTx input signal de-asserts since the fault input will combinationally disable the PWM outputs (as programmed in DISMAPn)."]
    #[inline]
    pub fn fsafe_0(self) -> &'a mut W {
        self.variant(FSAFEW::FSAFE_0)
    }
    #[doc = "Safe mode. PWM outputs disabled by this fault are not enabled until FSTS[FFLAGx] is clear and FSTS[FFPINx] is clear at the start of a half cycle or full cycle depending on the state of FSTS[FFULL]."]
    #[inline]
    pub fn fsafe_1(self) -> &'a mut W {
        self.variant(FSAFEW::FSAFE_1)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 4;
        self.w.bits &= !((MASK as u16) << OFFSET);
        self.w.bits |= ((value & MASK) as u16) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `FAUTO`"]
pub enum FAUTOW {
    #[doc = "Manual fault clearing. PWM outputs disabled by this fault are not enabled until FSTS[FFLAGx] is clear at the start of a half cycle or full cycle depending the state of FSTS[FFULL]. This is further controlled by FCTRL[FSAFE]."]
    FAUTO_0,
    #[doc = "Automatic fault clearing. PWM outputs disabled by this fault are enabled when FSTS[FFPINx] is clear at the start of a half cycle or full cycle depending on the state of FSTS[FFULL] without regard to the state of FSTS[FFLAGx]."]
    FAUTO_1,
}
impl FAUTOW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            FAUTOW::FAUTO_0 => 0,
            FAUTOW::FAUTO_1 => 1,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FAUTOW<'a> {
    w: &'a mut W,
}
impl<'a> _FAUTOW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FAUTOW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Manual fault clearing. PWM outputs disabled by this fault are not enabled until FSTS[FFLAGx] is clear at the start of a half cycle or full cycle depending the state of FSTS[FFULL]. This is further controlled by FCTRL[FSAFE]."]
    #[inline]
    pub fn fauto_0(self) -> &'a mut W {
        self.variant(FAUTOW::FAUTO_0)
    }
    #[doc = "Automatic fault clearing. PWM outputs disabled by this fault are enabled when FSTS[FFPINx] is clear at the start of a half cycle or full cycle depending on the state of FSTS[FFULL] without regard to the state of FSTS[FFLAGx]."]
    #[inline]
    pub fn fauto_1(self) -> &'a mut W {
        self.variant(FAUTOW::FAUTO_1)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u16) << OFFSET);
        self.w.bits |= ((value & MASK) as u16) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `FLVL`"]
pub enum FLVLW {
    #[doc = "A logic 0 on the fault input indicates a fault condition."]
    FLVL_0,
    #[doc = "A logic 1 on the fault input indicates a fault condition."]
    FLVL_1,
}
impl FLVLW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            FLVLW::FLVL_0 => 0,
            FLVLW::FLVL_1 => 1,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FLVLW<'a> {
    w: &'a mut W,
}
impl<'a> _FLVLW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FLVLW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "A logic 0 on the fault input indicates a fault condition."]
    #[inline]
    pub fn flvl_0(self) -> &'a mut W {
        self.variant(FLVLW::FLVL_0)
    }
    #[doc = "A logic 1 on the fault input indicates a fault condition."]
    #[inline]
    pub fn flvl_1(self) -> &'a mut W {
        self.variant(FLVLW::FLVL_1)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 12;
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
    #[doc = "Bits 0:3 - Fault Interrupt Enables"]
    #[inline]
    pub fn fie(&self) -> FIER {
        FIER::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u16) as u8
        })
    }
    #[doc = "Bits 4:7 - Fault Safety Mode"]
    #[inline]
    pub fn fsafe(&self) -> FSAFER {
        FSAFER::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u16) as u8
        })
    }
    #[doc = "Bits 8:11 - Automatic Fault Clearing"]
    #[inline]
    pub fn fauto(&self) -> FAUTOR {
        FAUTOR::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u16) as u8
        })
    }
    #[doc = "Bits 12:15 - Fault Level"]
    #[inline]
    pub fn flvl(&self) -> FLVLR {
        FLVLR::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u16) as u8
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
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:3 - Fault Interrupt Enables"]
    #[inline]
    pub fn fie(&mut self) -> _FIEW {
        _FIEW { w: self }
    }
    #[doc = "Bits 4:7 - Fault Safety Mode"]
    #[inline]
    pub fn fsafe(&mut self) -> _FSAFEW {
        _FSAFEW { w: self }
    }
    #[doc = "Bits 8:11 - Automatic Fault Clearing"]
    #[inline]
    pub fn fauto(&mut self) -> _FAUTOW {
        _FAUTOW { w: self }
    }
    #[doc = "Bits 12:15 - Fault Level"]
    #[inline]
    pub fn flvl(&mut self) -> _FLVLW {
        _FLVLW { w: self }
    }
}
