#[macro_export]
macro_rules! expect {
    ($expr:expr) => ({
        match $expr {
            ::std::option::Option::Some(x) => x,
            ::std::option::Option::None => return None,
        }
    })
}

#[macro_export]
macro_rules! try_expect {
    ($expr:expr) => ({
        match $expr {
            ::std::result::Result::Ok(x) => x,
            ::std::result::Result::Err(_) => return None,
        }
    })
}
