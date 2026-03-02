use catppuccin::{Flavor, PALETTE};

fn main() {
    let latte = &PALETTE.latte;
    println!("Latte colors available:");
    println!("- text: {:?}", latte.colors.text);
    println!("- subtext0: {:?}", latte.colors.subtext0);
    println!("- subtext1: {:?}", latte.colors.subtext1);
    println!("- overlay0: {:?}", latte.colors.overlay0);
    println!("- overlay1: {:?}", latte.colors.overlay1);
    println!("- overlay2: {:?}", latte.colors.overlay2);
    println!("- surface0: {:?}", latte.colors.surface0);
    println!("- surface1: {:?}", latte.colors.surface1);
    println!("- surface2: {:?}", latte.colors.surface2);
    println!("- base: {:?}", latte.colors.base);
    println!("- mantle: {:?}", latte.colors.mantle);
    println!("- crust: {:?}", latte.colors.crust);

    println!("\nAccent colors:");
    println!("- rosewater: {:?}", latte.colors.rosewater);
    println!("- flamingo: {:?}", latte.colors.flamingo);
    println!("- pink: {:?}", latte.colors.pink);
    println!("- mauve: {:?}", latte.colors.mauve);
    println!("- red: {:?}", latte.colors.red);
    println!("- maroon: {:?}", latte.colors.maroon);
    println!("- peach: {:?}", latte.colors.peach);
    println!("- yellow: {:?}", latte.colors.yellow);
    println!("- green: {:?}", latte.colors.green);
    println!("- teal: {:?}", latte.colors.teal);
    println!("- sky: {:?}", latte.colors.sky);
    println!("- sapphire: {:?}", latte.colors.sapphire);
    println!("- blue: {:?}", latte.colors.blue);
    println!("- lavender: {:?}", latte.colors.lavender);
}
