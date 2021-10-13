use fixed_bitmaps::BitmapKB;

fn main() {
    let mut bitmap = BitmapKB::default();
    let mut array = [0; 128];
    array[127] = usize::MAX;
    println!("{}", bitmap);

    bitmap |= array;
    println!("{}", bitmap);

    bitmap += 1;
    println!("{}", bitmap);

    bitmap += usize::MAX;
    println!("{}", bitmap);
    bitmap += 1;
    println!("{}", bitmap);

    let mut a = bitmap;
    a += 1;
    println!("A: {}", a);
    println!("Bitmap: {}", bitmap);

    let mut a = BitmapKB::default();
    a.set(1054, true).unwrap();
    a.set(1000, true).unwrap();
    let mut b = BitmapKB::default();
    b.set(1054, true).unwrap();
    b.set(1000, true).unwrap();
    let mut c = BitmapKB::default();
    c.set(1054, true).unwrap();
    assert_eq!(a, b);
    assert_ne!(a, c);
}
