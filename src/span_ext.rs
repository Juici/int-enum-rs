use proc_macro2::Span;

pub trait SpanExt {
    fn start(&self) -> Span;
    fn end(&self) -> Span;
}

#[cfg(proc_macro_span)]
impl SpanExt for Span {
    fn start(&self) -> Span {
        std::panic::catch_unwind(|| Span::from(self.unwrap().start())).unwrap_or(*self)
    }

    fn end(&self) -> Span {
        std::panic::catch_unwind(|| Span::from(self.unwrap().end())).unwrap_or(*self)
    }
}

#[cfg(not(proc_macro_span))]
impl SpanExt for Span {
    fn start(&self) -> Span {
        *self
    }

    fn end(&self) -> Span {
        *self
    }
}
