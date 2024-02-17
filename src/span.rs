use proc_macro2::Span;

pub fn end(span: Span) -> Span {
    #[cfg(proc_macro_span)]
    {
        Span::from(span.unwrap().end())
    }
    #[cfg(not(proc_macro_span))]
    {
        span
    }
}
