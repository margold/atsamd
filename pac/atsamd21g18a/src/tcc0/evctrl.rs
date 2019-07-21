#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::EVCTRL {
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
#[doc = "Possible values of the field `EVACT0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EVACT0R {
    #[doc = "Event action disabled"]
    OFF,
    #[doc = "Start, restart or re-trigger counter on event"]
    RETRIGGER,
    #[doc = "Count on event"]
    COUNTEV,
    #[doc = "Start counter on event"]
    START,
    #[doc = "Increment counter on event"]
    INC,
    #[doc = "Count on active state of asynchronous event"]
    COUNT,
    #[doc = "Non-recoverable fault"]
    FAULT,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl EVACT0R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            EVACT0R::OFF => 0,
            EVACT0R::RETRIGGER => 0x01,
            EVACT0R::COUNTEV => 0x02,
            EVACT0R::START => 0x03,
            EVACT0R::INC => 0x04,
            EVACT0R::COUNT => 0x05,
            EVACT0R::FAULT => 0x07,
            EVACT0R::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> EVACT0R {
        match value {
            0 => EVACT0R::OFF,
            1 => EVACT0R::RETRIGGER,
            2 => EVACT0R::COUNTEV,
            3 => EVACT0R::START,
            4 => EVACT0R::INC,
            5 => EVACT0R::COUNT,
            7 => EVACT0R::FAULT,
            i => EVACT0R::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `OFF`"]
    #[inline]
    pub fn is_off(&self) -> bool {
        *self == EVACT0R::OFF
    }
    #[doc = "Checks if the value of the field is `RETRIGGER`"]
    #[inline]
    pub fn is_retrigger(&self) -> bool {
        *self == EVACT0R::RETRIGGER
    }
    #[doc = "Checks if the value of the field is `COUNTEV`"]
    #[inline]
    pub fn is_countev(&self) -> bool {
        *self == EVACT0R::COUNTEV
    }
    #[doc = "Checks if the value of the field is `START`"]
    #[inline]
    pub fn is_start(&self) -> bool {
        *self == EVACT0R::START
    }
    #[doc = "Checks if the value of the field is `INC`"]
    #[inline]
    pub fn is_inc(&self) -> bool {
        *self == EVACT0R::INC
    }
    #[doc = "Checks if the value of the field is `COUNT`"]
    #[inline]
    pub fn is_count(&self) -> bool {
        *self == EVACT0R::COUNT
    }
    #[doc = "Checks if the value of the field is `FAULT`"]
    #[inline]
    pub fn is_fault(&self) -> bool {
        *self == EVACT0R::FAULT
    }
}
#[doc = "Possible values of the field `EVACT1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EVACT1R {
    #[doc = "Event action disabled"]
    OFF,
    #[doc = "Re-trigger counter on event"]
    RETRIGGER,
    #[doc = "Direction control"]
    DIR,
    #[doc = "Stop counter on event"]
    STOP,
    #[doc = "Decrement counter on event"]
    DEC,
    #[doc = "Period capture value in CC0 register, pulse width capture value in CC1 register"]
    PPW,
    #[doc = "Period capture value in CC1 register, pulse width capture value in CC0 register"]
    PWP,
    #[doc = "Non-recoverable fault"]
    FAULT,
}
impl EVACT1R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            EVACT1R::OFF => 0,
            EVACT1R::RETRIGGER => 0x01,
            EVACT1R::DIR => 0x02,
            EVACT1R::STOP => 0x03,
            EVACT1R::DEC => 0x04,
            EVACT1R::PPW => 0x05,
            EVACT1R::PWP => 0x06,
            EVACT1R::FAULT => 0x07,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> EVACT1R {
        match value {
            0 => EVACT1R::OFF,
            1 => EVACT1R::RETRIGGER,
            2 => EVACT1R::DIR,
            3 => EVACT1R::STOP,
            4 => EVACT1R::DEC,
            5 => EVACT1R::PPW,
            6 => EVACT1R::PWP,
            7 => EVACT1R::FAULT,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `OFF`"]
    #[inline]
    pub fn is_off(&self) -> bool {
        *self == EVACT1R::OFF
    }
    #[doc = "Checks if the value of the field is `RETRIGGER`"]
    #[inline]
    pub fn is_retrigger(&self) -> bool {
        *self == EVACT1R::RETRIGGER
    }
    #[doc = "Checks if the value of the field is `DIR`"]
    #[inline]
    pub fn is_dir(&self) -> bool {
        *self == EVACT1R::DIR
    }
    #[doc = "Checks if the value of the field is `STOP`"]
    #[inline]
    pub fn is_stop(&self) -> bool {
        *self == EVACT1R::STOP
    }
    #[doc = "Checks if the value of the field is `DEC`"]
    #[inline]
    pub fn is_dec(&self) -> bool {
        *self == EVACT1R::DEC
    }
    #[doc = "Checks if the value of the field is `PPW`"]
    #[inline]
    pub fn is_ppw(&self) -> bool {
        *self == EVACT1R::PPW
    }
    #[doc = "Checks if the value of the field is `PWP`"]
    #[inline]
    pub fn is_pwp(&self) -> bool {
        *self == EVACT1R::PWP
    }
    #[doc = "Checks if the value of the field is `FAULT`"]
    #[inline]
    pub fn is_fault(&self) -> bool {
        *self == EVACT1R::FAULT
    }
}
#[doc = "Possible values of the field `CNTSEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CNTSELR {
    #[doc = "An interrupt/event is generated when a new counter cycle starts"]
    START,
    #[doc = "An interrupt/event is generated when a counter cycle ends"]
    END,
    #[doc = "An interrupt/event is generated when a counter cycle ends, except for the first and last cycles"]
    BETWEEN,
    #[doc = "An interrupt/event is generated when a new counter cycle starts or a counter cycle ends"]
    BOUNDARY,
}
impl CNTSELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            CNTSELR::START => 0,
            CNTSELR::END => 0x01,
            CNTSELR::BETWEEN => 0x02,
            CNTSELR::BOUNDARY => 0x03,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> CNTSELR {
        match value {
            0 => CNTSELR::START,
            1 => CNTSELR::END,
            2 => CNTSELR::BETWEEN,
            3 => CNTSELR::BOUNDARY,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `START`"]
    #[inline]
    pub fn is_start(&self) -> bool {
        *self == CNTSELR::START
    }
    #[doc = "Checks if the value of the field is `END`"]
    #[inline]
    pub fn is_end(&self) -> bool {
        *self == CNTSELR::END
    }
    #[doc = "Checks if the value of the field is `BETWEEN`"]
    #[inline]
    pub fn is_between(&self) -> bool {
        *self == CNTSELR::BETWEEN
    }
    #[doc = "Checks if the value of the field is `BOUNDARY`"]
    #[inline]
    pub fn is_boundary(&self) -> bool {
        *self == CNTSELR::BOUNDARY
    }
}
#[doc = r" Value of the field"]
pub struct OVFEOR {
    bits: bool,
}
impl OVFEOR {
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
pub struct TRGEOR {
    bits: bool,
}
impl TRGEOR {
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
pub struct CNTEOR {
    bits: bool,
}
impl CNTEOR {
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
pub struct TCINV0R {
    bits: bool,
}
impl TCINV0R {
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
pub struct TCINV1R {
    bits: bool,
}
impl TCINV1R {
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
pub struct TCEI0R {
    bits: bool,
}
impl TCEI0R {
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
pub struct TCEI1R {
    bits: bool,
}
impl TCEI1R {
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
pub struct MCEI0R {
    bits: bool,
}
impl MCEI0R {
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
pub struct MCEI1R {
    bits: bool,
}
impl MCEI1R {
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
pub struct MCEI2R {
    bits: bool,
}
impl MCEI2R {
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
pub struct MCEI3R {
    bits: bool,
}
impl MCEI3R {
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
pub struct MCEO0R {
    bits: bool,
}
impl MCEO0R {
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
pub struct MCEO1R {
    bits: bool,
}
impl MCEO1R {
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
pub struct MCEO2R {
    bits: bool,
}
impl MCEO2R {
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
pub struct MCEO3R {
    bits: bool,
}
impl MCEO3R {
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
#[doc = "Values that can be written to the field `EVACT0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EVACT0W {
    #[doc = "Event action disabled"]
    OFF,
    #[doc = "Start, restart or re-trigger counter on event"]
    RETRIGGER,
    #[doc = "Count on event"]
    COUNTEV,
    #[doc = "Start counter on event"]
    START,
    #[doc = "Increment counter on event"]
    INC,
    #[doc = "Count on active state of asynchronous event"]
    COUNT,
    #[doc = "Non-recoverable fault"]
    FAULT,
}
impl EVACT0W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            EVACT0W::OFF => 0,
            EVACT0W::RETRIGGER => 1,
            EVACT0W::COUNTEV => 2,
            EVACT0W::START => 3,
            EVACT0W::INC => 4,
            EVACT0W::COUNT => 5,
            EVACT0W::FAULT => 7,
        }
    }
}
#[doc = r" Proxy"]
pub struct _EVACT0W<'a> {
    w: &'a mut W,
}
impl<'a> _EVACT0W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: EVACT0W) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Event action disabled"]
    #[inline]
    pub fn off(self) -> &'a mut W {
        self.variant(EVACT0W::OFF)
    }
    #[doc = "Start, restart or re-trigger counter on event"]
    #[inline]
    pub fn retrigger(self) -> &'a mut W {
        self.variant(EVACT0W::RETRIGGER)
    }
    #[doc = "Count on event"]
    #[inline]
    pub fn countev(self) -> &'a mut W {
        self.variant(EVACT0W::COUNTEV)
    }
    #[doc = "Start counter on event"]
    #[inline]
    pub fn start(self) -> &'a mut W {
        self.variant(EVACT0W::START)
    }
    #[doc = "Increment counter on event"]
    #[inline]
    pub fn inc(self) -> &'a mut W {
        self.variant(EVACT0W::INC)
    }
    #[doc = "Count on active state of asynchronous event"]
    #[inline]
    pub fn count(self) -> &'a mut W {
        self.variant(EVACT0W::COUNT)
    }
    #[doc = "Non-recoverable fault"]
    #[inline]
    pub fn fault(self) -> &'a mut W {
        self.variant(EVACT0W::FAULT)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits &= !(0x07 << 0);
        self.w.bits |= ((value as u32) & 0x07) << 0;
        self.w
    }
}
#[doc = "Values that can be written to the field `EVACT1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EVACT1W {
    #[doc = "Event action disabled"]
    OFF,
    #[doc = "Re-trigger counter on event"]
    RETRIGGER,
    #[doc = "Direction control"]
    DIR,
    #[doc = "Stop counter on event"]
    STOP,
    #[doc = "Decrement counter on event"]
    DEC,
    #[doc = "Period capture value in CC0 register, pulse width capture value in CC1 register"]
    PPW,
    #[doc = "Period capture value in CC1 register, pulse width capture value in CC0 register"]
    PWP,
    #[doc = "Non-recoverable fault"]
    FAULT,
}
impl EVACT1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            EVACT1W::OFF => 0,
            EVACT1W::RETRIGGER => 1,
            EVACT1W::DIR => 2,
            EVACT1W::STOP => 3,
            EVACT1W::DEC => 4,
            EVACT1W::PPW => 5,
            EVACT1W::PWP => 6,
            EVACT1W::FAULT => 7,
        }
    }
}
#[doc = r" Proxy"]
pub struct _EVACT1W<'a> {
    w: &'a mut W,
}
impl<'a> _EVACT1W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: EVACT1W) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Event action disabled"]
    #[inline]
    pub fn off(self) -> &'a mut W {
        self.variant(EVACT1W::OFF)
    }
    #[doc = "Re-trigger counter on event"]
    #[inline]
    pub fn retrigger(self) -> &'a mut W {
        self.variant(EVACT1W::RETRIGGER)
    }
    #[doc = "Direction control"]
    #[inline]
    pub fn dir(self) -> &'a mut W {
        self.variant(EVACT1W::DIR)
    }
    #[doc = "Stop counter on event"]
    #[inline]
    pub fn stop(self) -> &'a mut W {
        self.variant(EVACT1W::STOP)
    }
    #[doc = "Decrement counter on event"]
    #[inline]
    pub fn dec(self) -> &'a mut W {
        self.variant(EVACT1W::DEC)
    }
    #[doc = "Period capture value in CC0 register, pulse width capture value in CC1 register"]
    #[inline]
    pub fn ppw(self) -> &'a mut W {
        self.variant(EVACT1W::PPW)
    }
    #[doc = "Period capture value in CC1 register, pulse width capture value in CC0 register"]
    #[inline]
    pub fn pwp(self) -> &'a mut W {
        self.variant(EVACT1W::PWP)
    }
    #[doc = "Non-recoverable fault"]
    #[inline]
    pub fn fault(self) -> &'a mut W {
        self.variant(EVACT1W::FAULT)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits &= !(0x07 << 3);
        self.w.bits |= ((value as u32) & 0x07) << 3;
        self.w
    }
}
#[doc = "Values that can be written to the field `CNTSEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CNTSELW {
    #[doc = "An interrupt/event is generated when a new counter cycle starts"]
    START,
    #[doc = "An interrupt/event is generated when a counter cycle ends"]
    END,
    #[doc = "An interrupt/event is generated when a counter cycle ends, except for the first and last cycles"]
    BETWEEN,
    #[doc = "An interrupt/event is generated when a new counter cycle starts or a counter cycle ends"]
    BOUNDARY,
}
impl CNTSELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            CNTSELW::START => 0,
            CNTSELW::END => 1,
            CNTSELW::BETWEEN => 2,
            CNTSELW::BOUNDARY => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CNTSELW<'a> {
    w: &'a mut W,
}
impl<'a> _CNTSELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CNTSELW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "An interrupt/event is generated when a new counter cycle starts"]
    #[inline]
    pub fn start(self) -> &'a mut W {
        self.variant(CNTSELW::START)
    }
    #[doc = "An interrupt/event is generated when a counter cycle ends"]
    #[inline]
    pub fn end(self) -> &'a mut W {
        self.variant(CNTSELW::END)
    }
    #[doc = "An interrupt/event is generated when a counter cycle ends, except for the first and last cycles"]
    #[inline]
    pub fn between(self) -> &'a mut W {
        self.variant(CNTSELW::BETWEEN)
    }
    #[doc = "An interrupt/event is generated when a new counter cycle starts or a counter cycle ends"]
    #[inline]
    pub fn boundary(self) -> &'a mut W {
        self.variant(CNTSELW::BOUNDARY)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits &= !(0x03 << 6);
        self.w.bits |= ((value as u32) & 0x03) << 6;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _OVFEOW<'a> {
    w: &'a mut W,
}
impl<'a> _OVFEOW<'a> {
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
pub struct _TRGEOW<'a> {
    w: &'a mut W,
}
impl<'a> _TRGEOW<'a> {
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
pub struct _CNTEOW<'a> {
    w: &'a mut W,
}
impl<'a> _CNTEOW<'a> {
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
pub struct _TCINV0W<'a> {
    w: &'a mut W,
}
impl<'a> _TCINV0W<'a> {
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
pub struct _TCINV1W<'a> {
    w: &'a mut W,
}
impl<'a> _TCINV1W<'a> {
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
        self.w.bits &= !(0x01 << 13);
        self.w.bits |= ((value as u32) & 0x01) << 13;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _TCEI0W<'a> {
    w: &'a mut W,
}
impl<'a> _TCEI0W<'a> {
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
        self.w.bits &= !(0x01 << 14);
        self.w.bits |= ((value as u32) & 0x01) << 14;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _TCEI1W<'a> {
    w: &'a mut W,
}
impl<'a> _TCEI1W<'a> {
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
        self.w.bits &= !(0x01 << 15);
        self.w.bits |= ((value as u32) & 0x01) << 15;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _MCEI0W<'a> {
    w: &'a mut W,
}
impl<'a> _MCEI0W<'a> {
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
pub struct _MCEI1W<'a> {
    w: &'a mut W,
}
impl<'a> _MCEI1W<'a> {
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
pub struct _MCEI2W<'a> {
    w: &'a mut W,
}
impl<'a> _MCEI2W<'a> {
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
pub struct _MCEI3W<'a> {
    w: &'a mut W,
}
impl<'a> _MCEI3W<'a> {
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
pub struct _MCEO0W<'a> {
    w: &'a mut W,
}
impl<'a> _MCEO0W<'a> {
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
pub struct _MCEO1W<'a> {
    w: &'a mut W,
}
impl<'a> _MCEO1W<'a> {
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
pub struct _MCEO2W<'a> {
    w: &'a mut W,
}
impl<'a> _MCEO2W<'a> {
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
pub struct _MCEO3W<'a> {
    w: &'a mut W,
}
impl<'a> _MCEO3W<'a> {
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
    #[doc = "Bits 0:2 - Timer/counter Input Event0 Action"]
    #[inline]
    pub fn evact0(&self) -> EVACT0R {
        EVACT0R::_from(((self.bits >> 0) & 0x07) as u8)
    }
    #[doc = "Bits 3:5 - Timer/counter Input Event1 Action"]
    #[inline]
    pub fn evact1(&self) -> EVACT1R {
        EVACT1R::_from(((self.bits >> 3) & 0x07) as u8)
    }
    #[doc = "Bits 6:7 - Timer/counter Output Event Mode"]
    #[inline]
    pub fn cntsel(&self) -> CNTSELR {
        CNTSELR::_from(((self.bits >> 6) & 0x03) as u8)
    }
    #[doc = "Bit 8 - Overflow/Underflow Output Event Enable"]
    #[inline]
    pub fn ovfeo(&self) -> OVFEOR {
        let bits = ((self.bits >> 8) & 0x01) != 0;
        OVFEOR { bits }
    }
    #[doc = "Bit 9 - Retrigger Output Event Enable"]
    #[inline]
    pub fn trgeo(&self) -> TRGEOR {
        let bits = ((self.bits >> 9) & 0x01) != 0;
        TRGEOR { bits }
    }
    #[doc = "Bit 10 - Timer/counter Output Event Enable"]
    #[inline]
    pub fn cnteo(&self) -> CNTEOR {
        let bits = ((self.bits >> 10) & 0x01) != 0;
        CNTEOR { bits }
    }
    #[doc = "Bit 12 - Inverted Event 0 Input Enable"]
    #[inline]
    pub fn tcinv0(&self) -> TCINV0R {
        let bits = ((self.bits >> 12) & 0x01) != 0;
        TCINV0R { bits }
    }
    #[doc = "Bit 13 - Inverted Event 1 Input Enable"]
    #[inline]
    pub fn tcinv1(&self) -> TCINV1R {
        let bits = ((self.bits >> 13) & 0x01) != 0;
        TCINV1R { bits }
    }
    #[doc = "Bit 14 - Timer/counter Event 0 Input Enable"]
    #[inline]
    pub fn tcei0(&self) -> TCEI0R {
        let bits = ((self.bits >> 14) & 0x01) != 0;
        TCEI0R { bits }
    }
    #[doc = "Bit 15 - Timer/counter Event 1 Input Enable"]
    #[inline]
    pub fn tcei1(&self) -> TCEI1R {
        let bits = ((self.bits >> 15) & 0x01) != 0;
        TCEI1R { bits }
    }
    #[doc = "Bit 16 - Match or Capture Channel 0 Event Input Enable"]
    #[inline]
    pub fn mcei0(&self) -> MCEI0R {
        let bits = ((self.bits >> 16) & 0x01) != 0;
        MCEI0R { bits }
    }
    #[doc = "Bit 17 - Match or Capture Channel 1 Event Input Enable"]
    #[inline]
    pub fn mcei1(&self) -> MCEI1R {
        let bits = ((self.bits >> 17) & 0x01) != 0;
        MCEI1R { bits }
    }
    #[doc = "Bit 18 - Match or Capture Channel 2 Event Input Enable"]
    #[inline]
    pub fn mcei2(&self) -> MCEI2R {
        let bits = ((self.bits >> 18) & 0x01) != 0;
        MCEI2R { bits }
    }
    #[doc = "Bit 19 - Match or Capture Channel 3 Event Input Enable"]
    #[inline]
    pub fn mcei3(&self) -> MCEI3R {
        let bits = ((self.bits >> 19) & 0x01) != 0;
        MCEI3R { bits }
    }
    #[doc = "Bit 24 - Match or Capture Channel 0 Event Output Enable"]
    #[inline]
    pub fn mceo0(&self) -> MCEO0R {
        let bits = ((self.bits >> 24) & 0x01) != 0;
        MCEO0R { bits }
    }
    #[doc = "Bit 25 - Match or Capture Channel 1 Event Output Enable"]
    #[inline]
    pub fn mceo1(&self) -> MCEO1R {
        let bits = ((self.bits >> 25) & 0x01) != 0;
        MCEO1R { bits }
    }
    #[doc = "Bit 26 - Match or Capture Channel 2 Event Output Enable"]
    #[inline]
    pub fn mceo2(&self) -> MCEO2R {
        let bits = ((self.bits >> 26) & 0x01) != 0;
        MCEO2R { bits }
    }
    #[doc = "Bit 27 - Match or Capture Channel 3 Event Output Enable"]
    #[inline]
    pub fn mceo3(&self) -> MCEO3R {
        let bits = ((self.bits >> 27) & 0x01) != 0;
        MCEO3R { bits }
    }
}
impl W {
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:2 - Timer/counter Input Event0 Action"]
    #[inline]
    pub fn evact0(&mut self) -> _EVACT0W {
        _EVACT0W { w: self }
    }
    #[doc = "Bits 3:5 - Timer/counter Input Event1 Action"]
    #[inline]
    pub fn evact1(&mut self) -> _EVACT1W {
        _EVACT1W { w: self }
    }
    #[doc = "Bits 6:7 - Timer/counter Output Event Mode"]
    #[inline]
    pub fn cntsel(&mut self) -> _CNTSELW {
        _CNTSELW { w: self }
    }
    #[doc = "Bit 8 - Overflow/Underflow Output Event Enable"]
    #[inline]
    pub fn ovfeo(&mut self) -> _OVFEOW {
        _OVFEOW { w: self }
    }
    #[doc = "Bit 9 - Retrigger Output Event Enable"]
    #[inline]
    pub fn trgeo(&mut self) -> _TRGEOW {
        _TRGEOW { w: self }
    }
    #[doc = "Bit 10 - Timer/counter Output Event Enable"]
    #[inline]
    pub fn cnteo(&mut self) -> _CNTEOW {
        _CNTEOW { w: self }
    }
    #[doc = "Bit 12 - Inverted Event 0 Input Enable"]
    #[inline]
    pub fn tcinv0(&mut self) -> _TCINV0W {
        _TCINV0W { w: self }
    }
    #[doc = "Bit 13 - Inverted Event 1 Input Enable"]
    #[inline]
    pub fn tcinv1(&mut self) -> _TCINV1W {
        _TCINV1W { w: self }
    }
    #[doc = "Bit 14 - Timer/counter Event 0 Input Enable"]
    #[inline]
    pub fn tcei0(&mut self) -> _TCEI0W {
        _TCEI0W { w: self }
    }
    #[doc = "Bit 15 - Timer/counter Event 1 Input Enable"]
    #[inline]
    pub fn tcei1(&mut self) -> _TCEI1W {
        _TCEI1W { w: self }
    }
    #[doc = "Bit 16 - Match or Capture Channel 0 Event Input Enable"]
    #[inline]
    pub fn mcei0(&mut self) -> _MCEI0W {
        _MCEI0W { w: self }
    }
    #[doc = "Bit 17 - Match or Capture Channel 1 Event Input Enable"]
    #[inline]
    pub fn mcei1(&mut self) -> _MCEI1W {
        _MCEI1W { w: self }
    }
    #[doc = "Bit 18 - Match or Capture Channel 2 Event Input Enable"]
    #[inline]
    pub fn mcei2(&mut self) -> _MCEI2W {
        _MCEI2W { w: self }
    }
    #[doc = "Bit 19 - Match or Capture Channel 3 Event Input Enable"]
    #[inline]
    pub fn mcei3(&mut self) -> _MCEI3W {
        _MCEI3W { w: self }
    }
    #[doc = "Bit 24 - Match or Capture Channel 0 Event Output Enable"]
    #[inline]
    pub fn mceo0(&mut self) -> _MCEO0W {
        _MCEO0W { w: self }
    }
    #[doc = "Bit 25 - Match or Capture Channel 1 Event Output Enable"]
    #[inline]
    pub fn mceo1(&mut self) -> _MCEO1W {
        _MCEO1W { w: self }
    }
    #[doc = "Bit 26 - Match or Capture Channel 2 Event Output Enable"]
    #[inline]
    pub fn mceo2(&mut self) -> _MCEO2W {
        _MCEO2W { w: self }
    }
    #[doc = "Bit 27 - Match or Capture Channel 3 Event Output Enable"]
    #[inline]
    pub fn mceo3(&mut self) -> _MCEO3W {
        _MCEO3W { w: self }
    }
}
