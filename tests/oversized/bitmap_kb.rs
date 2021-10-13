use fixed_bitmaps::BitmapKB;

const NUM_ELEMENTS: usize = 1024 / std::mem::size_of::<usize>();

#[test]
fn default_is_0() {
    let bitmap = BitmapKB::default();
    assert_eq!(*bitmap, [0; NUM_ELEMENTS]);
}

#[test]
fn max_works_fine() {
    let bitmap = BitmapKB::from([usize::MAX; NUM_ELEMENTS]);
    assert_eq!(*bitmap, [usize::MAX; NUM_ELEMENTS]);
}

#[test]
fn copy_test() {
    let a = BitmapKB::default();
    let mut b = a;
    b += 1;

    assert_ne!(a, b);
}

#[test]
fn equality_test() {
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

// #[test]
// fn and_functionality() {
//     let a = BitmapKB::from(0b11110000);
//     let b = BitmapKB::from(0b11001100);
//     let c = BitmapKB::from(0b10101010);

//     let empty_mask = BitmapKB::default();
//     let full_mask = BitmapKB::from(u128::MAX);

//     assert_eq!((a & b).to_u128(), 0b11000000);
//     assert_eq!((a & c).to_u128(), 0b10100000);
//     assert_eq!((b & c).to_u128(), 0b10001000);

//     assert_eq!((a & empty_mask).to_u128(), 0);
//     assert_eq!((b & empty_mask).to_u128(), 0);
//     assert_eq!((c & empty_mask).to_u128(), 0);

//     assert_eq!(a & full_mask, a);
//     assert_eq!(b & full_mask, b);
//     assert_eq!(c & full_mask, c);

//     assert_eq!(a & empty_mask, b & empty_mask);
//     assert_eq!(a & empty_mask, c & empty_mask);
//     assert_eq!(b & empty_mask, c & empty_mask);
// }

// #[test]
// fn or_functionality() {
//     let a = BitmapKB::from(0b11110000);
//     let b = BitmapKB::from(0b11001100);
//     let c = BitmapKB::from(0b10101010);

//     let empty_mask = BitmapKB::default();
//     let full_mask = BitmapKB::from(u128::MAX);

//     assert_eq!((a | b).to_u128(), 0b11111100);
//     assert_eq!((a | c).to_u128(), 0b11111010);
//     assert_eq!((b | c).to_u128(), 0b11101110);

//     assert_eq!(a | empty_mask, a);
//     assert_eq!(b | empty_mask, b);
//     assert_eq!(c | empty_mask, c);

//     assert_eq!(a | full_mask, full_mask);
//     assert_eq!(b | full_mask, full_mask);
//     assert_eq!(c | full_mask, full_mask);

//     assert_eq!(a | full_mask, b | full_mask);
//     assert_eq!(a | full_mask, c | full_mask);
//     assert_eq!(b | full_mask, c | full_mask);
// }

// #[test]
// fn xor_functionality() {
//     let a = BitmapKB::from(0b11110000);
//     let b = BitmapKB::from(0b11001100);
//     let c = BitmapKB::from(0b10101010);

//     let empty_mask = BitmapKB::default();
//     let full_mask = BitmapKB::from(u128::MAX);

//     assert_eq!((a ^ b).to_u128(), 0b00111100);
//     assert_eq!((a ^ c).to_u128(), 0b01011010);
//     assert_eq!((b ^ c).to_u128(), 0b01100110);

//     assert_eq!(a ^ empty_mask, a);
//     assert_eq!(b ^ empty_mask, b);
//     assert_eq!(c ^ empty_mask, c);

//     assert_eq!(a ^ full_mask, !a);
//     assert_eq!(b ^ full_mask, !b);
//     assert_eq!(c ^ full_mask, !c);
// }

// #[test]
// fn not_functionality() {
//     let a = BitmapKB::default();
//     let b = BitmapKB::from(u128::MAX);
//     let c = BitmapKB::from(0b1010);

//     assert_eq!(!a, b);
//     assert_eq!(!b, a);

//     // !c is actually 1111...0101
//     assert_ne!(!c, BitmapKB::from(0b0101));
//     assert_eq!(!c, BitmapKB::from(u128::MAX - 0b1010));
// }

// #[test]
// #[should_panic]
// fn add_over_limit() {
//     let mut bitmap = BitmapKB::from(u128::MAX);
//     bitmap += 1;
// }

// #[test]
// #[should_panic]
// fn subtract_to_negative() {
//     let mut bitmap = BitmapKB::default();
//     bitmap -= 1;
// }

// #[test]
// #[should_panic]
// fn divide_by_0() {
//     let mut bitmap = BitmapKB::from(1);
//     bitmap /= 0;
// }

// #[test]
// #[should_panic]
// fn multiply_over_limit() {
//     let mut bitmap = BitmapKB::from(u128::MAX);
//     bitmap *= 2;
// }

// #[test]
// fn deref_works() {
//     let mut bitmap = BitmapKB::from(1);
//     bitmap.set(4, true).unwrap();
//     let value = *bitmap;
//     assert_eq!(value, 17);
// }
