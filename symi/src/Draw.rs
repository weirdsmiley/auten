//! Draw : A trait for debugging and utility purposes. Symbols implement this
//! trait.

/// Handle all kinds of formatting apart from fmt::Display here.
use std::fmt;

// Rust allows to implement two traits with two exactly same methods(and
// its signatures) but it prefers one over the other.
//
// GH Issue: https://github.com/rust-lang/rust/issues/26080
// Docs:
// https://doc.rust-lang.org/book/ch19-03-advanced-traits.html#fully-qualified-syntax-for-disambiguation-calling-methods-with-the-same-name
// https://doc.rust-lang.org/std/fmt/trait.Write.html
// TODO: Do we need public or only declare to be public?
pub trait Draw: fmt::Display {
    /// Dump type in a tree-like structure.
    // TODO : Implementing Write trait can be beneficial. There are two traits
    // one in fmt::Write, other in io::Write
    // https://doc.rust-lang.org/std/fmt/trait.Write.html
    fn dump(&self, tabl: Option<usize>) -> String;

    /// Dump type's declaration(s) into f Formatter.
    // FIXME: We need to move this to another trait which will not be
    // implemented by concrete types. Because it does not make sense to have a
    // declaration of concrete literals.
    fn declare(&self) -> String;

    // fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result;

    type OutputType;
    fn iter(&self) -> (bool, &Self::OutputType);
}
