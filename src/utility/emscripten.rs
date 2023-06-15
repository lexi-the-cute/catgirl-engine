/// Recreation of EMSCRIPTEN_RESULT as Rust enum
/// 
/// See https://github.com/emscripten-core/emscripten/blob/07053fbdc720e53172cae6dda33d2bb48266a49a/system/include/emscripten/html5.h#L75-L84
pub enum EmscriptenResult {
    Success = 0,
    Deferred = 1,
    NotSupported = -1,
    FailedNotDeferred = -2,
    InvalidTarget = -3,
    UnknownTarget = -4,
    InvalidParam = -5,
    Failed = -6,
    NoData = -7,
    TimedOut = -8
}

impl TryFrom<i32> for EmscriptenResult {
    type Error = ();

    fn try_from(v: i32) -> Result<Self, Self::Error> {
        match v {
            x if x == EmscriptenResult::Success as i32 => Ok(EmscriptenResult::Success),
            x if x == EmscriptenResult::Deferred as i32 => Ok(EmscriptenResult::Deferred),
            x if x == EmscriptenResult::NotSupported as i32 => Ok(EmscriptenResult::NotSupported),
            x if x == EmscriptenResult::FailedNotDeferred as i32 => Ok(EmscriptenResult::FailedNotDeferred),
            x if x == EmscriptenResult::InvalidTarget as i32 => Ok(EmscriptenResult::InvalidTarget),
            x if x == EmscriptenResult::UnknownTarget as i32 => Ok(EmscriptenResult::UnknownTarget),
            x if x == EmscriptenResult::InvalidParam as i32 => Ok(EmscriptenResult::InvalidParam),
            x if x == EmscriptenResult::Failed as i32 => Ok(EmscriptenResult::Failed),
            x if x == EmscriptenResult::NoData as i32 => Ok(EmscriptenResult::NoData),
            x if x == EmscriptenResult::TimedOut as i32 => Ok(EmscriptenResult::TimedOut),
            _ => Err(()),
        }
    }
}

#[allow(dead_code)]
pub fn read_emscripten_result(value: i32) -> String {
    match value.try_into() {
        Ok(EmscriptenResult::Success) => String::from("Success"),
        Ok(EmscriptenResult::Deferred) => String::from("Deferred"),
        Ok(EmscriptenResult::NotSupported) => String::from("NotSupported"),
        Ok(EmscriptenResult::FailedNotDeferred) => String::from("FailedNotDeferred"),
        Ok(EmscriptenResult::InvalidTarget) => String::from("InvalidTarget"),
        Ok(EmscriptenResult::UnknownTarget) => String::from("UnknownTarget"),
        Ok(EmscriptenResult::InvalidParam) => String::from("InvalidParam"),
        Ok(EmscriptenResult::Failed) => String::from("Failed"),
        Ok(EmscriptenResult::NoData) => String::from("NoData"),
        Ok(EmscriptenResult::TimedOut) => String::from("TimedOut"),
        Err(_) => String::from("Unknown"),
    }
}