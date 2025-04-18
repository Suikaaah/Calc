use std::collections::BTreeSet;

#[derive(Default)]
pub struct Cell {
    pub selected: bool,
    pub config_names: BTreeSet<String>,
}

impl Cell {
    pub fn clear(&mut self) {
        self.deselect();
        self.config_names.clear();
    }

    pub fn contains(&self, name: &str) -> bool {
        self.config_names.contains(name)
    }

    pub fn deselect(&mut self) {
        self.selected = false;
    }

    pub fn insert(&mut self, name: String) {
        self.config_names.insert(name);
    }

    pub fn remove(&mut self, name: &str) {
        self.config_names.remove(name);
    }
}
