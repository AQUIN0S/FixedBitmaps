use fixed_bitmaps::BitmapArch;

fn main() {
    let mut bitmap = BitmapArch::default();
    println!("{}", bitmap);
    // println!("Capacity: {}", BitmapArch::capacity());

    bitmap.set(5, true).unwrap();
    println!("{}", bitmap);
    println!("Value: {}", bitmap.to_usize());

    let a = BitmapArch::from(0b0001);
    let b = BitmapArch::from(0b0010);
    let c = BitmapArch::from(0b0100);
    let d = BitmapArch::from(0b1000);

    let set1 = a | b | c | d | d;
    let set2 = a | b | c | d;

    println!("Set 1: {}", set1);
    println!("Set 2: {:?}", set2);
}
