#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::VEND_SPEC {
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
#[doc = "Possible values of the field `VSELECT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VSELECTR {
    #[doc = "Change the voltage to high voltage range, around 3.0 V"]
    VSELECT_0,
    #[doc = "Change the voltage to low voltage range, around 1.8 V"]
    VSELECT_1,
}
impl VSELECTR {
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
            VSELECTR::VSELECT_0 => false,
            VSELECTR::VSELECT_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> VSELECTR {
        match value {
            false => VSELECTR::VSELECT_0,
            true => VSELECTR::VSELECT_1,
        }
    }
    #[doc = "Checks if the value of the field is `VSELECT_0`"]
    #[inline]
    pub fn is_vselect_0(&self) -> bool {
        *self == VSELECTR::VSELECT_0
    }
    #[doc = "Checks if the value of the field is `VSELECT_1`"]
    #[inline]
    pub fn is_vselect_1(&self) -> bool {
        *self == VSELECTR::VSELECT_1
    }
}
#[doc = "Possible values of the field `CONFLICT_CHK_EN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CONFLICT_CHK_ENR {
    #[doc = "Conflict check disable"]
    CONFLICT_CHK_EN_0,
    #[doc = "Conflict check enable"]
    CONFLICT_CHK_EN_1,
}
impl CONFLICT_CHK_ENR {
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
            CONFLICT_CHK_ENR::CONFLICT_CHK_EN_0 => false,
            CONFLICT_CHK_ENR::CONFLICT_CHK_EN_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CONFLICT_CHK_ENR {
        match value {
            false => CONFLICT_CHK_ENR::CONFLICT_CHK_EN_0,
            true => CONFLICT_CHK_ENR::CONFLICT_CHK_EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `CONFLICT_CHK_EN_0`"]
    #[inline]
    pub fn is_conflict_chk_en_0(&self) -> bool {
        *self == CONFLICT_CHK_ENR::CONFLICT_CHK_EN_0
    }
    #[doc = "Checks if the value of the field is `CONFLICT_CHK_EN_1`"]
    #[inline]
    pub fn is_conflict_chk_en_1(&self) -> bool {
        *self == CONFLICT_CHK_ENR::CONFLICT_CHK_EN_1
    }
}
#[doc = "Possible values of the field `AC12_WR_CHKBUSY_EN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AC12_WR_CHKBUSY_ENR {
    #[doc = "Do not check busy after auto CMD12 for write data packet"]
    AC12_WR_CHKBUSY_EN_0,
    #[doc = "Check busy after auto CMD12 for write data packet"]
    AC12_WR_CHKBUSY_EN_1,
}
impl AC12_WR_CHKBUSY_ENR {
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
            AC12_WR_CHKBUSY_ENR::AC12_WR_CHKBUSY_EN_0 => false,
            AC12_WR_CHKBUSY_ENR::AC12_WR_CHKBUSY_EN_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> AC12_WR_CHKBUSY_ENR {
        match value {
            false => AC12_WR_CHKBUSY_ENR::AC12_WR_CHKBUSY_EN_0,
            true => AC12_WR_CHKBUSY_ENR::AC12_WR_CHKBUSY_EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `AC12_WR_CHKBUSY_EN_0`"]
    #[inline]
    pub fn is_ac12_wr_chkbusy_en_0(&self) -> bool {
        *self == AC12_WR_CHKBUSY_ENR::AC12_WR_CHKBUSY_EN_0
    }
    #[doc = "Checks if the value of the field is `AC12_WR_CHKBUSY_EN_1`"]
    #[inline]
    pub fn is_ac12_wr_chkbusy_en_1(&self) -> bool {
        *self == AC12_WR_CHKBUSY_ENR::AC12_WR_CHKBUSY_EN_1
    }
}
#[doc = "Possible values of the field `FRC_SDCLK_ON`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FRC_SDCLK_ONR {
    #[doc = "CLK active or inactive is fully controlled by the hardware."]
    FRC_SDCLK_ON_0,
    #[doc = "Force CLK active."]
    FRC_SDCLK_ON_1,
}
impl FRC_SDCLK_ONR {
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
            FRC_SDCLK_ONR::FRC_SDCLK_ON_0 => false,
            FRC_SDCLK_ONR::FRC_SDCLK_ON_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> FRC_SDCLK_ONR {
        match value {
            false => FRC_SDCLK_ONR::FRC_SDCLK_ON_0,
            true => FRC_SDCLK_ONR::FRC_SDCLK_ON_1,
        }
    }
    #[doc = "Checks if the value of the field is `FRC_SDCLK_ON_0`"]
    #[inline]
    pub fn is_frc_sdclk_on_0(&self) -> bool {
        *self == FRC_SDCLK_ONR::FRC_SDCLK_ON_0
    }
    #[doc = "Checks if the value of the field is `FRC_SDCLK_ON_1`"]
    #[inline]
    pub fn is_frc_sdclk_on_1(&self) -> bool {
        *self == FRC_SDCLK_ONR::FRC_SDCLK_ON_1
    }
}
#[doc = "Possible values of the field `CRC_CHK_DIS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CRC_CHK_DISR {
    #[doc = "Check CRC16 for every read data packet and check CRC bits for every write data packet"]
    CRC_CHK_DIS_0,
    #[doc = "Ignore CRC16 check for every read data packet and ignore CRC bits check for every write data packet"]
    CRC_CHK_DIS_1,
}
impl CRC_CHK_DISR {
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
            CRC_CHK_DISR::CRC_CHK_DIS_0 => false,
            CRC_CHK_DISR::CRC_CHK_DIS_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CRC_CHK_DISR {
        match value {
            false => CRC_CHK_DISR::CRC_CHK_DIS_0,
            true => CRC_CHK_DISR::CRC_CHK_DIS_1,
        }
    }
    #[doc = "Checks if the value of the field is `CRC_CHK_DIS_0`"]
    #[inline]
    pub fn is_crc_chk_dis_0(&self) -> bool {
        *self == CRC_CHK_DISR::CRC_CHK_DIS_0
    }
    #[doc = "Checks if the value of the field is `CRC_CHK_DIS_1`"]
    #[inline]
    pub fn is_crc_chk_dis_1(&self) -> bool {
        *self == CRC_CHK_DISR::CRC_CHK_DIS_1
    }
}
#[doc = "Possible values of the field `CMD_BYTE_EN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CMD_BYTE_ENR {
    #[doc = "Disable"]
    CMD_BYTE_EN_0,
    #[doc = "Enable"]
    CMD_BYTE_EN_1,
}
impl CMD_BYTE_ENR {
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
            CMD_BYTE_ENR::CMD_BYTE_EN_0 => false,
            CMD_BYTE_ENR::CMD_BYTE_EN_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CMD_BYTE_ENR {
        match value {
            false => CMD_BYTE_ENR::CMD_BYTE_EN_0,
            true => CMD_BYTE_ENR::CMD_BYTE_EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `CMD_BYTE_EN_0`"]
    #[inline]
    pub fn is_cmd_byte_en_0(&self) -> bool {
        *self == CMD_BYTE_ENR::CMD_BYTE_EN_0
    }
    #[doc = "Checks if the value of the field is `CMD_BYTE_EN_1`"]
    #[inline]
    pub fn is_cmd_byte_en_1(&self) -> bool {
        *self == CMD_BYTE_ENR::CMD_BYTE_EN_1
    }
}
#[doc = "Values that can be written to the field `VSELECT`"]
pub enum VSELECTW {
    #[doc = "Change the voltage to high voltage range, around 3.0 V"]
    VSELECT_0,
    #[doc = "Change the voltage to low voltage range, around 1.8 V"]
    VSELECT_1,
}
impl VSELECTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            VSELECTW::VSELECT_0 => false,
            VSELECTW::VSELECT_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _VSELECTW<'a> {
    w: &'a mut W,
}
impl<'a> _VSELECTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: VSELECTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Change the voltage to high voltage range, around 3.0 V"]
    #[inline]
    pub fn vselect_0(self) -> &'a mut W {
        self.variant(VSELECTW::VSELECT_0)
    }
    #[doc = "Change the voltage to low voltage range, around 1.8 V"]
    #[inline]
    pub fn vselect_1(self) -> &'a mut W {
        self.variant(VSELECTW::VSELECT_1)
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
#[doc = "Values that can be written to the field `CONFLICT_CHK_EN`"]
pub enum CONFLICT_CHK_ENW {
    #[doc = "Conflict check disable"]
    CONFLICT_CHK_EN_0,
    #[doc = "Conflict check enable"]
    CONFLICT_CHK_EN_1,
}
impl CONFLICT_CHK_ENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CONFLICT_CHK_ENW::CONFLICT_CHK_EN_0 => false,
            CONFLICT_CHK_ENW::CONFLICT_CHK_EN_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CONFLICT_CHK_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _CONFLICT_CHK_ENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CONFLICT_CHK_ENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Conflict check disable"]
    #[inline]
    pub fn conflict_chk_en_0(self) -> &'a mut W {
        self.variant(CONFLICT_CHK_ENW::CONFLICT_CHK_EN_0)
    }
    #[doc = "Conflict check enable"]
    #[inline]
    pub fn conflict_chk_en_1(self) -> &'a mut W {
        self.variant(CONFLICT_CHK_ENW::CONFLICT_CHK_EN_1)
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
#[doc = "Values that can be written to the field `AC12_WR_CHKBUSY_EN`"]
pub enum AC12_WR_CHKBUSY_ENW {
    #[doc = "Do not check busy after auto CMD12 for write data packet"]
    AC12_WR_CHKBUSY_EN_0,
    #[doc = "Check busy after auto CMD12 for write data packet"]
    AC12_WR_CHKBUSY_EN_1,
}
impl AC12_WR_CHKBUSY_ENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            AC12_WR_CHKBUSY_ENW::AC12_WR_CHKBUSY_EN_0 => false,
            AC12_WR_CHKBUSY_ENW::AC12_WR_CHKBUSY_EN_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _AC12_WR_CHKBUSY_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _AC12_WR_CHKBUSY_ENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: AC12_WR_CHKBUSY_ENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Do not check busy after auto CMD12 for write data packet"]
    #[inline]
    pub fn ac12_wr_chkbusy_en_0(self) -> &'a mut W {
        self.variant(AC12_WR_CHKBUSY_ENW::AC12_WR_CHKBUSY_EN_0)
    }
    #[doc = "Check busy after auto CMD12 for write data packet"]
    #[inline]
    pub fn ac12_wr_chkbusy_en_1(self) -> &'a mut W {
        self.variant(AC12_WR_CHKBUSY_ENW::AC12_WR_CHKBUSY_EN_1)
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
#[doc = "Values that can be written to the field `FRC_SDCLK_ON`"]
pub enum FRC_SDCLK_ONW {
    #[doc = "CLK active or inactive is fully controlled by the hardware."]
    FRC_SDCLK_ON_0,
    #[doc = "Force CLK active."]
    FRC_SDCLK_ON_1,
}
impl FRC_SDCLK_ONW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            FRC_SDCLK_ONW::FRC_SDCLK_ON_0 => false,
            FRC_SDCLK_ONW::FRC_SDCLK_ON_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FRC_SDCLK_ONW<'a> {
    w: &'a mut W,
}
impl<'a> _FRC_SDCLK_ONW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FRC_SDCLK_ONW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "CLK active or inactive is fully controlled by the hardware."]
    #[inline]
    pub fn frc_sdclk_on_0(self) -> &'a mut W {
        self.variant(FRC_SDCLK_ONW::FRC_SDCLK_ON_0)
    }
    #[doc = "Force CLK active."]
    #[inline]
    pub fn frc_sdclk_on_1(self) -> &'a mut W {
        self.variant(FRC_SDCLK_ONW::FRC_SDCLK_ON_1)
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
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `CRC_CHK_DIS`"]
pub enum CRC_CHK_DISW {
    #[doc = "Check CRC16 for every read data packet and check CRC bits for every write data packet"]
    CRC_CHK_DIS_0,
    #[doc = "Ignore CRC16 check for every read data packet and ignore CRC bits check for every write data packet"]
    CRC_CHK_DIS_1,
}
impl CRC_CHK_DISW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CRC_CHK_DISW::CRC_CHK_DIS_0 => false,
            CRC_CHK_DISW::CRC_CHK_DIS_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CRC_CHK_DISW<'a> {
    w: &'a mut W,
}
impl<'a> _CRC_CHK_DISW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CRC_CHK_DISW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Check CRC16 for every read data packet and check CRC bits for every write data packet"]
    #[inline]
    pub fn crc_chk_dis_0(self) -> &'a mut W {
        self.variant(CRC_CHK_DISW::CRC_CHK_DIS_0)
    }
    #[doc = "Ignore CRC16 check for every read data packet and ignore CRC bits check for every write data packet"]
    #[inline]
    pub fn crc_chk_dis_1(self) -> &'a mut W {
        self.variant(CRC_CHK_DISW::CRC_CHK_DIS_1)
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
#[doc = "Values that can be written to the field `CMD_BYTE_EN`"]
pub enum CMD_BYTE_ENW {
    #[doc = "Disable"]
    CMD_BYTE_EN_0,
    #[doc = "Enable"]
    CMD_BYTE_EN_1,
}
impl CMD_BYTE_ENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CMD_BYTE_ENW::CMD_BYTE_EN_0 => false,
            CMD_BYTE_ENW::CMD_BYTE_EN_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CMD_BYTE_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _CMD_BYTE_ENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CMD_BYTE_ENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable"]
    #[inline]
    pub fn cmd_byte_en_0(self) -> &'a mut W {
        self.variant(CMD_BYTE_ENW::CMD_BYTE_EN_0)
    }
    #[doc = "Enable"]
    #[inline]
    pub fn cmd_byte_en_1(self) -> &'a mut W {
        self.variant(CMD_BYTE_ENW::CMD_BYTE_EN_1)
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
    #[doc = "Bit 1 - Voltage Selection"]
    #[inline]
    pub fn vselect(&self) -> VSELECTR {
        VSELECTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - Conflict check enable."]
    #[inline]
    pub fn conflict_chk_en(&self) -> CONFLICT_CHK_ENR {
        CONFLICT_CHK_ENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - AC12_WR_CHKBUSY_EN"]
    #[inline]
    pub fn ac12_wr_chkbusy_en(&self) -> AC12_WR_CHKBUSY_ENR {
        AC12_WR_CHKBUSY_ENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 8 - FRC_SDCLK_ON"]
    #[inline]
    pub fn frc_sdclk_on(&self) -> FRC_SDCLK_ONR {
        FRC_SDCLK_ONR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 15 - CRC Check Disable"]
    #[inline]
    pub fn crc_chk_dis(&self) -> CRC_CHK_DISR {
        CRC_CHK_DISR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 15;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 31 - CMD_BYTE_EN"]
    #[inline]
    pub fn cmd_byte_en(&self) -> CMD_BYTE_ENR {
        CMD_BYTE_ENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 31;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 536901641 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 1 - Voltage Selection"]
    #[inline]
    pub fn vselect(&mut self) -> _VSELECTW {
        _VSELECTW { w: self }
    }
    #[doc = "Bit 2 - Conflict check enable."]
    #[inline]
    pub fn conflict_chk_en(&mut self) -> _CONFLICT_CHK_ENW {
        _CONFLICT_CHK_ENW { w: self }
    }
    #[doc = "Bit 3 - AC12_WR_CHKBUSY_EN"]
    #[inline]
    pub fn ac12_wr_chkbusy_en(&mut self) -> _AC12_WR_CHKBUSY_ENW {
        _AC12_WR_CHKBUSY_ENW { w: self }
    }
    #[doc = "Bit 8 - FRC_SDCLK_ON"]
    #[inline]
    pub fn frc_sdclk_on(&mut self) -> _FRC_SDCLK_ONW {
        _FRC_SDCLK_ONW { w: self }
    }
    #[doc = "Bit 15 - CRC Check Disable"]
    #[inline]
    pub fn crc_chk_dis(&mut self) -> _CRC_CHK_DISW {
        _CRC_CHK_DISW { w: self }
    }
    #[doc = "Bit 31 - CMD_BYTE_EN"]
    #[inline]
    pub fn cmd_byte_en(&mut self) -> _CMD_BYTE_ENW {
        _CMD_BYTE_ENW { w: self }
    }
}
