///Implementation of unwrap for custom enums
///
/// ## Example
/// ```
/// pub enum Example {
///    A(u8),
///    B(String),
///}
///
///impl Unwrap for Example {
///    fn unwrap<T: 'static>(&self) -> T {
///        let n: Box<dyn Any> = match *self {
///            IInts::A(n) => Box::new(n),
///            IInts::B(n) => Box::new(n),
///        };
///
///        return *n.downcast::<T>().unwrap();
///    }
///}
/// ```
pub trait Unwrap {
    fn unwrap<T: 'static>(&self) -> T;
}
