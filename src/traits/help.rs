use serde::Serialize;

pub trait Help {
    fn help_message() -> String;
}

impl<T: Serialize + Default> Help for T {
    fn help_message() -> String {
        format!(
            "Help: Try sending a JSON body with the following:\n{}\n",
            serde_json::to_string_pretty(&T::default()).unwrap()
        )
    }
}
