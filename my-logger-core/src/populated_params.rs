#[derive(Debug, Clone)]
pub struct PopulatedParams {
    data: Vec<(String, String)>,
}

impl PopulatedParams {
    pub fn new_empty() -> Self {
        Self { data: vec![] }
    }
    pub fn new(data: Vec<(String, String)>) -> Self {
        Self { data }
    }
    pub fn get_data(&self) -> &[(String, String)] {
        self.data.as_slice()
    }

    pub fn push(&mut self, key: String, value: String) {
        self.data.push((key, value));
    }

    pub fn get(&self, key: &str) -> Option<&String> {
        for itm in &self.data {
            if itm.0 == key {
                return Some(&itm.1);
            }
        }

        return None;
    }

    pub fn iter(&self) -> std::slice::Iter<(String, String)> {
        self.data.iter()
    }
}
