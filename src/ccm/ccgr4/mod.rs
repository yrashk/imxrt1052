#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CCGR4 {
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
pub struct CG0R {
    bits: u8,
}
impl CG0R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct CG1R {
    bits: u8,
}
impl CG1R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct CG2R {
    bits: u8,
}
impl CG2R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct CG3R {
    bits: u8,
}
impl CG3R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct CG4R {
    bits: u8,
}
impl CG4R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct CG5R {
    bits: u8,
}
impl CG5R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct CG6R {
    bits: u8,
}
impl CG6R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct CG7R {
    bits: u8,
}
impl CG7R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct CG8R {
    bits: u8,
}
impl CG8R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct CG9R {
    bits: u8,
}
impl CG9R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct CG10R {
    bits: u8,
}
impl CG10R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct CG11R {
    bits: u8,
}
impl CG11R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct CG12R {
    bits: u8,
}
impl CG12R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct CG13R {
    bits: u8,
}
impl CG13R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct CG14R {
    bits: u8,
}
impl CG14R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct CG15R {
    bits: u8,
}
impl CG15R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Proxy"]
pub struct _CG0W<'a> {
    w: &'a mut W,
}
impl<'a> _CG0W<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _CG1W<'a> {
    w: &'a mut W,
}
impl<'a> _CG1W<'a> {
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
#[doc = r" Proxy"]
pub struct _CG2W<'a> {
    w: &'a mut W,
}
impl<'a> _CG2W<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 4;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _CG3W<'a> {
    w: &'a mut W,
}
impl<'a> _CG3W<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 6;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _CG4W<'a> {
    w: &'a mut W,
}
impl<'a> _CG4W<'a> {
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
pub struct _CG5W<'a> {
    w: &'a mut W,
}
impl<'a> _CG5W<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 10;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _CG6W<'a> {
    w: &'a mut W,
}
impl<'a> _CG6W<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 12;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _CG7W<'a> {
    w: &'a mut W,
}
impl<'a> _CG7W<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 14;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _CG8W<'a> {
    w: &'a mut W,
}
impl<'a> _CG8W<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 16;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _CG9W<'a> {
    w: &'a mut W,
}
impl<'a> _CG9W<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 18;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _CG10W<'a> {
    w: &'a mut W,
}
impl<'a> _CG10W<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 20;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _CG11W<'a> {
    w: &'a mut W,
}
impl<'a> _CG11W<'a> {
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
pub struct _CG12W<'a> {
    w: &'a mut W,
}
impl<'a> _CG12W<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 24;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _CG13W<'a> {
    w: &'a mut W,
}
impl<'a> _CG13W<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 26;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _CG14W<'a> {
    w: &'a mut W,
}
impl<'a> _CG14W<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 28;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _CG15W<'a> {
    w: &'a mut W,
}
impl<'a> _CG15W<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 30;
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
    #[doc = "Bits 0:1 - Reserved"]
    #[inline]
    pub fn cg0(&self) -> CG0R {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        CG0R { bits }
    }
    #[doc = "Bits 2:3 - iomuxc clock (iomuxc_clk_enable)"]
    #[inline]
    pub fn cg1(&self) -> CG1R {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        CG1R { bits }
    }
    #[doc = "Bits 4:5 - iomuxc gpr clock (iomuxc_gpr_clk_enable)"]
    #[inline]
    pub fn cg2(&self) -> CG2R {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        CG2R { bits }
    }
    #[doc = "Bits 6:7 - bee clock(bee_clk_enable)"]
    #[inline]
    pub fn cg3(&self) -> CG3R {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        CG3R { bits }
    }
    #[doc = "Bits 8:9 - sim_m7 clock (sim_m7_clk_enable)"]
    #[inline]
    pub fn cg4(&self) -> CG4R {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        CG4R { bits }
    }
    #[doc = "Bits 10:11 - tsc_dig clock (tsc_clk_enable)"]
    #[inline]
    pub fn cg5(&self) -> CG5R {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        CG5R { bits }
    }
    #[doc = "Bits 12:13 - sim_m clocks (sim_m_clk_enable)"]
    #[inline]
    pub fn cg6(&self) -> CG6R {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        CG6R { bits }
    }
    #[doc = "Bits 14:15 - sim_ems clocks (sim_ems_clk_enable)"]
    #[inline]
    pub fn cg7(&self) -> CG7R {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        CG7R { bits }
    }
    #[doc = "Bits 16:17 - pwm1 clocks (pwm1_clk_enable)"]
    #[inline]
    pub fn cg8(&self) -> CG8R {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        CG8R { bits }
    }
    #[doc = "Bits 18:19 - pwm2 clocks (pwm2_clk_enable)"]
    #[inline]
    pub fn cg9(&self) -> CG9R {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        CG9R { bits }
    }
    #[doc = "Bits 20:21 - pwm3 clocks (pwm3_clk_enable)"]
    #[inline]
    pub fn cg10(&self) -> CG10R {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        CG10R { bits }
    }
    #[doc = "Bits 22:23 - pwm4 clocks (pwm4_clk_enable)"]
    #[inline]
    pub fn cg11(&self) -> CG11R {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 22;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        CG11R { bits }
    }
    #[doc = "Bits 24:25 - enc1 clocks (enc1_clk_enable)"]
    #[inline]
    pub fn cg12(&self) -> CG12R {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        CG12R { bits }
    }
    #[doc = "Bits 26:27 - enc2 clocks (enc2_clk_enable)"]
    #[inline]
    pub fn cg13(&self) -> CG13R {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 26;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        CG13R { bits }
    }
    #[doc = "Bits 28:29 - enc3 clocks (enc3_clk_enable)"]
    #[inline]
    pub fn cg14(&self) -> CG14R {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 28;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        CG14R { bits }
    }
    #[doc = "Bits 30:31 - enc4 clocks (enc4_clk_enable)"]
    #[inline]
    pub fn cg15(&self) -> CG15R {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 30;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        CG15R { bits }
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 4294967295 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:1 - Reserved"]
    #[inline]
    pub fn cg0(&mut self) -> _CG0W {
        _CG0W { w: self }
    }
    #[doc = "Bits 2:3 - iomuxc clock (iomuxc_clk_enable)"]
    #[inline]
    pub fn cg1(&mut self) -> _CG1W {
        _CG1W { w: self }
    }
    #[doc = "Bits 4:5 - iomuxc gpr clock (iomuxc_gpr_clk_enable)"]
    #[inline]
    pub fn cg2(&mut self) -> _CG2W {
        _CG2W { w: self }
    }
    #[doc = "Bits 6:7 - bee clock(bee_clk_enable)"]
    #[inline]
    pub fn cg3(&mut self) -> _CG3W {
        _CG3W { w: self }
    }
    #[doc = "Bits 8:9 - sim_m7 clock (sim_m7_clk_enable)"]
    #[inline]
    pub fn cg4(&mut self) -> _CG4W {
        _CG4W { w: self }
    }
    #[doc = "Bits 10:11 - tsc_dig clock (tsc_clk_enable)"]
    #[inline]
    pub fn cg5(&mut self) -> _CG5W {
        _CG5W { w: self }
    }
    #[doc = "Bits 12:13 - sim_m clocks (sim_m_clk_enable)"]
    #[inline]
    pub fn cg6(&mut self) -> _CG6W {
        _CG6W { w: self }
    }
    #[doc = "Bits 14:15 - sim_ems clocks (sim_ems_clk_enable)"]
    #[inline]
    pub fn cg7(&mut self) -> _CG7W {
        _CG7W { w: self }
    }
    #[doc = "Bits 16:17 - pwm1 clocks (pwm1_clk_enable)"]
    #[inline]
    pub fn cg8(&mut self) -> _CG8W {
        _CG8W { w: self }
    }
    #[doc = "Bits 18:19 - pwm2 clocks (pwm2_clk_enable)"]
    #[inline]
    pub fn cg9(&mut self) -> _CG9W {
        _CG9W { w: self }
    }
    #[doc = "Bits 20:21 - pwm3 clocks (pwm3_clk_enable)"]
    #[inline]
    pub fn cg10(&mut self) -> _CG10W {
        _CG10W { w: self }
    }
    #[doc = "Bits 22:23 - pwm4 clocks (pwm4_clk_enable)"]
    #[inline]
    pub fn cg11(&mut self) -> _CG11W {
        _CG11W { w: self }
    }
    #[doc = "Bits 24:25 - enc1 clocks (enc1_clk_enable)"]
    #[inline]
    pub fn cg12(&mut self) -> _CG12W {
        _CG12W { w: self }
    }
    #[doc = "Bits 26:27 - enc2 clocks (enc2_clk_enable)"]
    #[inline]
    pub fn cg13(&mut self) -> _CG13W {
        _CG13W { w: self }
    }
    #[doc = "Bits 28:29 - enc3 clocks (enc3_clk_enable)"]
    #[inline]
    pub fn cg14(&mut self) -> _CG14W {
        _CG14W { w: self }
    }
    #[doc = "Bits 30:31 - enc4 clocks (enc4_clk_enable)"]
    #[inline]
    pub fn cg15(&mut self) -> _CG15W {
        _CG15W { w: self }
    }
}
