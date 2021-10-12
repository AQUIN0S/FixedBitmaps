use fixed_bitmaps::BitmapArch;

#[test]
fn default_is_0() {
    let bitmap = BitmapArch::default();
    assert_eq!(bitmap.to_usize(), 0);
}

#[test]
fn max_works_fine() {
    let bitmap = BitmapArch::from(usize::MAX);
    assert_eq!(bitmap.to_usize(), usize::MAX);
}

#[test]
fn equality_test() {
    let a = BitmapArch::from(24);
    let b = BitmapArch::from(24);
    let c = BitmapArch::from(42);
    assert_eq!(a, b);
    assert_ne!(a, c);
}

#[test]
fn and_functionality() {
    let a = BitmapArch::from(0b11110000);
    let b = BitmapArch::from(0b11001100);
    let c = BitmapArch::from(0b10101010);

    let empty_mask = BitmapArch::default();
    let full_mask = BitmapArch::from(usize::MAX);

    assert_eq!((a & b).to_usize(), 0b11000000);
    assert_eq!((a & c).to_usize(), 0b10100000);
    assert_eq!((b & c).to_usize(), 0b10001000);

    assert_eq!((a & empty_mask).to_usize(), 0);
    assert_eq!((b & empty_mask).to_usize(), 0);
    assert_eq!((c & empty_mask).to_usize(), 0);

    assert_eq!(a & full_mask, a);
    assert_eq!(b & full_mask, b);
    assert_eq!(c & full_mask, c);

    assert_eq!(a & empty_mask, b & empty_mask);
    assert_eq!(a & empty_mask, c & empty_mask);
    assert_eq!(b & empty_mask, c & empty_mask);
}

#[test]
fn or_functionality() {
    let a = BitmapArch::from(0b11110000);
    let b = BitmapArch::from(0b11001100);
    let c = BitmapArch::from(0b10101010);

    let empty_mask = BitmapArch::default();
    let full_mask = BitmapArch::from(usize::MAX);

    assert_eq!((a | b).to_usize(), 0b11111100);
    assert_eq!((a | c).to_usize(), 0b11111010);
    assert_eq!((b | c).to_usize(), 0b11101110);

    assert_eq!(a | empty_mask, a);
    assert_eq!(b | empty_mask, b);
    assert_eq!(c | empty_mask, c);

    assert_eq!(a | full_mask, full_mask);
    assert_eq!(b | full_mask, full_mask);
    assert_eq!(c | full_mask, full_mask);

    assert_eq!(a | full_mask, b | full_mask);
    assert_eq!(a | full_mask, c | full_mask);
    assert_eq!(b | full_mask, c | full_mask);
}

#[test]
fn xor_functionality() {
    let a = BitmapArch::from(0b11110000);
    let b = BitmapArch::from(0b11001100);
    let c = BitmapArch::from(0b10101010);

    let empty_mask = BitmapArch::default();
    let full_mask = BitmapArch::from(usize::MAX);

    assert_eq!((a ^ b).to_usize(), 0b00111100);
    assert_eq!((a ^ c).to_usize(), 0b01011010);
    assert_eq!((b ^ c).to_usize(), 0b01100110);

    assert_eq!(a ^ empty_mask, a);
    assert_eq!(b ^ empty_mask, b);
    assert_eq!(c ^ empty_mask, c);

    assert_eq!(a ^ full_mask, !a);
    assert_eq!(b ^ full_mask, !b);
    assert_eq!(c ^ full_mask, !c);
}

#[test]
fn not_functionality() {
    let a = BitmapArch::default();
    let b = BitmapArch::from(usize::MAX);
    let c = BitmapArch::from(0b1010);

    assert_eq!(!a, b);
    assert_eq!(!b, a);

    // !c is actually 1111...0101
    assert_ne!(!c, BitmapArch::from(0b0101));
    assert_eq!(!c, BitmapArch::from(usize::MAX - 0b1010));
}

#[test]
#[should_panic]
fn add_over_limit() {
    let mut bitmap = BitmapArch::from(usize::MAX);
    bitmap += 1;
}

#[test]
#[should_panic]
fn subtract_to_negative() {
    let mut bitmap = BitmapArch::default();
    bitmap -= 1;
}

#[test]
#[should_panic]
fn divide_by_0() {
    let mut bitmap = BitmapArch::from(1);
    bitmap /= 0;
}

#[test]
#[should_panic]
fn multiply_over_limit() {
    let mut bitmap = BitmapArch::from(usize::MAX);
    bitmap *= 2;
}

#[test]
fn deref_works() {
    let mut bitmap = BitmapArch::from(1);
    bitmap.set(4, true).unwrap();
    let value = *bitmap;
    assert_eq!(value, 17);
}
