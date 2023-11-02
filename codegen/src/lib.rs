use quote::quote;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

/// Generate Rust code to create an [`asset-bundler::Asset`] from the passed path.
pub fn codegen(path: &Path) -> std::io::Result<String> {
    // canonicalize also checks if the path exists
    // which is the only case that makes sense for us
    let base = path.canonicalize()?;

    let paths = gather_asset_paths(&base);
    Ok(generate_code(&paths, &base))
}

/// Recursively find all files in the passed directory.
fn gather_asset_paths(base: &Path) -> Vec<PathBuf> {
    let mut paths = Vec::new();
    for entry in WalkDir::new(base).into_iter().flatten() {
        // we only care about files, ignore directories
        if entry.file_type().is_file() {
            paths.push(entry.into_path())
        }
    }

    paths
}

/// Generate Rust code to create an [`asset-bundler::Asset`].
fn generate_code(paths: &[PathBuf], base: &Path) -> String {
    let keys = keys(paths, base);
    let values = paths.iter().map(|p| p.to_string_lossy());

    // double brackets to make it a block expression
    let output = quote! {{
        use ::asset_bundler::{Assets, phf::{self, phf_map}};
        Assets::from(phf_map! {
            #( #keys => include_str!(#values) ),*
        })
    }};

    output.to_string()
}

/// Turn paths into relative paths suitable for keys.
fn keys(paths: &[PathBuf], base: &Path) -> Vec<String> {
    let mut keys = Vec::new();

    for path in paths {
        // ignore this failure case for this example
        if let Ok(key) = path.strip_prefix(base) {
            keys.push(key.to_string_lossy().into())
        }
    }

    keys
}
