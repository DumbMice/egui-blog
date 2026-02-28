# blog_macros

Procedural macros for embedding files in the blog application.

## Macros

### `embed_file_map!`

Creates a lookup from basename to file bytes.

```rust
use blog_macros::embed_file_map;

// Create a map from hash to SVG bytes
let get_svg_bytes = embed_file_map!("../../assets/math/", pattern = "*.svg");
let svg_bytes = get_svg_bytes("some_hash"); // Returns Option<&'static [u8]>
```

### `embed_file_array!`

Creates an array of file contents as strings.

```rust
use blog_macros::embed_file_array;

// Create an array of markdown file contents
let post_contents = embed_file_array!("../../posts/", pattern = "*.md");
// post_contents is &[&'static str]
```

## Usage

Add to your `Cargo.toml`:

```toml
[dependencies]
blog_macros = { path = "../blog_macros" }
```

## Requirements

- The macros run at compile time and require access to the filesystem.
- Paths are relative to the source file using the macro (same as `include_bytes!()` and `include_str!()`).
- The `pattern` argument uses glob syntax (e.g., `"*.svg"`, `"*.md"`).

## Error Handling

The macros will fail compilation with descriptive errors if:
- The directory doesn't exist
- The glob pattern is invalid
- Files cannot be read

If no files match the pattern, an empty map/array is returned.