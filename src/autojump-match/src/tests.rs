use std::path;

use super::matcher::*;
use super::re_based::*;


macro_rules! assert_re {
    ($fn_name: ident, $x: tt, $y: expr) => {
        assert_eq!($fn_name(&vec! $x), $y);
    };
}


#[cfg(unix)]
#[test]
fn test_re_match_anywhere() {
    macro_rules! a {
        ($x: tt, $y: expr) => {
            assert_re!(re_match_anywhere, $x, $y);
        };
    }

    a!(["foo"], r".*foo.*");
    a!(["foo", "baz"], r".*foo.*baz.*");
    a!(["测试", "baz"], r".*\x{6d4b}\x{8bd5}.*baz.*");
}


#[cfg(unix)]
#[test]
fn test_re_match_consecutive() {
    macro_rules! a {
        ($x: tt, $y: expr) => {
            assert_re!(re_match_consecutive, $x, $y);
        };
    }

    a!(["foo"], r"foo[^/]*$");
    a!(["foo", "baz"], r"foo[^/]*/[^/]*baz[^/]*$");
    a!(["测试", "baz"], r"\x{6d4b}\x{8bd5}[^/]*/[^/]*baz[^/]*$");
}


#[test]
fn test_matcher() {
    let needles = vec!["foo", "baz"];
    let matcher = Matcher::new(needles, false);

    let haystack = vec![
        path::Path::new("/foo/bar/baz"),
        path::Path::new("/moo/foo/baz"),
        path::Path::new("/baz/foo/bar"),
        path::Path::new("/moo/baz/foo"),
        path::Path::new("/foo/baz"),
    ];

    assert_eq!(
        matcher.execute(&haystack),
        [
            // consecutive matcher
            path::Path::new("/moo/foo/baz"),
            path::Path::new("/foo/baz"),
            // anywhere matcher
            path::Path::new("/foo/bar/baz"),
            path::Path::new("/moo/foo/baz"),
            path::Path::new("/foo/baz"),
        ]);
}