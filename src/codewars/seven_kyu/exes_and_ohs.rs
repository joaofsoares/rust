pub fn xo(string: &'static str) -> bool {
    let lower_string = string.to_lowercase();
    lower_string.matches("x").count() == lower_string.matches("o").count()
}
