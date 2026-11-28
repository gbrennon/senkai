#![allow(dead_code)]
use std::cell::RefCell;

use senkai_core::error::Result;
use senkai_core::ports::SelectorRunner;
use senkai_core::selector::SelectionResult;

pub struct StubSelector;

impl SelectorRunner for StubSelector {
    fn select(
        &self,
        _items: Vec<String>,
        _allow_custom: bool,
        _prompt: &str,
        _header: &str,
    ) -> Result<SelectionResult> {
        unimplemented!()
    }

    fn select_many(
        &self,
        _items: Vec<String>,
        _prompt: &str,
        _header: &str,
    ) -> Result<SelectionResult> {
        unimplemented!()
    }
}

pub struct FakeSelector {
    pub results: RefCell<Vec<SelectionResult>>,
    pub calls: std::rc::Rc<RefCell<Vec<Vec<String>>>>,
    pub prompts: std::rc::Rc<RefCell<Vec<(String, String)>>>,
}

impl FakeSelector {
    pub fn new(results: Vec<SelectionResult>) -> Self {
        Self {
            results: RefCell::new(results),
            calls: std::rc::Rc::new(RefCell::new(Vec::new())),
            prompts: std::rc::Rc::new(RefCell::new(Vec::new())),
        }
    }
}

impl SelectorRunner for FakeSelector {
    fn select(
        &self,
        items: Vec<String>,
        _allow_custom: bool,
        prompt: &str,
        header: &str,
    ) -> Result<SelectionResult> {
        self.calls.borrow_mut().push(items);
        self.prompts
            .borrow_mut()
            .push((prompt.to_string(), header.to_string()));
        Ok(if self.results.borrow().is_empty() {
            SelectionResult::Cancelled
        } else {
            self.results.borrow_mut().remove(0)
        })
    }

    fn select_many(
        &self,
        items: Vec<String>,
        _prompt: &str,
        _header: &str,
    ) -> Result<SelectionResult> {
        self.calls.borrow_mut().push(items);
        Ok(if self.results.borrow().is_empty() {
            SelectionResult::Cancelled
        } else {
            self.results.borrow_mut().remove(0)
        })
    }
}
