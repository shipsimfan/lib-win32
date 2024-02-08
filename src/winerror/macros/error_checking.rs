/// Provides a generic test for success on any status value.
#[macro_export]
macro_rules! SUCCEEDED {
    ($hr: expr) => {
        ($hr as $crate::HRESULT) >= 0
    };
}

/// Provides a generic test for failure on any status value.
#[macro_export]
macro_rules! FAILED {
    ($hr: expr) => {
        ($hr as $crate::HRESULT) < 0
    };
}
