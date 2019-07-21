#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::INTFLAG {
    #[doc = r" Modifies the contents of the register"]
    #[inline]
    pub fn modify<F>(&self, f: F)
    where
        for<'w> F: FnOnce(&R, &'w mut W) -> &'w mut W,
    {
        let bits = self.register.get();
        self.register.set(f(&R { bits }, &mut W { bits }).bits);
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
        self.register.set(
            f(&mut W {
                bits: Self::reset_value(),
            })
            .bits,
        );
    }
    #[doc = r" Reset value of the register"]
    #[inline]
    pub const fn reset_value() -> u32 {
        0
    }
    #[doc = r" Writes the reset value to the register"]
    #[inline]
    pub fn reset(&self) {
        self.register.set(Self::reset_value())
    }
}
#[doc = r" Value of the field"]
pub struct XOSCRDY0R {
    bits: bool,
}
impl XOSCRDY0R {
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
pub struct XOSCRDY1R {
    bits: bool,
}
impl XOSCRDY1R {
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
pub struct XOSCFAIL0R {
    bits: bool,
}
impl XOSCFAIL0R {
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
pub struct XOSCFAIL1R {
    bits: bool,
}
impl XOSCFAIL1R {
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
pub struct DFLLRDYR {
    bits: bool,
}
impl DFLLRDYR {
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
pub struct DFLLOOBR {
    bits: bool,
}
impl DFLLOOBR {
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
pub struct DFLLLCKFR {
    bits: bool,
}
impl DFLLLCKFR {
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
pub struct DFLLLCKCR {
    bits: bool,
}
impl DFLLLCKCR {
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
pub struct DFLLRCSR {
    bits: bool,
}
impl DFLLRCSR {
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
pub struct DPLL0LCKRR {
    bits: bool,
}
impl DPLL0LCKRR {
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
pub struct DPLL0LCKFR {
    bits: bool,
}
impl DPLL0LCKFR {
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
pub struct DPLL0LTOR {
    bits: bool,
}
impl DPLL0LTOR {
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
pub struct DPLL0LDRTOR {
    bits: bool,
}
impl DPLL0LDRTOR {
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
pub struct DPLL1LCKRR {
    bits: bool,
}
impl DPLL1LCKRR {
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
pub struct DPLL1LCKFR {
    bits: bool,
}
impl DPLL1LCKFR {
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
pub struct DPLL1LTOR {
    bits: bool,
}
impl DPLL1LTOR {
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
pub struct DPLL1LDRTOR {
    bits: bool,
}
impl DPLL1LDRTOR {
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
pub struct _XOSCRDY0W<'a> {
    w: &'a mut W,
}
impl<'a> _XOSCRDY0W<'a> {
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
        self.w.bits &= !(0x01 << 0);
        self.w.bits |= ((value as u32) & 0x01) << 0;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _XOSCRDY1W<'a> {
    w: &'a mut W,
}
impl<'a> _XOSCRDY1W<'a> {
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
        self.w.bits &= !(0x01 << 1);
        self.w.bits |= ((value as u32) & 0x01) << 1;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _XOSCFAIL0W<'a> {
    w: &'a mut W,
}
impl<'a> _XOSCFAIL0W<'a> {
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
        self.w.bits &= !(0x01 << 2);
        self.w.bits |= ((value as u32) & 0x01) << 2;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _XOSCFAIL1W<'a> {
    w: &'a mut W,
}
impl<'a> _XOSCFAIL1W<'a> {
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
        self.w.bits &= !(0x01 << 3);
        self.w.bits |= ((value as u32) & 0x01) << 3;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _DFLLRDYW<'a> {
    w: &'a mut W,
}
impl<'a> _DFLLRDYW<'a> {
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
        self.w.bits &= !(0x01 << 8);
        self.w.bits |= ((value as u32) & 0x01) << 8;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _DFLLOOBW<'a> {
    w: &'a mut W,
}
impl<'a> _DFLLOOBW<'a> {
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
        self.w.bits &= !(0x01 << 9);
        self.w.bits |= ((value as u32) & 0x01) << 9;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _DFLLLCKFW<'a> {
    w: &'a mut W,
}
impl<'a> _DFLLLCKFW<'a> {
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
        self.w.bits &= !(0x01 << 10);
        self.w.bits |= ((value as u32) & 0x01) << 10;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _DFLLLCKCW<'a> {
    w: &'a mut W,
}
impl<'a> _DFLLLCKCW<'a> {
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
        self.w.bits &= !(0x01 << 11);
        self.w.bits |= ((value as u32) & 0x01) << 11;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _DFLLRCSW<'a> {
    w: &'a mut W,
}
impl<'a> _DFLLRCSW<'a> {
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
        self.w.bits &= !(0x01 << 12);
        self.w.bits |= ((value as u32) & 0x01) << 12;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _DPLL0LCKRW<'a> {
    w: &'a mut W,
}
impl<'a> _DPLL0LCKRW<'a> {
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
        self.w.bits &= !(0x01 << 16);
        self.w.bits |= ((value as u32) & 0x01) << 16;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _DPLL0LCKFW<'a> {
    w: &'a mut W,
}
impl<'a> _DPLL0LCKFW<'a> {
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
        self.w.bits &= !(0x01 << 17);
        self.w.bits |= ((value as u32) & 0x01) << 17;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _DPLL0LTOW<'a> {
    w: &'a mut W,
}
impl<'a> _DPLL0LTOW<'a> {
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
        self.w.bits &= !(0x01 << 18);
        self.w.bits |= ((value as u32) & 0x01) << 18;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _DPLL0LDRTOW<'a> {
    w: &'a mut W,
}
impl<'a> _DPLL0LDRTOW<'a> {
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
        self.w.bits &= !(0x01 << 19);
        self.w.bits |= ((value as u32) & 0x01) << 19;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _DPLL1LCKRW<'a> {
    w: &'a mut W,
}
impl<'a> _DPLL1LCKRW<'a> {
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
        self.w.bits &= !(0x01 << 24);
        self.w.bits |= ((value as u32) & 0x01) << 24;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _DPLL1LCKFW<'a> {
    w: &'a mut W,
}
impl<'a> _DPLL1LCKFW<'a> {
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
        self.w.bits &= !(0x01 << 25);
        self.w.bits |= ((value as u32) & 0x01) << 25;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _DPLL1LTOW<'a> {
    w: &'a mut W,
}
impl<'a> _DPLL1LTOW<'a> {
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
        self.w.bits &= !(0x01 << 26);
        self.w.bits |= ((value as u32) & 0x01) << 26;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _DPLL1LDRTOW<'a> {
    w: &'a mut W,
}
impl<'a> _DPLL1LDRTOW<'a> {
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
        self.w.bits &= !(0x01 << 27);
        self.w.bits |= ((value as u32) & 0x01) << 27;
        self.w
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - XOSC 0 Ready"]
    #[inline]
    pub fn xoscrdy0(&self) -> XOSCRDY0R {
        let bits = ((self.bits >> 0) & 0x01) != 0;
        XOSCRDY0R { bits }
    }
    #[doc = "Bit 1 - XOSC 1 Ready"]
    #[inline]
    pub fn xoscrdy1(&self) -> XOSCRDY1R {
        let bits = ((self.bits >> 1) & 0x01) != 0;
        XOSCRDY1R { bits }
    }
    #[doc = "Bit 2 - XOSC 0 Clock Failure Detector"]
    #[inline]
    pub fn xoscfail0(&self) -> XOSCFAIL0R {
        let bits = ((self.bits >> 2) & 0x01) != 0;
        XOSCFAIL0R { bits }
    }
    #[doc = "Bit 3 - XOSC 1 Clock Failure Detector"]
    #[inline]
    pub fn xoscfail1(&self) -> XOSCFAIL1R {
        let bits = ((self.bits >> 3) & 0x01) != 0;
        XOSCFAIL1R { bits }
    }
    #[doc = "Bit 8 - DFLL Ready"]
    #[inline]
    pub fn dfllrdy(&self) -> DFLLRDYR {
        let bits = ((self.bits >> 8) & 0x01) != 0;
        DFLLRDYR { bits }
    }
    #[doc = "Bit 9 - DFLL Out Of Bounds"]
    #[inline]
    pub fn dflloob(&self) -> DFLLOOBR {
        let bits = ((self.bits >> 9) & 0x01) != 0;
        DFLLOOBR { bits }
    }
    #[doc = "Bit 10 - DFLL Lock Fine"]
    #[inline]
    pub fn dflllckf(&self) -> DFLLLCKFR {
        let bits = ((self.bits >> 10) & 0x01) != 0;
        DFLLLCKFR { bits }
    }
    #[doc = "Bit 11 - DFLL Lock Coarse"]
    #[inline]
    pub fn dflllckc(&self) -> DFLLLCKCR {
        let bits = ((self.bits >> 11) & 0x01) != 0;
        DFLLLCKCR { bits }
    }
    #[doc = "Bit 12 - DFLL Reference Clock Stopped"]
    #[inline]
    pub fn dfllrcs(&self) -> DFLLRCSR {
        let bits = ((self.bits >> 12) & 0x01) != 0;
        DFLLRCSR { bits }
    }
    #[doc = "Bit 16 - DPLL0 Lock Rise"]
    #[inline]
    pub fn dpll0lckr(&self) -> DPLL0LCKRR {
        let bits = ((self.bits >> 16) & 0x01) != 0;
        DPLL0LCKRR { bits }
    }
    #[doc = "Bit 17 - DPLL0 Lock Fall"]
    #[inline]
    pub fn dpll0lckf(&self) -> DPLL0LCKFR {
        let bits = ((self.bits >> 17) & 0x01) != 0;
        DPLL0LCKFR { bits }
    }
    #[doc = "Bit 18 - DPLL0 Lock Timeout"]
    #[inline]
    pub fn dpll0lto(&self) -> DPLL0LTOR {
        let bits = ((self.bits >> 18) & 0x01) != 0;
        DPLL0LTOR { bits }
    }
    #[doc = "Bit 19 - DPLL0 Loop Divider Ratio Update Complete"]
    #[inline]
    pub fn dpll0ldrto(&self) -> DPLL0LDRTOR {
        let bits = ((self.bits >> 19) & 0x01) != 0;
        DPLL0LDRTOR { bits }
    }
    #[doc = "Bit 24 - DPLL1 Lock Rise"]
    #[inline]
    pub fn dpll1lckr(&self) -> DPLL1LCKRR {
        let bits = ((self.bits >> 24) & 0x01) != 0;
        DPLL1LCKRR { bits }
    }
    #[doc = "Bit 25 - DPLL1 Lock Fall"]
    #[inline]
    pub fn dpll1lckf(&self) -> DPLL1LCKFR {
        let bits = ((self.bits >> 25) & 0x01) != 0;
        DPLL1LCKFR { bits }
    }
    #[doc = "Bit 26 - DPLL1 Lock Timeout"]
    #[inline]
    pub fn dpll1lto(&self) -> DPLL1LTOR {
        let bits = ((self.bits >> 26) & 0x01) != 0;
        DPLL1LTOR { bits }
    }
    #[doc = "Bit 27 - DPLL1 Loop Divider Ratio Update Complete"]
    #[inline]
    pub fn dpll1ldrto(&self) -> DPLL1LDRTOR {
        let bits = ((self.bits >> 27) & 0x01) != 0;
        DPLL1LDRTOR { bits }
    }
}
impl W {
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - XOSC 0 Ready"]
    #[inline]
    pub fn xoscrdy0(&mut self) -> _XOSCRDY0W {
        _XOSCRDY0W { w: self }
    }
    #[doc = "Bit 1 - XOSC 1 Ready"]
    #[inline]
    pub fn xoscrdy1(&mut self) -> _XOSCRDY1W {
        _XOSCRDY1W { w: self }
    }
    #[doc = "Bit 2 - XOSC 0 Clock Failure Detector"]
    #[inline]
    pub fn xoscfail0(&mut self) -> _XOSCFAIL0W {
        _XOSCFAIL0W { w: self }
    }
    #[doc = "Bit 3 - XOSC 1 Clock Failure Detector"]
    #[inline]
    pub fn xoscfail1(&mut self) -> _XOSCFAIL1W {
        _XOSCFAIL1W { w: self }
    }
    #[doc = "Bit 8 - DFLL Ready"]
    #[inline]
    pub fn dfllrdy(&mut self) -> _DFLLRDYW {
        _DFLLRDYW { w: self }
    }
    #[doc = "Bit 9 - DFLL Out Of Bounds"]
    #[inline]
    pub fn dflloob(&mut self) -> _DFLLOOBW {
        _DFLLOOBW { w: self }
    }
    #[doc = "Bit 10 - DFLL Lock Fine"]
    #[inline]
    pub fn dflllckf(&mut self) -> _DFLLLCKFW {
        _DFLLLCKFW { w: self }
    }
    #[doc = "Bit 11 - DFLL Lock Coarse"]
    #[inline]
    pub fn dflllckc(&mut self) -> _DFLLLCKCW {
        _DFLLLCKCW { w: self }
    }
    #[doc = "Bit 12 - DFLL Reference Clock Stopped"]
    #[inline]
    pub fn dfllrcs(&mut self) -> _DFLLRCSW {
        _DFLLRCSW { w: self }
    }
    #[doc = "Bit 16 - DPLL0 Lock Rise"]
    #[inline]
    pub fn dpll0lckr(&mut self) -> _DPLL0LCKRW {
        _DPLL0LCKRW { w: self }
    }
    #[doc = "Bit 17 - DPLL0 Lock Fall"]
    #[inline]
    pub fn dpll0lckf(&mut self) -> _DPLL0LCKFW {
        _DPLL0LCKFW { w: self }
    }
    #[doc = "Bit 18 - DPLL0 Lock Timeout"]
    #[inline]
    pub fn dpll0lto(&mut self) -> _DPLL0LTOW {
        _DPLL0LTOW { w: self }
    }
    #[doc = "Bit 19 - DPLL0 Loop Divider Ratio Update Complete"]
    #[inline]
    pub fn dpll0ldrto(&mut self) -> _DPLL0LDRTOW {
        _DPLL0LDRTOW { w: self }
    }
    #[doc = "Bit 24 - DPLL1 Lock Rise"]
    #[inline]
    pub fn dpll1lckr(&mut self) -> _DPLL1LCKRW {
        _DPLL1LCKRW { w: self }
    }
    #[doc = "Bit 25 - DPLL1 Lock Fall"]
    #[inline]
    pub fn dpll1lckf(&mut self) -> _DPLL1LCKFW {
        _DPLL1LCKFW { w: self }
    }
    #[doc = "Bit 26 - DPLL1 Lock Timeout"]
    #[inline]
    pub fn dpll1lto(&mut self) -> _DPLL1LTOW {
        _DPLL1LTOW { w: self }
    }
    #[doc = "Bit 27 - DPLL1 Loop Divider Ratio Update Complete"]
    #[inline]
    pub fn dpll1ldrto(&mut self) -> _DPLL1LDRTOW {
        _DPLL1LDRTOW { w: self }
    }
}
