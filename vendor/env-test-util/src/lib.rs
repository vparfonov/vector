/// Temporary environment variable manager
///
/// When initialising the variable manager with `new`, the actual content will be removed and stored
/// in `initial_value`. You can then set a temporary value using the method `with`. The environment
/// variable will then be reset to it's initial value when it will be dropped.
///
/// # Examples
///
/// ```
/// use env_test_util::TempEnvVar;
///
/// std::env::set_var("MY_VARIABLE", "ORIGINAL"); // set the variable to "ORIGINAL"
/// let variable = TempEnvVar::new("MY_VARIABLE"); // read the variable and stores it
/// assert_eq!(std::env::var("MY_VARIABLE").ok(), None);
/// let variable = variable.with("NEW_CONTENT"); // set the environment variable with a new content
/// assert_eq!(std::env::var("MY_VARIABLE").ok(), Some("NEW_CONTENT".into()));
/// drop(variable);
/// assert_eq!(std::env::var("MY_VARIABLE").ok(), Some("ORIGINAL".into()));
/// ```
///
/// Don't forget to assign the variable in your tests, otherwise the `drop` function will be called right away
///
/// ```
/// use env_test_util::TempEnvVar;
///
/// std::env::set_var("MY_VARIABLE", "ORIGINAL"); // set the variable to "ORIGINAL"
/// TempEnvVar::new("MY_VARIABLE").with("SOMETHING_ELSE"); // read the variable and stores it
/// assert_eq!(std::env::var("MY_VARIABLE").ok(), Some("ORIGINAL".into()));
/// let _variable = TempEnvVar::new("MY_VARIABLE").with("SOMETHING_ELSE"); // Instead, store it in a variable
/// assert_eq!(std::env::var("MY_VARIABLE").ok(), Some("SOMETHING_ELSE".into()));
/// ```
pub struct TempEnvVar {
    /// name of the environment variable
    pub key: String,
    /// initial value of the environment variable
    pub initial_value: Option<String>,
}

impl TempEnvVar {
    /// creates a new temporary environment variable manager
    pub fn new(key: &str) -> Self {
        let initial_value = std::env::var(key).ok();
        std::env::remove_var(key);
        Self {
            key: key.into(),
            initial_value,
        }
    }

    /// set the environment with a new temporary value
    pub fn with(self, value: &str) -> Self {
        std::env::set_var(self.key.as_str(), value);
        self
    }
}

impl Drop for TempEnvVar {
    fn drop(&mut self) {
        match self.initial_value.as_ref() {
            Some(value) => std::env::set_var(self.key.as_str(), value),
            None => std::env::remove_var(self.key.as_str()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn with_non_existing_variable() {
        let name = "MISSINGVAR";
        std::env::remove_var(name);
        let variable = TempEnvVar::new(name);
        assert_eq!(variable.initial_value, None);
        assert_eq!(std::env::var(name).ok(), None);
        let variable = variable.with("SOMETHING");
        assert_eq!(variable.initial_value, None);
        assert_eq!(std::env::var(name).ok(), Some("SOMETHING".into()));
        drop(variable);
        assert_eq!(std::env::var(name).ok(), None);
    }

    #[test]
    fn with_existing_variable() {
        let name = "EXISTINGVAR";
        std::env::set_var(name, "INITIAL");
        let variable = TempEnvVar::new(name);
        assert_eq!(variable.initial_value, Some("INITIAL".into()));
        assert_eq!(std::env::var(name).ok(), None);
        let variable = variable.with("SOMETHING");
        assert_eq!(variable.initial_value, Some("INITIAL".into()));
        assert_eq!(std::env::var(name).ok(), Some("SOMETHING".into()));
        drop(variable);
        assert_eq!(std::env::var(name).ok(), Some("INITIAL".into()));
    }
}
