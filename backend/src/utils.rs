pub fn is_password_valid(pw: &str) -> bool {
    !pw.is_empty() && pw.chars().all(|c| c.is_ascii_alphanumeric())
}
