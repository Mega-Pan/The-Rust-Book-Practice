pub fn greeting(name: &str)->String{
    format!("Hello {}!",name)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn greeting_containing_name(){
        let result = greeting("Carol");
        assert!(
            result.contains("Carol"),
            "Greeting did not contain name, value was `{}`", result
        );
    }
}
