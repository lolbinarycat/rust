// Regression test for <https://github.com/rust-lang/rust/issues/154694>.
// The goal is to ensure that declarative macros re-exported by name
// inherit the `#[doc(inline)]` attribute from intermediate re-exports,
// matching the behavior of glob re-exports.

#![crate_name = "foo"]

#[macro_use]
mod macros {
    #[macro_export]
    #[doc(hidden)]
    macro_rules! explicit_macro {
        () => {};
    }

    #[macro_export]
    #[doc(hidden)]
    macro_rules! wild_macro {
        () => {};
    }
}

pub mod bar {
    mod hidden_explicit {
        #[doc(inline)]
        pub use crate::explicit_macro;
    }

    mod hidden_wild {
        #[doc(inline)]
        pub use crate::wild_macro;
    }

    // First, we check that the explicitly named macro inherits the inline attribute
    // from `hidden_explicit` and is successfully rendered.
    //@ has 'foo/bar/macro.explicit_macro.html'
    //@ has 'foo/bar/index.html' '//a[@href="macro.explicit_macro.html"]' 'explicit_macro'
    pub use self::hidden_explicit::explicit_macro;

    // Next, we ensure that the glob-imported macro continues to render correctly
    // as a control case.
    //@ has 'foo/bar/macro.wild_macro.html'
    //@ has 'foo/bar/index.html' '//a[@href="macro.wild_macro.html"]' 'wild_macro'
    pub use self::hidden_wild::*;
}
