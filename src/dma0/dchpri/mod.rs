#[doc = r" Value read from the register"]
pub struct R {
    bits: u8,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u8,
}
impl super::DCHPRI {
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
pub struct CHPRIR {
    bits: u8,
}
impl CHPRIR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct GRPPRIR {
    bits: u8,
}
impl GRPPRIR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Possible values of the field `DPA`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DPAR {
    #[doc = "Channel n can suspend a lower priority channel."]
    DPA_0,
    #[doc = "Channel n cannot suspend any channel, regardless of channel priority."]
    DPA_1,
}
impl DPAR {
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
            DPAR::DPA_0 => false,
            DPAR::DPA_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DPAR {
        match value {
            false => DPAR::DPA_0,
            true => DPAR::DPA_1,
        }
    }
    #[doc = "Checks if the value of the field is `DPA_0`"]
    #[inline]
    pub fn is_dpa_0(&self) -> bool {
        *self == DPAR::DPA_0
    }
    #[doc = "Checks if the value of the field is `DPA_1`"]
    #[inline]
    pub fn is_dpa_1(&self) -> bool {
        *self == DPAR::DPA_1
    }
}
#[doc = "Possible values of the field `ECP`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ECPR {
    #[doc = "Channel n cannot be suspended by a higher priority channel's service request."]
    ECP_0,
    #[doc = "Channel n can be temporarily suspended by the service request of a higher priority channel."]
    ECP_1,
}
impl ECPR {
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
            ECPR::ECP_0 => false,
            ECPR::ECP_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ECPR {
        match value {
            false => ECPR::ECP_0,
            true => ECPR::ECP_1,
        }
    }
    #[doc = "Checks if the value of the field is `ECP_0`"]
    #[inline]
    pub fn is_ecp_0(&self) -> bool {
        *self == ECPR::ECP_0
    }
    #[doc = "Checks if the value of the field is `ECP_1`"]
    #[inline]
    pub fn is_ecp_1(&self) -> bool {
        *self == ECPR::ECP_1
    }
}
#[doc = r" Proxy"]
pub struct _CHPRIW<'a> {
    w: &'a mut W,
}
impl<'a> _CHPRIW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u8) << OFFSET);
        self.w.bits |= ((value & MASK) as u8) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `DPA`"]
pub enum DPAW {
    #[doc = "Channel n can suspend a lower priority channel."]
    DPA_0,
    #[doc = "Channel n cannot suspend any channel, regardless of channel priority."]
    DPA_1,
}
impl DPAW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DPAW::DPA_0 => false,
            DPAW::DPA_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DPAW<'a> {
    w: &'a mut W,
}
impl<'a> _DPAW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DPAW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Channel n can suspend a lower priority channel."]
    #[inline]
    pub fn dpa_0(self) -> &'a mut W {
        self.variant(DPAW::DPA_0)
    }
    #[doc = "Channel n cannot suspend any channel, regardless of channel priority."]
    #[inline]
    pub fn dpa_1(self) -> &'a mut W {
        self.variant(DPAW::DPA_1)
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
        const OFFSET: u8 = 6;
        self.w.bits &= !((MASK as u8) << OFFSET);
        self.w.bits |= ((value & MASK) as u8) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `ECP`"]
pub enum ECPW {
    #[doc = "Channel n cannot be suspended by a higher priority channel's service request."]
    ECP_0,
    #[doc = "Channel n can be temporarily suspended by the service request of a higher priority channel."]
    ECP_1,
}
impl ECPW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ECPW::ECP_0 => false,
            ECPW::ECP_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ECPW<'a> {
    w: &'a mut W,
}
impl<'a> _ECPW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ECPW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Channel n cannot be suspended by a higher priority channel's service request."]
    #[inline]
    pub fn ecp_0(self) -> &'a mut W {
        self.variant(ECPW::ECP_0)
    }
    #[doc = "Channel n can be temporarily suspended by the service request of a higher priority channel."]
    #[inline]
    pub fn ecp_1(self) -> &'a mut W {
        self.variant(ECPW::ECP_1)
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
        self.w.bits &= !((MASK as u8) << OFFSET);
        self.w.bits |= ((value & MASK) as u8) << OFFSET;
        self.w
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
    #[doc = "Bits 0:3 - Channel n Arbitration Priority"]
    #[inline]
    pub fn chpri(&self) -> CHPRIR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u8) as u8
        };
        CHPRIR { bits }
    }
    #[doc = "Bits 4:5 - Channel n Current Group Priority"]
    #[inline]
    pub fn grppri(&self) -> GRPPRIR {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u8) as u8
        };
        GRPPRIR { bits }
    }
    #[doc = "Bit 6 - Disable Preempt Ability. This field resets to 0."]
    #[inline]
    pub fn dpa(&self) -> DPAR {
        DPAR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        })
    }
    #[doc = "Bit 7 - Enable Channel Preemption. This field resets to 0."]
    #[inline]
    pub fn ecp(&self) -> ECPR {
        ECPR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        })
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 3 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:3 - Channel n Arbitration Priority"]
    #[inline]
    pub fn chpri(&mut self) -> _CHPRIW {
        _CHPRIW { w: self }
    }
    #[doc = "Bit 6 - Disable Preempt Ability. This field resets to 0."]
    #[inline]
    pub fn dpa(&mut self) -> _DPAW {
        _DPAW { w: self }
    }
    #[doc = "Bit 7 - Enable Channel Preemption. This field resets to 0."]
    #[inline]
    pub fn ecp(&mut self) -> _ECPW {
        _ECPW { w: self }
    }
}
