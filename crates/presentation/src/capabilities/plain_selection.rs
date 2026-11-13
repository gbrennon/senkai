use senkai_core::error::{Error, Result};
use senkai_core::selector::SelectionResult;

pub struct PlainSelection;

impl PlainSelection {
    pub fn single(input: &str, items: &[String], allow_custom: bool) -> Result<SelectionResult> {
        let input = input.trim();
        if input.is_empty() {
            return Ok(SelectionResult::Cancelled);
        }
        if let Ok(index) = input.parse::<usize>() {
            return Self::select_index(index, items.len());
        }
        if allow_custom {
            return Ok(SelectionResult::Custom(input.to_string()));
        }
        Err(Error::new("Selection must be a listed number"))
    }

    pub fn many(input: &str, item_count: usize) -> Result<SelectionResult> {
        let input = input.trim();
        if input.is_empty() {
            return Ok(SelectionResult::Cancelled);
        }
        let indexes = input
            .split(',')
            .map(str::trim)
            .map(str::parse::<usize>)
            .collect::<std::result::Result<Vec<_>, _>>()
            .map_err(|error| Error::new(error.to_string()))?;
        let selected = indexes
            .into_iter()
            .filter(|index| (1..=item_count).contains(index))
            .map(|index| index - 1)
            .collect::<Vec<_>>();
        if selected.is_empty() {
            return Err(Error::new("Selection must contain a listed number"));
        }
        Ok(SelectionResult::SelectedMany(selected))
    }

    pub fn select_index(index: usize, item_count: usize) -> Result<SelectionResult> {
        if (1..=item_count).contains(&index) {
            Ok(SelectionResult::Selected(index - 1))
        } else {
            Err(Error::new("Selection must be a listed number"))
        }
    }
}
