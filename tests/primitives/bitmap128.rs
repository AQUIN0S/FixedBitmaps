use fixed_bitmaps::{Bitmap128, ConstantLength};

#[test]
fn default_is_0() {
    let bitmap = Bitmap128::default();
    assert_eq!(bitmap.to_u128(), 0);
}

#[test]
fn max_works_fine() {
    let bitmap = Bitmap128::from(u128::MAX);
    assert_eq!(bitmap.to_u128(), u128::MAX);
}

#[test]
fn copy_test() {
    let a = Bitmap128::from(45);
    let mut b = a;
    b += 1;

    assert_ne!(a, b);
}

#[test]
fn equality_test() {
    let a = Bitmap128::from(24);
    let b = Bitmap128::from(24);
    let c = Bitmap128::from(42);
    assert_eq!(a, b);
    assert_ne!(a, c);
}

#[test]
fn and_functionality() {
    let a = Bitmap128::from(0b11110000);
    let b = Bitmap128::from(0b11001100);
    let c = Bitmap128::from(0b10101010);

    let empty_mask = Bitmap128::default();
    let full_mask = Bitmap128::from(u128::MAX);

    assert_eq!((a & b).to_u128(), 0b11000000);
    assert_eq!((a & c).to_u128(), 0b10100000);
    assert_eq!((b & c).to_u128(), 0b10001000);

    assert_eq!((a & empty_mask).to_u128(), 0);
    assert_eq!((b & empty_mask).to_u128(), 0);
    assert_eq!((c & empty_mask).to_u128(), 0);

    assert_eq!(a & full_mask, a);
    assert_eq!(b & full_mask, b);
    assert_eq!(c & full_mask, c);

    assert_eq!(a & empty_mask, b & empty_mask);
    assert_eq!(a & empty_mask, c & empty_mask);
    assert_eq!(b & empty_mask, c & empty_mask);
}

#[test]
fn or_functionality() {
    let a = Bitmap128::from(0b11110000);
    let b = Bitmap128::from(0b11001100);
    let c = Bitmap128::from(0b10101010);

    let empty_mask = Bitmap128::default();
    let full_mask = Bitmap128::from(u128::MAX);

    assert_eq!((a | b).to_u128(), 0b11111100);
    assert_eq!((a | c).to_u128(), 0b11111010);
    assert_eq!((b | c).to_u128(), 0b11101110);

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
    let a = Bitmap128::from(0b11110000);
    let b = Bitmap128::from(0b11001100);
    let c = Bitmap128::from(0b10101010);

    let empty_mask = Bitmap128::default();
    let full_mask = Bitmap128::from(u128::MAX);

    assert_eq!((a ^ b).to_u128(), 0b00111100);
    assert_eq!((a ^ c).to_u128(), 0b01011010);
    assert_eq!((b ^ c).to_u128(), 0b01100110);

    assert_eq!(a ^ empty_mask, a);
    assert_eq!(b ^ empty_mask, b);
    assert_eq!(c ^ empty_mask, c);

    assert_eq!(a ^ full_mask, !a);
    assert_eq!(b ^ full_mask, !b);
    assert_eq!(c ^ full_mask, !c);
}

#[test]
fn not_functionality() {
    let a = Bitmap128::default();
    let b = Bitmap128::from(u128::MAX);
    let c = Bitmap128::from(0b1010);

    assert_eq!(!a, b);
    assert_eq!(!b, a);

    // !c is actually 1111...0101
    assert_ne!(!c, Bitmap128::from(0b0101));
    assert_eq!(!c, Bitmap128::from(u128::MAX - 0b1010));
}

#[test]
#[should_panic]
fn add_over_limit() {
    let mut bitmap = Bitmap128::from(u128::MAX);
    bitmap += 1;
}

#[test]
#[should_panic]
fn subtract_to_negative() {
    let mut bitmap = Bitmap128::default();
    bitmap -= 1;
}

#[test]
#[should_panic]
fn divide_by_0() {
    let mut bitmap = Bitmap128::from(1);
    bitmap /= 0;
}

#[test]
#[should_panic]
fn multiply_over_limit() {
    let mut bitmap = Bitmap128::from(u128::MAX);
    bitmap *= 2;
}

#[test]
fn deref_works() {
    let mut bitmap = Bitmap128::from(1);
    bitmap.set(4, true).unwrap();
    let value = *bitmap;
    assert_eq!(value, 17);
}

#[test]
fn create_bit_mask_true() {
    let a = Bitmap128::create_bit_mask(3, 6, true);
    let b = Bitmap128::create_bit_mask(7, 8, true);
    let c = Bitmap128::create_bit_mask(0, 1, true);
    let d = Bitmap128::create_bit_mask(0, 0, true);
    let e = Bitmap128::create_bit_mask(8, 8, true);
    let f = Bitmap128::create_bit_mask(0, Bitmap128::MAP_LENGTH, true);
    assert_eq!(*a, 0b111000);
    assert_eq!(*b, 0b10000000);
    assert_eq!(*c, 0b1);
    assert_eq!(*d, 0);
    assert_eq!(*e, 0);
    assert_eq!(*f, u128::MAX);
}

#[test]
fn create_bit_mask_false() {
    let a = Bitmap128::create_bit_mask(3, 6, false);
    let b = Bitmap128::create_bit_mask(7, 8, false);
    let c = Bitmap128::create_bit_mask(0, 1, false);
    let d = Bitmap128::create_bit_mask(0, 0, false);
    let e = Bitmap128::create_bit_mask(Bitmap128::MAP_LENGTH, Bitmap128::MAP_LENGTH, false);
    let f = Bitmap128::create_bit_mask(0, Bitmap128::MAP_LENGTH, false);
    assert_eq!(
        a,
        Bitmap128::create_bit_mask(0, 3, true)
            | Bitmap128::create_bit_mask(6, Bitmap128::MAP_LENGTH, true)
    );
    assert_eq!(
        b,
        Bitmap128::create_bit_mask(0, 7, true)
            | Bitmap128::create_bit_mask(8, Bitmap128::MAP_LENGTH, true)
    );
    assert_eq!(
        c,
        Bitmap128::create_bit_mask(1, Bitmap128::MAP_LENGTH, true)
    );
    assert_eq!(*d, u128::MAX);
    assert_eq!(*e, u128::MAX);
    assert_eq!(*f, 0);
}
