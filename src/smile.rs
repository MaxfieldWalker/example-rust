const EYES: &'static str = ":";


pub fn smile() -> String {
    format!("{}{}", EYES, ")")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_smile() {
        assert_eq!(smile(), ":)");
    }
}
