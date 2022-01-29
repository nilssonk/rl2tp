use num_enum::{IntoPrimitive, TryFromPrimitive};

#[derive(Clone, Copy, Debug, IntoPrimitive, TryFromPrimitive, PartialEq)]
#[repr(u16)]
pub enum Error {
    Ok,
    NoControlConnectionExists,
    WrongLength,
    OutOfRangeOrBadReserved,
    InsufficientResources,
    InvalidSessionId,
    Generic,
    TryAnotherDestination,
    UnknownMandatoryAvp,
}
