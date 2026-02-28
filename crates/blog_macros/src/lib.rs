//! Procedural macros for embedding files in the blog application.
//!
//! This crate provides macros to embed file contents at compile time,
//! eliminating the need for generated `.rs` files.
//!
//! ## Macros
//!
//! - [`embed_file_map!`]: Creates a lookup from basename to file bytes
//! - [`embed_file_array!`]: Creates an array of file contents as strings
//!
//! ## Examples
//!
//! ```rust
//! use blog_macros::{embed_file_map, embed_file_array};
//!
//! // Create a map from hash to SVG bytes
//! let get_svg_bytes = embed_file_map!("../../assets/math/", pattern = "*.svg");
//! let svg_bytes = get_svg_bytes("some_hash"); // Returns Option<&'static [u8]>
//!
//! // Create an array of markdown file contents
//! let post_contents = embed_file_array!("../../posts/", pattern = "*.md");
//! // post_contents is &[&'static str]
//! ```

mod utils;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, LitStr};

/// Input for the `embed_file_map!` macro.
struct EmbedFileMapInput {
    /// Relative directory path (e.g., "../../assets/math/")
    relative_dir: String,
    /// Glob pattern (e.g., "*.svg")
    pattern: String,
}

impl syn::parse::Parse for EmbedFileMapInput {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        // Parse relative directory path
        let relative_dir: LitStr = input.parse()?;

        // Parse comma separator
        input.parse::<syn::Token![,]>()?;

        // Parse "pattern = "
        let ident: syn::Ident = input.parse()?;
        if ident != "pattern" {
            return Err(syn::Error::new(ident.span(), "expected 'pattern' keyword"));
        }

        input.parse::<syn::Token![=]>()?;

        // Parse pattern string
        let pattern: LitStr = input.parse()?;

        // Ensure no trailing tokens
        if !input.is_empty() {
            return Err(syn::Error::new(
                input.span(),
                "unexpected tokens after pattern",
            ));
        }

        Ok(Self {
            relative_dir: relative_dir.value(),
            pattern: pattern.value(),
        })
    }
}

/// Embed files from a directory and create a lookup map from basename to file bytes.
///
/// # Syntax
/// `embed_file_map!(relative_dir, pattern = "*.svg")`
///
/// # Arguments
/// - `relative_dir`: Relative path from the source file to the target directory
///   (e.g., `"../../assets/math/"`).
/// - `pattern`: Glob pattern to match files (e.g., `"*.svg"`).
///
/// # Returns
/// An expression that implements `Fn(&str) -> Option<&'static [u8]>`.
///
/// # Example
/// ```rust
/// let get_svg_bytes = embed_file_map!("../../assets/math/", pattern = "*.svg");
/// let svg_bytes = get_svg_bytes("some_hash"); // Returns Option<&'static [u8]>
/// ```
#[proc_macro]
pub fn embed_file_map(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as EmbedFileMapInput);

    // Validate inputs
    if let Err(e) = utils::validate_relative_dir(&input.relative_dir) {
        return syn::Error::new(proc_macro2::Span::call_site(), e.to_string())
            .to_compile_error()
            .into();
    }

    // Scan directory for matching files
    let files = match utils::scan_directory(&input.relative_dir, &input.pattern) {
        Ok(files) => files,
        Err(e) => {
            return syn::Error::new(proc_macro2::Span::call_site(), e.to_string())
                .to_compile_error()
                .into();
        }
    };

    // Generate match arms for each file
    let mut match_arms = Vec::new();

    for file in &files {
        let basename = &file.basename;
        let include_path = utils::include_path(&input.relative_dir, &file.filename);

        match_arms.push(quote! {
            #basename => Some(include_bytes!(#include_path)),
        });
    }

    // Generate the closure
    let result = if match_arms.is_empty() {
        // No files found - return a closure that always returns None
        quote! {
            |_key: &str| -> Option<&'static [u8]> {
                None
            }
        }
    } else {
        quote! {
            |key: &str| -> Option<&'static [u8]> {
                match key {
                    #(#match_arms)*
                    _ => None,
                }
            }
        }
    };

    result.into()
}

/// Input for the `embed_file_array!` macro.
struct EmbedFileArrayInput {
    /// Relative directory path (e.g., "../../posts/")
    relative_dir: String,
    /// Glob pattern (e.g., "*.md")
    pattern: String,
}

impl syn::parse::Parse for EmbedFileArrayInput {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        // Parse relative directory path
        let relative_dir: LitStr = input.parse()?;

        // Parse comma separator
        input.parse::<syn::Token![,]>()?;

        // Parse "pattern = "
        let ident: syn::Ident = input.parse()?;
        if ident != "pattern" {
            return Err(syn::Error::new(ident.span(), "expected 'pattern' keyword"));
        }

        input.parse::<syn::Token![=]>()?;

        // Parse pattern string
        let pattern: LitStr = input.parse()?;

        // Ensure no trailing tokens
        if !input.is_empty() {
            return Err(syn::Error::new(
                input.span(),
                "unexpected tokens after pattern",
            ));
        }

        Ok(Self {
            relative_dir: relative_dir.value(),
            pattern: pattern.value(),
        })
    }
}

/// Embed files from a directory and create an array of file contents as strings.
///
/// # Syntax
/// `embed_file_array!(relative_dir, pattern = "*.md")`
///
/// # Arguments
/// - `relative_dir`: Relative path from the source file to the target directory
///   (e.g., `"../../posts/"`).
/// - `pattern`: Glob pattern to match files (e.g., `"*.md"`).
///
/// # Returns
/// An expression of type `&[&'static str]` containing the file contents.
///
/// # Example
/// ```rust
/// let post_contents = embed_file_array!("../../posts/", pattern = "*.md");
/// // post_contents is &[&'static str]
/// ```
#[proc_macro]
pub fn embed_file_array(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as EmbedFileArrayInput);

    // Validate inputs
    if let Err(e) = utils::validate_relative_dir(&input.relative_dir) {
        return syn::Error::new(proc_macro2::Span::call_site(), e.to_string())
            .to_compile_error()
            .into();
    }

    // Scan directory for matching files
    let files = match utils::scan_directory(&input.relative_dir, &input.pattern) {
        Ok(files) => files,
        Err(e) => {
            return syn::Error::new(proc_macro2::Span::call_site(), e.to_string())
                .to_compile_error()
                .into();
        }
    };

    // Generate include_str! calls for each file
    let mut include_items = Vec::new();

    for file in &files {
        let include_path = utils::include_path(&input.relative_dir, &file.filename);
        include_items.push(quote! {
            include_str!(#include_path),
        });
    }

    // Generate the array
    let result = if include_items.is_empty() {
        // No files found - return empty array
        quote! {
            &[]
        }
    } else {
        quote! {
            &[
                #(#include_items)*
            ]
        }
    };

    result.into()
}
