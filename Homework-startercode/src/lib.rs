use std::collections::HashMap;

/// Returns the first `n` Fibonacci numbers.
pub fn fib(_n: u32) -> Vec<u32> {
    let mut result = Vec::new();
    if _n >= 1 { result.push(0); }
    if _n >= 2 { result.push(1); }
    
    for i in 2.._n as usize {
        let next = result[i - 1] + result[i - 2];
        result.push(next);
    }
    
    result
}

/// Returns true if `n` is a palindrome, false otherwise.
pub fn is_palindrome(_n: u32) -> bool {
    let mut reversed = 0;
    let mut temp = _n;
    
    while temp > 0 {
        let digit = temp % 10;
        reversed = reversed * 10 + digit;
        temp /= 10;
    }
    
    reversed == _n
}

/// Returns the nth largest element in `a`, or None if it does not exist.
pub fn nthmax(_n: usize, _a: &[i32]) -> Option<i32> {
    if _n >= _a.len() {
        return None;
    }
    
    let mut sorted = _a.to_vec();
    sorted.sort();
    
    Some(sorted[_a.len() - _n - 1])
}

/// Returns a one-character String containing the most frequent character in `s`.
pub fn freq(_s: &str) -> String {
    let mut counts: HashMap<char, u32> = HashMap::new();
    let mut max_count = 0;
    let mut max_char = String::new();
    
    for ch in _s.chars() {
        let count = counts.entry(ch).or_insert(0);
        *count += 1;
        
        if *count > max_count {
            max_count = *count;
            max_char = ch.to_string();
        }
    }
    
    max_char
}

/// Zips two slices into a HashMap, mapping arr1[i] -> arr2[i].
pub fn zip_hash(
    _arr1: &[String],
    _arr2: &[String],
) -> Option<HashMap<String, String>> {
    if _arr1.len() != _arr2.len() {
        return None;
    }
    
    let mut result = HashMap::new();
    for i in 0.._arr1.len() {
        result.insert(_arr1[i].clone(), _arr2[i].clone());
    }
    
    Some(result)
}

/// Converts a HashMap into a Vec of (key, value) pairs.
pub fn hash_to_array(
    _map: &HashMap<String, String>,
) -> Vec<(String, String)> {
    let mut keys: Vec<&String> = _map.keys().collect();
    keys.sort();
    
    let mut result = Vec::new();
    for key in keys {
        let value = _map.get(key).unwrap();
        result.push((key.clone(), value.clone()));
    }
    
    result
}

// ========================
// Part 2: PhoneBook
// ========================

/// A single phone book entry.
#[derive(Debug, Clone)]
pub struct PhoneEntry {
    pub name: String,
    pub number: String,
    pub is_listed: bool,
}

/// PhoneBook holds name/number pairs and whether each is listed.
#[derive(Debug, Default)]
pub struct PhoneBook {
    // You are free to change this internal representation if you want.
    pub entries: Vec<PhoneEntry>,
}

impl PhoneBook {
    /// Constructor: create an empty PhoneBook.
    pub fn new() -> Self {
        PhoneBook {
            entries: Vec::new(),
        }
    }

    fn validate_number(_number: &str) -> bool {
        let parts: Vec<&str> = _number.split('-').collect();
        
        if parts.len() != 3 {
            return false;
        }
        
        if parts[0].len() != 3 || parts[1].len() != 3 || parts[2].len() != 4 {
            return false;
        }
        
        for part in parts {
            if part.parse::<u32>().is_err() {
                return false;
            }
        }
        
        true
    }

    /// Attempts to add a new entry.
    ///
    /// Rules:
    /// 1. If the name already exists, return false.
    /// 2. If the number is not in the format NNN-NNN-NNNN, return false.
    /// 3. A number can be unlisted any number of times, but listed at most once.
    ///    - If the number already exists as listed, adding another listed entry
    ///      with the same number must return false.
    ///
    /// Returns true if the entry was successfully added.
    /// 
    
    pub fn add(
        &mut self,
        _name: String,
        _number: String,
        _is_listed: bool,
    ) -> bool {
        for entry in &self.entries {
            if entry.name == _name {
                return false;
            }
            if entry.number == _number && entry.is_listed && _is_listed {
                return false;
            }
        }
        
        if !Self::validate_number(&_number) {
            return false;
        }
        
        self.entries.push(PhoneEntry {
            name: _name,
            number: _number,
            is_listed: _is_listed,
        });
        
        true
    }

    /// Looks up `name` and returns the number ONLY if the entry is listed.
    ///
    /// Otherwise returns None.
    pub fn lookup(&self, _name: &str) -> Option<String> {
        for entry in &self.entries {
            if entry.name == _name {
                if entry.is_listed {
                    return Some(entry.number.clone());
                } else {
                    return None;
                }
            }
        }
        None
    }

    /// Looks up `num` and returns the associated name ONLY if the entry is listed.
    ///
    /// Otherwise returns None.
    pub fn lookup_by_num(&self, _num: &str) -> Option<String> {
        for entry in &self.entries {
            if entry.number == _num {
                if entry.is_listed {
                    return Some(entry.name.clone());
                } else {
                    return None;
                }
            }
        }
        None
    }

    /// Returns all names (listed and unlisted) whose numbers begin with `areacode`.
    pub fn names_by_ac(&self, _areacode: &str) -> Vec<String> {
        let mut result = Vec::new();
        
        for entry in &self.entries {
            if entry.number.starts_with(_areacode) {
                result.push(entry.name.clone());
            }
        }
        
        result
    }
}
