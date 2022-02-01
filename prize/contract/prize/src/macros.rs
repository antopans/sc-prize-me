#[macro_export]
macro_rules! require_with_opt {
    ($expression:expr, $error_msg:expr) => {
        if (!($expression)) {
            return MultiArg2((sc_error!($error_msg), OptionalArg::None))
        }
    };
}

#[macro_export]
macro_rules! Ok_some {
    ($some:expr) => {
        return MultiArg2((Ok(()), OptionalArg::Some($some)))
    };
}