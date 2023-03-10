use std::fmt::Debug;

mod non_order_array {
  pub struct Array<T> {
    value: Vec<T>,
    deleted_indexes: Vec<uint8>,
    length: uint8,
  }

  impl <T> Array<T> {
    pub fn new(self, value: Vec<T>) {
      self.value = value;
    }

    pub fn get(self, index: uint8) -> T {
      let len = self.value.len() - 1;
      if index > len {
        self.value[len]
      }

      self.value[index]
    }

    pub fn set(self, index: uint8, value: T) {
      self.value[self.value.len()] = self.value[index];
      self.value[index] = value;
    }

    pub fn push(self, value: T) {
      self.value.push(value);
    }

    pub fn remove(self, index: uint8) {
    }

    fn real_remove() {
      if self.value.len() == length {

      }
    }

  }

  impl Debug for Array {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
      println!("{:?}", &self.value);
    }
  }

}
