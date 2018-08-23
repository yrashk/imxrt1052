#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CTRL1 {
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
pub struct PROPSEGR {
    bits: u8,
}
impl PROPSEGR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Possible values of the field `LOM`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LOMR {
    #[doc = "Listen Only Mode is deactivated"]
    LOM_0,
    #[doc = "FLEXCAN module operates in Listen Only Mode"]
    LOM_1,
}
impl LOMR {
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
            LOMR::LOM_0 => false,
            LOMR::LOM_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> LOMR {
        match value {
            false => LOMR::LOM_0,
            true => LOMR::LOM_1,
        }
    }
    #[doc = "Checks if the value of the field is `LOM_0`"]
    #[inline]
    pub fn is_lom_0(&self) -> bool {
        *self == LOMR::LOM_0
    }
    #[doc = "Checks if the value of the field is `LOM_1`"]
    #[inline]
    pub fn is_lom_1(&self) -> bool {
        *self == LOMR::LOM_1
    }
}
#[doc = "Possible values of the field `LBUF`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LBUFR {
    #[doc = "Buffer with highest priority is transmitted first"]
    LBUF_0,
    #[doc = "Lowest number buffer is transmitted first"]
    LBUF_1,
}
impl LBUFR {
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
            LBUFR::LBUF_0 => false,
            LBUFR::LBUF_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> LBUFR {
        match value {
            false => LBUFR::LBUF_0,
            true => LBUFR::LBUF_1,
        }
    }
    #[doc = "Checks if the value of the field is `LBUF_0`"]
    #[inline]
    pub fn is_lbuf_0(&self) -> bool {
        *self == LBUFR::LBUF_0
    }
    #[doc = "Checks if the value of the field is `LBUF_1`"]
    #[inline]
    pub fn is_lbuf_1(&self) -> bool {
        *self == LBUFR::LBUF_1
    }
}
#[doc = "Possible values of the field `TSYN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TSYNR {
    #[doc = "Timer Sync feature disabled"]
    TSYN_0,
    #[doc = "Timer Sync feature enabled"]
    TSYN_1,
}
impl TSYNR {
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
            TSYNR::TSYN_0 => false,
            TSYNR::TSYN_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TSYNR {
        match value {
            false => TSYNR::TSYN_0,
            true => TSYNR::TSYN_1,
        }
    }
    #[doc = "Checks if the value of the field is `TSYN_0`"]
    #[inline]
    pub fn is_tsyn_0(&self) -> bool {
        *self == TSYNR::TSYN_0
    }
    #[doc = "Checks if the value of the field is `TSYN_1`"]
    #[inline]
    pub fn is_tsyn_1(&self) -> bool {
        *self == TSYNR::TSYN_1
    }
}
#[doc = "Possible values of the field `BOFFREC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BOFFRECR {
    #[doc = "Automatic recovering from Bus Off state enabled, according to CAN Spec 2.0 part B"]
    BOFFREC_0,
    #[doc = "Automatic recovering from Bus Off state disabled"]
    BOFFREC_1,
}
impl BOFFRECR {
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
            BOFFRECR::BOFFREC_0 => false,
            BOFFRECR::BOFFREC_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> BOFFRECR {
        match value {
            false => BOFFRECR::BOFFREC_0,
            true => BOFFRECR::BOFFREC_1,
        }
    }
    #[doc = "Checks if the value of the field is `BOFFREC_0`"]
    #[inline]
    pub fn is_boffrec_0(&self) -> bool {
        *self == BOFFRECR::BOFFREC_0
    }
    #[doc = "Checks if the value of the field is `BOFFREC_1`"]
    #[inline]
    pub fn is_boffrec_1(&self) -> bool {
        *self == BOFFRECR::BOFFREC_1
    }
}
#[doc = "Possible values of the field `SMP`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SMPR {
    #[doc = "Just one sample is used to determine the bit value"]
    SMP_0,
    #[doc = "Three samples are used to determine the value of the received bit: the regular one (sample point) and 2 preceding samples, a majority rule is used"]
    SMP_1,
}
impl SMPR {
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
            SMPR::SMP_0 => false,
            SMPR::SMP_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SMPR {
        match value {
            false => SMPR::SMP_0,
            true => SMPR::SMP_1,
        }
    }
    #[doc = "Checks if the value of the field is `SMP_0`"]
    #[inline]
    pub fn is_smp_0(&self) -> bool {
        *self == SMPR::SMP_0
    }
    #[doc = "Checks if the value of the field is `SMP_1`"]
    #[inline]
    pub fn is_smp_1(&self) -> bool {
        *self == SMPR::SMP_1
    }
}
#[doc = "Possible values of the field `RWRNMSK`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RWRNMSKR {
    #[doc = "Rx Warning Interrupt disabled"]
    RWRNMSK_0,
    #[doc = "Rx Warning Interrupt enabled"]
    RWRNMSK_1,
}
impl RWRNMSKR {
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
            RWRNMSKR::RWRNMSK_0 => false,
            RWRNMSKR::RWRNMSK_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RWRNMSKR {
        match value {
            false => RWRNMSKR::RWRNMSK_0,
            true => RWRNMSKR::RWRNMSK_1,
        }
    }
    #[doc = "Checks if the value of the field is `RWRNMSK_0`"]
    #[inline]
    pub fn is_rwrnmsk_0(&self) -> bool {
        *self == RWRNMSKR::RWRNMSK_0
    }
    #[doc = "Checks if the value of the field is `RWRNMSK_1`"]
    #[inline]
    pub fn is_rwrnmsk_1(&self) -> bool {
        *self == RWRNMSKR::RWRNMSK_1
    }
}
#[doc = "Possible values of the field `TWRNMSK`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TWRNMSKR {
    #[doc = "Tx Warning Interrupt disabled"]
    TWRNMSK_0,
    #[doc = "Tx Warning Interrupt enabled"]
    TWRNMSK_1,
}
impl TWRNMSKR {
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
            TWRNMSKR::TWRNMSK_0 => false,
            TWRNMSKR::TWRNMSK_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TWRNMSKR {
        match value {
            false => TWRNMSKR::TWRNMSK_0,
            true => TWRNMSKR::TWRNMSK_1,
        }
    }
    #[doc = "Checks if the value of the field is `TWRNMSK_0`"]
    #[inline]
    pub fn is_twrnmsk_0(&self) -> bool {
        *self == TWRNMSKR::TWRNMSK_0
    }
    #[doc = "Checks if the value of the field is `TWRNMSK_1`"]
    #[inline]
    pub fn is_twrnmsk_1(&self) -> bool {
        *self == TWRNMSKR::TWRNMSK_1
    }
}
#[doc = "Possible values of the field `LPB`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LPBR {
    #[doc = "Loop Back disabled"]
    LPB_0,
    #[doc = "Loop Back enabled"]
    LPB_1,
}
impl LPBR {
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
            LPBR::LPB_0 => false,
            LPBR::LPB_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> LPBR {
        match value {
            false => LPBR::LPB_0,
            true => LPBR::LPB_1,
        }
    }
    #[doc = "Checks if the value of the field is `LPB_0`"]
    #[inline]
    pub fn is_lpb_0(&self) -> bool {
        *self == LPBR::LPB_0
    }
    #[doc = "Checks if the value of the field is `LPB_1`"]
    #[inline]
    pub fn is_lpb_1(&self) -> bool {
        *self == LPBR::LPB_1
    }
}
#[doc = "Possible values of the field `ERRMSK`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ERRMSKR {
    #[doc = "Error interrupt disabled"]
    ERRMSK_0,
    #[doc = "Error interrupt enabled"]
    ERRMSK_1,
}
impl ERRMSKR {
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
            ERRMSKR::ERRMSK_0 => false,
            ERRMSKR::ERRMSK_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ERRMSKR {
        match value {
            false => ERRMSKR::ERRMSK_0,
            true => ERRMSKR::ERRMSK_1,
        }
    }
    #[doc = "Checks if the value of the field is `ERRMSK_0`"]
    #[inline]
    pub fn is_errmsk_0(&self) -> bool {
        *self == ERRMSKR::ERRMSK_0
    }
    #[doc = "Checks if the value of the field is `ERRMSK_1`"]
    #[inline]
    pub fn is_errmsk_1(&self) -> bool {
        *self == ERRMSKR::ERRMSK_1
    }
}
#[doc = "Possible values of the field `BOFFMSK`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BOFFMSKR {
    #[doc = "Bus Off interrupt disabled"]
    BOFFMSK_0,
    #[doc = "Bus Off interrupt enabled"]
    BOFFMSK_1,
}
impl BOFFMSKR {
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
            BOFFMSKR::BOFFMSK_0 => false,
            BOFFMSKR::BOFFMSK_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> BOFFMSKR {
        match value {
            false => BOFFMSKR::BOFFMSK_0,
            true => BOFFMSKR::BOFFMSK_1,
        }
    }
    #[doc = "Checks if the value of the field is `BOFFMSK_0`"]
    #[inline]
    pub fn is_boffmsk_0(&self) -> bool {
        *self == BOFFMSKR::BOFFMSK_0
    }
    #[doc = "Checks if the value of the field is `BOFFMSK_1`"]
    #[inline]
    pub fn is_boffmsk_1(&self) -> bool {
        *self == BOFFMSKR::BOFFMSK_1
    }
}
#[doc = r" Value of the field"]
pub struct PSEG2R {
    bits: u8,
}
impl PSEG2R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct PSEG1R {
    bits: u8,
}
impl PSEG1R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct RJWR {
    bits: u8,
}
impl RJWR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct PRESDIVR {
    bits: u8,
}
impl PRESDIVR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Proxy"]
pub struct _PROPSEGW<'a> {
    w: &'a mut W,
}
impl<'a> _PROPSEGW<'a> {
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
#[doc = "Values that can be written to the field `LOM`"]
pub enum LOMW {
    #[doc = "Listen Only Mode is deactivated"]
    LOM_0,
    #[doc = "FLEXCAN module operates in Listen Only Mode"]
    LOM_1,
}
impl LOMW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            LOMW::LOM_0 => false,
            LOMW::LOM_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _LOMW<'a> {
    w: &'a mut W,
}
impl<'a> _LOMW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: LOMW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Listen Only Mode is deactivated"]
    #[inline]
    pub fn lom_0(self) -> &'a mut W {
        self.variant(LOMW::LOM_0)
    }
    #[doc = "FLEXCAN module operates in Listen Only Mode"]
    #[inline]
    pub fn lom_1(self) -> &'a mut W {
        self.variant(LOMW::LOM_1)
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
#[doc = "Values that can be written to the field `LBUF`"]
pub enum LBUFW {
    #[doc = "Buffer with highest priority is transmitted first"]
    LBUF_0,
    #[doc = "Lowest number buffer is transmitted first"]
    LBUF_1,
}
impl LBUFW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            LBUFW::LBUF_0 => false,
            LBUFW::LBUF_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _LBUFW<'a> {
    w: &'a mut W,
}
impl<'a> _LBUFW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: LBUFW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Buffer with highest priority is transmitted first"]
    #[inline]
    pub fn lbuf_0(self) -> &'a mut W {
        self.variant(LBUFW::LBUF_0)
    }
    #[doc = "Lowest number buffer is transmitted first"]
    #[inline]
    pub fn lbuf_1(self) -> &'a mut W {
        self.variant(LBUFW::LBUF_1)
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
#[doc = "Values that can be written to the field `TSYN`"]
pub enum TSYNW {
    #[doc = "Timer Sync feature disabled"]
    TSYN_0,
    #[doc = "Timer Sync feature enabled"]
    TSYN_1,
}
impl TSYNW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TSYNW::TSYN_0 => false,
            TSYNW::TSYN_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TSYNW<'a> {
    w: &'a mut W,
}
impl<'a> _TSYNW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TSYNW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Timer Sync feature disabled"]
    #[inline]
    pub fn tsyn_0(self) -> &'a mut W {
        self.variant(TSYNW::TSYN_0)
    }
    #[doc = "Timer Sync feature enabled"]
    #[inline]
    pub fn tsyn_1(self) -> &'a mut W {
        self.variant(TSYNW::TSYN_1)
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
        const OFFSET: u8 = 5;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `BOFFREC`"]
