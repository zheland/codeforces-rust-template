pub use util_lazy_fmt::*;
mod util_lazy_fmt {
    #[macro_export]
    macro_rules! lazy_fmt {
        () => {{ "" }};
        ( @impl ($($pattern:literal),*) $(, ($($args:tt)*))? ) => {{
            use ::core::fmt::{Debug, Display, Formatter, Result};

            #[derive(Clone, Copy)]
            struct LazyFormat<F: Fn(&mut Formatter<'_>) -> Result>(F);

            impl<F: Fn(&mut Formatter<'_>) -> Result> Debug for LazyFormat<F> {
                #[inline]
                fn fmt(&self, f: &mut Formatter<'_>) -> Result {
                    f.write_str(concat!("lazy_format(", stringify!(
                        ::core::concat!($($pattern)*),
                        $(, $($args)*)?), ")"))?;
                    (self.0)(f)
                }
            }

            impl<F: Fn(&mut Formatter<'_>) -> Result> Display for LazyFormat<F> {
                #[inline]
                fn fmt(&self, f: &mut Formatter<'_>) -> Result {
                    (self.0)(f)
                }
            }

            LazyFormat(#[inline] move |fmt: &mut Formatter<'_>| -> Result {
                ::core::write!(fmt, ::core::concat!($($pattern),*) $(, $($args)*)?)
            })
        }};
        ( @impl $($pattern:literal: $args:expr),* ) => {{
            $crate::lfmt!( @impl ($($pattern),*), ($($args),*) )
        }};
        ( $pattern:literal $(, $($args:tt)*)? ) => {{
            $crate::lfmt!( @impl ($pattern) $(, ($($args)*))? )
        }};
        ( , $arg:expr $(, $args:expr)* ) => {{
            $crate::lfmt!( @impl "{}": $arg $(, " {}": $args)* )
        }};
    }
}
