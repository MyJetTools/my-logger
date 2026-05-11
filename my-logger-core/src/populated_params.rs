use rust_extensions::StrOrString;

#[derive(Debug, Clone)]
pub struct PopulatedParams {
    data: Vec<(&'static str, StrOrString<'static>)>,
}

impl PopulatedParams {
    pub fn new_empty() -> Self {
        Self { data: vec![] }
    }
    pub fn new(data: Vec<(&'static str, StrOrString<'static>)>) -> Self {
        Self { data }
    }
    pub fn get_data(&self) -> &[(&'static str, StrOrString<'static>)] {
        self.data.as_slice()
    }

    pub fn push(&mut self, key: &'static str, value: StrOrString<'static>) {
        self.data.push((key, value));
    }

    pub fn get(&self, key: &str) -> Option<&str> {
        for itm in &self.data {
            if itm.0 == key {
                return Some(itm.1.as_str());
            }
        }

        return None;
    }

    pub fn iter<'s>(&'s self) -> impl Iterator<Item = (&'static str, &'s str)> {
        self.data.iter().map(|itm| (itm.0, itm.1.as_str()))
    }
}
