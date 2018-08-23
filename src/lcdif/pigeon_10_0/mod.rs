#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::PIGEON_10_0 {
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
pub struct ENR {
    bits: bool,
}
impl ENR {
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
#[doc = "Possible values of the field `POL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum POLR {
    #[doc = "Normal Signal (Active high)"]
    ACTIVE_HIGH,
    #[doc = "Inverted signal (Active low)"]
    ACTIVE_LOW,
}
impl POLR {
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
            POLR::ACTIVE_HIGH => false,
            POLR::ACTIVE_LOW => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> POLR {
        match value {
            false => POLR::ACTIVE_HIGH,
            true => POLR::ACTIVE_LOW,
        }
    }
    #[doc = "Checks if the value of the field is `ACTIVE_HIGH`"]
    #[inline]
    pub fn is_active_high(&self) -> bool {
        *self == POLR::ACTIVE_HIGH
    }
    #[doc = "Checks if the value of the field is `ACTIVE_LOW`"]
    #[inline]
    pub fn is_active_low(&self) -> bool {
        *self == POLR::ACTIVE_LOW
    }
}
#[doc = "Possible values of the field `INC_SEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INC_SELR {
    #[doc = "pclk"]
    PCLK,
    #[doc = "Line start pulse"]
    LINE,
    #[doc = "Frame start pulse"]
    FRAME,
    #[doc = "Use another signal as tick event"]
    SIG_ANOTHER,
}
impl INC_SELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            INC_SELR::PCLK => 0,
            INC_SELR::LINE => 1,
            INC_SELR::FRAME => 2,
            INC_SELR::SIG_ANOTHER => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> INC_SELR {
        match value {
            0 => INC_SELR::PCLK,
            1 => INC_SELR::LINE,
            2 => INC_SELR::FRAME,
            3 => INC_SELR::SIG_ANOTHER,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `PCLK`"]
    #[inline]
    pub fn is_pclk(&self) -> bool {
        *self == INC_SELR::PCLK
    }
    #[doc = "Checks if the value of the field is `LINE`"]
    #[inline]
    pub fn is_line(&self) -> bool {
        *self == INC_SELR::LINE
    }
    #[doc = "Checks if the value of the field is `FRAME`"]
    #[inline]
    pub fn is_frame(&self) -> bool {
        *self == INC_SELR::FRAME
    }
    #[doc = "Checks if the value of the field is `SIG_ANOTHER`"]
    #[inline]
    pub fn is_sig_another(&self) -> bool {
        *self == INC_SELR::SIG_ANOTHER
    }
}
#[doc = r" Value of the field"]
pub struct OFFSETR {
    bits: u8,
}
impl OFFSETR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Possible values of the field `MASK_CNT_SEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MASK_CNT_SELR {
    #[doc = "pclk counter within one hscan state"]
    HSTATE_CNT,
    #[doc = "pclk cycle within one hscan state"]
    HSTATE_CYCLE,
    #[doc = "line counter within one vscan state"]
    VSTATE_CNT,
    #[doc = "line cycle within one vscan state"]
    VSTATE_CYCLE,
    #[doc = "frame counter"]
    FRAME_CNT,
    #[doc = "frame cycle"]
    FRAME_CYCLE,
    #[doc = "horizontal counter (pclk counter within one line )"]
    HCNT,
    #[doc = "vertical counter (line counter within one frame)"]
    VCNT,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl MASK_CNT_SELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            MASK_CNT_SELR::HSTATE_CNT => 0,
            MASK_CNT_SELR::HSTATE_CYCLE => 1,
            MASK_CNT_SELR::VSTATE_CNT => 2,
            MASK_CNT_SELR::VSTATE_CYCLE => 3,
            MASK_CNT_SELR::FRAME_CNT => 4,
            MASK_CNT_SELR::FRAME_CYCLE => 5,
            MASK_CNT_SELR::HCNT => 6,
            MASK_CNT_SELR::VCNT => 7,
            MASK_CNT_SELR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> MASK_CNT_SELR {
        match value {
            0 => MASK_CNT_SELR::HSTATE_CNT,
            1 => MASK_CNT_SELR::HSTATE_CYCLE,
            2 => MASK_CNT_SELR::VSTATE_CNT,
            3 => MASK_CNT_SELR::VSTATE_CYCLE,
            4 => MASK_CNT_SELR::FRAME_CNT,
            5 => MASK_CNT_SELR::FRAME_CYCLE,
            6 => MASK_CNT_SELR::HCNT,
            7 => MASK_CNT_SELR::VCNT,
            i => MASK_CNT_SELR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `HSTATE_CNT`"]
    #[inline]
    pub fn is_hstate_cnt(&self) -> bool {
        *self == MASK_CNT_SELR::HSTATE_CNT
    }
    #[doc = "Checks if the value of the field is `HSTATE_CYCLE`"]
    #[inline]
    pub fn is_hstate_cycle(&self) -> bool {
        *self == MASK_CNT_SELR::HSTATE_CYCLE
    }
    #[doc = "Checks if the value of the field is `VSTATE_CNT`"]
    #[inline]
    pub fn is_vstate_cnt(&self) -> bool {
        *self == MASK_CNT_SELR::VSTATE_CNT
    }
    #[doc = "Checks if the value of the field is `VSTATE_CYCLE`"]
    #[inline]
    pub fn is_vstate_cycle(&self) -> bool {
        *self == MASK_CNT_SELR::VSTATE_CYCLE
    }
    #[doc = "Checks if the value of the field is `FRAME_CNT`"]
    #[inline]
    pub fn is_frame_cnt(&self) -> bool {
        *self == MASK_CNT_SELR::FRAME_CNT
    }
    #[doc = "Checks if the value of the field is `FRAME_CYCLE`"]
    #[inline]
    pub fn is_frame_cycle(&self) -> bool {
        *self == MASK_CNT_SELR::FRAME_CYCLE
    }
    #[doc = "Checks if the value of the field is `HCNT`"]
    #[inline]
    pub fn is_hcnt(&self) -> bool {
        *self == MASK_CNT_SELR::HCNT
    }
    #[doc = "Checks if the value of the field is `VCNT`"]
    #[inline]
    pub fn is_vcnt(&self) -> bool {
        *self == MASK_CNT_SELR::VCNT
    }
}
#[doc = r" Value of the field"]
pub struct MASK_CNTR {
    bits: u16,
}
impl MASK_CNTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }
}
#[doc = "Possible values of the field `STATE_MASK`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum STATE_MASKR {
    #[doc = "FRAME SYNC"]
    FS,
    #[doc = "FRAME BEGIN"]
    FB,
    #[doc = "FRAME DATA"]
    FD,
    #[doc = "FRAME END"]
    FE,
    #[doc = "LINE SYNC"]
    LS,
    #[doc = "LINE BEGIN"]
    LB,
    #[doc = "LINE DATA"]
    LD,
    #[doc = "LINE END"]
    LE,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl STATE_MASKR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            STATE_MASKR::FS => 1,
            STATE_MASKR::FB => 2,
            STATE_MASKR::FD => 4,
            STATE_MASKR::FE => 8,
            STATE_MASKR::LS => 16,
            STATE_MASKR::LB => 32,
            STATE_MASKR::LD => 64,
            STATE_MASKR::LE => 128,
            STATE_MASKR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> STATE_MASKR {
        match value {
            1 => STATE_MASKR::FS,
            2 => STATE_MASKR::FB,
            4 => STATE_MASKR::FD,
            8 => STATE_MASKR::FE,
            16 => STATE_MASKR::LS,
            32 => STATE_MASKR::LB,
            64 => STATE_MASKR::LD,
            128 => STATE_MASKR::LE,
            i => STATE_MASKR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `FS`"]
    #[inline]
    pub fn is_fs(&self) -> bool {
        *self == STATE_MASKR::FS
    }
    #[doc = "Checks if the value of the field is `FB`"]
    #[inline]
    pub fn is_fb(&self) -> bool {
        *self == STATE_MASKR::FB
    }
    #[doc = "Checks if the value of the field is `FD`"]
    #[inline]
    pub fn is_fd(&self) -> bool {
        *self == STATE_MASKR::FD
    }
    #[doc = "Checks if the value of the field is `FE`"]
    #[inline]
    pub fn is_fe(&self) -> bool {
        *self == STATE_MASKR::FE
    }
    #[doc = "Checks if the value of the field is `LS`"]
    #[inline]
    pub fn is_ls(&self) -> bool {
        *self == STATE_MASKR::LS
    }
    #[doc = "Checks if the value of the field is `LB`"]
    #[inline]
    pub fn is_lb(&self) -> bool {
        *self == STATE_MASKR::LB
    }
    #[doc = "Checks if the value of the field is `LD`"]
    #[inline]
    pub fn is_ld(&self) -> bool {
        *self == STATE_MASKR::LD
    }
    #[doc = "Checks if the value of the field is `LE`"]
    #[inline]
    pub fn is_le(&self) -> bool {
        *self == STATE_MASKR::LE
    }
}
#[doc = r" Proxy"]
pub struct _ENW<'a> {
    w: &'a mut W,
}
impl<'a> _ENW<'a> {
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
#[doc = "Values that can be written to the field `POL`"]
pub enum POLW {
    #[doc = "Normal Signal (Active high)"]
    ACTIVE_HIGH,
    #[doc = "Inverted signal (Active low)"]
    ACTIVE_LOW,
}
impl POLW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            POLW::ACTIVE_HIGH => false,
            POLW::ACTIVE_LOW => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _POLW<'a> {
    w: &'a mut W,
}
impl<'a> _POLW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: POLW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Normal Signal (Active high)"]
    #[inline]
    pub fn active_high(self) -> &'a mut W {
        self.variant(POLW::ACTIVE_HIGH)
    }
    #[doc = "Inverted signal (Active low)"]
    #[inline]
    pub fn active_low(self) -> &'a mut W {
        self.variant(POLW::ACTIVE_LOW)
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
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `INC_SEL`"]
pub enum INC_SELW {
    #[doc = "pclk"]
    PCLK,
    #[doc = "Line start pulse"]
    LINE,
    #[doc = "Frame start pulse"]
    FRAME,
    #[doc = "Use another signal as tick event"]
    SIG_ANOTHER,
}
impl INC_SELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            INC_SELW::PCLK => 0,
            INC_SELW::LINE => 1,
            INC_SELW::FRAME => 2,
            INC_SELW::SIG_ANOTHER => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _INC_SELW<'a> {
    w: &'a mut W,
}
impl<'a> _INC_SELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: INC_SELW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "pclk"]
    #[inline]
    pub fn pclk(self) -> &'a mut W {
        self.variant(INC_SELW::PCLK)
    }
    #[doc = "Line start pulse"]
    #[inline]
    pub fn line(self) -> &'a mut W {
        self.variant(INC_SELW::LINE)
    }
    #[doc = "Frame start pulse"]
    #[inline]
    pub fn frame(self) -> &'a mut W {
        self.variant(INC_SELW::FRAME)
    }
    #[doc = "Use another signal as tick event"]
    #[inline]
    pub fn sig_another(self) -> &'a mut W {
        self.variant(INC_SELW::SIG_ANOTHER)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 2;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _OFFSETW<'a> {
    w: &'a mut W,
}
impl<'a> _OFFSETW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 4;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `MASK_CNT_SEL`"]
pub enum MASK_CNT_SELW {
    #[doc = "pclk counter within one hscan state"]
    HSTATE_CNT,
    #[doc = "pclk cycle within one hscan state"]
    HSTATE_CYCLE,
    #[doc = "line counter within one vscan state"]
    VSTATE_CNT,
    #[doc = "line cycle within one vscan state"]
    VSTATE_CYCLE,
    #[doc = "frame counter"]
    FRAME_CNT,
    #[doc = "frame cycle"]
    FRAME_CYCLE,
    #[doc = "horizontal counter (pclk counter within one line )"]
    HCNT,
    #[doc = "vertical counter (line counter within one frame)"]
    VCNT,
}
impl MASK_CNT_SELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            MASK_CNT_SELW::HSTATE_CNT => 0,
            MASK_CNT_SELW::HSTATE_CYCLE => 1,
            MASK_CNT_SELW::VSTATE_CNT => 2,
            MASK_CNT_SELW::VSTATE_CYCLE => 3,
            MASK_CNT_SELW::FRAME_CNT => 4,
            MASK_CNT_SELW::FRAME_CYCLE => 5,
            MASK_CNT_SELW::HCNT => 6,
            MASK_CNT_SELW::VCNT => 7,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MASK_CNT_SELW<'a> {
    w: &'a mut W,
}
impl<'a> _MASK_CNT_SELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MASK_CNT_SELW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "pclk counter within one hscan state"]
    #[inline]
    pub fn hstate_cnt(self) -> &'a mut W {
        self.variant(MASK_CNT_SELW::HSTATE_CNT)
    }
    #[doc = "pclk cycle within one hscan state"]
    #[inline]
    pub fn hstate_cycle(self) -> &'a mut W {
        self.variant(MASK_CNT_SELW::HSTATE_CYCLE)
    }
    #[doc = "line counter within one vscan state"]
    #[inline]
    pub fn vstate_cnt(self) -> &'a mut W {
        self.variant(MASK_CNT_SELW::VSTATE_CNT)
    }
    #[doc = "line cycle within one vscan state"]
    #[inline]
    pub fn vstate_cycle(self) -> &'a mut W {
        self.variant(MASK_CNT_SELW::VSTATE_CYCLE)
    }
    #[doc = "frame counter"]
    #[inline]
    pub fn frame_cnt(self) -> &'a mut W {
        self.variant(MASK_CNT_SELW::FRAME_CNT)
    }
    #[doc = "frame cycle"]
    #[inline]
    pub fn frame_cycle(self) -> &'a mut W {
        self.variant(MASK_CNT_SELW::FRAME_CYCLE)
    }
    #[doc = "horizontal counter (pclk counter within one line )"]
    #[inline]
    pub fn hcnt(self) -> &'a mut W {
        self.variant(MASK_CNT_SELW::HCNT)
    }
    #[doc = "vertical counter (line counter within one frame)"]
    #[inline]
    pub fn vcnt(self) -> &'a mut W {
        self.variant(MASK_CNT_SELW::VCNT)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _MASK_CNTW<'a> {
    w: &'a mut W,
}
impl<'a> _MASK_CNTW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        const MASK: u16 = 4095;
        const OFFSET: u8 = 12;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `STATE_MASK`"]
pub enum STATE_MASKW {
    #[doc = "FRAME SYNC"]
    FS,
    #[doc = "FRAME BEGIN"]
    FB,
    #[doc = "FRAME DATA"]
    FD,
    #[doc = "FRAME END"]
    FE,
    #[doc = "LINE SYNC"]
    LS,
    #[doc = "LINE BEGIN"]
    LB,
    #[doc = "LINE DATA"]
    LD,
    #[doc = "LINE END"]
    LE,
}
impl STATE_MASKW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            STATE_MASKW::FS => 1,
            STATE_MASKW::FB => 2,
            STATE_MASKW::FD => 4,
            STATE_MASKW::FE => 8,
            STATE_MASKW::LS => 16,
            STATE_MASKW::LB => 32,
            STATE_MASKW::LD => 64,
            STATE_MASKW::LE => 128,
        }
    }
}
#[doc = r" Proxy"]
pub struct _STATE_MASKW<'a> {
    w: &'a mut W,
}
impl<'a> _STATE_MASKW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: STATE_MASKW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "FRAME SYNC"]
    #[inline]
    pub fn fs(self) -> &'a mut W {
        self.variant(STATE_MASKW::FS)
    }
    #[doc = "FRAME BEGIN"]
    #[inline]
    pub fn fb(self) -> &'a mut W {
        self.variant(STATE_MASKW::FB)
    }
    #[doc = "FRAME DATA"]
    #[inline]
    pub fn fd(self) -> &'a mut W {
        self.variant(STATE_MASKW::FD)
    }
    #[doc = "FRAME END"]
    #[inline]
    pub fn fe(self) -> &'a mut W {
        self.variant(STATE_MASKW::FE)
    }
    #[doc = "LINE SYNC"]
    #[inline]
    pub fn ls(self) -> &'a mut W {
        self.variant(STATE_MASKW::LS)
    }
    #[doc = "LINE BEGIN"]
    #[inline]
    pub fn lb(self) -> &'a mut W {
        self.variant(STATE_MASKW::LB)
    }
    #[doc = "LINE DATA"]
    #[inline]
    pub fn ld(self) -> &'a mut W {
        self.variant(STATE_MASKW::LD)
    }
    #[doc = "LINE END"]
    #[inline]
    pub fn le(self) -> &'a mut W {
        self.variant(STATE_MASKW::LE)
    }
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
    #[doc = "Bit 0 - Enable pigeon Mode on this signal"]
    #[inline]
    pub fn en(&self) -> ENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        ENR { bits }
    }
    #[doc = "Bit 1 - Polarity of signal output"]
    #[inline]
    pub fn pol(&self) -> POLR {
        POLR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 2:3 - Event to incrment local counter"]
    #[inline]
    pub fn inc_sel(&self) -> INC_SELR {
        INC_SELR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 4:7 - offset on pclk unit"]
    #[inline]
    pub fn offset(&self) -> OFFSETR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        OFFSETR { bits }
    }
    #[doc = "Bits 8:11 - select global counters as mask condition, use together with MASK_CNT"]
    #[inline]
    pub fn mask_cnt_sel(&self) -> MASK_CNT_SELR {
        MASK_CNT_SELR::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 12:23 - When the global counter selected through MASK_CNT_SEL matches value in this reg, pigeon local counter start ticking"]
    #[inline]
    pub fn mask_cnt(&self) -> MASK_CNTR {
        let bits = {
            const MASK: u16 = 4095;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        };
        MASK_CNTR { bits }
    }
    #[doc = "Bits 24:31 - state_mask = (FS|FB|FD|FE) and (LS|LB|LD|LE) , select any combination of scan states as reference point for local counter to start ticking"]
    #[inline]
    pub fn state_mask(&self) -> STATE_MASKR {
        STATE_MASKR::_from({
            const MASK: u8 = 255;
            const OFFSET: u8 = 24;
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
    #[doc = "Bit 0 - Enable pigeon Mode on this signal"]
    #[inline]
    pub fn en(&mut self) -> _ENW {
        _ENW { w: self }
    }
    #[doc = "Bit 1 - Polarity of signal output"]
    #[inline]
    pub fn pol(&mut self) -> _POLW {
        _POLW { w: self }
    }
    #[doc = "Bits 2:3 - Event to incrment local counter"]
    #[inline]
    pub fn inc_sel(&mut self) -> _INC_SELW {
        _INC_SELW { w: self }
    }
    #[doc = "Bits 4:7 - offset on pclk unit"]
    #[inline]
    pub fn offset(&mut self) -> _OFFSETW {
        _OFFSETW { w: self }
    }
    #[doc = "Bits 8:11 - select global counters as mask condition, use together with MASK_CNT"]
    #[inline]
    pub fn mask_cnt_sel(&mut self) -> _MASK_CNT_SELW {
        _MASK_CNT_SELW { w: self }
    }
    #[doc = "Bits 12:23 - When the global counter selected through MASK_CNT_SEL matches value in this reg, pigeon local counter start ticking"]
    #[inline]
    pub fn mask_cnt(&mut self) -> _MASK_CNTW {
        _MASK_CNTW { w: self }
    }
    #[doc = "Bits 24:31 - state_mask = (FS|FB|FD|FE) and (LS|LB|LD|LE) , select any combination of scan states as reference point for local counter to start ticking"]
    #[inline]
    pub fn state_mask(&mut self) -> _STATE_MASKW {
        _STATE_MASKW { w: self }
    }
}
