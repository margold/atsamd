#[doc = r" Value read from the register"]
pub struct R {
    bits: u16,
}
impl super::PINTSMRY {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = r" Value of the field"]
pub struct EPINT0R {
    bits: bool,
}
impl EPINT0R {
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
pub struct EPINT1R {
    bits: bool,
}
impl EPINT1R {
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
pub struct EPINT2R {
    bits: bool,
}
impl EPINT2R {
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
pub struct EPINT3R {
    bits: bool,
}
impl EPINT3R {
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
pub struct EPINT4R {
    bits: bool,
}
impl EPINT4R {
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
pub struct EPINT5R {
    bits: bool,
}
impl EPINT5R {
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
pub struct EPINT6R {
    bits: bool,
}
impl EPINT6R {
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
pub struct EPINT7R {
    bits: bool,
}
impl EPINT7R {
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
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }
    #[doc = "Bit 0 - Pipe 0 Interrupt"]
    #[inline]
    pub fn epint0(&self) -> EPINT0R {
        let bits = ((self.bits >> 0) & 0x01) != 0;
        EPINT0R { bits }
    }
    #[doc = "Bit 1 - Pipe 1 Interrupt"]
    #[inline]
    pub fn epint1(&self) -> EPINT1R {
        let bits = ((self.bits >> 1) & 0x01) != 0;
        EPINT1R { bits }
    }
    #[doc = "Bit 2 - Pipe 2 Interrupt"]
    #[inline]
    pub fn epint2(&self) -> EPINT2R {
        let bits = ((self.bits >> 2) & 0x01) != 0;
        EPINT2R { bits }
    }
    #[doc = "Bit 3 - Pipe 3 Interrupt"]
    #[inline]
    pub fn epint3(&self) -> EPINT3R {
        let bits = ((self.bits >> 3) & 0x01) != 0;
        EPINT3R { bits }
    }
    #[doc = "Bit 4 - Pipe 4 Interrupt"]
    #[inline]
    pub fn epint4(&self) -> EPINT4R {
        let bits = ((self.bits >> 4) & 0x01) != 0;
        EPINT4R { bits }
    }
    #[doc = "Bit 5 - Pipe 5 Interrupt"]
    #[inline]
    pub fn epint5(&self) -> EPINT5R {
        let bits = ((self.bits >> 5) & 0x01) != 0;
        EPINT5R { bits }
    }
    #[doc = "Bit 6 - Pipe 6 Interrupt"]
    #[inline]
    pub fn epint6(&self) -> EPINT6R {
        let bits = ((self.bits >> 6) & 0x01) != 0;
        EPINT6R { bits }
    }
    #[doc = "Bit 7 - Pipe 7 Interrupt"]
    #[inline]
    pub fn epint7(&self) -> EPINT7R {
        let bits = ((self.bits >> 7) & 0x01) != 0;
        EPINT7R { bits }
    }
}
