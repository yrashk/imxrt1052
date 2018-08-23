#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::IFLAG1 {
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
#[doc = "Possible values of the field `BUF4TO0I`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BUF4TO0IR {
    #[doc = "No such occurrence"]
    BUF4TO0I_0,
    #[doc = "Corresponding MB completed transmission/reception"]
    BUF4TO0I_1,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl BUF4TO0IR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            BUF4TO0IR::BUF4TO0I_0 => 0,
            BUF4TO0IR::BUF4TO0I_1 => 1,
            BUF4TO0IR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> BUF4TO0IR {
        match value {
            0 => BUF4TO0IR::BUF4TO0I_0,
            1 => BUF4TO0IR::BUF4TO0I_1,
            i => BUF4TO0IR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `BUF4TO0I_0`"]
    #[inline]
    pub fn is_buf4to0i_0(&self) -> bool {
        *self == BUF4TO0IR::BUF4TO0I_0
    }
    #[doc = "Checks if the value of the field is `BUF4TO0I_1`"]
    #[inline]
    pub fn is_buf4to0i_1(&self) -> bool {
        *self == BUF4TO0IR::BUF4TO0I_1
    }
}
#[doc = "Possible values of the field `BUF5I`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BUF5IR {
    #[doc = "No such occurrence"]
    BUF5I_0,
    #[doc = "MB5 completed transmission/reception or frames available in the FIFO"]
    BUF5I_1,
}
impl BUF5IR {
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
            BUF5IR::BUF5I_0 => false,
            BUF5IR::BUF5I_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> BUF5IR {
        match value {
            false => BUF5IR::BUF5I_0,
            true => BUF5IR::BUF5I_1,
        }
    }
    #[doc = "Checks if the value of the field is `BUF5I_0`"]
    #[inline]
    pub fn is_buf5i_0(&self) -> bool {
        *self == BUF5IR::BUF5I_0
    }
    #[doc = "Checks if the value of the field is `BUF5I_1`"]
    #[inline]
    pub fn is_buf5i_1(&self) -> bool {
        *self == BUF5IR::BUF5I_1
    }
}
#[doc = "Possible values of the field `BUF6I`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BUF6IR {
    #[doc = "No such occurrence"]
    BUF6I_0,
    #[doc = "MB6 completed transmission/reception or FIFO almost full"]
    BUF6I_1,
}
impl BUF6IR {
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
            BUF6IR::BUF6I_0 => false,
            BUF6IR::BUF6I_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> BUF6IR {
        match value {
            false => BUF6IR::BUF6I_0,
            true => BUF6IR::BUF6I_1,
        }
    }
    #[doc = "Checks if the value of the field is `BUF6I_0`"]
    #[inline]
    pub fn is_buf6i_0(&self) -> bool {
        *self == BUF6IR::BUF6I_0
    }
    #[doc = "Checks if the value of the field is `BUF6I_1`"]
    #[inline]
    pub fn is_buf6i_1(&self) -> bool {
        *self == BUF6IR::BUF6I_1
    }
}
#[doc = "Possible values of the field `BUF7I`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BUF7IR {
    #[doc = "No such occurrence"]
    BUF7I_0,
    #[doc = "MB7 completed transmission/reception or FIFO overflow"]
    BUF7I_1,
}
impl BUF7IR {
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
            BUF7IR::BUF7I_0 => false,
            BUF7IR::BUF7I_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> BUF7IR {
        match value {
            false => BUF7IR::BUF7I_0,
            true => BUF7IR::BUF7I_1,
        }
    }
    #[doc = "Checks if the value of the field is `BUF7I_0`"]
    #[inline]
    pub fn is_buf7i_0(&self) -> bool {
        *self == BUF7IR::BUF7I_0
    }
    #[doc = "Checks if the value of the field is `BUF7I_1`"]
    #[inline]
    pub fn is_buf7i_1(&self) -> bool {
        *self == BUF7IR::BUF7I_1
    }
}
#[doc = "Possible values of the field `BUF31TO8I`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BUF31TO8IR {
    #[doc = "No such occurrence"]
    BUF31TO8I_0,
    #[doc = "The corresponding MB has successfully completed transmission or reception"]
    BUF31TO8I_1,
    #[doc = r" Reserved"]
    _Reserved(u32),
}
impl BUF31TO8IR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        match *self {
            BUF31TO8IR::BUF31TO8I_0 => 0,
            BUF31TO8IR::BUF31TO8I_1 => 1,
            BUF31TO8IR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u32) -> BUF31TO8IR {
        match value {
            0 => BUF31TO8IR::BUF31TO8I_0,
            1 => BUF31TO8IR::BUF31TO8I_1,
            i => BUF31TO8IR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `BUF31TO8I_0`"]
    #[inline]
    pub fn is_buf31to8i_0(&self) -> bool {
        *self == BUF31TO8IR::BUF31TO8I_0
    }
    #[doc = "Checks if the value of the field is `BUF31TO8I_1`"]
    #[inline]
    pub fn is_buf31to8i_1(&self) -> bool {
        *self == BUF31TO8IR::BUF31TO8I_1
    }
}
#[doc = "Values that can be written to the field `BUF4TO0I`"]
pub enum BUF4TO0IW {
    #[doc = "No such occurrence"]
    BUF4TO0I_0,
    #[doc = "Corresponding MB completed transmission/reception"]
    BUF4TO0I_1,
}
impl BUF4TO0IW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            BUF4TO0IW::BUF4TO0I_0 => 0,
            BUF4TO0IW::BUF4TO0I_1 => 1,
        }
    }
}
#[doc = r" Proxy"]
pub struct _BUF4TO0IW<'a> {
    w: &'a mut W,
}
impl<'a> _BUF4TO0IW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: BUF4TO0IW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "No such occurrence"]
    #[inline]
    pub fn buf4to0i_0(self) -> &'a mut W {
        self.variant(BUF4TO0IW::BUF4TO0I_0)
    }
    #[doc = "Corresponding MB completed transmission/reception"]
    #[inline]
    pub fn buf4to0i_1(self) -> &'a mut W {
        self.variant(BUF4TO0IW::BUF4TO0I_1)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 31;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `BUF5I`"]
