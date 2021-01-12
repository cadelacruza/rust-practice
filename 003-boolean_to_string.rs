//Implement a function which convert the given boolean value into its string representation.

fn boolean_to_string(b: bool) -> String {
    let mut result = String::new();
    
    if b {
        result = "true".to_string();
    } else {
        result = "false".to_string();
    };
    
    return result;
}
