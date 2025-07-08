use crate::grammar::expression::ExpressionType;

// TODO: decide whether ExpressionType should be a recursive type/enum for parsing
// or do the recursive parsing via functions like in the book.
// Recursive types/enums were a pain to debug in the Grep challenge because LLDB would crash
// with a SIGBUS/SIGTERM/SIGSEGV when dereferencing a Box type.
// This will be fixed in RustRover 2025.2
pub struct AbstractSyntaxTree {
    root: ExpressionType
}
