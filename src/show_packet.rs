pub(crate) fn show(object_type: &str, values: &Vec<u8>) {
    println!("{}:", object_type);
    for i in (0..values.len()).step_by(16) {
        let next_len = if i + 8 < values.len() {
            i + 8 } else { values.len() };

        print!("{:02x?}", &values[i..next_len]);
        print!("\t");
        if i + 8 < values.len() {
            let next_len = if i + 16 < values.len() {
                i + 16 } else { values.len() };
            println!("{:02x?}", &values[i + 8..next_len]);
        }
    }
    println!();
}