use kolor::{Color, spaces};

pub fn main() {
    let srgb = Color::srgb(0.35, 0.75, 0.8);
    let mut oklab = srgb.to(spaces::OK_LAB);

    // modify `a`
    oklab.value.y += 0.2;
    let modified_srgb = oklab.to(srgb.space);

    println!(" {srgb:?} -> {modified_srgb:?}");
}