pub enum BUF5IW {
    #[doc = "No such occurrence"]
    BUF5I_0,
    #[doc = "MB5 completed transmission/reception or frames available in the FIFO"]
    BUF5I_1,
}
impl BUF5IW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            BUF5IW::BUF5I_0 => false,
            BUF5IW::BUF5I_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _BUF5IW<'a> {
    w: &'a mut W,
}
impl<'a> _BUF5IW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: BUF5IW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No such occurrence"]
    #[inline]
    pub fn buf5i_0(self) -> &'a mut W {
        self.variant(BUF5IW::BUF5I_0)
    }
    #[doc = "MB5 completed transmission/reception or frames available in the FIFO"]
    #[inline]
    pub fn buf5i_1(self) -> &'a mut W {
        self.variant(BUF5IW::BUF5I_1)
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
#[doc = "Values that can be written to the field `BUF6I`"]
pub enum BUF6IW {
    #[doc = "No such occurrence"]
    BUF6I_0,
    #[doc = "MB6 completed transmission/reception or FIFO almost full"]
    BUF6I_1,
}
impl BUF6IW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            BUF6IW::BUF6I_0 => false,
            BUF6IW::BUF6I_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _BUF6IW<'a> {
    w: &'a mut W,
}
impl<'a> _BUF6IW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: BUF6IW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No such occurrence"]
    #[inline]
    pub fn buf6i_0(self) -> &'a mut W {
        self.variant(BUF6IW::BUF6I_0)
    }
    #[doc = "MB6 completed transmission/reception or FIFO almost full"]
    #[inline]
    pub fn buf6i_1(self) -> &'a mut W {
        self.variant(BUF6IW::BUF6I_1)
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
#[doc = "Values that can be written to the field `BUF7I`"]
pub enum BUF7IW {
    #[doc = "No such occurrence"]
    BUF7I_0,
    #[doc = "MB7 completed transmission/reception or FIFO overflow"]
    BUF7I_1,
}
impl BUF7IW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            BUF7IW::BUF7I_0 => false,
            BUF7IW::BUF7I_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _BUF7IW<'a> {
    w: &'a mut W,
}
impl<'a> _BUF7IW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: BUF7IW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No such occurrence"]
    #[inline]
    pub fn buf7i_0(self) -> &'a mut W {
        self.variant(BUF7IW::BUF7I_0)
    }
    #[doc = "MB7 completed transmission/reception or FIFO overflow"]
    #[inline]
    pub fn buf7i_1(self) -> &'a mut W {
        self.variant(BUF7IW::BUF7I_1)
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
#[doc = "Values that can be written to the field `BUF31TO8I`"]
pub enum BUF31TO8IW {
    #[doc = "No such occurrence"]
    BUF31TO8I_0,
    #[doc = "The corresponding MB has successfully completed transmission or reception"]
    BUF31TO8I_1,
}
impl BUF31TO8IW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u32 {
        match *self {
            BUF31TO8IW::BUF31TO8I_0 => 0,
            BUF31TO8IW::BUF31TO8I_1 => 1,
        }
    }
}
#[doc = r" Proxy"]
pub struct _BUF31TO8IW<'a> {
    w: &'a mut W,
}
impl<'a> _BUF31TO8IW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: BUF31TO8IW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "No such occurrence"]
    #[inline]
    pub fn buf31to8i_0(self) -> &'a mut W {
        self.variant(BUF31TO8IW::BUF31TO8I_0)
    }
    #[doc = "The corresponding MB has successfully completed transmission or reception"]
    #[inline]
    pub fn buf31to8i_1(self) -> &'a mut W {
        self.variant(BUF31TO8IW::BUF31TO8I_1)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        const MASK: u32 = 16777215;
        const OFFSET: u8 = 8;
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
    #[doc = "Bits 0:4 - If the Rx FIFO is not enabled, these bits flag the interrupts for MB0 to MB4"]
    #[inline]
    pub fn buf4to0i(&self) -> BUF4TO0IR {
        BUF4TO0IR::_from({
            const MASK: u8 = 31;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 5 - If the Rx FIFO is not enabled, this bit flags the interrupt for MB5"]
    #[inline]
    pub fn buf5i(&self) -> BUF5IR {
        BUF5IR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 6 - If the Rx FIFO is not enabled, this bit flags the interrupt for MB6"]
    #[inline]
    pub fn buf6i(&self) -> BUF6IR {
        BUF6IR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 7 - If the Rx FIFO is not enabled, this bit flags the interrupt for MB7"]
    #[inline]
    pub fn buf7i(&self) -> BUF7IR {
        BUF7IR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 8:31 - Each bit flags the respective FLEXCAN Message Buffer (MB8 to MB31) interrupt."]
    #[inline]
    pub fn buf31to8i(&self) -> BUF31TO8IR {
        BUF31TO8IR::_from({
            const MASK: u32 = 16777215;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u32
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
    #[doc = "Bits 0:4 - If the Rx FIFO is not enabled, these bits flag the interrupts for MB0 to MB4"]
    #[inline]
    pub fn buf4to0i(&mut self) -> _BUF4TO0IW {
        _BUF4TO0IW { w: self }
    }
    #[doc = "Bit 5 - If the Rx FIFO is not enabled, this bit flags the interrupt for MB5"]
    #[inline]
    pub fn buf5i(&mut self) -> _BUF5IW {
        _BUF5IW { w: self }
    }
    #[doc = "Bit 6 - If the Rx FIFO is not enabled, this bit flags the interrupt for MB6"]
    #[inline]
    pub fn buf6i(&mut self) -> _BUF6IW {
        _BUF6IW { w: self }
    }
    #[doc = "Bit 7 - If the Rx FIFO is not enabled, this bit flags the interrupt for MB7"]
    #[inline]
    pub fn buf7i(&mut self) -> _BUF7IW {
        _BUF7IW { w: self }
    }
    #[doc = "Bits 8:31 - Each bit flags the respective FLEXCAN Message Buffer (MB8 to MB31) interrupt."]
    #[inline]
    pub fn buf31to8i(&mut self) -> _BUF31TO8IW {
        _BUF31TO8IW { w: self }
    }
}
