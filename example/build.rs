use std::path::Path;

fn main() {
    let assets = Path::new("assets");
    let codegen = match asset_bundler_codegen::codegen(assets) {
        Ok(codegen) => codegen,
        Err(err) => panic!("failed to generate asset bundler codegen: {err}"),
    };

    let out = std::env::var("OUT_DIR").unwrap();
    let out = Path::new(&out).join("assets.rs");
    std::fs::write(out, codegen.as_bytes()).unwrap();
}
