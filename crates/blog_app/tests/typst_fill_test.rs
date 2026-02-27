//! Test Typst's behavior with #set text(fill: white)
//! Verifies that adding fill: white to Typst templates generates
//! SVGs with white fills AND strokes, enabling simpler post-processing.

use std::collections::HashMap;
use std::fs;
use std::process::Command;
use tempfile::TempDir;

#[derive(Debug)]
struct TypstTest {
    formula: &'static str,
    is_display: bool,
    description: &'static str,
}

const TEST_FORMULAS: [TypstTest; 8] = [
    TypstTest {
        formula: "E = m c^2",
        is_display: false,
        description: "Simple inline",
    },
    TypstTest {
        formula: "1/2 + 1/4 = 3/4",
        is_display: false,
        description: "Fractions",
    },
    TypstTest {
        formula: "integral_(-infinity)^infinity e^(-x^2) dif x = sqrt(pi)",
        is_display: true,
        description: "Integral with sqrt",
    },
    TypstTest {
        formula: "sum_(n=1)^infinity 1/n^2 = pi^2/6",
        is_display: true,
        description: "Summation",
    },
    TypstTest {
        formula: "x = (-b +- sqrt(b^2 - 4 a c)) / (2a)",
        is_display: true,
        description: "Quadratic formula",
    },
    TypstTest {
        formula: "mat(1, 2; 3, 4)",
        is_display: true,
        description: "Matrix",
    },
    TypstTest {
        formula: "sum_(i=1)^n i = n(n+1)/2",
        is_display: false,
        description: "Inline summation",
    },
    TypstTest {
        formula: "a^2 + b^2 = c^2",
        is_display: false,
        description: "Pythagorean theorem",
    },
];

#[derive(Debug, Default)]
struct SvgAnalysis {
    has_white_fill: bool,
    has_white_stroke: bool,
    has_black_fill: bool,
    has_black_stroke: bool,
    has_white_background: bool,
    element_counts: HashMap<String, usize>,
    fill_colors: Vec<String>,
    stroke_colors: Vec<String>,
}

fn get_typst_version() -> String {
    match Command::new("typst").arg("--version").output() {
        Ok(output) => {
            if output.status.success() {
                String::from_utf8_lossy(&output.stdout).trim().to_string()
            } else {
                "Unknown (command failed)".to_string()
            }
        }
        Err(_) => "Not installed".to_string(),
    }
}

fn generate_svg_with_template(
    template: &str,
    formula: &str,
    is_display: bool,
) -> Result<String, String> {
    let temp_dir = TempDir::new().map_err(|e| format!("Failed to create temp dir: {}", e))?;

    let typst_file = temp_dir.path().join("formula.typ");
    let svg_file = temp_dir.path().join("formula.svg");

    // Create Typst content based on template
    let typst_content = if is_display {
        format!("{}\n\n$ {} $", template, formula)
    } else {
        format!("{}\n\n${}$", template, formula)
    };

    fs::write(&typst_file, &typst_content)
        .map_err(|e| format!("Failed to write Typst file: {}", e))?;

    // Run typst CLI
    let output = Command::new("typst")
        .arg("compile")
        .arg(&typst_file)
        .arg(&svg_file)
        .output()
        .map_err(|e| format!("Failed to run Typst CLI: {}", e))?;

    if !output.status.success() {
        let error_msg = String::from_utf8_lossy(&output.stderr);
        return Err(format!("Typst compilation failed: {}", error_msg));
    }

    if !svg_file.exists() {
        return Err("SVG file was not created".to_string());
    }

    let svg_content =
        fs::read_to_string(&svg_file).map_err(|e| format!("Failed to read SVG file: {}", e))?;

    Ok(svg_content)
}

fn analyze_svg_attributes(svg: &str) -> SvgAnalysis {
    let mut analysis = SvgAnalysis::default();
    let lines: Vec<&str> = svg.lines().collect();

    for line in lines {
        let line = line.trim();

        // Check for fill attributes
        if line.contains("fill=") {
            if line.contains("fill=\"#ffffff\"") || line.contains("fill='#ffffff'") {
                analysis.has_white_fill = true;
                analysis.fill_colors.push("#ffffff".to_string());

                // Check if this is likely a background rectangle
                if line.contains("<rect") {
                    // Try to parse rect dimensions to see if it's a full-page background
                    if let Some(start) = line.find("width=") {
                        if let Some(end) = line[start..].find(' ') {
                            let width_str = &line[start + 7..start + end].trim_matches('"');
                            if let Ok(width) = width_str.parse::<f32>() {
                                // If width is large (likely full page), it's probably a background
                                if width > 100.0 {
                                    analysis.has_white_background = true;
                                }
                            }
                        }
                    }
                }
            }
            if line.contains("fill=\"#000000\"")
                || line.contains("fill='#000000'")
                || line.contains("fill=\"#000\"")
                || line.contains("fill='#000'")
            {
                analysis.has_black_fill = true;
                analysis.fill_colors.push("#000000".to_string());
            }
        }

        // Check for stroke attributes
        if line.contains("stroke=") {
            if line.contains("stroke=\"#ffffff\"") || line.contains("stroke='#ffffff'") {
                analysis.has_white_stroke = true;
                analysis.stroke_colors.push("#ffffff".to_string());
            }
            if line.contains("stroke=\"#000000\"")
                || line.contains("stroke='#000000'")
                || line.contains("stroke=\"#000\"")
                || line.contains("stroke='#000'")
            {
                analysis.has_black_stroke = true;
                analysis.stroke_colors.push("#000000".to_string());
            }
        }

        // Count element types
        if line.contains("<path ") {
            *analysis
                .element_counts
                .entry("path".to_string())
                .or_insert(0) += 1;
        }
        if line.contains("<rect ") {
            *analysis
                .element_counts
                .entry("rect".to_string())
                .or_insert(0) += 1;
        }
        if line.contains("<use ") {
            *analysis
                .element_counts
                .entry("use".to_string())
                .or_insert(0) += 1;
        }
        if line.contains("<g ") {
            *analysis.element_counts.entry("g".to_string()).or_insert(0) += 1;
        }
    }

    analysis
}

