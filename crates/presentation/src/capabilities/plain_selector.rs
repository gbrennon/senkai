use std::io::BufRead;
use std::rc::Rc;

use senkai_core::error::{Error, Result};
use senkai_core::ports::SelectorRunner;
use senkai_core::selector::SelectionResult;

use super::plain_selection::PlainSelection;

pub struct PlainSelector {
    input: Rc<dyn Fn() -> Result<String>>,
}

impl Default for PlainSelector {
    fn default() -> Self {
        Self::new()
    }
}

impl PlainSelector {
    pub fn new() -> Self {
        Self {
            input: Rc::new(|| {
                std::io::stdin()
                    .lock()
                    .lines()
                    .next()
                    .transpose()
                    .map_err(|error| Error::new(error.to_string()))?
                    .ok_or_else(|| Error::new("Input ended"))
            }),
        }
    }

    pub fn from_input(input: &str) -> Self {
        let input = input.to_string();
        Self {
            input: Rc::new(move || Ok(input.clone())),
        }
    }
}

impl SelectorRunner for PlainSelector {
    fn select(
        &self,
        items: Vec<String>,
        allow_custom: bool,
        prompt: &str,
        header: &str,
    ) -> Result<SelectionResult> {
        println!("{header}");
        items
            .iter()
            .enumerate()
            .for_each(|(index, item)| println!("{}: {item}", index + 1));
        println!("{prompt} Enter a number or name:");
        PlainSelection::single(&(self.input)()?, &items, allow_custom)
    }

    fn select_many(
        &self,
        items: Vec<String>,
        prompt: &str,
        header: &str,
    ) -> Result<SelectionResult> {
        println!("{header}");
        items
            .iter()
            .enumerate()
            .for_each(|(index, item)| println!("{}: {item}", index + 1));
        println!("{prompt} Enter numbers separated by commas:");
        PlainSelection::many(&(self.input)()?, items.len())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn selects_an_item_by_number() {
        let selector = PlainSelector::from_input("2");

        let result = selector.select(
            vec!["first".to_string(), "second".to_string()],
            false,
            "Select",
            "Items",
        );

        assert_eq!(result.unwrap(), SelectionResult::Selected(1));
    }

    #[test]
    fn selects_a_custom_value() {
        let selector = PlainSelector::from_input("feature/new");

        let result = selector.select(vec!["first".to_string()], true, "Select", "Items");

        assert_eq!(
            result.unwrap(),
            SelectionResult::Custom("feature/new".to_string())
        );
    }

    #[test]
    fn selects_multiple_items_by_number() {
        let selector = PlainSelector::from_input("1, 3");

        let result = selector.select_many(
            vec![
                "first".to_string(),
                "second".to_string(),
                "third".to_string(),
            ],
            "Select",
            "Items",
        );

        assert_eq!(result.unwrap(), SelectionResult::SelectedMany(vec![0, 2]));
    }
}
