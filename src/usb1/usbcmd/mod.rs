#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::USBCMD {
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
pub struct RSR {
    bits: bool,
}
impl RSR {
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
pub struct RSTR {
    bits: bool,
}
impl RSTR {
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
pub struct FS_1R {
    bits: u8,
}
impl FS_1R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Possible values of the field `PSE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PSER {
    #[doc = "Do not process the Periodic Schedule"]
    PSE_0,
    #[doc = "Use the PERIODICLISTBASE register to access the Periodic Schedule."]
    PSE_1,
}
impl PSER {
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
            PSER::PSE_0 => false,
            PSER::PSE_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PSER {
        match value {
            false => PSER::PSE_0,
            true => PSER::PSE_1,
        }
    }
    #[doc = "Checks if the value of the field is `PSE_0`"]
    #[inline]
    pub fn is_pse_0(&self) -> bool {
        *self == PSER::PSE_0
    }
    #[doc = "Checks if the value of the field is `PSE_1`"]
    #[inline]
    pub fn is_pse_1(&self) -> bool {
        *self == PSER::PSE_1
    }
}
#[doc = "Possible values of the field `ASE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ASER {
    #[doc = "Do not process the Asynchronous Schedule."]
    ASE_0,
    #[doc = "Use the ASYNCLISTADDR register to access the Asynchronous Schedule."]
    ASE_1,
}
impl ASER {
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
            ASER::ASE_0 => false,
            ASER::ASE_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ASER {
        match value {
            false => ASER::ASE_0,
            true => ASER::ASE_1,
        }
    }
    #[doc = "Checks if the value of the field is `ASE_0`"]
    #[inline]
    pub fn is_ase_0(&self) -> bool {
        *self == ASER::ASE_0
    }
    #[doc = "Checks if the value of the field is `ASE_1`"]
    #[inline]
    pub fn is_ase_1(&self) -> bool {
        *self == ASER::ASE_1
    }
}
#[doc = r" Value of the field"]
pub struct IAAR {
    bits: bool,
}
impl IAAR {
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
pub struct ASPR {
    bits: u8,
}
impl ASPR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct ASPER {
    bits: bool,
}
impl ASPER {
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
pub struct ATDTWR {
    bits: bool,
}
impl ATDTWR {
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
pub struct SUTWR {
    bits: bool,
}
impl SUTWR {
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
#[doc = "Possible values of the field `FS_2`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FS_2R {
    #[doc = "1024 elements (4096 bytes) Default value"]
    FS_2_0,
    #[doc = "512 elements (2048 bytes)"]
    FS_2_1,
}
impl FS_2R {
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
            FS_2R::FS_2_0 => false,
            FS_2R::FS_2_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> FS_2R {
        match value {
            false => FS_2R::FS_2_0,
            true => FS_2R::FS_2_1,
        }
    }
    #[doc = "Checks if the value of the field is `FS_2_0`"]
    #[inline]
    pub fn is_fs_2_0(&self) -> bool {
        *self == FS_2R::FS_2_0
    }
    #[doc = "Checks if the value of the field is `FS_2_1`"]
    #[inline]
    pub fn is_fs_2_1(&self) -> bool {
        *self == FS_2R::FS_2_1
    }
}
#[doc = "Possible values of the field `ITC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ITCR {
    #[doc = "Immediate (no threshold)"]
    ITC_0,
    #[doc = "1 micro-frame"]
    ITC_1,
    #[doc = "2 micro-frames"]
    ITC_2,
    #[doc = "4 micro-frames"]
    ITC_4,
    #[doc = "8 micro-frames"]
    ITC_8,
    #[doc = "16 micro-frames"]
    ITC_16,
    #[doc = "32 micro-frames"]
    ITC_32,
    #[doc = "64 micro-frames"]
    ITC_64,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl ITCR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            ITCR::ITC_0 => 0,
            ITCR::ITC_1 => 1,
            ITCR::ITC_2 => 2,
            ITCR::ITC_4 => 4,
            ITCR::ITC_8 => 8,
            ITCR::ITC_16 => 16,
            ITCR::ITC_32 => 32,
            ITCR::ITC_64 => 64,
            ITCR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> ITCR {
        match value {
            0 => ITCR::ITC_0,
            1 => ITCR::ITC_1,
            2 => ITCR::ITC_2,
            4 => ITCR::ITC_4,
            8 => ITCR::ITC_8,
            16 => ITCR::ITC_16,
            32 => ITCR::ITC_32,
            64 => ITCR::ITC_64,
            i => ITCR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `ITC_0`"]
    #[inline]
    pub fn is_itc_0(&self) -> bool {
        *self == ITCR::ITC_0
    }
    #[doc = "Checks if the value of the field is `ITC_1`"]
    #[inline]
    pub fn is_itc_1(&self) -> bool {
        *self == ITCR::ITC_1
    }
    #[doc = "Checks if the value of the field is `ITC_2`"]
    #[inline]
    pub fn is_itc_2(&self) -> bool {
        *self == ITCR::ITC_2
    }
    #[doc = "Checks if the value of the field is `ITC_4`"]
    #[inline]
    pub fn is_itc_4(&self) -> bool {
        *self == ITCR::ITC_4
    }
    #[doc = "Checks if the value of the field is `ITC_8`"]
    #[inline]
    pub fn is_itc_8(&self) -> bool {
        *self == ITCR::ITC_8
    }
    #[doc = "Checks if the value of the field is `ITC_16`"]
    #[inline]
    pub fn is_itc_16(&self) -> bool {
        *self == ITCR::ITC_16
    }
    #[doc = "Checks if the value of the field is `ITC_32`"]
    #[inline]
    pub fn is_itc_32(&self) -> bool {
        *self == ITCR::ITC_32
    }
    #[doc = "Checks if the value of the field is `ITC_64`"]
    #[inline]
    pub fn is_itc_64(&self) -> bool {
        *self == ITCR::ITC_64
    }
}
#[doc = r" Proxy"]
pub struct _RSW<'a> {
    w: &'a mut W,
}
impl<'a> _RSW<'a> {
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
pub struct _RSTW<'a> {
    w: &'a mut W,
}
impl<'a> _RSTW<'a> {
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
pub struct _FS_1W<'a> {
    w: &'a mut W,
}
impl<'a> _FS_1W<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 2;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `PSE`"]
pub enum PSEW {
    #[doc = "Do not process the Periodic Schedule"]
    PSE_0,
    #[doc = "Use the PERIODICLISTBASE register to access the Periodic Schedule."]
    PSE_1,
}
impl PSEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PSEW::PSE_0 => false,
            PSEW::PSE_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PSEW<'a> {
    w: &'a mut W,
}
impl<'a> _PSEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PSEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Do not process the Periodic Schedule"]
    #[inline]
    pub fn pse_0(self) -> &'a mut W {
        self.variant(PSEW::PSE_0)
    }
    #[doc = "Use the PERIODICLISTBASE register to access the Periodic Schedule."]
    #[inline]
    pub fn pse_1(self) -> &'a mut W {
        self.variant(PSEW::PSE_1)
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
#[doc = "Values that can be written to the field `ASE`"]
pub enum ASEW {
    #[doc = "Do not process the Asynchronous Schedule."]
    ASE_0,
    #[doc = "Use the ASYNCLISTADDR register to access the Asynchronous Schedule."]
    ASE_1,
}
impl ASEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ASEW::ASE_0 => false,
            ASEW::ASE_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ASEW<'a> {
    w: &'a mut W,
}
impl<'a> _ASEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ASEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Do not process the Asynchronous Schedule."]
    #[inline]
    pub fn ase_0(self) -> &'a mut W {
        self.variant(ASEW::ASE_0)
    }
    #[doc = "Use the ASYNCLISTADDR register to access the Asynchronous Schedule."]
    #[inline]
    pub fn ase_1(self) -> &'a mut W {
        self.variant(ASEW::ASE_1)
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
#[doc = r" Proxy"]
pub struct _IAAW<'a> {
    w: &'a mut W,
}
impl<'a> _IAAW<'a> {
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
pub struct _ASPW<'a> {
    w: &'a mut W,
}
impl<'a> _ASPW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _ASPEW<'a> {
    w: &'a mut W,
}
impl<'a> _ASPEW<'a> {
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
pub struct _ATDTWW<'a> {
    w: &'a mut W,
}
impl<'a> _ATDTWW<'a> {
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
pub struct _SUTWW<'a> {
    w: &'a mut W,
}
impl<'a> _SUTWW<'a> {
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
#[doc = "Values that can be written to the field `FS_2`"]
pub enum FS_2W {
    #[doc = "1024 elements (4096 bytes) Default value"]
    FS_2_0,
    #[doc = "512 elements (2048 bytes)"]
    FS_2_1,
}
impl FS_2W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            FS_2W::FS_2_0 => false,
            FS_2W::FS_2_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FS_2W<'a> {
    w: &'a mut W,
}
impl<'a> _FS_2W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FS_2W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "1024 elements (4096 bytes) Default value"]
    #[inline]
    pub fn fs_2_0(self) -> &'a mut W {
        self.variant(FS_2W::FS_2_0)
    }
    #[doc = "512 elements (2048 bytes)"]
    #[inline]
    pub fn fs_2_1(self) -> &'a mut W {
        self.variant(FS_2W::FS_2_1)
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
#[doc = "Values that can be written to the field `ITC`"]
pub enum ITCW {
    #[doc = "Immediate (no threshold)"]
    ITC_0,
    #[doc = "1 micro-frame"]
    ITC_1,
    #[doc = "2 micro-frames"]
    ITC_2,
    #[doc = "4 micro-frames"]
    ITC_4,
    #[doc = "8 micro-frames"]
    ITC_8,
    #[doc = "16 micro-frames"]
    ITC_16,
    #[doc = "32 micro-frames"]
    ITC_32,
    #[doc = "64 micro-frames"]
    ITC_64,
}
impl ITCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            ITCW::ITC_0 => 0,
            ITCW::ITC_1 => 1,
            ITCW::ITC_2 => 2,
            ITCW::ITC_4 => 4,
            ITCW::ITC_8 => 8,
            ITCW::ITC_16 => 16,
            ITCW::ITC_32 => 32,
            ITCW::ITC_64 => 64,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ITCW<'a> {
    w: &'a mut W,
}
impl<'a> _ITCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ITCW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Immediate (no threshold)"]
    #[inline]
    pub fn itc_0(self) -> &'a mut W {
        self.variant(ITCW::ITC_0)
    }
    #[doc = "1 micro-frame"]
    #[inline]
    pub fn itc_1(self) -> &'a mut W {
        self.variant(ITCW::ITC_1)
    }
    #[doc = "2 micro-frames"]
    #[inline]
    pub fn itc_2(self) -> &'a mut W {
        self.variant(ITCW::ITC_2)
    }
    #[doc = "4 micro-frames"]
    #[inline]
    pub fn itc_4(self) -> &'a mut W {
        self.variant(ITCW::ITC_4)
    }
    #[doc = "8 micro-frames"]
    #[inline]
    pub fn itc_8(self) -> &'a mut W {
        self.variant(ITCW::ITC_8)
    }
    #[doc = "16 micro-frames"]
    #[inline]
    pub fn itc_16(self) -> &'a mut W {
        self.variant(ITCW::ITC_16)
    }
    #[doc = "32 micro-frames"]
    #[inline]
    pub fn itc_32(self) -> &'a mut W {
        self.variant(ITCW::ITC_32)
    }
    #[doc = "64 micro-frames"]
    #[inline]
    pub fn itc_64(self) -> &'a mut W {
        self.variant(ITCW::ITC_64)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 255;
        const OFFSET: u8 = 16;
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
    #[doc = "Bit 0 - Run/Stop (RS) - Read/Write"]
    #[inline]
    pub fn rs(&self) -> RSR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        RSR { bits }
    }
    #[doc = "Bit 1 - Controller Reset (RESET) - Read/Write"]
    #[inline]
    pub fn rst(&self) -> RSTR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        RSTR { bits }
    }
    #[doc = "Bits 2:3 - See description at bit 15"]
    #[inline]
    pub fn fs_1(&self) -> FS_1R {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        FS_1R { bits }
    }
    #[doc = "Bit 4 - Periodic Schedule Enable- Read/Write"]
    #[inline]
    pub fn pse(&self) -> PSER {
        PSER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 5 - Asynchronous Schedule Enable - Read/Write"]
    #[inline]
    pub fn ase(&self) -> ASER {
        ASER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 6 - Interrupt on Async Advance Doorbell - Read/Write"]
    #[inline]
    pub fn iaa(&self) -> IAAR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        IAAR { bits }
    }
    #[doc = "Bits 8:9 - Asynchronous Schedule Park Mode Count - Read/Write"]
    #[inline]
    pub fn asp(&self) -> ASPR {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        ASPR { bits }
    }
    #[doc = "Bit 11 - Asynchronous Schedule Park Mode Enable - Read/Write"]
    #[inline]
    pub fn aspe(&self) -> ASPER {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        ASPER { bits }
    }
    #[doc = "Bit 12 - Add dTD TripWire - Read/Write"]
    #[inline]
    pub fn atdtw(&self) -> ATDTWR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        ATDTWR { bits }
    }
    #[doc = "Bit 13 - Setup TripWire - Read/Write"]
    #[inline]
    pub fn sutw(&self) -> SUTWR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        SUTWR { bits }
    }
    #[doc = "Bit 15 - See also bits 3-2 Frame List Size - (Read/Write or Read Only)"]
    #[inline]
    pub fn fs_2(&self) -> FS_2R {
        FS_2R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 15;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 16:23 - Interrupt Threshold Control -Read/Write"]
    #[inline]
    pub fn itc(&self) -> ITCR {
        ITCR::_from({
            const MASK: u8 = 255;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 524288 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - Run/Stop (RS) - Read/Write"]
    #[inline]
    pub fn rs(&mut self) -> _RSW {
        _RSW { w: self }
    }
    #[doc = "Bit 1 - Controller Reset (RESET) - Read/Write"]
    #[inline]
    pub fn rst(&mut self) -> _RSTW {
        _RSTW { w: self }
    }
    #[doc = "Bits 2:3 - See description at bit 15"]
    #[inline]
    pub fn fs_1(&mut self) -> _FS_1W {
        _FS_1W { w: self }
    }
    #[doc = "Bit 4 - Periodic Schedule Enable- Read/Write"]
    #[inline]
    pub fn pse(&mut self) -> _PSEW {
        _PSEW { w: self }
    }
    #[doc = "Bit 5 - Asynchronous Schedule Enable - Read/Write"]
    #[inline]
    pub fn ase(&mut self) -> _ASEW {
        _ASEW { w: self }
    }
    #[doc = "Bit 6 - Interrupt on Async Advance Doorbell - Read/Write"]
    #[inline]
    pub fn iaa(&mut self) -> _IAAW {
        _IAAW { w: self }
    }
    #[doc = "Bits 8:9 - Asynchronous Schedule Park Mode Count - Read/Write"]
    #[inline]
    pub fn asp(&mut self) -> _ASPW {
        _ASPW { w: self }
    }
    #[doc = "Bit 11 - Asynchronous Schedule Park Mode Enable - Read/Write"]
    #[inline]
    pub fn aspe(&mut self) -> _ASPEW {
        _ASPEW { w: self }
    }
    #[doc = "Bit 12 - Add dTD TripWire - Read/Write"]
    #[inline]
    pub fn atdtw(&mut self) -> _ATDTWW {
        _ATDTWW { w: self }
    }
    #[doc = "Bit 13 - Setup TripWire - Read/Write"]
    #[inline]
    pub fn sutw(&mut self) -> _SUTWW {
        _SUTWW { w: self }
    }
    #[doc = "Bit 15 - See also bits 3-2 Frame List Size - (Read/Write or Read Only)"]
    #[inline]
    pub fn fs_2(&mut self) -> _FS_2W {
        _FS_2W { w: self }
    }
    #[doc = "Bits 16:23 - Interrupt Threshold Control -Read/Write"]
    #[inline]
    pub fn itc(&mut self) -> _ITCW {
        _ITCW { w: self }
    }
}
