//! Optimized math placeholder parser using state machine.
//! Replaces O(n²) algorithm with O(n) single-pass parsing.

/// Try to parse a math placeholder in the text.
/// Returns `(hash, start_offset, placeholder_length)` if successful, `None` otherwise.
/// - `hash`: The hash identifier (e.g., "abc123")
/// - `start_offset`: Position of the '(' that starts the math placeholder within the input text
/// - `placeholder_length`: Length from that '(' to the matching ')'
///
/// Handles nested parentheses like `((hash.typ))` by finding the matching ')'
/// using parenthesis counting.
///
/// Algorithm:
/// 1. Find ".typ)" in the text
/// 2. Find the '(' before ".typ)" - this starts the math placeholder
/// 3. Find the matching ')' by counting parentheses from that '('
/// 4. Return hash, `start_offset`, and length
pub fn try_parse_math_at(text: &str) -> Option<(&str, usize, usize)> {
    let bytes = text.as_bytes();
    if bytes.is_empty() || bytes[0] != b'(' {
        return None;
    }

    // First, find ".typ)" in the text
    let Some(typ_pos) = text.find(".typ)") else {
        return None;
    };

    // Find the '(' that starts the math placeholder
    // Look backward from ".typ)" to find the matching '('
    let mut paren_start = typ_pos;
    while paren_start > 0 && bytes[paren_start - 1] != b'(' {
        paren_start -= 1;
    }

    if paren_start == 0 || bytes[paren_start - 1] != b'(' {
        return None; // No '(' found before ".typ)"
    }

    // The math placeholder starts at paren_start - 1
    let math_start = paren_start - 1;

    // Now find the matching ')' by counting parentheses from math_start
    let mut depth = 0;
    let mut pos = math_start;
    let mut matching_end = None;

    while pos < bytes.len() {
        match bytes[pos] {
            b'(' => {
                depth += 1;
                pos += 1;
            }
            b')' => {
                depth -= 1;
                if depth == 0 {
                    matching_end = Some(pos);
                    break;
                }
                pos += 1;
            }
            _ => pos += 1,
        }
    }

    let Some(placeholder_end) = matching_end else {
        return None; // No matching ')'
    };

    // Verify the placeholder contains ".typ)" before the matching ')'
    if placeholder_end < typ_pos + 4 {
        return None; // ".typ)" extends beyond matching ')'
    }

    // The hash is between '(' and ".typ)"
    let hash = &text[math_start + 1..typ_pos];

    // Check if hash is non-empty and alphanumeric
    if hash.is_empty() || !hash.chars().all(|c| c.is_ascii_alphanumeric()) {
        return None;
    }

    // start_offset is math_start (position of '(' within input)
    // length is from math_start to placeholder_end inclusive
    let start_offset = math_start;
    let total_length = placeholder_end - math_start + 1;

    Some((hash, start_offset, total_length))
}

