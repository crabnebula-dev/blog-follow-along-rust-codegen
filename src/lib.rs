pub use phf; // re-export phf so we can use it later

type Map = phf::Map<&'static str, &'static str>;

/// Container for compile-time embedded assets.
pub struct Assets(Map);

impl From<Map> for Assets {
    fn from(value: Map) -> Self {
        Self(value)
    }
}

impl Assets {
    /// Get the contents of the specified asset path.
    pub fn get(&self, path: &str) -> Option<&str> {
        self.0.get(path).copied()
    }
}
