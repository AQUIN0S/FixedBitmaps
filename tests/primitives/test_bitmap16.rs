use fixed_bitmaps::Bitmap16;

#[test]
fn default_is_0() {
    let bitmap = Bitmap16::default();
    assert_eq!(bitmap.to_u16(), 0);
}

#[test]
fn max_works_fine() {
    let bitmap = Bitmap16::from(u16::MAX);
    assert_eq!(bitmap.to_u16(), u16::MAX);
}

#[test]
fn copy_test() {
    let a = Bitmap16::from(45);
    let mut b = a;
    b += 1;

    assert_ne!(a, b);
}

#[test]
fn equality_test() {
    let a = Bitmap16::from(24);
    let b = Bitmap16::from(24);
    let c = Bitmap16::from(42);
    assert_eq!(a, b);
    assert_ne!(a, c);
}

#[test]
fn and_functionality() {
    let a = Bitmap16::from(0b11110000);
    let b = Bitmap16::from(0b11001100);
    let c = Bitmap16::from(0b10101010);

    let empty_mask = Bitmap16::default();
    let full_mask = Bitmap16::from(u16::MAX);

    assert_eq!((a & b).to_u16(), 0b11000000);
    assert_eq!((a & c).to_u16(), 0b10100000);
    assert_eq!((b & c).to_u16(), 0b10001000);

    assert_eq!((a & empty_mask).to_u16(), 0);
    assert_eq!((b & empty_mask).to_u16(), 0);
    assert_eq!((c & empty_mask).to_u16(), 0);

    assert_eq!(a & full_mask, a);
    assert_eq!(b & full_mask, b);
    assert_eq!(c & full_mask, c);

    assert_eq!(a & empty_mask, b & empty_mask);
    assert_eq!(a & empty_mask, c & empty_mask);
    assert_eq!(b & empty_mask, c & empty_mask);
}

#[test]
fn or_functionality() {
    let a = Bitmap16::from(0b11110000);
    let b = Bitmap16::from(0b11001100);
    let c = Bitmap16::from(0b10101010);

    let empty_mask = Bitmap16::default();
    let full_mask = Bitmap16::from(u16::MAX);

    assert_eq!((a | b).to_u16(), 0b11111100);
    assert_eq!((a | c).to_u16(), 0b11111010);
    assert_eq!((b | c).to_u16(), 0b11101110);

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
    let a = Bitmap16::from(0b11110000);
    let b = Bitmap16::from(0b11001100);
    let c = Bitmap16::from(0b10101010);

    let empty_mask = Bitmap16::default();
    let full_mask = Bitmap16::from(u16::MAX);

    assert_eq!((a ^ b).to_u16(), 0b00111100);
    assert_eq!((a ^ c).to_u16(), 0b01011010);
    assert_eq!((b ^ c).to_u16(), 0b01100110);

    assert_eq!(a ^ empty_mask, a);
    assert_eq!(b ^ empty_mask, b);
    assert_eq!(c ^ empty_mask, c);

    assert_eq!(a ^ full_mask, !a);
    assert_eq!(b ^ full_mask, !b);
    assert_eq!(c ^ full_mask, !c);
}

#[test]
fn not_functionality() {
    let a = Bitmap16::default();
    let b = Bitmap16::from(u16::MAX);
    let c = Bitmap16::from(0b1010);

    assert_eq!(!a, b);
    assert_eq!(!b, a);

    // !c is actually 1111...0101
    assert_ne!(!c, Bitmap16::from(0b0101));
    assert_eq!(!c, Bitmap16::from(u16::MAX - 0b1010));
}

#[test]
#[should_panic]
fn add_over_limit() {
    let mut bitmap = Bitmap16::from(u16::MAX);
    bitmap += 1;
}

#[test]
#[should_panic]
fn subtract_to_negative() {
    let mut bitmap = Bitmap16::default();
    bitmap -= 1;
}

#[test]
#[should_panic]
fn divide_by_0() {
    let mut bitmap = Bitmap16::from(1);
    bitmap /= 0;
}

#[test]
#[should_panic]
fn multiply_over_limit() {
    let mut bitmap = Bitmap16::from(u16::MAX);
    bitmap *= 2;
}

#[test]
fn deref_works() {
    let mut bitmap = Bitmap16::from(1);
    bitmap.set(4, true).unwrap();
    let value = *bitmap;
    assert_eq!(value, 17);
}
