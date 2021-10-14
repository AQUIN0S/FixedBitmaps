use fixed_bitmaps::Bitmap2048;
use std::time::Instant;

fn main() {
    let max = 2048;

    let mut bitmap = Bitmap2048::from([usize::MAX; 32]);

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
    println!("Time elapsed: {}", elapsed);
}
