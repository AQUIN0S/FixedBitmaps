use fixed_bitmaps::Bitmap32;

#[test]
fn default_is_0() {
    let bitmap = Bitmap32::default();
    assert_eq!(bitmap.to_u32(), 0);
}

#[test]
fn max_works_fine() {
    let bitmap = Bitmap32::from(u32::MAX);
    assert_eq!(bitmap.to_u32(), u32::MAX);
}

#[test]
fn equality_test() {
    let a = Bitmap32::from(24);
    let b = Bitmap32::from(24);
    let c = Bitmap32::from(42);
    assert_eq!(a, b);
    assert_ne!(a, c);
}

#[test]
fn and_functionality() {
    let a = Bitmap32::from(0b11110000);
    let b = Bitmap32::from(0b11001100);
    let c = Bitmap32::from(0b10101010);

    let empty_mask = Bitmap32::default();
    let full_mask = Bitmap32::from(u32::MAX);

    assert_eq!((a & b).to_u32(), 0b11000000);
    assert_eq!((a & c).to_u32(), 0b10100000);
    assert_eq!((b & c).to_u32(), 0b10001000);

    assert_eq!((a & empty_mask).to_u32(), 0);
    assert_eq!((b & empty_mask).to_u32(), 0);
    assert_eq!((c & empty_mask).to_u32(), 0);

    assert_eq!(a & full_mask, a);
    assert_eq!(b & full_mask, b);
    assert_eq!(c & full_mask, c);

    assert_eq!(a & empty_mask, b & empty_mask);
    assert_eq!(a & empty_mask, c & empty_mask);
    assert_eq!(b & empty_mask, c & empty_mask);
}

#[test]
fn or_functionality() {
    let a = Bitmap32::from(0b11110000);
    let b = Bitmap32::from(0b11001100);
    let c = Bitmap32::from(0b10101010);

    let empty_mask = Bitmap32::default();
    let full_mask = Bitmap32::from(u32::MAX);

    assert_eq!((a | b).to_u32(), 0b11111100);
    assert_eq!((a | c).to_u32(), 0b11111010);
    assert_eq!((b | c).to_u32(), 0b11101110);

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
    let a = Bitmap32::from(0b11110000);
    let b = Bitmap32::from(0b11001100);
    let c = Bitmap32::from(0b10101010);

    let empty_mask = Bitmap32::default();
    let full_mask = Bitmap32::from(u32::MAX);

    assert_eq!((a ^ b).to_u32(), 0b00111100);
    assert_eq!((a ^ c).to_u32(), 0b01011010);
    assert_eq!((b ^ c).to_u32(), 0b01100110);

    assert_eq!(a ^ empty_mask, a);
    assert_eq!(b ^ empty_mask, b);
    assert_eq!(c ^ empty_mask, c);

    assert_eq!(a ^ full_mask, !a);
    assert_eq!(b ^ full_mask, !b);
    assert_eq!(c ^ full_mask, !c);
}

#[test]
fn not_functionality() {
    let a = Bitmap32::default();
    let b = Bitmap32::from(u32::MAX);
    let c = Bitmap32::from(0b10101010);

    assert_eq!(!a, b);
    assert_eq!(!b, a);

    // !c is actually 1111111...01010101
    assert_ne!(!c, Bitmap32::from(0b01010101));
    assert_eq!(!c, Bitmap32::from(u32::MAX - 0b10101010));
}