#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::ERRADDR {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = r" Value of the field"]
pub struct ERRADDRR {
    bits: u32,
}
impl ERRADDRR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:16 - Error Address"]
    #[inline]
    pub fn erraddr(&self) -> ERRADDRR {
        let bits = ((self.bits >> 0) & 0x0001_ffff) as u32;
        ERRADDRR { bits }
    }
}
