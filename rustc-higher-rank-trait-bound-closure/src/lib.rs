trait Foo {
    type Content<'a>;
}

mod fails {
    /// Fails (I think correctly) but with misleading error message
    /// ```text
    /// error[E0582]: binding for associated type `Output` references lifetime `'a`, which does not appear in the trait input types
    ///   --> src/lib.rs:11:42
    ///    |
    /// 18 |         F: for<'a> Fn(X::Content<'a>) -> &'a i32
    ///    |                                          ^^^^^^^
    /// ```
    /// 
    /// The issue: At first glance `'a` *does* appear in the trait input type, except we cannot know `'a` 
    /// will be part of Foo::Content, so there is no guarentee after monomorphisation that `'a` will be present.
    /// 
    /// Suggestion: Add this kind of example to the 
    #[allow(dead_code)]
    struct Bar<X, F>
    where
        X: super::Foo,
        F: for<'a> Fn(X::Content<'a>) -> &'a i32
    {
        x: X,
        f: F
    }
}

mod correct {
    /// Correct code by passing a dummy parameter that is a reference
    #[allow(dead_code)]
    struct Bar<X, F>
    where
        X: super::Foo,
        F: for<'a> Fn(X::Content<'a>, /* dummy param */ &'a ()) -> &'a i32
    {
        x: X,
        f: F
    }
}

