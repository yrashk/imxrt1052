#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::FLEXPWM1_PWMB3_SELECT_INPUT {
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
    #[doc = "Selecting Pad: GPIO_SD_B1_01 for Mode: ALT2"]
    GPIO_SD_B1_01_ALT2,
    #[doc = "Selecting Pad: GPIO_EMC_13 for Mode: ALT4"]
    GPIO_EMC_13_ALT4,
    #[doc = "Selecting Pad: GPIO_EMC_39 for Mode: ALT1"]
    GPIO_EMC_39_ALT1,
    #[doc = "Selecting Pad: GPIO_AD_B0_11 for Mode: ALT1"]
    GPIO_AD_B0_11_ALT1,
    #[doc = "Selecting Pad: GPIO_B1_01 for Mode: ALT6"]
    GPIO_B1_01_ALT6,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl DAISYR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            DAISYR::GPIO_SD_B1_01_ALT2 => 0,
            DAISYR::GPIO_EMC_13_ALT4 => 1,
            DAISYR::GPIO_EMC_39_ALT1 => 2,
            DAISYR::GPIO_AD_B0_11_ALT1 => 3,
            DAISYR::GPIO_B1_01_ALT6 => 4,
            DAISYR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> DAISYR {
        match value {
            0 => DAISYR::GPIO_SD_B1_01_ALT2,
            1 => DAISYR::GPIO_EMC_13_ALT4,
            2 => DAISYR::GPIO_EMC_39_ALT1,
            3 => DAISYR::GPIO_AD_B0_11_ALT1,
            4 => DAISYR::GPIO_B1_01_ALT6,
            i => DAISYR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `GPIO_SD_B1_01_ALT2`"]
    #[inline]
    pub fn is_gpio_sd_b1_01_alt2(&self) -> bool {
        *self == DAISYR::GPIO_SD_B1_01_ALT2
    }
    #[doc = "Checks if the value of the field is `GPIO_EMC_13_ALT4`"]
    #[inline]
    pub fn is_gpio_emc_13_alt4(&self) -> bool {
        *self == DAISYR::GPIO_EMC_13_ALT4
    }
    #[doc = "Checks if the value of the field is `GPIO_EMC_39_ALT1`"]
    #[inline]
    pub fn is_gpio_emc_39_alt1(&self) -> bool {
        *self == DAISYR::GPIO_EMC_39_ALT1
    }
    #[doc = "Checks if the value of the field is `GPIO_AD_B0_11_ALT1`"]
    #[inline]
    pub fn is_gpio_ad_b0_11_alt1(&self) -> bool {
        *self == DAISYR::GPIO_AD_B0_11_ALT1
    }
    #[doc = "Checks if the value of the field is `GPIO_B1_01_ALT6`"]
    #[inline]
    pub fn is_gpio_b1_01_alt6(&self) -> bool {
        *self == DAISYR::GPIO_B1_01_ALT6
    }
}
#[doc = "Values that can be written to the field `DAISY`"]
pub enum DAISYW {
    #[doc = "Selecting Pad: GPIO_SD_B1_01 for Mode: ALT2"]
    GPIO_SD_B1_01_ALT2,
    #[doc = "Selecting Pad: GPIO_EMC_13 for Mode: ALT4"]
    GPIO_EMC_13_ALT4,
    #[doc = "Selecting Pad: GPIO_EMC_39 for Mode: ALT1"]
    GPIO_EMC_39_ALT1,
    #[doc = "Selecting Pad: GPIO_AD_B0_11 for Mode: ALT1"]
    GPIO_AD_B0_11_ALT1,
    #[doc = "Selecting Pad: GPIO_B1_01 for Mode: ALT6"]
    GPIO_B1_01_ALT6,
}
impl DAISYW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            DAISYW::GPIO_SD_B1_01_ALT2 => 0,
            DAISYW::GPIO_EMC_13_ALT4 => 1,
            DAISYW::GPIO_EMC_39_ALT1 => 2,
            DAISYW::GPIO_AD_B0_11_ALT1 => 3,
            DAISYW::GPIO_B1_01_ALT6 => 4,
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
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Selecting Pad: GPIO_SD_B1_01 for Mode: ALT2"]
    #[inline]
    pub fn gpio_sd_b1_01_alt2(self) -> &'a mut W {
        self.variant(DAISYW::GPIO_SD_B1_01_ALT2)
    }
    #[doc = "Selecting Pad: GPIO_EMC_13 for Mode: ALT4"]
    #[inline]
    pub fn gpio_emc_13_alt4(self) -> &'a mut W {
        self.variant(DAISYW::GPIO_EMC_13_ALT4)
    }
    #[doc = "Selecting Pad: GPIO_EMC_39 for Mode: ALT1"]
    #[inline]
    pub fn gpio_emc_39_alt1(self) -> &'a mut W {
        self.variant(DAISYW::GPIO_EMC_39_ALT1)
    }
    #[doc = "Selecting Pad: GPIO_AD_B0_11 for Mode: ALT1"]
    #[inline]
    pub fn gpio_ad_b0_11_alt1(self) -> &'a mut W {
        self.variant(DAISYW::GPIO_AD_B0_11_ALT1)
    }
    #[doc = "Selecting Pad: GPIO_B1_01 for Mode: ALT6"]
    #[inline]
    pub fn gpio_b1_01_alt6(self) -> &'a mut W {
        self.variant(DAISYW::GPIO_B1_01_ALT6)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
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
    #[doc = "Bits 0:2 - Selecting Pads Involved in Daisy Chain."]
    #[inline]
    pub fn daisy(&self) -> DAISYR {
        DAISYR::_from({
            const MASK: u8 = 7;
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
    #[doc = "Bits 0:2 - Selecting Pads Involved in Daisy Chain."]
    #[inline]
    pub fn daisy(&mut self) -> _DAISYW {
        _DAISYW { w: self }
    }
}
