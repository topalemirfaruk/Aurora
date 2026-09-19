use serde::{Deserialize, Serialize};
use std::cell::RefCell;
use std::fs;
use std::path::PathBuf;
use std::rc::Rc;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SettingsData {
    pub dark_theme: bool,
    pub require_pkgbuild_review: bool,
    pub require_summary_confirmation: bool,
    pub aur_helper: String,
}

impl Default for SettingsData {
    fn default() -> Self {
        Self {
            dark_theme: true,
            require_pkgbuild_review: true,
            require_summary_confirmation: true,
            aur_helper: "paru".to_string(),
        }
    }
}

type SettingsListener = Box<dyn Fn(&SettingsData) + 'static>;

#[derive(Clone)]
pub struct SettingsState {
    data: Rc<RefCell<SettingsData>>,
    listeners: Rc<RefCell<Vec<SettingsListener>>>,
}

impl Default for SettingsState {
    fn default() -> Self {
        Self::new()
    }
}

impl SettingsState {
    pub fn new() -> Self {
        let loaded_data = Self::load_from_disk().unwrap_or_default();
        Self {
            data: Rc::new(RefCell::new(loaded_data)),
            listeners: Rc::new(RefCell::new(Vec::new())),
        }
    }

    pub fn get(&self) -> SettingsData {
        self.data.borrow().clone()
    }

    pub fn set_dark_theme(&self, enabled: bool) {
        {
            let mut data = self.data.borrow_mut();
            data.dark_theme = enabled;
        }
        self.save_and_notify();
    }

    pub fn set_require_pkgbuild_review(&self, enabled: bool) {
        {
            let mut data = self.data.borrow_mut();
            data.require_pkgbuild_review = enabled;
        }
        self.save_and_notify();
    }

    pub fn set_require_summary_confirmation(&self, enabled: bool) {
        {
            let mut data = self.data.borrow_mut();
            data.require_summary_confirmation = enabled;
        }
        self.save_and_notify();
    }

    pub fn set_aur_helper(&self, helper: &str) {
        {
            let mut data = self.data.borrow_mut();
            data.aur_helper = helper.to_string();
        }
        self.save_and_notify();
    }

    pub fn on_change<F: Fn(&SettingsData) + 'static>(&self, callback: F) {
        self.listeners.borrow_mut().push(Box::new(callback));
    }

    fn save_and_notify(&self) {
        let current = self.get();
        let _ = Self::save_to_disk(&current);
        for listener in self.listeners.borrow().iter() {
            listener(&current);
        }
    }

    fn config_path() -> Option<PathBuf> {
        let home = std::env::var("HOME").ok()?;
        let path = PathBuf::from(home).join(".config").join("aurora");
        let _ = fs::create_dir_all(&path);
        Some(path.join("settings.json"))
    }

    fn load_from_disk() -> Option<SettingsData> {
        let path = Self::config_path()?;
        if path.exists() {
            let content = fs::read_to_string(path).ok()?;
            serde_json::from_str(&content).ok()
        } else {
            None
        }
    }

    fn save_to_disk(data: &SettingsData) -> Result<(), std::io::Error> {
        if let Some(path) = Self::config_path() {
            let json = serde_json::to_string_pretty(data)?;
            fs::write(path, json)?;
        }
        Ok(())
    }
}
