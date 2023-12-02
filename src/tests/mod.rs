#[cfg(test)]
mod user;

#[cfg(test)]
mod default {
    #[test]
    fn test_function_splitting_data_at() {
        let data = "anime:1234".to_string();
        let result: String = crate::splitted_data_at(data, ":");
        assert_eq!(result, "1234");
    }
}
