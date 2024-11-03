pub fn encode_array_response(values: Vec<String>) -> String {
    let mut response = String::from("*");
    response.push_str(&values.len().to_string());
    response.push_str("\r\n");
    for value in values {
        response.push_str(format!("${}\r\n", value.len()).as_str());
        response.push_str(&value);
        response.push_str("\r\n");
    }
    response
}
