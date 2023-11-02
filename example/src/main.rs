fn main() {
    // include the assets our build script created
    let assets = include!(concat!(env!("OUT_DIR"), "/assets.rs"));

    let index = assets.get("index.html").unwrap();
    let loader = assets.get(index).unwrap();
    let dashboard = assets.get(loader).unwrap();

    assert_eq!(dashboard, "console.log('dashboard stuff')");
}
