use crate::error::Result;
use crate::selector::SelectionResult;

pub trait SelectorRunner {
    fn select(
        &self,
        items: Vec<String>,
        allow_custom: bool,
        prompt: &str,
        header: &str,
    ) -> Result<SelectionResult>;

    fn select_many(
        &self,
        items: Vec<String>,
        prompt: &str,
        header: &str,
    ) -> Result<SelectionResult>;
}
