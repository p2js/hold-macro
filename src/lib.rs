#![crate_name = "hold_macro"]
#![no_std]

#[macro_export]
/// Macro to use during development to mark something as todo, and suppress unused value warnings by using ("Holding") the values.
///
/// # Usage
///
/// The macro can be used identically to rust's `todo!()`, either with or without a formatted message:
/// ```
/// use hold_macro::hold;
///
/// fn example_function_1() {
///     hold!();
/// }
///
/// fn example_function_2() {
///     hold!("must implement {} more features", 5);
/// }
/// ```
///
/// However, if you have some placeholder variables (eg. to make a function signature) that you aren't using yet, you can "hold" those values to suppress any warnings:
/// ```
/// fn multiply(x: i32, y: i32) {
///     hold!(x, y); // No warning !!
/// }
/// ```
/// The macro will accept any identifier.
///
/// If you want to both hold values and leave a message, separate the two with a semicolon:
/// ```
/// fn add(x: i32, y: i32) {
///     hold!(x, y; "Don't know addition yet");
/// }
///
/// fn subtract(x: i32, y: i32) {
///     hold!(x, y; "I'm reading a book to learn arithmetic: {} chapter(s) completed", 0);
/// }
/// ```
///
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
