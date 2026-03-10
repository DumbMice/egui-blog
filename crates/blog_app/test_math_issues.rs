use blog_app::math;
use blog_app::ui::markdown;

fn main() {
    let manifest = math::load_manifest();
    
    // Test 1: Formula in parentheses
    let text1 = "Text with formula in parentheses: ($x$)";
    let result1 = markdown::extract_and_replace_math_formulas(text1, &manifest);
    println!("Test 1 - Formula in parentheses:");
    println!("  Input:  {}", text1);
    println!("  Output: {}", result1);
    println!("  Expected: Should have single parentheses around placeholder");
    
    // Test 2: Formula in blockquote
    let text2 = "> This is a blockquote with $E=mc^2$ formula";
    let result2 = markdown::extract_and_replace_math_formulas(text2, &manifest);
    println!("\nTest 2 - Formula in blockquote:");
    println!("  Input:  {}", text2);
    println!("  Output: {}", result2);
    
    // Test 3: Formula in list
    let text3 = "- Item with $x^2$ formula";
    let result3 = markdown::extract_and_replace_math_formulas(text3, &manifest);
    println!("\nTest 3 - Formula in list:");
    println!("  Input:  {}", text3);
    println!("  Output: {}", result3);
}
