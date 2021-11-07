use std::{convert::TryInto, mem::size_of};

use fixed_bitmaps::{Bitmap1024, BitmapSize};

const SIZE_USIZE: usize = size_of::<usize>() * 8;
const NUM_ELEMENTS: usize = Bitmap1024::MAP_LENGTH / SIZE_USIZE;

#[test]
fn default_is_0() {
    let bitmap = Bitmap1024::default();
    assert_eq!(*bitmap, [0; NUM_ELEMENTS]);
}

#[test]
fn max_works_fine() {
    let bitmap = Bitmap1024::from([usize::MAX; NUM_ELEMENTS]);
    assert_eq!(*bitmap, [usize::MAX; NUM_ELEMENTS]);
}

#[test]
fn copy_test() {
    let a = Bitmap1024::default();
    let mut b = a;
    b += 1;

    assert_ne!(a, b);
}

#[test]
fn equality_test() {
    let mut a = Bitmap1024::default();
    a.set(1054, true).unwrap();
    a.set(1000, true).unwrap();
    let mut b = Bitmap1024::default();
    b.set(1054, true).unwrap();
    b.set(1000, true).unwrap();
    let mut c = Bitmap1024::default();
    c.set(1054, true).unwrap();
    assert_eq!(a, b);
    assert_ne!(a, c);
}

#[test]
fn and_functionality() {
    let mut a = Bitmap1024::new(false);
    let mut b = Bitmap1024::new(false);
    let mut c = Bitmap1024::new(false);

    let mut array_a = [0; NUM_ELEMENTS];
    let mut array_b = [0; NUM_ELEMENTS];
    let mut array_c = [0; NUM_ELEMENTS];

    let empty_mask = Bitmap1024::default();
    let full_mask = Bitmap1024::new(true);

    for i in 50..54 {
        a.set(i, true).unwrap();
        array_a[NUM_ELEMENTS - 1 - i / SIZE_USIZE] +=
            2usize.pow((i % SIZE_USIZE).try_into().unwrap());
    }

    b.set(50, true).unwrap();
    b.set(51, true).unwrap();
    b.set(100, true).unwrap();
    b.set(101, true).unwrap();

    array_b[NUM_ELEMENTS - 1 - 50 / SIZE_USIZE] +=
        2usize.pow((50 % SIZE_USIZE).try_into().unwrap());
    array_b[NUM_ELEMENTS - 1 - 51 / SIZE_USIZE] +=
        2usize.pow((51 % SIZE_USIZE).try_into().unwrap());
    array_b[NUM_ELEMENTS - 1 - 100 / SIZE_USIZE] +=
        2usize.pow((100 % SIZE_USIZE).try_into().unwrap());
    array_b[NUM_ELEMENTS - 1 - 101 / SIZE_USIZE] +=
        2usize.pow((101 % SIZE_USIZE).try_into().unwrap());

    c.set(50, true).unwrap();
    c.set(52, true).unwrap();
    c.set(100, true).unwrap();
    c.set(102, true).unwrap();

    array_c[NUM_ELEMENTS - 1 - 50 / SIZE_USIZE] +=
        2usize.pow((50 % SIZE_USIZE).try_into().unwrap());
    array_c[NUM_ELEMENTS - 1 - 52 / SIZE_USIZE] +=
        2usize.pow((52 % SIZE_USIZE).try_into().unwrap());
    array_c[NUM_ELEMENTS - 1 - 100 / SIZE_USIZE] +=
        2usize.pow((100 % SIZE_USIZE).try_into().unwrap());
    array_c[NUM_ELEMENTS - 1 - 102 / SIZE_USIZE] +=
        2usize.pow((102 % SIZE_USIZE).try_into().unwrap());

    let mut first_test = Bitmap1024::default();
    let mut second_test = Bitmap1024::default();
    let mut third_test = Bitmap1024::default();

    first_test.set(50, true).unwrap();
    first_test.set(51, true).unwrap();

    second_test.set(50, true).unwrap();
    second_test.set(52, true).unwrap();

    third_test.set(50, true).unwrap();
    third_test.set(100, true).unwrap();

    assert_eq!((a & b), first_test);
    assert_eq!((a & c), second_test);
    assert_eq!((b & c), third_test);

    assert_eq!((a & empty_mask).to_array(), [0; NUM_ELEMENTS]);
    assert_eq!((b & empty_mask).to_array(), [0; NUM_ELEMENTS]);
    assert_eq!((c & empty_mask).to_array(), [0; NUM_ELEMENTS]);

    assert_eq!(a & full_mask, a);
    assert_eq!(b & full_mask, b);
    assert_eq!(c & full_mask, c);

    assert_eq!(a & empty_mask, b & empty_mask);
    assert_eq!(a & empty_mask, c & empty_mask);
    assert_eq!(b & empty_mask, c & empty_mask);
}

// #[test]
// fn or_functionality() {
//     let a = Bitmap1024::from(0b11110000);
//     let b = Bitmap1024::from(0b11001100);
//     let c = Bitmap1024::from(0b10101010);

//     let empty_mask = Bitmap1024::default();
//     let full_mask = Bitmap1024::from(u128::MAX);

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
//     let a = Bitmap1024::from(0b11110000);
//     let b = Bitmap1024::from(0b11001100);
//     let c = Bitmap1024::from(0b10101010);

//     let empty_mask = Bitmap1024::default();
//     let full_mask = Bitmap1024::from(u128::MAX);

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
//     let a = Bitmap1024::default();
//     let b = Bitmap1024::from(u128::MAX);
//     let c = Bitmap1024::from(0b1010);

//     assert_eq!(!a, b);
//     assert_eq!(!b, a);

//     // !c is actually 1111...0101
//     assert_ne!(!c, Bitmap1024::from(0b0101));
//     assert_eq!(!c, Bitmap1024::from(u128::MAX - 0b1010));
// }

// #[test]
// #[should_panic]
// fn add_over_limit() {
//     let mut bitmap = Bitmap1024::from(u128::MAX);
//     bitmap += 1;
// }

// #[test]
// #[should_panic]
// fn subtract_to_negative() {
//     let mut bitmap = Bitmap1024::default();
//     bitmap -= 1;
// }

// #[test]
// #[should_panic]
// fn divide_by_0() {
//     let mut bitmap = Bitmap1024::from(1);
//     bitmap /= 0;
// }

// #[test]
// #[should_panic]
// fn multiply_over_limit() {
//     let mut bitmap = Bitmap1024::from(u128::MAX);
//     bitmap *= 2;
// }

// #[test]
// fn deref_works() {
//     let mut bitmap = Bitmap1024::from(1);
//     bitmap.set(4, true).unwrap();
//     let value = *bitmap;
//     assert_eq!(value, 17);
// }
