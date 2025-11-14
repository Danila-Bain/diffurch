// Source - https://stackoverflow.com/a
// https://stackoverflow.com/questions/55951472/how-to-replace-one-identifier-in-an-expression-with-another-one-via-rust-macro
// Posted by Lukas Kalbertodt, modified by community. See post 'Timeline' for change history
// Retrieved 2025-11-14, License - CC BY-SA 4.0

#[macro_export]
macro_rules! replace {
    // This is the "public interface". The only thing we do here is to delegate
    // to the actual implementation. The implementation is more complicated to
    // call, because it has an "out" parameter which accumulates the token we
    // will generate.
    ($x:ident, $y:expr, $($e:tt)*) => {
        replace!(@impl $x, $y, [], $($e)*)
    };

    // Recursion stop: if there are no tokens to check anymore, we just emit
    // what we accumulated in the out parameter so far.
    (@impl $x:ident, $y:expr, [$($out:tt)*], ) => {
        $($out)*
    };

    // This is the arm that's used when the first token in the stream is an
    // identifier. We potentially replace the identifier and push it to the
    // out tokens.
    (@impl $x:ident, $y:expr, [$($out:tt)*], $head:ident $($tail:tt)*) => {{
        replace!(
            @impl $x, $y, 
            [$($out)* replace!(@replace $x, $y, $head)],
            $($tail)*
        )
    }};

    // These arms are here to recurse into "groups" (tokens inside of a 
    // (), [] or {} pair)
    (@impl $x:ident, $y:expr, [$($out:tt)*], ( $($head:tt)* ) $($tail:tt)*) => {{
        replace!(
            @impl $x, $y, 
            [$($out)* ( replace!($x, $y, $($head)*) ) ], 
            $($tail)*
        )
    }};
    (@impl $x:ident, $y:expr, [$($out:tt)*], [ $($head:tt)* ] $($tail:tt)*) => {{
        replace!(
            @impl $x, $y, 
            [$($out)* [ replace!($x, $y, $($head)*) ] ], 
            $($tail)*
        )
    }};
    (@impl $x:ident, $y:expr, [$($out:tt)*], { $($head:tt)* } $($tail:tt)*) => {{
        replace!(
            @impl $x, $y, 
            [$($out)* { replace!($x, $y, $($head)*) } ], 
            $($tail)*
        )
    }};

    // This is the standard recusion case: we have a non-identifier token as
    // head, so we just put it into the out parameter.
    (@impl $x:ident, $y:expr, [$($out:tt)*], $head:tt $($tail:tt)*) => {{
        replace!(@impl $x, $y, [$($out)* $head], $($tail)*)
    }};

    // Helper to replace the identifier if its the needle. 
    (@replace $needle:ident, $replacement:expr, $i:ident) => {{
        // This is a trick to check two identifiers for equality. Note that 
        // the patterns in this macro don't contain any meta variables (the 
        // out meta variables $needle and $i are interpolated).
        macro_rules! __inner_helper {
            // Identifiers equal, emit $replacement
            ($needle $needle) => { $replacement };
            // Identifiers not equal, emit original
            ($needle $i) => { $i };                
        }

        __inner_helper!($needle $i)
    }}
}

#[cfg(test)]
mod test {

    #[test]
    fn main() {
        let foo = 3;
        let bar = 7;
        let z = 5;

        assert_eq!(705, replace!(abc, foo, bar * 100 + z));  // no replacement
        assert_eq!(305, replace!(bar, foo, bar * 100 + z));  // replace `bar` with `foo`
        assert_eq!(405, replace!(bar, 4, bar * 100 + z));  // replace `bar` with `foo`
    }
}
