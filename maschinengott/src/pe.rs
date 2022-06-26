use exe::pe::{VecPE, PE};
use exe::{ImageSectionHeader, RVA};
use std::path::Path;

pub fn read_machine_code(file: &Path) -> (Vec<u8>, u64) {
    let image = VecPE::from_disk_file(file)
        .unwrap_or_else(|e| panic!("Failed to read PE: {:?}: {}", file, e));
    let target_name = String::from(".text");
    let sections = image
        .get_section_table()
        .expect("Failed to extract section table!");
    let mut text_index: isize = -1;
    for (i, section) in sections.iter().enumerate() {
        let mut name = String::new();
        for c in section.name {
            if c.0 != b'\0' {
                name.push(char::from(c.0));
            }
        }
        if target_name == name {
            text_index = i as _;
            break;
        }
    }
    assert_ne!(
        text_index, -1,
        "Could not find code section: {}",
        target_name
    );
    let code_section: &ImageSectionHeader = &sections[text_index as usize];
    let rip = image.get_entrypoint().unwrap_or(RVA(0)).0 as u64;
    let data = code_section
        .read(&image)
        .expect("Could not read data of code section!");
    (Vec::from(data), rip)
}
