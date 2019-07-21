#[doc = r" Value read from the register"]
pub struct R {
    bits: u8,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u8,
}
impl super::REFCTRL {
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
    pub const fn reset_value() -> u8 {
        0
    }
    #[doc = r" Writes the reset value to the register"]
    #[inline]
    pub fn reset(&self) {
        self.register.set(Self::reset_value())
    }
}
#[doc = "Possible values of the field `REFSEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REFSELR {
    #[doc = "Internal Bandgap Reference"]
    INTREF,
    #[doc = "1/2 VDDANA"]
    INTVCC0,
    #[doc = "VDDANA"]
    INTVCC1,
    #[doc = "External Reference"]
    AREFA,
    #[doc = "External Reference"]
    AREFB,
    #[doc = "External Reference (only on ADC1)"]
    AREFC,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl REFSELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            REFSELR::INTREF => 0,
            REFSELR::INTVCC0 => 0x02,
            REFSELR::INTVCC1 => 0x03,
            REFSELR::AREFA => 0x04,
            REFSELR::AREFB => 0x05,
            REFSELR::AREFC => 0x06,
            REFSELR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> REFSELR {
        match value {
            0 => REFSELR::INTREF,
            2 => REFSELR::INTVCC0,
            3 => REFSELR::INTVCC1,
            4 => REFSELR::AREFA,
            5 => REFSELR::AREFB,
            6 => REFSELR::AREFC,
            i => REFSELR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `INTREF`"]
    #[inline]
    pub fn is_intref(&self) -> bool {
        *self == REFSELR::INTREF
    }
    #[doc = "Checks if the value of the field is `INTVCC0`"]
    #[inline]
    pub fn is_intvcc0(&self) -> bool {
        *self == REFSELR::INTVCC0
    }
    #[doc = "Checks if the value of the field is `INTVCC1`"]
    #[inline]
    pub fn is_intvcc1(&self) -> bool {
        *self == REFSELR::INTVCC1
    }
    #[doc = "Checks if the value of the field is `AREFA`"]
    #[inline]
    pub fn is_arefa(&self) -> bool {
        *self == REFSELR::AREFA
    }
    #[doc = "Checks if the value of the field is `AREFB`"]
    #[inline]
    pub fn is_arefb(&self) -> bool {
        *self == REFSELR::AREFB
    }
    #[doc = "Checks if the value of the field is `AREFC`"]
    #[inline]
    pub fn is_arefc(&self) -> bool {
        *self == REFSELR::AREFC
    }
}
#[doc = r" Value of the field"]
pub struct REFCOMPR {
    bits: bool,
}
impl REFCOMPR {
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
#[doc = "Values that can be written to the field `REFSEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REFSELW {
    #[doc = "Internal Bandgap Reference"]
    INTREF,
    #[doc = "1/2 VDDANA"]
    INTVCC0,
    #[doc = "VDDANA"]
    INTVCC1,
    #[doc = "External Reference"]
    AREFA,
    #[doc = "External Reference"]
    AREFB,
    #[doc = "External Reference (only on ADC1)"]
    AREFC,
}
impl REFSELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            REFSELW::INTREF => 0,
            REFSELW::INTVCC0 => 2,
            REFSELW::INTVCC1 => 3,
            REFSELW::AREFA => 4,
            REFSELW::AREFB => 5,
            REFSELW::AREFC => 6,
        }
    }
}
#[doc = r" Proxy"]
pub struct _REFSELW<'a> {
    w: &'a mut W,
}
impl<'a> _REFSELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: REFSELW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Internal Bandgap Reference"]
    #[inline]
    pub fn intref(self) -> &'a mut W {
        self.variant(REFSELW::INTREF)
    }
    #[doc = "1/2 VDDANA"]
    #[inline]
    pub fn intvcc0(self) -> &'a mut W {
        self.variant(REFSELW::INTVCC0)
    }
    #[doc = "VDDANA"]
    #[inline]
    pub fn intvcc1(self) -> &'a mut W {
        self.variant(REFSELW::INTVCC1)
    }
    #[doc = "External Reference"]
    #[inline]
    pub fn arefa(self) -> &'a mut W {
        self.variant(REFSELW::AREFA)
    }
    #[doc = "External Reference"]
    #[inline]
    pub fn arefb(self) -> &'a mut W {
        self.variant(REFSELW::AREFB)
    }
    #[doc = "External Reference (only on ADC1)"]
    #[inline]
    pub fn arefc(self) -> &'a mut W {
        self.variant(REFSELW::AREFC)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits &= !(0x0f << 0);
        self.w.bits |= ((value as u8) & 0x0f) << 0;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _REFCOMPW<'a> {
    w: &'a mut W,
}
impl<'a> _REFCOMPW<'a> {
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
        self.w.bits &= !(0x01 << 7);
        self.w.bits |= ((value as u8) & 0x01) << 7;
        self.w
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
    #[doc = "Bits 0:3 - Reference Selection"]
    #[inline]
    pub fn refsel(&self) -> REFSELR {
        REFSELR::_from(((self.bits >> 0) & 0x0f) as u8)
    }
    #[doc = "Bit 7 - Reference Buffer Offset Compensation Enable"]
    #[inline]
    pub fn refcomp(&self) -> REFCOMPR {
        let bits = ((self.bits >> 7) & 0x01) != 0;
        REFCOMPR { bits }
    }
}
impl W {
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:3 - Reference Selection"]
    #[inline]
    pub fn refsel(&mut self) -> _REFSELW {
        _REFSELW { w: self }
    }
    #[doc = "Bit 7 - Reference Buffer Offset Compensation Enable"]
    #[inline]
    pub fn refcomp(&mut self) -> _REFCOMPW {
        _REFCOMPW { w: self }
    }
}