fn compare_svgs(_baseline: &SvgAnalysis, white_fill: &SvgAnalysis, _formula: &str) -> String {
    let mut result = Vec::new();

    // Check if white-fill version has white elements
    if white_fill.has_white_fill {
        result.push("fills=white".to_string());
    }
    if white_fill.has_white_stroke {
        result.push("strokes=white".to_string());
    }

    // Check for problematic black elements in white-fill version
    let mut problems = Vec::new();
    if white_fill.has_black_fill {
        problems.push("has black fills".to_string());
    }
    if white_fill.has_black_stroke {
        problems.push("has black strokes".to_string());
    }
    if white_fill.has_white_background {
        problems.push("has white background".to_string());
    }

    if problems.is_empty() {
        result.push("no black elements".to_string());
        format!("✓ {}", result.join(", "))
    } else {
        format!("✗ {}: {}", result.join(", "), problems.join(", "))
    }
}

fn test_single_formula(formula: &str, is_display: bool) -> Result<String, String> {
    // Baseline template (current)
    let baseline_template = if is_display {
        r#"#set page(width: auto, height: auto, margin: 0pt)
#set text(size: 16pt)
#show math.equation: set text(top-edge: "bounds", bottom-edge: "bounds")"#
    } else {
        r#"#set page(width: auto, height: auto, margin: 0pt)
#set text(size: 16pt)
#show math.equation: set text(top-edge: "bounds", bottom-edge: "bounds")"#
    };

    // White-fill template
    let white_fill_template = if is_display {
        r#"#set page(width: auto, height: auto, margin: 0pt)
#set text(size: 16pt, fill: white)
#show math.equation: set text(top-edge: "bounds", bottom-edge: "bounds")"#
    } else {
        r#"#set page(width: auto, height: auto, margin: 0pt)
#set text(size: 16pt, fill: white)
#show math.equation: set text(top-edge: "bounds", bottom-edge: "bounds")"#
    };

    // Generate SVGs
    let baseline_svg = generate_svg_with_template(baseline_template, formula, is_display)?;
    let white_fill_svg = generate_svg_with_template(white_fill_template, formula, is_display)?;

    // Analyze both
    let baseline_analysis = analyze_svg_attributes(&baseline_svg);
    let white_fill_analysis = analyze_svg_attributes(&white_fill_svg);

    // Debug: Show what elements have white fill
    if white_fill_analysis.has_white_background {
        println!("  Debug: Found white background rectangle(s)");
        println!(
            "  Baseline has black fill: {}",
            baseline_analysis.has_black_fill
        );
        println!(
            "  Baseline has black stroke: {}",
            baseline_analysis.has_black_stroke
        );
        println!(
            "  White-fill has black fill: {}",
            white_fill_analysis.has_black_fill
        );
        println!(
            "  White-fill has black stroke: {}",
            white_fill_analysis.has_black_stroke
        );
    }

    // Compare and return result
    Ok(compare_svgs(
        &baseline_analysis,
        &white_fill_analysis,
        formula,
    ))
}

#[test]
#[ignore = "requires Typst CLI to be installed"]
fn test_typst_fill_white_behavior() {
    println!("=== Testing Typst fill:white behavior ===");
    println!("Typst version: {}", get_typst_version());

    let mut all_passed = true;
    let mut results = Vec::new();

    for test in TEST_FORMULAS.iter() {
        println!(
            "\n--- Testing: {} ({}) ---",
            test.description,
            if test.is_display { "display" } else { "inline" }
        );
        println!("Formula: {}", test.formula);

        match test_single_formula(test.formula, test.is_display) {
            Ok(result) => {
                println!("{}", result);
                results.push((test.description.to_string(), result.clone()));

                if result.starts_with("✗") {
                    all_passed = false;
                }
            }
            Err(e) => {
                println!("✗ Failed to test: {}", e);
                results.push((test.description.to_string(), format!("✗ Failed: {}", e)));
                all_passed = false;
            }
        }
    }

    // Print summary
    println!("\n=== Summary ===");
    for (desc, result) in results {
        println!("{}: {}", desc, result);
    }

    if !all_passed {
        println!("\n⚠️  Some tests failed. Typst may not be setting stroke colors to white.");
        println!("   We may need to keep stroke processing in build.rs");
    } else {
        println!("\n✅ All tests passed! Typst correctly sets both fill and stroke to white.");
        println!("   We can simplify SVG processing in build.rs");
    }

    // Don't fail the test - we want to see results even if Typst doesn't behave as expected
    println!("\n=== Test completed (checking behavior, not enforcing) ===");
}
