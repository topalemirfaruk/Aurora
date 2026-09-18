use crate::models::AppItem;
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Clone, Default)]
pub struct CartState {
    items: Rc<RefCell<Vec<AppItem>>>,
    listeners: Rc<RefCell<Vec<Box<dyn Fn(usize) + 'static>>>>,
}

impl CartState {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add(&self, item: AppItem) {
        let mut items = self.items.borrow_mut();
        if !items.iter().any(|i| i.package_name == item.package_name) {
            items.push(item);
            let count = items.len();
            drop(items);
            self.notify(count);
        }
    }

    pub fn remove(&self, package_name: &str) {
        let mut items = self.items.borrow_mut();
        if let Some(pos) = items.iter().position(|i| i.package_name == package_name) {
            items.remove(pos);
            let count = items.len();
            drop(items);
            self.notify(count);
        }
    }

    pub fn toggle(&self, item: AppItem) {
        if self.contains(&item.package_name) {
            self.remove(&item.package_name);
        } else {
            self.add(item);
        }
    }

    pub fn contains(&self, package_name: &str) -> bool {
        self.items.borrow().iter().any(|i| i.package_name == package_name)
    }

    pub fn items(&self) -> Vec<AppItem> {
        self.items.borrow().clone()
    }

    pub fn count(&self) -> usize {
        self.items.borrow().len()
    }

    pub fn clear(&self) {
        let mut items = self.items.borrow_mut();
        items.clear();
        drop(items);
        self.notify(0);
    }

    pub fn on_change<F: Fn(usize) + 'static>(&self, callback: F) {
        self.listeners.borrow_mut().push(Box::new(callback));
    }

    fn notify(&self, count: usize) {
        for listener in self.listeners.borrow().iter() {
            listener(count);
        }
    }
}
