use quote::{quote, TokenStreamExt};
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
    let mut tokens = quote!();

    // generate the body of the `phf_map!` call we make at the end
    for path in paths {
        // we want to use the relative path as a key but still need to tell
        // `include_str!()` the full path. we use unwrap here, because we use
        // the same base path that we generated the paths with
        let relative = path.strip_prefix(base).unwrap().to_string_lossy();
        let full = path.to_string_lossy();

        tokens.append_all(quote! {
            #relative => include_str!(#full),
        });
    }

    // use the full module resolution syntax to ensure
    // we get the right crate and not clash with other items
    let output = quote! {
        {
            use ::asset_bundler::{Assets, phf::{self, phf_map}};
            Assets::from(phf_map!(#tokens))
        }
    };

    output.to_string()
}
