/// A lending conversion trait
///
/// For lending a type as another.  Should be a lightweight operation.
pub trait LendAs {
    /// Type converting from
    type From<'a>;
    /// Type converting into
    type Into<'a>;

    /// Lend a type to another type.
    fn lend_as(self, from: Self::From<'_>) -> Self::Into<'_>;
}
