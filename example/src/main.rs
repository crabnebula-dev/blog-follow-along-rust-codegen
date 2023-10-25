fn main() {
    // include the assets our build script created
    let assets = include!(concat!(env!("OUT_DIR"), "/assets.rs"));

    assert_eq!(Some("foo\n"), assets.fetch("foo"));
    assert_eq!(Some("bar\n"), assets.fetch("deeply/bar"));
    assert_eq!(Some("baz\n"), assets.fetch("deeply/nested/baz"));
    assert!(assets.fetch("404").is_none());
}
