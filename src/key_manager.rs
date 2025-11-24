pub struct KeyManager {
    keys: Vec<String>,
    current_index: usize,
}

impl KeyManager {
    pub fn new(api_keys: Vec<String>) -> Self {
        Self {
            keys: api_keys,
            current_index: 0,
        }
    }

    pub fn get_key(&mut self) -> String {
        let len = self.keys.len(); 

        if  self.current_index >= len {
            self.current_index = 0;
        }

        let key = self.keys[self.current_index].clone();
        
        key
    }

    pub fn switch_key(&mut self) {
        self.current_index += 1;        
    }
}

