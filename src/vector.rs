// #[derive(Eq, PartialEq, Clone)]
struct Element {
    index: usize,
    value: f64,
}

pub struct Vector {
    elements: Vec<Element>,
}

impl Vector {
    pub fn new() -> Vector {
        Vector {
            elements: Vec::new(),
        }
    }
    pub fn from(values: Vec<f64>) -> Vector {
        let mut eles = Vec::with_capacity(values.len());
        for (i, v) in values.into_iter().enumerate() {
            eles.push(Element { index: i, value: v })
        }
        Vector { elements: eles }
    }

    pub fn magnitude(&self) -> f64 {
        let mut sum = 0.0;
        for e in self.elements.iter() {
            sum += e.value * e.value;
        }
        return sum.sqrt();
    }

    pub fn dot(&self, other: &Self) -> f64 {
        let alen = self.elements.len();
        let blen = other.elements.len();
        let mut i = 0;
        let mut j = 0;
        let mut product = 0.0;
        while i < alen && j < blen {
            let aidx = self.elements[i].index;
            let bidx = other.elements[i].index;
            if aidx < bidx {
                i += 1;
            } else if aidx > bidx {
                j += 1;
            } else {
                product += self.elements[i].value * other.elements[j].value;
                i += 1;
                j += 1;
            }
        }
        product
    }

    pub fn insert(&mut self, index: usize, value: f64) {
        if let Some(pos) = self.elements.iter().position(|e| e.index >= index) {
            println!("inserting {} and {}", self.elements[pos].index, index);
            assert_ne!(self.elements[pos].index, index);
            self.elements.insert(pos, Element { index, value })
        } else {
            self.elements.push(Element { index, value })
        }
    }

    pub fn upsert(&mut self, index: usize, value: f64) {
        if let Some(pos) = self.elements.iter().position(|e| e.index >= index) {
            self.elements.insert(pos, Element { index, value })
        } else {
            self.elements.push(Element { index, value })
        }
    }

    pub fn similarity(&self, other: &Self) -> f64 {
        let magnitude = self.magnitude();
        if magnitude == 0.0 {
            0.0
        } else {
            self.dot(other) / magnitude
        }
    }
}
