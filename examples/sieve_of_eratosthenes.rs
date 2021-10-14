use fixed_bitmaps::BitmapKB;
use std::time::Instant;

fn main() {
    let max = 8_000;

    let mut bitmap = BitmapKB::from([usize::MAX; 128]);
    println!("{}", bitmap);
    println!();

    let time = Instant::now();

    for i in 2..max {
        if bitmap.get(i).unwrap() {
            let mut j = i;
            while i * j < max {
                bitmap.set(i * j, false).unwrap();
                j += 1;
            }
        }
    }

    let elapsed = time.elapsed().as_nanos();

    for i in 2..max {
        if bitmap.get(i).unwrap() {
            print!("{}, ", i);
        }
    }

    println!();
    println!();
    println!("{}", bitmap);
    println!("Time elapsed: {}", elapsed);
}
