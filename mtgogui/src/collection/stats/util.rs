#[derive(Debug, Clone, Default, Copy)]
pub struct UniqueTotal {
    unique: usize,
    total: usize,
}

impl UniqueTotal {
    pub fn new(unique: usize, total: usize) -> Self {
        Self { unique, total }
    }

    pub fn unique(&self) -> usize {
        self.unique
    }

    pub fn total(&self) -> usize {
        self.total
    }
}

#[derive(Debug, Clone, Default)]
pub struct MultiValueStat {
    title: String,
    values: Option<Vec<String>>,
}

impl MultiValueStat {
    pub fn new(title: String, values: Vec<String>) -> Self {
        Self {
            title,
            values: Some(values),
        }
    }

    pub fn add_value(&mut self, value: String) {
        if let Some(values) = &mut self.values {
            values.push(value);
        } else {
            self.values = Some(vec![value]);
        }
    }

    pub fn title(&self) -> &str {
        &self.title
    }

    pub fn take_values(&mut self) -> Vec<String> {
        self.values.take().unwrap_or_default()
    }
}

#[derive(Debug, Clone, Default)]
pub struct CategoryStat {
    title: String,
    value_pairs: Option<Vec<(String, String)>>,
}

impl CategoryStat {
    pub fn new(title: String, value_pairs: Vec<(String, String)>) -> Self {
        Self {
            title,
            value_pairs: Some(value_pairs),
        }
    }

    pub fn title(&self) -> &str {
        &self.title
    }

    pub fn take_value_pairs(&mut self) -> Vec<(String, String)> {
        self.value_pairs.take().unwrap_or_default()
    }

    pub fn add_value_pair(&mut self, description: String, value: String) {
        if let Some(value_pairs) = &mut self.value_pairs {
            value_pairs.push((description, value));
        } else {
            self.value_pairs = Some(vec![(description, value)]);
        }
    }
}
