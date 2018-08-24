#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::FLEXCAN2_RX_SELECT_INPUT {
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
#[doc = "Possible values of the field `DAISY`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DAISYR {
    #[doc = "Selecting Pad: GPIO_EMC_10 for Mode: ALT3"]
    GPIO_EMC_10_ALT3,
    #[doc = "Selecting Pad: GPIO_AD_B0_03 for Mode: ALT0"]
    GPIO_AD_B0_03_ALT0,
    #[doc = "Selecting Pad: GPIO_AD_B0_15 for Mode: ALT6"]
    GPIO_AD_B0_15_ALT6,
    #[doc = "Selecting Pad: GPIO_B1_09 for Mode: ALT6"]
    GPIO_B1_09_ALT6,
}
impl DAISYR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            DAISYR::GPIO_EMC_10_ALT3 => 0,
            DAISYR::GPIO_AD_B0_03_ALT0 => 1,
            DAISYR::GPIO_AD_B0_15_ALT6 => 2,
            DAISYR::GPIO_B1_09_ALT6 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> DAISYR {
        match value {
            0 => DAISYR::GPIO_EMC_10_ALT3,
            1 => DAISYR::GPIO_AD_B0_03_ALT0,
            2 => DAISYR::GPIO_AD_B0_15_ALT6,
            3 => DAISYR::GPIO_B1_09_ALT6,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `GPIO_EMC_10_ALT3`"]
    #[inline]
    pub fn is_gpio_emc_10_alt3(&self) -> bool {
        *self == DAISYR::GPIO_EMC_10_ALT3
    }
    #[doc = "Checks if the value of the field is `GPIO_AD_B0_03_ALT0`"]
    #[inline]
    pub fn is_gpio_ad_b0_03_alt0(&self) -> bool {
        *self == DAISYR::GPIO_AD_B0_03_ALT0
    }
    #[doc = "Checks if the value of the field is `GPIO_AD_B0_15_ALT6`"]
    #[inline]
    pub fn is_gpio_ad_b0_15_alt6(&self) -> bool {
        *self == DAISYR::GPIO_AD_B0_15_ALT6
    }
    #[doc = "Checks if the value of the field is `GPIO_B1_09_ALT6`"]
    #[inline]
    pub fn is_gpio_b1_09_alt6(&self) -> bool {
        *self == DAISYR::GPIO_B1_09_ALT6
    }
}
#[doc = "Values that can be written to the field `DAISY`"]
pub enum DAISYW {
    #[doc = "Selecting Pad: GPIO_EMC_10 for Mode: ALT3"]
    GPIO_EMC_10_ALT3,
    #[doc = "Selecting Pad: GPIO_AD_B0_03 for Mode: ALT0"]
    GPIO_AD_B0_03_ALT0,
    #[doc = "Selecting Pad: GPIO_AD_B0_15 for Mode: ALT6"]
    GPIO_AD_B0_15_ALT6,
    #[doc = "Selecting Pad: GPIO_B1_09 for Mode: ALT6"]
    GPIO_B1_09_ALT6,
}
impl DAISYW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            DAISYW::GPIO_EMC_10_ALT3 => 0,
            DAISYW::GPIO_AD_B0_03_ALT0 => 1,
            DAISYW::GPIO_AD_B0_15_ALT6 => 2,
            DAISYW::GPIO_B1_09_ALT6 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DAISYW<'a> {
    w: &'a mut W,
}
impl<'a> _DAISYW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DAISYW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Selecting Pad: GPIO_EMC_10 for Mode: ALT3"]
    #[inline]
    pub fn gpio_emc_10_alt3(self) -> &'a mut W {
        self.variant(DAISYW::GPIO_EMC_10_ALT3)
    }
    #[doc = "Selecting Pad: GPIO_AD_B0_03 for Mode: ALT0"]
    #[inline]
    pub fn gpio_ad_b0_03_alt0(self) -> &'a mut W {
        self.variant(DAISYW::GPIO_AD_B0_03_ALT0)
    }
    #[doc = "Selecting Pad: GPIO_AD_B0_15 for Mode: ALT6"]
    #[inline]
    pub fn gpio_ad_b0_15_alt6(self) -> &'a mut W {
        self.variant(DAISYW::GPIO_AD_B0_15_ALT6)
    }
    #[doc = "Selecting Pad: GPIO_B1_09 for Mode: ALT6"]
    #[inline]
    pub fn gpio_b1_09_alt6(self) -> &'a mut W {
        self.variant(DAISYW::GPIO_B1_09_ALT6)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 0;
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
    #[doc = "Bits 0:1 - Selecting Pads Involved in Daisy Chain."]
    #[inline]
    pub fn daisy(&self) -> DAISYR {
        DAISYR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 0;
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
    #[doc = "Bits 0:1 - Selecting Pads Involved in Daisy Chain."]
    #[inline]
    pub fn daisy(&mut self) -> _DAISYW {
        _DAISYW { w: self }
    }
}
