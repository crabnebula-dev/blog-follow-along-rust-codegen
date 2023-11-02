fn main() {
    // include the assets our build script created
    let assets = include!(concat!(env!("OUT_DIR"), "/assets.rs"));

    let index = assets.fetch("index.html").unwrap();
    let loader = assets.fetch(index).unwrap();
    let dashboard = assets.fetch(loader).unwrap();

    assert_eq!(dashboard, "console.log('dashboard stuff')");
}
