#[macro_export]
macro_rules! require_with_opt {
    ($expression:expr, $error_msg:expr) => {
        if (!($expression)) {
            return MultiValue2((sc_error!($error_msg), OptionalValue::None))
        }
    };
}

#[macro_export]
macro_rules! Ok_some {
    ($some:expr) => {
        return MultiValue2((Ok(()), OptionalValue::Some($some)))
    };
}