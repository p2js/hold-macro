#![no_std]

#[macro_export]
macro_rules! hold {
    ($($vars:ident),+ $(; $($msg:tt)+)?) => {{
        let __unused = ($($vars),*);
        ::core::todo!($($($msg)+)?);
    }};
    ($($vars:ident),*) => {{
        let __unused = ($($vars),*);
        ::core::todo!();
    }};
    ($($msg:tt)+) => {{
        ::core::todo!($($msg)+);
    }}
}
