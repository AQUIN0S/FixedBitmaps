use fixed_bitmaps::BitmapArch;

fn main() {
    let mut bitmap = BitmapArch::default();
    println!("{}", bitmap);
    println!("Capacity: {}", BitmapArch::capacity());

    bitmap.set(5, true).unwrap();
    println!("{}", bitmap);
    println!("Value: {}", bitmap.to_usize());
}