pub enum BOFFRECW {
    #[doc = "Automatic recovering from Bus Off state enabled, according to CAN Spec 2.0 part B"]
    BOFFREC_0,
    #[doc = "Automatic recovering from Bus Off state disabled"]
    BOFFREC_1,
}
impl BOFFRECW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            BOFFRECW::BOFFREC_0 => false,
            BOFFRECW::BOFFREC_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _BOFFRECW<'a> {
    w: &'a mut W,
}
impl<'a> _BOFFRECW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: BOFFRECW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Automatic recovering from Bus Off state enabled, according to CAN Spec 2.0 part B"]
    #[inline]
    pub fn boffrec_0(self) -> &'a mut W {
        self.variant(BOFFRECW::BOFFREC_0)
    }
    #[doc = "Automatic recovering from Bus Off state disabled"]
    #[inline]
    pub fn boffrec_1(self) -> &'a mut W {
        self.variant(BOFFRECW::BOFFREC_1)
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
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `SMP`"]
pub enum SMPW {
    #[doc = "Just one sample is used to determine the bit value"]
    SMP_0,
    #[doc = "Three samples are used to determine the value of the received bit: the regular one (sample point) and 2 preceding samples, a majority rule is used"]
    SMP_1,
}
impl SMPW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SMPW::SMP_0 => false,
            SMPW::SMP_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SMPW<'a> {
    w: &'a mut W,
}
impl<'a> _SMPW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SMPW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Just one sample is used to determine the bit value"]
    #[inline]
    pub fn smp_0(self) -> &'a mut W {
        self.variant(SMPW::SMP_0)
    }
    #[doc = "Three samples are used to determine the value of the received bit: the regular one (sample point) and 2 preceding samples, a majority rule is used"]
    #[inline]
    pub fn smp_1(self) -> &'a mut W {
        self.variant(SMPW::SMP_1)
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
#[doc = "Values that can be written to the field `RWRNMSK`"]
pub enum RWRNMSKW {
    #[doc = "Rx Warning Interrupt disabled"]
    RWRNMSK_0,
    #[doc = "Rx Warning Interrupt enabled"]
    RWRNMSK_1,
}
impl RWRNMSKW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RWRNMSKW::RWRNMSK_0 => false,
            RWRNMSKW::RWRNMSK_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RWRNMSKW<'a> {
    w: &'a mut W,
}
impl<'a> _RWRNMSKW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RWRNMSKW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Rx Warning Interrupt disabled"]
    #[inline]
    pub fn rwrnmsk_0(self) -> &'a mut W {
        self.variant(RWRNMSKW::RWRNMSK_0)
    }
    #[doc = "Rx Warning Interrupt enabled"]
    #[inline]
    pub fn rwrnmsk_1(self) -> &'a mut W {
        self.variant(RWRNMSKW::RWRNMSK_1)
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
#[doc = "Values that can be written to the field `TWRNMSK`"]
pub enum TWRNMSKW {
    #[doc = "Tx Warning Interrupt disabled"]
    TWRNMSK_0,
    #[doc = "Tx Warning Interrupt enabled"]
    TWRNMSK_1,
}
impl TWRNMSKW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TWRNMSKW::TWRNMSK_0 => false,
            TWRNMSKW::TWRNMSK_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TWRNMSKW<'a> {
    w: &'a mut W,
}
impl<'a> _TWRNMSKW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TWRNMSKW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Tx Warning Interrupt disabled"]
    #[inline]
    pub fn twrnmsk_0(self) -> &'a mut W {
        self.variant(TWRNMSKW::TWRNMSK_0)
    }
    #[doc = "Tx Warning Interrupt enabled"]
    #[inline]
    pub fn twrnmsk_1(self) -> &'a mut W {
        self.variant(TWRNMSKW::TWRNMSK_1)
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
#[doc = "Values that can be written to the field `LPB`"]
pub enum LPBW {
    #[doc = "Loop Back disabled"]
    LPB_0,
    #[doc = "Loop Back enabled"]
    LPB_1,
}
impl LPBW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            LPBW::LPB_0 => false,
            LPBW::LPB_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _LPBW<'a> {
    w: &'a mut W,
}
impl<'a> _LPBW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: LPBW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Loop Back disabled"]
    #[inline]
    pub fn lpb_0(self) -> &'a mut W {
        self.variant(LPBW::LPB_0)
    }
    #[doc = "Loop Back enabled"]
    #[inline]
    pub fn lpb_1(self) -> &'a mut W {
        self.variant(LPBW::LPB_1)
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
#[doc = "Values that can be written to the field `ERRMSK`"]
pub enum ERRMSKW {
    #[doc = "Error interrupt disabled"]
    ERRMSK_0,
    #[doc = "Error interrupt enabled"]
    ERRMSK_1,
}
impl ERRMSKW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ERRMSKW::ERRMSK_0 => false,
            ERRMSKW::ERRMSK_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ERRMSKW<'a> {
    w: &'a mut W,
}
impl<'a> _ERRMSKW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ERRMSKW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Error interrupt disabled"]
    #[inline]
    pub fn errmsk_0(self) -> &'a mut W {
        self.variant(ERRMSKW::ERRMSK_0)
    }
    #[doc = "Error interrupt enabled"]
    #[inline]
    pub fn errmsk_1(self) -> &'a mut W {
        self.variant(ERRMSKW::ERRMSK_1)
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
#[doc = "Values that can be written to the field `BOFFMSK`"]
pub enum BOFFMSKW {
    #[doc = "Bus Off interrupt disabled"]
    BOFFMSK_0,
    #[doc = "Bus Off interrupt enabled"]
    BOFFMSK_1,
}
impl BOFFMSKW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            BOFFMSKW::BOFFMSK_0 => false,
            BOFFMSKW::BOFFMSK_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _BOFFMSKW<'a> {
    w: &'a mut W,
}
impl<'a> _BOFFMSKW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: BOFFMSKW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Bus Off interrupt disabled"]
    #[inline]
    pub fn boffmsk_0(self) -> &'a mut W {
        self.variant(BOFFMSKW::BOFFMSK_0)
    }
    #[doc = "Bus Off interrupt enabled"]
    #[inline]
    pub fn boffmsk_1(self) -> &'a mut W {
        self.variant(BOFFMSKW::BOFFMSK_1)
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
#[doc = r" Proxy"]
pub struct _PSEG2W<'a> {
    w: &'a mut W,
}
impl<'a> _PSEG2W<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 16;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _PSEG1W<'a> {
    w: &'a mut W,
}
impl<'a> _PSEG1W<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 19;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _RJWW<'a> {
    w: &'a mut W,
}
impl<'a> _RJWW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 22;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _PRESDIVW<'a> {
    w: &'a mut W,
}
impl<'a> _PRESDIVW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 255;
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
    #[doc = "Bits 0:2 - This 3-bit field defines the length of the Propagation Segment in the bit time"]
    #[inline]
    pub fn propseg(&self) -> PROPSEGR {
        let bits = {
            const MASK: u8 = 7;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        PROPSEGR { bits }
    }
    #[doc = "Bit 3 - This bit configures FLEXCAN to operate in Listen Only Mode"]
    #[inline]
    pub fn lom(&self) -> LOMR {
        LOMR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 4 - This bit defines the ordering mechanism for Message Buffer transmission"]
    #[inline]
    pub fn lbuf(&self) -> LBUFR {
        LBUFR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 5 - This bit enables a mechanism that resets the free-running timer each time a message is received in Message Buffer 0"]
    #[inline]
    pub fn tsyn(&self) -> TSYNR {
        TSYNR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 6 - This bit defines how FLEXCAN recovers from Bus Off state"]
    #[inline]
    pub fn boffrec(&self) -> BOFFRECR {
        BOFFRECR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 7 - This bit defines the sampling mode of CAN bits at the FLEXCAN_RX"]
    #[inline]
    pub fn smp(&self) -> SMPR {
        SMPR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 10 - This bit provides a mask for the Rx Warning Interrupt associated with the RWRN_INT flag in the Error and Status Register"]
    #[inline]
    pub fn rwrnmsk(&self) -> RWRNMSKR {
        RWRNMSKR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 11 - This bit provides a mask for the Tx Warning Interrupt associated with the TWRN_INT flag in the Error and Status Register"]
    #[inline]
    pub fn twrnmsk(&self) -> TWRNMSKR {
        TWRNMSKR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 12 - This bit configures FlexCAN to operate in Loop-Back Mode"]
    #[inline]
    pub fn lpb(&self) -> LPBR {
        LPBR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 14 - This bit provides a mask for the Error Interrupt."]
    #[inline]
    pub fn errmsk(&self) -> ERRMSKR {
        ERRMSKR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 15 - This bit provides a mask for the Bus Off Interrupt."]
    #[inline]
    pub fn boffmsk(&self) -> BOFFMSKR {
        BOFFMSKR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 15;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 16:18 - This 3-bit field defines the length of Phase Buffer Segment 2 in the bit time"]
    #[inline]
    pub fn pseg2(&self) -> PSEG2R {
        let bits = {
            const MASK: u8 = 7;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        PSEG2R { bits }
    }
    #[doc = "Bits 19:21 - This 3-bit field defines the length of Phase Buffer Segment 1 in the bit time"]
    #[inline]
    pub fn pseg1(&self) -> PSEG1R {
        let bits = {
            const MASK: u8 = 7;
            const OFFSET: u8 = 19;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        PSEG1R { bits }
    }
    #[doc = "Bits 22:23 - This 2-bit field defines the maximum number of time quanta One time quantum is equal to the Sclock period"]
    #[inline]
    pub fn rjw(&self) -> RJWR {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 22;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        RJWR { bits }
    }
    #[doc = "Bits 24:31 - This 8-bit field defines the ratio between the PE clock frequency and the Serial Clock (Sclock) frequency"]
    #[inline]
    pub fn presdiv(&self) -> PRESDIVR {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        PRESDIVR { bits }
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
    #[doc = "Bits 0:2 - This 3-bit field defines the length of the Propagation Segment in the bit time"]
    #[inline]
    pub fn propseg(&mut self) -> _PROPSEGW {
        _PROPSEGW { w: self }
    }
    #[doc = "Bit 3 - This bit configures FLEXCAN to operate in Listen Only Mode"]
    #[inline]
    pub fn lom(&mut self) -> _LOMW {
        _LOMW { w: self }
    }
    #[doc = "Bit 4 - This bit defines the ordering mechanism for Message Buffer transmission"]
    #[inline]
    pub fn lbuf(&mut self) -> _LBUFW {
        _LBUFW { w: self }
    }
    #[doc = "Bit 5 - This bit enables a mechanism that resets the free-running timer each time a message is received in Message Buffer 0"]
    #[inline]
    pub fn tsyn(&mut self) -> _TSYNW {
        _TSYNW { w: self }
    }
    #[doc = "Bit 6 - This bit defines how FLEXCAN recovers from Bus Off state"]
    #[inline]
    pub fn boffrec(&mut self) -> _BOFFRECW {
        _BOFFRECW { w: self }
    }
    #[doc = "Bit 7 - This bit defines the sampling mode of CAN bits at the FLEXCAN_RX"]
    #[inline]
    pub fn smp(&mut self) -> _SMPW {
        _SMPW { w: self }
    }
    #[doc = "Bit 10 - This bit provides a mask for the Rx Warning Interrupt associated with the RWRN_INT flag in the Error and Status Register"]
    #[inline]
    pub fn rwrnmsk(&mut self) -> _RWRNMSKW {
        _RWRNMSKW { w: self }
    }
    #[doc = "Bit 11 - This bit provides a mask for the Tx Warning Interrupt associated with the TWRN_INT flag in the Error and Status Register"]
    #[inline]
    pub fn twrnmsk(&mut self) -> _TWRNMSKW {
        _TWRNMSKW { w: self }
    }
    #[doc = "Bit 12 - This bit configures FlexCAN to operate in Loop-Back Mode"]
    #[inline]
    pub fn lpb(&mut self) -> _LPBW {
        _LPBW { w: self }
    }
    #[doc = "Bit 14 - This bit provides a mask for the Error Interrupt."]
    #[inline]
    pub fn errmsk(&mut self) -> _ERRMSKW {
        _ERRMSKW { w: self }
    }
    #[doc = "Bit 15 - This bit provides a mask for the Bus Off Interrupt."]
    #[inline]
    pub fn boffmsk(&mut self) -> _BOFFMSKW {
        _BOFFMSKW { w: self }
    }
    #[doc = "Bits 16:18 - This 3-bit field defines the length of Phase Buffer Segment 2 in the bit time"]
    #[inline]
    pub fn pseg2(&mut self) -> _PSEG2W {
        _PSEG2W { w: self }
    }
    #[doc = "Bits 19:21 - This 3-bit field defines the length of Phase Buffer Segment 1 in the bit time"]
    #[inline]
    pub fn pseg1(&mut self) -> _PSEG1W {
        _PSEG1W { w: self }
    }
    #[doc = "Bits 22:23 - This 2-bit field defines the maximum number of time quanta One time quantum is equal to the Sclock period"]
    #[inline]
    pub fn rjw(&mut self) -> _RJWW {
        _RJWW { w: self }
    }
    #[doc = "Bits 24:31 - This 8-bit field defines the ratio between the PE clock frequency and the Serial Clock (Sclock) frequency"]
    #[inline]
    pub fn presdiv(&mut self) -> _PRESDIVW {
        _PRESDIVW { w: self }
    }
}
