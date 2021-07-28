pub fn get_string() -> String {
    "string".to_string()
}

#[cfg(feature = "default_feature_check")]
pub fn get_default_feature() -> String {
    "default_feature_check".to_string()
}

#[cfg(feature = "feature_a")]
pub fn get_feature_a() -> String {
    "feature_a".to_string()
}

#[cfg(feature = "feature_b")]
pub fn get_feature_b() -> String {
    "feature_b".to_string()
}

#[cfg(feature = "implied_by_b")]
pub fn get_feature_implied_by_b() -> String {
    "implied_by_b".to_string()
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
