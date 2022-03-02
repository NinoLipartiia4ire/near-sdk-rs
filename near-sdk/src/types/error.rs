pub trait SdkFunctionError {
    fn message(&self) -> String;
}

impl<T> SdkFunctionError for T
where
    T: ToString,
{
    fn message(&self) -> String {
        self.to_string()
    }
}
