use proc_macro2::Span;

#[cfg(proc_macro_span)]
pub fn end(span: Span) -> Span {
    std::panic::catch_unwind(|| Span::from(span.unwrap().end())).unwrap_or(span)
}

#[cfg(not(proc_macro_span))]
pub fn end(span: Span) -> Span {
    span
}
