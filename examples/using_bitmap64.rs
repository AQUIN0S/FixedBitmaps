use fixed_bitmaps::Bitmap64;

fn main() {
    // Creates an empty bitmap
    let mut bitmap = Bitmap64::default();

    // Bitmaps implement Display so you can view what the map looks like
    println!("Default bitmap: {}", bitmap);

    // Bitmaps also convert to their respective unsigned int versions and back again easily
    println!("Value of bitmap: {}", bitmap.to_u64());

    // Let's do the same as above, but actually setting the values in the bitmap to something
    bitmap |= Bitmap64::from(101);

    // Will show 0000000000000000000000000000000000000000000000000000000001100101
    println!("Bitmap after OR-ing with 101: {}", bitmap);

    // Will show that 101 is the value of the bitmap
    println!("Bitmap value: {}", bitmap.to_u64());
}
