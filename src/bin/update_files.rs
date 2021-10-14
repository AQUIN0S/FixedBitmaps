//! This file is used to vastly reduce the amount of copying and pasting I would have to do if I were to keep things as they
//! were around version 0.6.0. At that point every time I made a change to one of the `Bitmap` modules, I would have to copy
//! the change, go through all the other modules, paste it in and change the types to match the context. This is tedious and
//! error-prone, and very repetitive.
//!
//! There were two solutions to this that I could see:
//!
//! - Abandon all of the repetitive modules and focus on what might be the most useful one. This would most likely be the
//!   `Bitmap128`. However this goes against the purpose of this crate - the idea is to wrap **all** of the primitive unsigned
//!   integer types, and so I didn't consider this option for long.
//! - The other option was to find a way to make it so that changes in one file could propogate automatically to all of the other
//!   modules. This is the method I chose, hence this binary.
//!
//! This program currently focusses on two files: `fixed_bitmaps/src/primitives/bitmap128.rs` and `fixed_bitmaps/tests/primitives/bitmap128.rs`.
//! When benchmarking becomes a thing I'll probably add in a file there too.
//!
//! All it does is when run, it takes the contents of the above two files, and overwrites or creates 5 more files each, one
//! for each unsigned int type. It modifies the content according to the context of the new primitive type, for example all
//! references to `u128` in `bitmap128.rs`, will be changed to `u64` in `bitmap64.rs`.
//!
//! Overall, this program makes it **much** easier to modify the `Bitmap` structs, and keeps everything consistent.

use std::{fs, path::PathBuf, str};

const REPLACE_PRIMITIVES: (&str, &str, &str, &str) = ("u128", "Bitmap128", "128", "bitmap128");

const WITH_PRIMITIVES: [(&str, &str, &str, &str); 5] = [
    ("u64", "64", "64", "bitmap64"),
    ("u32", "32", "32", "bitmap32"),
    ("u16", "16", "16", "bitmap16"),
    ("u8", "8", "8", "bitmap8"),
    ("usize", "Arch", "usize", "bitmap_arch"),
];

const REPLACE_OVERSIZED: (&str, &str, &str, &str) = ("8_192", "BitmapKB", "bitmap_kb", "bitmap_kb");

const WITH_OVERSIZED: [(&str, &str, &str, &str); 4] = [
    ("256", "256", "bitmap_256", "bitmap_256"),
    ("512", "512", "bitmap_512", "bitmap_512"),
    ("1_024", "1024", "bitmap_1024", "bitmap_1024"),
    ("2_048", "2048", "bitmap_2048", "bitmap_2048"),
];

fn create_or_replace(
    src_dir_path: String,
    replace: (&str, &str, &str, &str),
    with: Vec<(&str, &str, &str, &str)>,
) {
    let original = fs::read_to_string(String::from(&src_dir_path) + replace.3 + ".rs").unwrap();

    for write_values in with {
        let path: PathBuf = [".", &src_dir_path, &(String::from(write_values.3) + ".rs")]
            .iter()
            .collect();

        let path = path.as_path();

        let mut new_content = String::from(&original).replace(replace.0, write_values.0);
        new_content = new_content.replace(replace.1, &(String::from("Bitmap") + write_values.1));
        new_content = new_content.replace(replace.2, write_values.2);

        match fs::write(path, new_content) {
            Ok(_) => {}
            Err(error) => {
                eprintln!("Could not write to file!");
                eprintln!("File path: {}", path.to_str().unwrap());
                eprintln!("Caused by: {}", error);
            }
        }
    }
}

fn create_or_replace_tests() {
    create_or_replace(
        String::from("./tests/primitives/"),
        REPLACE_PRIMITIVES,
        Vec::from(WITH_PRIMITIVES),
    );
}

fn create_or_replace_primitive_modules() {
    create_or_replace(
        String::from("./src/primitives/"),
        REPLACE_PRIMITIVES,
        Vec::from(WITH_PRIMITIVES),
    );
}

fn create_or_replace_oversized_modules() {
    create_or_replace(
        String::from("./src/oversized/"),
        REPLACE_OVERSIZED,
        Vec::from(WITH_OVERSIZED),
    );
}

fn main() {
    create_or_replace_tests();
    create_or_replace_primitive_modules();
    create_or_replace_oversized_modules();
}
