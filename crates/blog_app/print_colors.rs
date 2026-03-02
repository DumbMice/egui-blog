use catppuccin::{Flavor, PALETTE};

fn main() {
    println!("=== Latte (Light) ===");
    print_colors(&PALETTE.latte);
    
    println!("\n=== Frappe (Light) ===");
    print_colors(&PALETTE.frappe);
    
    println!("\n=== Macchiato (Dark) ===");
    print_colors(&PALETTE.macchiato);
    
    println!("\n=== Mocha (Dark) ===");
    print_colors(&PALETTE.mocha);
}

fn print_colors(flavor: &Flavor) {
    println!("text: RGB({}, {}, {})", flavor.colors.text.rgb.r, flavor.colors.text.rgb.g, flavor.colors.text.rgb.b);
    println!("subtext0: RGB({}, {}, {})", flavor.colors.subtext0.rgb.r, flavor.colors.subtext0.rgb.g, flavor.colors.subtext0.rgb.b);
    println!("base: RGB({}, {}, {})", flavor.colors.base.rgb.r, flavor.colors.base.rgb.g, flavor.colors.base.rgb.b);
    println!("surface0: RGB({}, {}, {})", flavor.colors.surface0.rgb.r, flavor.colors.surface0.rgb.g, flavor.colors.surface0.rgb.b);
    println!("surface1: RGB({}, {}, {})", flavor.colors.surface1.rgb.r, flavor.colors.surface1.rgb.g, flavor.colors.surface1.rgb.b);
    println!("surface2: RGB({}, {}, {})", flavor.colors.surface2.rgb.r, flavor.colors.surface2.rgb.g, flavor.colors.surface2.rgb.b);
    println!("blue: RGB({}, {}, {})", flavor.colors.blue.rgb.r, flavor.colors.blue.rgb.g, flavor.colors.blue.rgb.b);
}
