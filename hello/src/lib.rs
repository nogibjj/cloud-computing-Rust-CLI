/* A library for doing Marco Polo */

pub fn marco_polo(input: &str) -> String {
    if input == "Marco" {
        "Polo".to_string()
    } else {
        "NOT Marco".to_string()
    }
}
