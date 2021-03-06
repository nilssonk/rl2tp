use crate::common::ResultStr;
use num_enum::{IntoPrimitive, TryFromPrimitive};

#[derive(Clone, Copy, Debug, IntoPrimitive, TryFromPrimitive, Eq, PartialEq)]
#[repr(u16)]
pub enum StopCcnCode {
    Reserved,
    GeneralRequestToClearControlConnection,
    GeneralError,
    ControlChannelAlreadyExists,
    RequesterNotAuthorizedToEstablishControlChannel,
    RequesterProtocolVersionUnsupported,
    RequesterShutdown,
    FsmError,
}

#[derive(Clone, Copy, Debug, IntoPrimitive, TryFromPrimitive, Eq, PartialEq)]
#[repr(u16)]
pub enum CdnCode {
    Reserved,
    CallDisconnectedLossOfCarrier,
    CallDisconnectedWithErrorCode,
    CallDisconnectedAdministrative,
    CallFailedTemporarilyUnavailable,
    CallFailedPermanentlyUnavailable,
    InvalidDestination,
    CallFailedNoCarrier,
    CallFailedBusySignal,
    CallFailedNoDialTone,
    CallEstablishTimeout,
    CallNoFramingDetected,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CodeValue {
    value: u16,
}

impl CodeValue {
    #[inline]
    pub fn as_stop_ccn(&self) -> ResultStr<StopCcnCode> {
        let maybe_code = self.value.try_into();
        maybe_code.map_err(|_| "Invalid StopCcnCode")
    }

    #[inline]
    pub fn as_cdn(&self) -> ResultStr<CdnCode> {
        let maybe_code = self.value.try_into();
        maybe_code.map_err(|_| "Invalid CdnCode")
    }
}

impl From<CodeValue> for u16 {
    #[inline]
    fn from(value: CodeValue) -> Self {
        value.value
    }
}

impl From<u16> for CodeValue {
    #[inline]
    fn from(value: u16) -> Self {
        Self { value }
    }
}

impl From<StopCcnCode> for CodeValue {
    #[inline]
    fn from(value: StopCcnCode) -> Self {
        Self {
            value: value.into(),
        }
    }
}

impl From<CdnCode> for CodeValue {
    #[inline]
    fn from(value: CdnCode) -> Self {
        Self {
            value: value.into(),
        }
    }
}
