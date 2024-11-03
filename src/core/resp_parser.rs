pub fn read_length(data: &[u8]) -> (i32, usize) {
    // $6\r\nfoobar\r\n
    let pos = data.iter().position(|&x| x == b'\r').unwrap();
    let length = String::from_utf8(data[1..pos].to_vec())
        .unwrap()
        .parse::<i32>()
        .unwrap();

    (length, pos + 2)
}

pub fn read_simple_string(data: &[u8]) -> (String, usize) {
    //$6\r\nfoobar\r\n

    let (length, length_till) = read_length(data);

    let start = length_till;
    let end = start + length as usize;
    let value = String::from_utf8(data[start..end].to_vec()).unwrap();
    (value, end+2 as usize)
}

pub fn read_error(data: [u8; 1024]) -> (String, usize) {
    // -Error message\r\n
    let pos = data.iter().position(|&x| x == b'\r').unwrap();
    let value = String::from_utf8(data[1..pos].to_vec()).unwrap();
    (value, pos + 2)
}

pub fn read_integer(data: [u8; 1024]) -> i32 {
    let pos = data.iter().position(|&x| x == b'\r').unwrap();
    String::from_utf8(data[1..pos].to_vec())
        .unwrap()
        .parse::<i32>()
        .unwrap()
}

pub fn read_bulk_string(data: [u8; 1024]) -> String {
    let pos = data.iter().position(|&x| x == b'\r').unwrap();
    String::from_utf8(data[1..pos].to_vec()).unwrap()
}

pub fn read_array(data: [u8; 1024]) -> Vec<String> {
    println!("readArray {:?}", data);
    Vec::new()
}

pub fn read_bulk_errors(data: [u8; 1024]) -> Vec<String> {
    println!("readBulkErrors {:?}", data);
    Vec::new()
}

pub fn read_nulls(data: [u8; 1024]) -> Vec<String> {
    println!("readNulls {:?}", data);
    Vec::new()
}

pub fn read_booleans(data: [u8; 1024]) -> Vec<String> {
    println!("readBooleans {:?}", data);
    Vec::new()
}

pub fn read_doubles(data: [u8; 1024]) -> Vec<String> {
    println!("readDoubles {:?}", data);
    Vec::new()
}

pub fn read_big_numbers(data: [u8; 1024]) -> Vec<String> {
    println!("readBigNumbers {:?}", data);
    Vec::new()
}

pub fn read_maps(data: [u8; 1024]) -> Vec<String> {
    println!("readMaps {:?}", data);
    Vec::new()
}

pub fn read_sets(data: [u8; 1024]) -> Vec<String> {
    println!("readSets {:?}", data);
    Vec::new()
}

pub fn read_attributes(data: [u8; 1024]) -> Vec<String> {
    println!("readAttributes {:?}", data);
    Vec::new()
}

pub fn read_pushes(data: [u8; 1024]) -> Vec<String> {
    println!("readPushes {:?}", data);
    Vec::new()
}
