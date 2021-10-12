use std::{fs, path::PathBuf, str};

fn main() {
    const WRITE_VALUES: [(&str, &str, &str, &str); 5] = [
        ("u64", "64", "64", "bitmap64"),
        ("u32", "32", "32", "bitmap32"),
        ("u16", "16", "16", "bitmap16"),
        ("u8", "8", "8", "bitmap8"),
        ("usize", "Arch", "usize", "bitmap_arch"),
    ];

    const READ_VALUES: (&str, &str, &str, &str) = ("u128", "Bitmap128", "128", "bitmap128");

    let src_dir_path = String::from("./tests/");

    let original =
        fs::read_to_string(String::from(&src_dir_path) + "test_" + READ_VALUES.3 + ".rs").unwrap();

    for write_values in WRITE_VALUES {
        let path: PathBuf = [
            ".",
            &src_dir_path,
            &(String::from("test_") + write_values.3 + ".rs"),
        ]
        .iter()
        .collect();

        let path = path.as_path();

        let mut new_content = String::from(&original).replace(READ_VALUES.0, write_values.0);
        new_content =
            new_content.replace(READ_VALUES.1, &(String::from("Bitmap") + write_values.1));
        new_content = new_content.replace(READ_VALUES.2, write_values.2);

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
