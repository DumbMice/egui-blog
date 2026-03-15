//! Custom SVG loader that understands resolution scaling from URI patterns.
//!
//! URIs format: `bytes://math/{hash}@2x.svg` where `@2x` indicates 2x resolution scaling.
//! The loader strips the resolution suffix, loads the SVG bytes, and renders them
//! at the appropriate resolution using `SizeHint::Scale`.

use std::{
    mem::size_of,
    sync::{
        atomic::{AtomicU64, Ordering::Relaxed},
        Arc,
    },
};

use egui::{
    load::{BytesPoll, ImageLoadResult, ImageLoader, ImagePoll, LoadError, SizeHint},
    mutex::Mutex,
    ColorImage,
};
use egui_extras;
use std::collections::HashMap;

struct Entry {
    last_used: AtomicU64,
    result: Result<Arc<ColorImage>, String>,
}

/// Custom SVG loader that understands resolution scaling from URI patterns.
///
/// URIs format: `bytes://math/{hash}@2x.svg` where `@2x` indicates 2x resolution scaling.
/// The `@` and `x` suffix are stripped to get the base hash, then the SVG is loaded
/// and rendered with `SizeHint::Scale(resolution)`.
pub struct ResolutionAwareSvgLoader {
    pass_index: AtomicU64,
    cache: Mutex<HashMap<String, HashMap<SizeHint, Entry>>>,
    options: resvg::usvg::Options<'static>,
}

impl ResolutionAwareSvgLoader {
    pub const ID: &'static str = egui::generate_loader_id!(ResolutionAwareSvgLoader);

    /// Create a new loader with default options.
    pub fn new() -> Self {
        // Use default options (same as egui_extras::loaders::svg_loader::SvgLoader)
        #[expect(unused_mut)]
        let mut options = resvg::usvg::Options::default();

        Self {
            pass_index: AtomicU64::new(0),
            cache: Mutex::new(HashMap::default()),
            options,
        }
    }

    /// Check if a URI is supported by this loader.
    ///
    /// Supports URIs ending with `.svg` and optionally containing `@` for resolution.
    fn is_supported(uri: &str) -> bool {
        uri.ends_with(".svg") && uri.contains("bytes://math/")
    }

    /// Extract resolution scale from URI.
    ///
    /// URI pattern: `bytes://math/{hash}@{resolution}x.svg`
    /// Returns (`base_hash`, `resolution_scale`)
    fn extract_resolution(uri: &str) -> Option<(String, f32)> {
        // Strip "bytes://math/" prefix
        let prefix = "bytes://math/";
        if !uri.starts_with(prefix) {
            return None;
        }

        let rest = &uri[prefix.len()..];

        // Find @ symbol for resolution
        if let Some(at_pos) = rest.find('@') {
            let hash = &rest[..at_pos];
            let after_at = &rest[at_pos + 1..];

            // Find x before .svg
            if let Some(x_pos) = after_at.find('x') {
                let resolution_str = &after_at[..x_pos];
                let resolution = resolution_str.parse::<f32>().ok()?;

                // Reconstruct base URI without resolution suffix
                let base_uri = format!("{prefix}{hash}.svg");

                Some((base_uri, resolution))
            } else {
                None
            }
        } else {
            // No resolution suffix, default to 1.0
            Some((uri.to_owned(), 1.0))
        }
    }
}

impl Default for ResolutionAwareSvgLoader {
    fn default() -> Self {
        Self::new()
    }
}

impl ImageLoader for ResolutionAwareSvgLoader {
    fn id(&self) -> &str {
        Self::ID
    }

    fn load(&self, ctx: &egui::Context, uri: &str, size_hint: SizeHint) -> ImageLoadResult {
        if !Self::is_supported(uri) {
            return Err(LoadError::NotSupported);
        }

        // Extract resolution from URI
        let Some((base_uri, resolution_scale)) = Self::extract_resolution(uri) else {
            return Err(LoadError::NotSupported);
        };

        // Combine URI resolution with size_hint
        // If size_hint is Scale(s), we multiply by resolution_scale
        // If size_hint is something else, we use it as-is (display size determines resolution)
        let effective_size_hint = match size_hint {
            SizeHint::Scale(scale) => {
                // Multiply scale by resolution factor
                SizeHint::Scale((scale.0 * resolution_scale).into())
            }
            _ => {
                // For other size hints, resolution_scale is ignored
                // The display size determines the resolution
                size_hint
            }
        };

        let mut cache = self.cache.lock();
        let bucket = cache.entry(uri.to_owned()).or_default();

        if let Some(entry) = bucket.get(&effective_size_hint) {
            entry
                .last_used
                .store(self.pass_index.load(Relaxed), Relaxed);
            match entry.result.clone() {
                Ok(image) => Ok(ImagePoll::Ready { image }),
                Err(err) => Err(LoadError::Loading(err)),
            }
        } else {
            match ctx.try_load_bytes(&base_uri) {
                Ok(BytesPoll::Ready { bytes, .. }) => {
                    log::trace!("Started loading {uri:?} (resolution: {resolution_scale}x)");
                    let result = egui_extras::image::load_svg_bytes_with_size(
                        &bytes,
                        effective_size_hint,
                        &self.options,
                    )
                    .map(Arc::new);

                    log::trace!("Finished loading {uri:?}");
                    bucket.insert(
                        effective_size_hint,
                        Entry {
                            last_used: AtomicU64::new(self.pass_index.load(Relaxed)),
                            result: result.clone(),
                        },
                    );
                    match result {
                        Ok(image) => Ok(ImagePoll::Ready { image }),
                        Err(err) => Err(LoadError::Loading(err)),
                    }
                }
                Ok(BytesPoll::Pending { size }) => Ok(ImagePoll::Pending { size }),
                Err(err) => Err(err),
            }
        }
    }

    fn forget(&self, uri: &str) {
        let mut cache = self.cache.lock();
        cache.remove(uri);
    }

    fn forget_all(&self) {
        let mut cache = self.cache.lock();
        cache.clear();
    }

    fn byte_size(&self) -> usize {
        let cache = self.cache.lock();
        let mut total = 0;

        #[expect(clippy::iter_over_hash_type)]
        for bucket in cache.values() {
            #[expect(clippy::iter_over_hash_type)]
            for entry in bucket.values() {
                total += size_of::<Entry>();
                if let Ok(image) = &entry.result {
                    total += image.pixels.len() * std::mem::size_of::<egui::Color32>();
                }
            }
        }

        total
    }
}
