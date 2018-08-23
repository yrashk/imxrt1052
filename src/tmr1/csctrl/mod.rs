#[doc = r" Value read from the register"]
pub struct R {
    bits: u16,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u16,
}
impl super::CSCTRL {
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
#[doc = "Possible values of the field `CL1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CL1R {
    #[doc = "Never preload"]
    CL1_0,
    #[doc = "Load upon successful compare with the value in COMP1"]
    CL1_1,
    #[doc = "Load upon successful compare with the value in COMP2"]
    CL1_2,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl CL1R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            CL1R::CL1_0 => 0,
            CL1R::CL1_1 => 1,
            CL1R::CL1_2 => 2,
            CL1R::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> CL1R {
        match value {
            0 => CL1R::CL1_0,
            1 => CL1R::CL1_1,
            2 => CL1R::CL1_2,
            i => CL1R::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `CL1_0`"]
    #[inline]
    pub fn is_cl1_0(&self) -> bool {
        *self == CL1R::CL1_0
    }
    #[doc = "Checks if the value of the field is `CL1_1`"]
    #[inline]
    pub fn is_cl1_1(&self) -> bool {
        *self == CL1R::CL1_1
    }
    #[doc = "Checks if the value of the field is `CL1_2`"]
    #[inline]
    pub fn is_cl1_2(&self) -> bool {
        *self == CL1R::CL1_2
    }
}
#[doc = "Possible values of the field `CL2`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CL2R {
    #[doc = "Never preload"]
    CL2_0,
    #[doc = "Load upon successful compare with the value in COMP1"]
    CL2_1,
    #[doc = "Load upon successful compare with the value in COMP2"]
    CL2_2,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl CL2R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            CL2R::CL2_0 => 0,
            CL2R::CL2_1 => 1,
            CL2R::CL2_2 => 2,
            CL2R::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> CL2R {
        match value {
            0 => CL2R::CL2_0,
            1 => CL2R::CL2_1,
            2 => CL2R::CL2_2,
            i => CL2R::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `CL2_0`"]
    #[inline]
    pub fn is_cl2_0(&self) -> bool {
        *self == CL2R::CL2_0
    }
    #[doc = "Checks if the value of the field is `CL2_1`"]
    #[inline]
    pub fn is_cl2_1(&self) -> bool {
        *self == CL2R::CL2_1
    }
    #[doc = "Checks if the value of the field is `CL2_2`"]
    #[inline]
    pub fn is_cl2_2(&self) -> bool {
        *self == CL2R::CL2_2
    }
}
#[doc = r" Value of the field"]
pub struct TCF1R {
    bits: bool,
}
impl TCF1R {
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
pub struct TCF2R {
    bits: bool,
}
impl TCF2R {
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
pub struct TCF1ENR {
    bits: bool,
}
impl TCF1ENR {
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
pub struct TCF2ENR {
    bits: bool,
}
impl TCF2ENR {
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
#[doc = "Possible values of the field `UP`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UPR {
    #[doc = "The last count was in the DOWN direction."]
    UP_0,
    #[doc = "The last count was in the UP direction."]
    UP_1,
}
impl UPR {
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
            UPR::UP_0 => false,
            UPR::UP_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> UPR {
        match value {
            false => UPR::UP_0,
            true => UPR::UP_1,
        }
    }
    #[doc = "Checks if the value of the field is `UP_0`"]
    #[inline]
    pub fn is_up_0(&self) -> bool {
        *self == UPR::UP_0
    }
    #[doc = "Checks if the value of the field is `UP_1`"]
    #[inline]
    pub fn is_up_1(&self) -> bool {
        *self == UPR::UP_1
    }
}
#[doc = "Possible values of the field `TCI`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TCIR {
    #[doc = "Stop counter upon receiving a second trigger event while still counting from the first trigger event."]
    TCI_0,
    #[doc = "Reload the counter upon receiving a second trigger event while still counting from the first trigger event."]
    TCI_1,
}
impl TCIR {
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
            TCIR::TCI_0 => false,
            TCIR::TCI_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TCIR {
        match value {
            false => TCIR::TCI_0,
            true => TCIR::TCI_1,
        }
    }
    #[doc = "Checks if the value of the field is `TCI_0`"]
    #[inline]
    pub fn is_tci_0(&self) -> bool {
        *self == TCIR::TCI_0
    }
    #[doc = "Checks if the value of the field is `TCI_1`"]
    #[inline]
    pub fn is_tci_1(&self) -> bool {
        *self == TCIR::TCI_1
    }
}
#[doc = "Possible values of the field `ROC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ROCR {
    #[doc = "Do not reload the counter on a capture event."]
    ROC_0,
    #[doc = "Reload the counter on a capture event."]
    ROC_1,
}
impl ROCR {
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
            ROCR::ROC_0 => false,
            ROCR::ROC_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ROCR {
        match value {
            false => ROCR::ROC_0,
            true => ROCR::ROC_1,
        }
    }
    #[doc = "Checks if the value of the field is `ROC_0`"]
    #[inline]
    pub fn is_roc_0(&self) -> bool {
        *self == ROCR::ROC_0
    }
    #[doc = "Checks if the value of the field is `ROC_1`"]
    #[inline]
    pub fn is_roc_1(&self) -> bool {
        *self == ROCR::ROC_1
    }
}
#[doc = "Possible values of the field `ALT_LOAD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ALT_LOADR {
    #[doc = "Counter can be re-initialized only with the LOAD register."]
    ALT_LOAD_0,
    #[doc = "Counter can be re-initialized with the LOAD or CMPLD2 registers depending on count direction."]
    ALT_LOAD_1,
}
impl ALT_LOADR {
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
            ALT_LOADR::ALT_LOAD_0 => false,
            ALT_LOADR::ALT_LOAD_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ALT_LOADR {
        match value {
            false => ALT_LOADR::ALT_LOAD_0,
            true => ALT_LOADR::ALT_LOAD_1,
        }
    }
    #[doc = "Checks if the value of the field is `ALT_LOAD_0`"]
    #[inline]
    pub fn is_alt_load_0(&self) -> bool {
        *self == ALT_LOADR::ALT_LOAD_0
    }
    #[doc = "Checks if the value of the field is `ALT_LOAD_1`"]
    #[inline]
    pub fn is_alt_load_1(&self) -> bool {
        *self == ALT_LOADR::ALT_LOAD_1
    }
}
#[doc = "Possible values of the field `FAULT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FAULTR {
    #[doc = "Fault function disabled."]
    FAULT_0,
    #[doc = "Fault function enabled."]
    FAULT_1,
}
impl FAULTR {
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
            FAULTR::FAULT_0 => false,
            FAULTR::FAULT_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> FAULTR {
        match value {
            false => FAULTR::FAULT_0,
            true => FAULTR::FAULT_1,
        }
    }
    #[doc = "Checks if the value of the field is `FAULT_0`"]
    #[inline]
    pub fn is_fault_0(&self) -> bool {
        *self == FAULTR::FAULT_0
    }
    #[doc = "Checks if the value of the field is `FAULT_1`"]
    #[inline]
    pub fn is_fault_1(&self) -> bool {
        *self == FAULTR::FAULT_1
    }
}
#[doc = "Possible values of the field `DBG_EN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DBG_ENR {
    #[doc = "Continue with normal operation during debug mode. (default)"]
    DBG_EN_0,
    #[doc = "Halt TMR counter during debug mode."]
    DBG_EN_1,
    #[doc = "Force TMR output to logic 0 (prior to consideration of SCTRL[OPS])."]
    DBG_EN_2,
    #[doc = "Both halt counter and force output to 0 during debug mode."]
    DBG_EN_3,
}
impl DBG_ENR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            DBG_ENR::DBG_EN_0 => 0,
            DBG_ENR::DBG_EN_1 => 1,
            DBG_ENR::DBG_EN_2 => 2,
            DBG_ENR::DBG_EN_3 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> DBG_ENR {
        match value {
            0 => DBG_ENR::DBG_EN_0,
            1 => DBG_ENR::DBG_EN_1,
            2 => DBG_ENR::DBG_EN_2,
            3 => DBG_ENR::DBG_EN_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DBG_EN_0`"]
    #[inline]
    pub fn is_dbg_en_0(&self) -> bool {
        *self == DBG_ENR::DBG_EN_0
    }
    #[doc = "Checks if the value of the field is `DBG_EN_1`"]
    #[inline]
    pub fn is_dbg_en_1(&self) -> bool {
        *self == DBG_ENR::DBG_EN_1
    }
    #[doc = "Checks if the value of the field is `DBG_EN_2`"]
    #[inline]
    pub fn is_dbg_en_2(&self) -> bool {
        *self == DBG_ENR::DBG_EN_2
    }
    #[doc = "Checks if the value of the field is `DBG_EN_3`"]
    #[inline]
    pub fn is_dbg_en_3(&self) -> bool {
        *self == DBG_ENR::DBG_EN_3
    }
}
#[doc = "Values that can be written to the field `CL1`"]
pub enum CL1W {
    #[doc = "Never preload"]
    CL1_0,
    #[doc = "Load upon successful compare with the value in COMP1"]
    CL1_1,
    #[doc = "Load upon successful compare with the value in COMP2"]
    CL1_2,
}
impl CL1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            CL1W::CL1_0 => 0,
            CL1W::CL1_1 => 1,
            CL1W::CL1_2 => 2,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CL1W<'a> {
    w: &'a mut W,
}
impl<'a> _CL1W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CL1W) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Never preload"]
    #[inline]
    pub fn cl1_0(self) -> &'a mut W {
        self.variant(CL1W::CL1_0)
    }
    #[doc = "Load upon successful compare with the value in COMP1"]
    #[inline]
    pub fn cl1_1(self) -> &'a mut W {
        self.variant(CL1W::CL1_1)
    }
    #[doc = "Load upon successful compare with the value in COMP2"]
    #[inline]
    pub fn cl1_2(self) -> &'a mut W {
        self.variant(CL1W::CL1_2)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u16) << OFFSET);
        self.w.bits |= ((value & MASK) as u16) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `CL2`"]
pub enum CL2W {
    #[doc = "Never preload"]
    CL2_0,
    #[doc = "Load upon successful compare with the value in COMP1"]
    CL2_1,
    #[doc = "Load upon successful compare with the value in COMP2"]
    CL2_2,
}
impl CL2W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            CL2W::CL2_0 => 0,
            CL2W::CL2_1 => 1,
            CL2W::CL2_2 => 2,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CL2W<'a> {
    w: &'a mut W,
}
impl<'a> _CL2W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CL2W) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Never preload"]
    #[inline]
    pub fn cl2_0(self) -> &'a mut W {
        self.variant(CL2W::CL2_0)
    }
    #[doc = "Load upon successful compare with the value in COMP1"]
    #[inline]
    pub fn cl2_1(self) -> &'a mut W {
        self.variant(CL2W::CL2_1)
    }
    #[doc = "Load upon successful compare with the value in COMP2"]
    #[inline]
    pub fn cl2_2(self) -> &'a mut W {
        self.variant(CL2W::CL2_2)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 2;
        self.w.bits &= !((MASK as u16) << OFFSET);
        self.w.bits |= ((value & MASK) as u16) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _TCF1W<'a> {
    w: &'a mut W,
}
impl<'a> _TCF1W<'a> {
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
        self.w.bits &= !((MASK as u16) << OFFSET);
        self.w.bits |= ((value & MASK) as u16) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _TCF2W<'a> {
    w: &'a mut W,
}
impl<'a> _TCF2W<'a> {
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
        self.w.bits &= !((MASK as u16) << OFFSET);
        self.w.bits |= ((value & MASK) as u16) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _TCF1ENW<'a> {
    w: &'a mut W,
}
impl<'a> _TCF1ENW<'a> {
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
        self.w.bits &= !((MASK as u16) << OFFSET);
        self.w.bits |= ((value & MASK) as u16) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _TCF2ENW<'a> {
    w: &'a mut W,
}
impl<'a> _TCF2ENW<'a> {
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
        self.w.bits &= !((MASK as u16) << OFFSET);
        self.w.bits |= ((value & MASK) as u16) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `TCI`"]
pub enum TCIW {
    #[doc = "Stop counter upon receiving a second trigger event while still counting from the first trigger event."]
    TCI_0,
    #[doc = "Reload the counter upon receiving a second trigger event while still counting from the first trigger event."]
    TCI_1,
}
impl TCIW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TCIW::TCI_0 => false,
            TCIW::TCI_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TCIW<'a> {
    w: &'a mut W,
}
impl<'a> _TCIW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TCIW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Stop counter upon receiving a second trigger event while still counting from the first trigger event."]
    #[inline]
    pub fn tci_0(self) -> &'a mut W {
        self.variant(TCIW::TCI_0)
    }
    #[doc = "Reload the counter upon receiving a second trigger event while still counting from the first trigger event."]
    #[inline]
    pub fn tci_1(self) -> &'a mut W {
        self.variant(TCIW::TCI_1)
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
        self.w.bits &= !((MASK as u16) << OFFSET);
        self.w.bits |= ((value & MASK) as u16) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `ROC`"]
pub enum ROCW {
    #[doc = "Do not reload the counter on a capture event."]
    ROC_0,
    #[doc = "Reload the counter on a capture event."]
    ROC_1,
}
impl ROCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ROCW::ROC_0 => false,
            ROCW::ROC_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ROCW<'a> {
    w: &'a mut W,
}
impl<'a> _ROCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ROCW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Do not reload the counter on a capture event."]
    #[inline]
    pub fn roc_0(self) -> &'a mut W {
        self.variant(ROCW::ROC_0)
    }
    #[doc = "Reload the counter on a capture event."]
    #[inline]
    pub fn roc_1(self) -> &'a mut W {
        self.variant(ROCW::ROC_1)
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
        self.w.bits &= !((MASK as u16) << OFFSET);
        self.w.bits |= ((value & MASK) as u16) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `ALT_LOAD`"]
pub enum ALT_LOADW {
    #[doc = "Counter can be re-initialized only with the LOAD register."]
    ALT_LOAD_0,
    #[doc = "Counter can be re-initialized with the LOAD or CMPLD2 registers depending on count direction."]
    ALT_LOAD_1,
}
impl ALT_LOADW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ALT_LOADW::ALT_LOAD_0 => false,
            ALT_LOADW::ALT_LOAD_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ALT_LOADW<'a> {
    w: &'a mut W,
}
impl<'a> _ALT_LOADW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ALT_LOADW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Counter can be re-initialized only with the LOAD register."]
    #[inline]
    pub fn alt_load_0(self) -> &'a mut W {
        self.variant(ALT_LOADW::ALT_LOAD_0)
    }
    #[doc = "Counter can be re-initialized with the LOAD or CMPLD2 registers depending on count direction."]
    #[inline]
    pub fn alt_load_1(self) -> &'a mut W {
        self.variant(ALT_LOADW::ALT_LOAD_1)
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
        self.w.bits &= !((MASK as u16) << OFFSET);
        self.w.bits |= ((value & MASK) as u16) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `FAULT`"]
pub enum FAULTW {
    #[doc = "Fault function disabled."]
    FAULT_0,
    #[doc = "Fault function enabled."]
    FAULT_1,
}
impl FAULTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            FAULTW::FAULT_0 => false,
            FAULTW::FAULT_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FAULTW<'a> {
    w: &'a mut W,
}
impl<'a> _FAULTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FAULTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Fault function disabled."]
    #[inline]
    pub fn fault_0(self) -> &'a mut W {
        self.variant(FAULTW::FAULT_0)
    }
    #[doc = "Fault function enabled."]
    #[inline]
    pub fn fault_1(self) -> &'a mut W {
        self.variant(FAULTW::FAULT_1)
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
        self.w.bits &= !((MASK as u16) << OFFSET);
        self.w.bits |= ((value & MASK) as u16) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `DBG_EN`"]
pub enum DBG_ENW {
    #[doc = "Continue with normal operation during debug mode. (default)"]
    DBG_EN_0,
    #[doc = "Halt TMR counter during debug mode."]
    DBG_EN_1,
    #[doc = "Force TMR output to logic 0 (prior to consideration of SCTRL[OPS])."]
    DBG_EN_2,
    #[doc = "Both halt counter and force output to 0 during debug mode."]
    DBG_EN_3,
}
impl DBG_ENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            DBG_ENW::DBG_EN_0 => 0,
            DBG_ENW::DBG_EN_1 => 1,
            DBG_ENW::DBG_EN_2 => 2,
            DBG_ENW::DBG_EN_3 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DBG_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _DBG_ENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DBG_ENW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Continue with normal operation during debug mode. (default)"]
    #[inline]
    pub fn dbg_en_0(self) -> &'a mut W {
        self.variant(DBG_ENW::DBG_EN_0)
    }
    #[doc = "Halt TMR counter during debug mode."]
    #[inline]
    pub fn dbg_en_1(self) -> &'a mut W {
        self.variant(DBG_ENW::DBG_EN_1)
    }
    #[doc = "Force TMR output to logic 0 (prior to consideration of SCTRL[OPS])."]
    #[inline]
    pub fn dbg_en_2(self) -> &'a mut W {
        self.variant(DBG_ENW::DBG_EN_2)
    }
    #[doc = "Both halt counter and force output to 0 during debug mode."]
    #[inline]
    pub fn dbg_en_3(self) -> &'a mut W {
        self.variant(DBG_ENW::DBG_EN_3)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 14;
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
    #[doc = "Bits 0:1 - Compare Load Control 1"]
    #[inline]
    pub fn cl1(&self) -> CL1R {
        CL1R::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u16) as u8
        })
    }
    #[doc = "Bits 2:3 - Compare Load Control 2"]
    #[inline]
    pub fn cl2(&self) -> CL2R {
        CL2R::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u16) as u8
        })
    }
    #[doc = "Bit 4 - Timer Compare 1 Interrupt Flag"]
    #[inline]
    pub fn tcf1(&self) -> TCF1R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        };
        TCF1R { bits }
    }
    #[doc = "Bit 5 - Timer Compare 2 Interrupt Flag"]
    #[inline]
    pub fn tcf2(&self) -> TCF2R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        };
        TCF2R { bits }
    }
    #[doc = "Bit 6 - Timer Compare 1 Interrupt Enable"]
    #[inline]
    pub fn tcf1en(&self) -> TCF1ENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        };
        TCF1ENR { bits }
    }
    #[doc = "Bit 7 - Timer Compare 2 Interrupt Enable"]
    #[inline]
    pub fn tcf2en(&self) -> TCF2ENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        };
        TCF2ENR { bits }
    }
    #[doc = "Bit 9 - Counting Direction Indicator"]
    #[inline]
    pub fn up(&self) -> UPR {
        UPR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        })
    }
    #[doc = "Bit 10 - Triggered Count Initialization Control"]
    #[inline]
    pub fn tci(&self) -> TCIR {
        TCIR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        })
    }
    #[doc = "Bit 11 - Reload on Capture"]
    #[inline]
    pub fn roc(&self) -> ROCR {
        ROCR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        })
    }
    #[doc = "Bit 12 - Alternative Load Enable"]
    #[inline]
    pub fn alt_load(&self) -> ALT_LOADR {
        ALT_LOADR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        })
    }
    #[doc = "Bit 13 - Fault Enable"]
    #[inline]
    pub fn fault(&self) -> FAULTR {
        FAULTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        })
    }
    #[doc = "Bits 14:15 - Debug Actions Enable"]
    #[inline]
    pub fn dbg_en(&self) -> DBG_ENR {
        DBG_ENR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 14;
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
    #[doc = "Bits 0:1 - Compare Load Control 1"]
    #[inline]
    pub fn cl1(&mut self) -> _CL1W {
        _CL1W { w: self }
    }
    #[doc = "Bits 2:3 - Compare Load Control 2"]
    #[inline]
    pub fn cl2(&mut self) -> _CL2W {
        _CL2W { w: self }
    }
    #[doc = "Bit 4 - Timer Compare 1 Interrupt Flag"]
    #[inline]
    pub fn tcf1(&mut self) -> _TCF1W {
        _TCF1W { w: self }
    }
    #[doc = "Bit 5 - Timer Compare 2 Interrupt Flag"]
    #[inline]
    pub fn tcf2(&mut self) -> _TCF2W {
        _TCF2W { w: self }
    }
    #[doc = "Bit 6 - Timer Compare 1 Interrupt Enable"]
    #[inline]
    pub fn tcf1en(&mut self) -> _TCF1ENW {
        _TCF1ENW { w: self }
    }
    #[doc = "Bit 7 - Timer Compare 2 Interrupt Enable"]
    #[inline]
    pub fn tcf2en(&mut self) -> _TCF2ENW {
        _TCF2ENW { w: self }
    }
    #[doc = "Bit 10 - Triggered Count Initialization Control"]
    #[inline]
    pub fn tci(&mut self) -> _TCIW {
        _TCIW { w: self }
    }
    #[doc = "Bit 11 - Reload on Capture"]
    #[inline]
    pub fn roc(&mut self) -> _ROCW {
        _ROCW { w: self }
    }
    #[doc = "Bit 12 - Alternative Load Enable"]
    #[inline]
    pub fn alt_load(&mut self) -> _ALT_LOADW {
        _ALT_LOADW { w: self }
    }
    #[doc = "Bit 13 - Fault Enable"]
    #[inline]
    pub fn fault(&mut self) -> _FAULTW {
        _FAULTW { w: self }
    }
    #[doc = "Bits 14:15 - Debug Actions Enable"]
    #[inline]
    pub fn dbg_en(&mut self) -> _DBG_ENW {
        _DBG_ENW { w: self }
    }
}