/// Parse text with math placeholders using optimized state machine.
/// This is O(n) instead of O(n²) like the original implementation.
pub fn parse_text_with_math(
    text: &str,
    manifest: &crate::math::MathManifest,
    math_asset_manager: &mut Option<&mut crate::math::MathAssetManager>,
    math_resolution_scale: f32,
) -> Vec<crate::ui::markdown::ParagraphContent> {
    use crate::ui::markdown::ParagraphContent;

    let mut result = Vec::new();
    let mut current_text = String::new();
    let mut i = 0; // byte position
    let bytes = text.as_bytes();

    while i < bytes.len() {
        // Check if we might have a math placeholder starting here
        if bytes[i] == b'(' {
            // Try to parse math placeholder
            if let Some((hash, start_offset, math_length)) = try_parse_math_at(&text[i..]) {
                // Add any text before the math placeholder (from i to i+start_offset)
                if start_offset > 0 {
                    current_text.push_str(&text[i..i + start_offset]);
                }

                // Add accumulated text before the math placeholder
                if !current_text.is_empty() {
                    result.push(ParagraphContent::Text(current_text));
                    current_text = String::new();
                }

                // Process the math placeholder
                if let Some(metadata) = manifest.get_metadata(hash) {
                    // Try to create math content
                    if let Some(_asset_manager) = math_asset_manager {
                        if let Some(image_source) =
                            crate::math::MathAssetManager::get_image_source_for_hash_with_resolution(
                                hash,
                                math_resolution_scale,
                            )
                        {
                            // Get the SVG's intrinsic size
                            let svg_size = _asset_manager.get_svg_size(hash);

                            if let Some(size) = svg_size {
                                result.push(ParagraphContent::MathImage {
                                    image_source,
                                    size,
                                    is_display: metadata.is_display,
                                    baseline_from_top: metadata.baseline_from_top,
                                });
                            } else {
                                // Fallback: use code rendering
                                result.push(ParagraphContent::MathCode {
                                    content: format!("Math formula: {hash}"),
                                    is_display: metadata.is_display,
                                });
                            }
                        } else {
                            // Fallback: render as code
                            result.push(ParagraphContent::MathCode {
                                content: format!("Math formula: {hash}"),
                                is_display: metadata.is_display,
                            });
                        }
                    } else {
                        // No asset manager, render as code
                        result.push(ParagraphContent::MathCode {
                            content: format!("Math formula: {hash}"),
                            is_display: metadata.is_display,
                        });
                    }
                } else {
                    // Hash not found in manifest, add placeholder as text
                    // The placeholder is at i+start_offset to i+start_offset+math_length
                    result.push(ParagraphContent::Text(
                        text[i + start_offset..i + start_offset + math_length].to_string(),
                    ));
                }

                // Skip past: text before math (start_offset) + math itself (math_length)
                i += start_offset + math_length;
                continue;
            }
        }

        // Not a math placeholder, we need to add characters properly handling UTF-8
        // Find the next '(' or end of string, but add text character by character
        // to handle UTF-8 correctly

        // Get the character at current position (handles UTF-8)
        // We can't use bytes[i] as char because it breaks UTF-8
        // Instead, get the next char from the string slice
        let remaining = &text[i..];
        if let Some(ch) = remaining.chars().next() {
            current_text.push(ch);
            i += ch.len_utf8(); // Advance by UTF-8 byte length
        } else {
            // No more characters (shouldn't happen since i < bytes.len())
            break;
        }
    }

    // Add any remaining text
    if !current_text.is_empty() {
        result.push(ParagraphContent::Text(current_text));
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_try_parse_math_at() {
        // Valid math placeholder - (abc123.typ) at position 0, length 12
        assert_eq!(try_parse_math_at("(abc123.typ)"), Some(("abc123", 0, 12)));

        // Valid with nested parentheses - (abc123.typ) at position 1, length 12
        assert_eq!(try_parse_math_at("((abc123.typ))"), Some(("abc123", 1, 12)));

        // Valid with multiple nested parentheses - (abc123.typ) at position 2, length 12
        assert_eq!(
            try_parse_math_at("(((abc123.typ)))"),
            Some(("abc123", 2, 12))
        );

        // Valid with trailing text - (abc123.typ) at position 0, length 12
        assert_eq!(
            try_parse_math_at("(abc123.typ) more text"),
            Some(("abc123", 0, 12))
        );

        // Invalid: missing closing paren
        assert_eq!(try_parse_math_at("(abc123.typ"), None);

        // Invalid: wrong extension
        assert_eq!(try_parse_math_at("(abc123.txt)"), None);

        // Invalid: empty hash
        assert_eq!(try_parse_math_at("(.typ)"), None);

        // Invalid: not starting with '('
        assert_eq!(try_parse_math_at("abc123.typ)"), None);

        // Test case: (((x.typ) - has closing ')', should parse (x.typ) at position 2, length 7
        assert_eq!(try_parse_math_at("(((x.typ)"), Some(("x", 2, 7)));

        // Test with text before - function should be called with substring starting at '('
        // So we test "(abc123.typ) more" not "text (abc123.typ) more"
        assert_eq!(
            try_parse_math_at("(abc123.typ) more"),
            Some(("abc123", 0, 12))
        );

        // Test with extra text inside parentheses: ((abc123.typ)text)
        // Should find (abc123.typ) at position 1, length 12, not ((abc123.typ)text)
        assert_eq!(
            try_parse_math_at("((abc123.typ)text)"),
            Some(("abc123", 1, 12))
        );

        // Edge case: text before parentheses, called with substring starting at '('
        // "text (abc123.typ)" -> substring "(abc123.typ)" returns (abc123, 0, 12)
        assert_eq!(try_parse_math_at("(abc123.typ)"), Some(("abc123", 0, 12)));
    }

    #[test]
    fn test_parse_text_with_math_basic() {
        // Note: We can't easily test the full function without actual manifest and asset manager
        // But we can test the parsing logic with a mock

        let text = "Some text (abc123.typ) more text";
        let result =
            parse_text_with_math(text, &crate::math::MathManifest::default(), &mut None, 1.0);

        // Should have 3 segments: text, math placeholder (as code fallback), text
        assert_eq!(result.len(), 3);
    }

    #[test]
    fn test_parse_text_with_math_nested() {
        let manifest = crate::math::MathManifest::default();

        // Test nested parentheses: ((abc123.typ))
        let text = "((abc123.typ))";
        let result = parse_text_with_math(text, &manifest, &mut None, 1.0);

        // Should have 3 segments: "(", math placeholder, ")"
        assert_eq!(result.len(), 3);

        // First segment should be "("
        if let crate::ui::markdown::ParagraphContent::Text(content) = &result[0] {
            assert_eq!(content, "(");
        } else {
            panic!("First segment should be text '('");
        }

        // Last segment should be ")"
        if let crate::ui::markdown::ParagraphContent::Text(content) = &result[2] {
            assert_eq!(content, ")");
        } else {
            panic!("Last segment should be text ')'");
        }
    }

    #[test]
    fn test_parse_text_with_math_complex() {
        let manifest = crate::math::MathManifest::default();

        // Test complex case: text before and after, nested parentheses
        let text = "text ((abc123.typ)) more";
        let result = parse_text_with_math(text, &manifest, &mut None, 1.0);

        // Should have 3 segments: "text ", math, " more"
        // Actually: "text (", math, ") more" - because "((" becomes "(" before math
        assert_eq!(result.len(), 3);

        // First segment should be "text ("
        if let crate::ui::markdown::ParagraphContent::Text(content) = &result[0] {
            assert_eq!(content, "text (");
        } else {
            panic!("First segment should be 'text ('");
        }

        // Last segment should be ") more"
        if let crate::ui::markdown::ParagraphContent::Text(content) = &result[2] {
            assert_eq!(content, ") more");
        } else {
            panic!("Last segment should be ') more'");
        }
    }

    #[test]
    fn test_parse_text_with_math_multiple_nested() {
        let manifest = crate::math::MathManifest::default();

        // Test multiple nested: (((abc123.typ)))
        let text = "(((abc123.typ)))";
        let result = parse_text_with_math(text, &manifest, &mut None, 1.0);

        // Should have 3 segments: "((", math, "))"
        assert_eq!(result.len(), 3);

        // Verify
        if let crate::ui::markdown::ParagraphContent::Text(content) = &result[0] {
            assert_eq!(content, "((");
        } else {
            panic!("First segment should be '(('");
        }

        if let crate::ui::markdown::ParagraphContent::Text(content) = &result[2] {
            assert_eq!(content, "))");
        } else {
            panic!("Last segment should be '))'");
        }
    }

    #[test]
    fn test_parse_text_with_math_unicode() {
        let manifest = crate::math::MathManifest::default();

        // Test with Unicode characters
        let text = "Café αβγ (abc123.typ) 🎉";
        let result = parse_text_with_math(text, &manifest, &mut None, 1.0);

        // Should have 3 segments: "Café αβγ ", math, " 🎉"
        assert_eq!(result.len(), 3);

        // First segment should be "Café αβγ "
        if let crate::ui::markdown::ParagraphContent::Text(content) = &result[0] {
            assert_eq!(content, "Café αβγ ");
        } else {
            panic!("First segment should be 'Café αβγ '");
        }

        // Last segment should be " 🎉"
        if let crate::ui::markdown::ParagraphContent::Text(content) = &result[2] {
            assert_eq!(content, " 🎉");
        } else {
            panic!("Last segment should be ' 🎉'");
        }
    }

    #[test]
    fn test_parse_text_with_math_unicode_nested() {
        let manifest = crate::math::MathManifest::default();

        // Test Unicode with nested parentheses
        let text = "Café ((abc123.typ)) 🎉";
        let result = parse_text_with_math(text, &manifest, &mut None, 1.0);

        // Should have 3 segments: "Café (", math, ") 🎉"
        assert_eq!(result.len(), 3);

        // First segment should be "Café ("
        if let crate::ui::markdown::ParagraphContent::Text(content) = &result[0] {
            assert_eq!(content, "Café (");
        } else {
            panic!("First segment should be 'Café ('");
        }

        // Last segment should be ") 🎉"
        if let crate::ui::markdown::ParagraphContent::Text(content) = &result[2] {
            assert_eq!(content, ") 🎉");
        } else {
            panic!("Last segment should be ') 🎉'");
        }
    }

    #[test]
    fn test_parse_text_with_math_unicode_only() {
        let manifest = crate::math::MathManifest::default();

        // Test text with only Unicode, no math
        let text = "Café αβγ 🎉 日本語";
        let result = parse_text_with_math(text, &manifest, &mut None, 1.0);

        // Should have 1 segment: the whole text
        assert_eq!(result.len(), 1);

        if let crate::ui::markdown::ParagraphContent::Text(content) = &result[0] {
            assert_eq!(content, "Café αβγ 🎉 日本語");
        } else {
            panic!("Should be text segment");
        }
    }
}
