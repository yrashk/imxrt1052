#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CTRL {
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
pub struct ENOTG_ID_CHG_IRQR {
    bits: bool,
}
impl ENOTG_ID_CHG_IRQR {
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
pub struct ENHOSTDISCONDETECTR {
    bits: bool,
}
impl ENHOSTDISCONDETECTR {
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
pub struct ENIRQHOSTDISCONR {
    bits: bool,
}
impl ENIRQHOSTDISCONR {
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
pub struct HOSTDISCONDETECT_IRQR {
    bits: bool,
}
impl HOSTDISCONDETECT_IRQR {
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
pub struct ENDEVPLUGINDETECTR {
    bits: bool,
}
impl ENDEVPLUGINDETECTR {
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
pub struct DEVPLUGIN_POLARITYR {
    bits: bool,
}
impl DEVPLUGIN_POLARITYR {
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
pub struct OTG_ID_CHG_IRQR {
    bits: bool,
}
impl OTG_ID_CHG_IRQR {
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
pub struct ENOTGIDDETECTR {
    bits: bool,
}
impl ENOTGIDDETECTR {
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
pub struct RESUMEIRQSTICKYR {
    bits: bool,
}
impl RESUMEIRQSTICKYR {
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
pub struct ENIRQRESUMEDETECTR {
    bits: bool,
}
impl ENIRQRESUMEDETECTR {
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
pub struct RESUME_IRQR {
    bits: bool,
}
impl RESUME_IRQR {
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
pub struct ENIRQDEVPLUGINR {
    bits: bool,
}
impl ENIRQDEVPLUGINR {
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
pub struct DEVPLUGIN_IRQR {
    bits: bool,
}
impl DEVPLUGIN_IRQR {
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
pub struct DATA_ON_LRADCR {
    bits: bool,
}
impl DATA_ON_LRADCR {
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
pub struct ENUTMILEVEL2R {
    bits: bool,
}
impl ENUTMILEVEL2R {
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
pub struct ENUTMILEVEL3R {
    bits: bool,
}
impl ENUTMILEVEL3R {
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
pub struct ENIRQWAKEUPR {
    bits: bool,
}
impl ENIRQWAKEUPR {
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
pub struct WAKEUP_IRQR {
    bits: bool,
}
impl WAKEUP_IRQR {
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
pub struct ENAUTO_PWRON_PLLR {
    bits: bool,
}
impl ENAUTO_PWRON_PLLR {
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
pub struct ENAUTOCLR_CLKGATER {
    bits: bool,
}
impl ENAUTOCLR_CLKGATER {
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
pub struct ENAUTOCLR_PHY_PWDR {
    bits: bool,
}
impl ENAUTOCLR_PHY_PWDR {
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
pub struct ENDPDMCHG_WKUPR {
    bits: bool,
}
impl ENDPDMCHG_WKUPR {
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
pub struct ENIDCHG_WKUPR {
    bits: bool,
}
impl ENIDCHG_WKUPR {
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
pub struct ENVBUSCHG_WKUPR {
    bits: bool,
}
impl ENVBUSCHG_WKUPR {
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
pub struct FSDLL_RST_ENR {
    bits: bool,
}
impl FSDLL_RST_ENR {
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
pub struct RSVD1R {
    bits: u8,
}
impl RSVD1R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct OTG_ID_VALUER {
    bits: bool,
}
impl OTG_ID_VALUER {
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
pub struct HOST_FORCE_LS_SE0R {
    bits: bool,
}
impl HOST_FORCE_LS_SE0R {
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
pub struct UTMI_SUSPENDMR {
    bits: bool,
}
impl UTMI_SUSPENDMR {
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
pub struct CLKGATER {
    bits: bool,
}
impl CLKGATER {
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
pub struct SFTRSTR {
    bits: bool,
}
impl SFTRSTR {
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
#[doc = r" Proxy"]
pub struct _ENOTG_ID_CHG_IRQW<'a> {
    w: &'a mut W,
}
impl<'a> _ENOTG_ID_CHG_IRQW<'a> {
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
#[doc = r" Proxy"]
pub struct _ENHOSTDISCONDETECTW<'a> {
    w: &'a mut W,
}
impl<'a> _ENHOSTDISCONDETECTW<'a> {
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
#[doc = r" Proxy"]
pub struct _ENIRQHOSTDISCONW<'a> {
    w: &'a mut W,
}
impl<'a> _ENIRQHOSTDISCONW<'a> {
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
#[doc = r" Proxy"]
pub struct _HOSTDISCONDETECT_IRQW<'a> {
    w: &'a mut W,
}
impl<'a> _HOSTDISCONDETECT_IRQW<'a> {
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
pub struct _ENDEVPLUGINDETECTW<'a> {
    w: &'a mut W,
}
impl<'a> _ENDEVPLUGINDETECTW<'a> {
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
pub struct _DEVPLUGIN_POLARITYW<'a> {
    w: &'a mut W,
}
impl<'a> _DEVPLUGIN_POLARITYW<'a> {
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
#[doc = r" Proxy"]
pub struct _OTG_ID_CHG_IRQW<'a> {
    w: &'a mut W,
}
impl<'a> _OTG_ID_CHG_IRQW<'a> {
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
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _ENOTGIDDETECTW<'a> {
    w: &'a mut W,
}
impl<'a> _ENOTGIDDETECTW<'a> {
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
pub struct _RESUMEIRQSTICKYW<'a> {
    w: &'a mut W,
}
impl<'a> _RESUMEIRQSTICKYW<'a> {
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
#[doc = r" Proxy"]
pub struct _ENIRQRESUMEDETECTW<'a> {
    w: &'a mut W,
}
impl<'a> _ENIRQRESUMEDETECTW<'a> {
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
#[doc = r" Proxy"]
pub struct _RESUME_IRQW<'a> {
    w: &'a mut W,
}
impl<'a> _RESUME_IRQW<'a> {
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
#[doc = r" Proxy"]
pub struct _ENIRQDEVPLUGINW<'a> {
    w: &'a mut W,
}
impl<'a> _ENIRQDEVPLUGINW<'a> {
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
#[doc = r" Proxy"]
pub struct _DEVPLUGIN_IRQW<'a> {
    w: &'a mut W,
}
impl<'a> _DEVPLUGIN_IRQW<'a> {
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
#[doc = r" Proxy"]
pub struct _DATA_ON_LRADCW<'a> {
    w: &'a mut W,
}
impl<'a> _DATA_ON_LRADCW<'a> {
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
#[doc = r" Proxy"]
pub struct _ENUTMILEVEL2W<'a> {
    w: &'a mut W,
}
impl<'a> _ENUTMILEVEL2W<'a> {
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
#[doc = r" Proxy"]
pub struct _ENUTMILEVEL3W<'a> {
    w: &'a mut W,
}
impl<'a> _ENUTMILEVEL3W<'a> {
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
pub struct _ENIRQWAKEUPW<'a> {
    w: &'a mut W,
}
impl<'a> _ENIRQWAKEUPW<'a> {
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
#[doc = r" Proxy"]
pub struct _WAKEUP_IRQW<'a> {
    w: &'a mut W,
}
impl<'a> _WAKEUP_IRQW<'a> {
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
#[doc = r" Proxy"]
pub struct _ENAUTO_PWRON_PLLW<'a> {
    w: &'a mut W,
}
impl<'a> _ENAUTO_PWRON_PLLW<'a> {
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
        const OFFSET: u8 = 18;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _ENAUTOCLR_CLKGATEW<'a> {
    w: &'a mut W,
}
impl<'a> _ENAUTOCLR_CLKGATEW<'a> {
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
#[doc = r" Proxy"]
pub struct _ENAUTOCLR_PHY_PWDW<'a> {
    w: &'a mut W,
}
impl<'a> _ENAUTOCLR_PHY_PWDW<'a> {
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
#[doc = r" Proxy"]
pub struct _ENDPDMCHG_WKUPW<'a> {
    w: &'a mut W,
}
impl<'a> _ENDPDMCHG_WKUPW<'a> {
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
#[doc = r" Proxy"]
pub struct _ENIDCHG_WKUPW<'a> {
    w: &'a mut W,
}
impl<'a> _ENIDCHG_WKUPW<'a> {
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
        const OFFSET: u8 = 22;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _ENVBUSCHG_WKUPW<'a> {
    w: &'a mut W,
}
impl<'a> _ENVBUSCHG_WKUPW<'a> {
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
#[doc = r" Proxy"]
pub struct _FSDLL_RST_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _FSDLL_RST_ENW<'a> {
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
        const OFFSET: u8 = 24;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _HOST_FORCE_LS_SE0W<'a> {
    w: &'a mut W,
}
impl<'a> _HOST_FORCE_LS_SE0W<'a> {
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
        const OFFSET: u8 = 28;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _CLKGATEW<'a> {
    w: &'a mut W,
}
impl<'a> _CLKGATEW<'a> {
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
#[doc = r" Proxy"]
pub struct _SFTRSTW<'a> {
    w: &'a mut W,
}
impl<'a> _SFTRSTW<'a> {
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
    #[doc = "Bit 0 - Enable OTG_ID_CHG_IRQ."]
    #[inline]
    pub fn enotg_id_chg_irq(&self) -> ENOTG_ID_CHG_IRQR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        ENOTG_ID_CHG_IRQR { bits }
    }
    #[doc = "Bit 1 - For host mode, enables high-speed disconnect detector"]
    #[inline]
    pub fn enhostdiscondetect(&self) -> ENHOSTDISCONDETECTR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        ENHOSTDISCONDETECTR { bits }
    }
    #[doc = "Bit 2 - Enables interrupt for detection of disconnection to Device when in high-speed host mode"]
    #[inline]
    pub fn enirqhostdiscon(&self) -> ENIRQHOSTDISCONR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        ENIRQHOSTDISCONR { bits }
    }
    #[doc = "Bit 3 - Indicates that the device has disconnected in high-speed mode"]
    #[inline]
    pub fn hostdiscondetect_irq(&self) -> HOSTDISCONDETECT_IRQR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        HOSTDISCONDETECT_IRQR { bits }
    }
    #[doc = "Bit 4 - For device mode, enables 200-KOhm pullups for detecting connectivity to the host."]
    #[inline]
    pub fn endevplugindetect(&self) -> ENDEVPLUGINDETECTR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        ENDEVPLUGINDETECTR { bits }
    }
    #[doc = "Bit 5 - For device mode, if this bit is cleared to 0, then it trips the interrupt if the device is plugged in"]
    #[inline]
    pub fn devplugin_polarity(&self) -> DEVPLUGIN_POLARITYR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        DEVPLUGIN_POLARITYR { bits }
    }
    #[doc = "Bit 6 - OTG ID change interrupt. Indicates the value of ID pin changed."]
    #[inline]
    pub fn otg_id_chg_irq(&self) -> OTG_ID_CHG_IRQR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        OTG_ID_CHG_IRQR { bits }
    }
    #[doc = "Bit 7 - Enables circuit to detect resistance of MiniAB ID pin."]
    #[inline]
    pub fn enotgiddetect(&self) -> ENOTGIDDETECTR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        ENOTGIDDETECTR { bits }
    }
    #[doc = "Bit 8 - Set to 1 will make RESUME_IRQ bit a sticky bit until software clear it"]
    #[inline]
    pub fn resumeirqsticky(&self) -> RESUMEIRQSTICKYR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        RESUMEIRQSTICKYR { bits }
    }
    #[doc = "Bit 9 - Enables interrupt for detection of a non-J state on the USB line"]
    #[inline]
    pub fn enirqresumedetect(&self) -> ENIRQRESUMEDETECTR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        ENIRQRESUMEDETECTR { bits }
    }
    #[doc = "Bit 10 - Indicates that the host is sending a wake-up after suspend"]
    #[inline]
    pub fn resume_irq(&self) -> RESUME_IRQR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        RESUME_IRQR { bits }
    }
    #[doc = "Bit 11 - Enables interrupt for the detection of connectivity to the USB line."]
    #[inline]
    pub fn enirqdevplugin(&self) -> ENIRQDEVPLUGINR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        ENIRQDEVPLUGINR { bits }
    }
    #[doc = "Bit 12 - Indicates that the device is connected"]
    #[inline]
    pub fn devplugin_irq(&self) -> DEVPLUGIN_IRQR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        DEVPLUGIN_IRQR { bits }
    }
    #[doc = "Bit 13 - Enables the LRADC to monitor USB_DP and USB_DM. This is for use in non-USB modes only."]
    #[inline]
    pub fn data_on_lradc(&self) -> DATA_ON_LRADCR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        DATA_ON_LRADCR { bits }
    }
    #[doc = "Bit 14 - Enables UTMI+ Level2. This should be enabled if needs to support LS device"]
    #[inline]
    pub fn enutmilevel2(&self) -> ENUTMILEVEL2R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        ENUTMILEVEL2R { bits }
    }
    #[doc = "Bit 15 - Enables UTMI+ Level3"]
    #[inline]
    pub fn enutmilevel3(&self) -> ENUTMILEVEL3R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 15;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        ENUTMILEVEL3R { bits }
    }
    #[doc = "Bit 16 - Enables interrupt for the wakeup events."]
    #[inline]
    pub fn enirqwakeup(&self) -> ENIRQWAKEUPR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        ENIRQWAKEUPR { bits }
    }
    #[doc = "Bit 17 - Indicates that there is a wakeup event"]
    #[inline]
    pub fn wakeup_irq(&self) -> WAKEUP_IRQR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 17;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        WAKEUP_IRQR { bits }
    }
    #[doc = "Bit 18 - Enables the feature to auto-enable the POWER bit of HW_CLKCTRL_PLLxCTRL0 if there is wakeup event if USB is suspended"]
    #[inline]
    pub fn enauto_pwron_pll(&self) -> ENAUTO_PWRON_PLLR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        ENAUTO_PWRON_PLLR { bits }
    }
    #[doc = "Bit 19 - Enables the feature to auto-clear the CLKGATE bit if there is wakeup event while USB is suspended"]
    #[inline]
    pub fn enautoclr_clkgate(&self) -> ENAUTOCLR_CLKGATER {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 19;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        ENAUTOCLR_CLKGATER { bits }
    }
    #[doc = "Bit 20 - Enables the feature to auto-clear the PWD register bits in USBPHYx_PWD if there is wakeup event while USB is suspended"]
    #[inline]
    pub fn enautoclr_phy_pwd(&self) -> ENAUTOCLR_PHY_PWDR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        ENAUTOCLR_PHY_PWDR { bits }
    }
    #[doc = "Bit 21 - Enables the feature to wakeup USB if DP/DM is toggled when USB is suspended"]
    #[inline]
    pub fn endpdmchg_wkup(&self) -> ENDPDMCHG_WKUPR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 21;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        ENDPDMCHG_WKUPR { bits }
    }
    #[doc = "Bit 22 - Enables the feature to wakeup USB if ID is toggled when USB is suspended."]
    #[inline]
    pub fn enidchg_wkup(&self) -> ENIDCHG_WKUPR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 22;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        ENIDCHG_WKUPR { bits }
    }
    #[doc = "Bit 23 - Enables the feature to wakeup USB if VBUS is toggled when USB is suspended."]
    #[inline]
    pub fn envbuschg_wkup(&self) -> ENVBUSCHG_WKUPR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 23;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        ENVBUSCHG_WKUPR { bits }
    }
    #[doc = "Bit 24 - Enables the feature to reset the FSDLL lock detection logic at the end of each TX packet."]
    #[inline]
    pub fn fsdll_rst_en(&self) -> FSDLL_RST_ENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        FSDLL_RST_ENR { bits }
    }
    #[doc = "Bits 25:26 - Reserved."]
    #[inline]
    pub fn rsvd1(&self) -> RSVD1R {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 25;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        RSVD1R { bits }
    }
    #[doc = "Bit 27 - Almost same as OTGID_STATUS in USBPHYx_STATUS Register"]
    #[inline]
    pub fn otg_id_value(&self) -> OTG_ID_VALUER {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 27;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        OTG_ID_VALUER { bits }
    }
    #[doc = "Bit 28 - Forces the next FS packet that is transmitted to have a EOP with LS timing"]
    #[inline]
    pub fn host_force_ls_se0(&self) -> HOST_FORCE_LS_SE0R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 28;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        HOST_FORCE_LS_SE0R { bits }
    }
    #[doc = "Bit 29 - Used by the PHY to indicate a powered-down state"]
    #[inline]
    pub fn utmi_suspendm(&self) -> UTMI_SUSPENDMR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 29;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        UTMI_SUSPENDMR { bits }
    }
    #[doc = "Bit 30 - Gate UTMI Clocks"]
    #[inline]
    pub fn clkgate(&self) -> CLKGATER {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 30;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        CLKGATER { bits }
    }
    #[doc = "Bit 31 - Writing a 1 to this bit will soft-reset the USBPHYx_PWD, USBPHYx_TX, USBPHYx_RX, and USBPHYx_CTRL registers"]
    #[inline]
    pub fn sftrst(&self) -> SFTRSTR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 31;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        SFTRSTR { bits }
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 3223322624 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - Enable OTG_ID_CHG_IRQ."]
    #[inline]
    pub fn enotg_id_chg_irq(&mut self) -> _ENOTG_ID_CHG_IRQW {
        _ENOTG_ID_CHG_IRQW { w: self }
    }
    #[doc = "Bit 1 - For host mode, enables high-speed disconnect detector"]
    #[inline]
    pub fn enhostdiscondetect(&mut self) -> _ENHOSTDISCONDETECTW {
        _ENHOSTDISCONDETECTW { w: self }
    }
    #[doc = "Bit 2 - Enables interrupt for detection of disconnection to Device when in high-speed host mode"]
    #[inline]
    pub fn enirqhostdiscon(&mut self) -> _ENIRQHOSTDISCONW {
        _ENIRQHOSTDISCONW { w: self }
    }
    #[doc = "Bit 3 - Indicates that the device has disconnected in high-speed mode"]
    #[inline]
    pub fn hostdiscondetect_irq(&mut self) -> _HOSTDISCONDETECT_IRQW {
        _HOSTDISCONDETECT_IRQW { w: self }
    }
    #[doc = "Bit 4 - For device mode, enables 200-KOhm pullups for detecting connectivity to the host."]
    #[inline]
    pub fn endevplugindetect(&mut self) -> _ENDEVPLUGINDETECTW {
        _ENDEVPLUGINDETECTW { w: self }
    }
    #[doc = "Bit 5 - For device mode, if this bit is cleared to 0, then it trips the interrupt if the device is plugged in"]
    #[inline]
    pub fn devplugin_polarity(&mut self) -> _DEVPLUGIN_POLARITYW {
        _DEVPLUGIN_POLARITYW { w: self }
    }
    #[doc = "Bit 6 - OTG ID change interrupt. Indicates the value of ID pin changed."]
    #[inline]
    pub fn otg_id_chg_irq(&mut self) -> _OTG_ID_CHG_IRQW {
        _OTG_ID_CHG_IRQW { w: self }
    }
    #[doc = "Bit 7 - Enables circuit to detect resistance of MiniAB ID pin."]
    #[inline]
    pub fn enotgiddetect(&mut self) -> _ENOTGIDDETECTW {
        _ENOTGIDDETECTW { w: self }
    }
    #[doc = "Bit 8 - Set to 1 will make RESUME_IRQ bit a sticky bit until software clear it"]
    #[inline]
    pub fn resumeirqsticky(&mut self) -> _RESUMEIRQSTICKYW {
        _RESUMEIRQSTICKYW { w: self }
    }
    #[doc = "Bit 9 - Enables interrupt for detection of a non-J state on the USB line"]
    #[inline]
    pub fn enirqresumedetect(&mut self) -> _ENIRQRESUMEDETECTW {
        _ENIRQRESUMEDETECTW { w: self }
    }
    #[doc = "Bit 10 - Indicates that the host is sending a wake-up after suspend"]
    #[inline]
    pub fn resume_irq(&mut self) -> _RESUME_IRQW {
        _RESUME_IRQW { w: self }
    }
    #[doc = "Bit 11 - Enables interrupt for the detection of connectivity to the USB line."]
    #[inline]
    pub fn enirqdevplugin(&mut self) -> _ENIRQDEVPLUGINW {
        _ENIRQDEVPLUGINW { w: self }
    }
    #[doc = "Bit 12 - Indicates that the device is connected"]
    #[inline]
    pub fn devplugin_irq(&mut self) -> _DEVPLUGIN_IRQW {
        _DEVPLUGIN_IRQW { w: self }
    }
    #[doc = "Bit 13 - Enables the LRADC to monitor USB_DP and USB_DM. This is for use in non-USB modes only."]
    #[inline]
    pub fn data_on_lradc(&mut self) -> _DATA_ON_LRADCW {
        _DATA_ON_LRADCW { w: self }
    }
    #[doc = "Bit 14 - Enables UTMI+ Level2. This should be enabled if needs to support LS device"]
    #[inline]
    pub fn enutmilevel2(&mut self) -> _ENUTMILEVEL2W {
        _ENUTMILEVEL2W { w: self }
    }
    #[doc = "Bit 15 - Enables UTMI+ Level3"]
    #[inline]
    pub fn enutmilevel3(&mut self) -> _ENUTMILEVEL3W {
        _ENUTMILEVEL3W { w: self }
    }
    #[doc = "Bit 16 - Enables interrupt for the wakeup events."]
    #[inline]
    pub fn enirqwakeup(&mut self) -> _ENIRQWAKEUPW {
        _ENIRQWAKEUPW { w: self }
    }
    #[doc = "Bit 17 - Indicates that there is a wakeup event"]
    #[inline]
    pub fn wakeup_irq(&mut self) -> _WAKEUP_IRQW {
        _WAKEUP_IRQW { w: self }
    }
    #[doc = "Bit 18 - Enables the feature to auto-enable the POWER bit of HW_CLKCTRL_PLLxCTRL0 if there is wakeup event if USB is suspended"]
    #[inline]
    pub fn enauto_pwron_pll(&mut self) -> _ENAUTO_PWRON_PLLW {
        _ENAUTO_PWRON_PLLW { w: self }
    }
    #[doc = "Bit 19 - Enables the feature to auto-clear the CLKGATE bit if there is wakeup event while USB is suspended"]
    #[inline]
    pub fn enautoclr_clkgate(&mut self) -> _ENAUTOCLR_CLKGATEW {
        _ENAUTOCLR_CLKGATEW { w: self }
    }
    #[doc = "Bit 20 - Enables the feature to auto-clear the PWD register bits in USBPHYx_PWD if there is wakeup event while USB is suspended"]
    #[inline]
    pub fn enautoclr_phy_pwd(&mut self) -> _ENAUTOCLR_PHY_PWDW {
        _ENAUTOCLR_PHY_PWDW { w: self }
    }
    #[doc = "Bit 21 - Enables the feature to wakeup USB if DP/DM is toggled when USB is suspended"]
    #[inline]
    pub fn endpdmchg_wkup(&mut self) -> _ENDPDMCHG_WKUPW {
        _ENDPDMCHG_WKUPW { w: self }
    }
    #[doc = "Bit 22 - Enables the feature to wakeup USB if ID is toggled when USB is suspended."]
    #[inline]
    pub fn enidchg_wkup(&mut self) -> _ENIDCHG_WKUPW {
        _ENIDCHG_WKUPW { w: self }
    }
    #[doc = "Bit 23 - Enables the feature to wakeup USB if VBUS is toggled when USB is suspended."]
    #[inline]
    pub fn envbuschg_wkup(&mut self) -> _ENVBUSCHG_WKUPW {
        _ENVBUSCHG_WKUPW { w: self }
    }
    #[doc = "Bit 24 - Enables the feature to reset the FSDLL lock detection logic at the end of each TX packet."]
    #[inline]
    pub fn fsdll_rst_en(&mut self) -> _FSDLL_RST_ENW {
        _FSDLL_RST_ENW { w: self }
    }
    #[doc = "Bit 28 - Forces the next FS packet that is transmitted to have a EOP with LS timing"]
    #[inline]
    pub fn host_force_ls_se0(&mut self) -> _HOST_FORCE_LS_SE0W {
        _HOST_FORCE_LS_SE0W { w: self }
    }
    #[doc = "Bit 30 - Gate UTMI Clocks"]
    #[inline]
    pub fn clkgate(&mut self) -> _CLKGATEW {
        _CLKGATEW { w: self }
    }
    #[doc = "Bit 31 - Writing a 1 to this bit will soft-reset the USBPHYx_PWD, USBPHYx_TX, USBPHYx_RX, and USBPHYx_CTRL registers"]
    #[inline]
    pub fn sftrst(&mut self) -> _SFTRSTW {
        _SFTRSTW { w: self }
    }
}
