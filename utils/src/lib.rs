use std::fmt;

///Extension for Result<T>
pub trait ResultExt {
    type Ok;

    fn map_err_to_string(self, prefix: &str) -> Result<Self::Ok, String>;
}

impl<T, E: fmt::Display> ResultExt for Result<T, E> {
    type Ok = T;

    fn map_err_to_string(self, prefix: &str) -> Result<Self::Ok, String> {
        self.map_err(|error| format!("{} Error: {}", prefix, error))
    }
}
