pub struct TaggedVec {
    vec: Vec<i32>,
    tag: String,
}

pub struct DescriptiveVec {
    vec: TaggedVec,
    description: String,
}

pub struct CountingDescriptiveVec {
    vec: DescriptiveVec,
    push_count: usize,
    pop_count: usize
}

impl TaggedVec {
    pub fn new() -> Self {
        Self { vec: Vec::<i32>::new(), tag: String::new() }
    }
    pub fn push(&mut self, value: i32) {
        self.vec.push(value);
    }
    pub fn pop(&mut self) -> Option<i32> {
        self.vec.pop()
    }
    pub fn len(&self) -> usize {
        self.vec.len()
    }
    pub fn set_tag(&mut self, tag: String) {
        self.tag = tag;
    }
    pub fn get_tag(&self) -> &str {
        self.tag.as_str()
    }
}

impl DescriptiveVec {
    pub fn new() -> Self {
        Self { vec: TaggedVec::new(), description: String::new() }
    }
    pub fn push(&mut self, value: i32) {
        self.vec.push(value);
    }
    pub fn pop(&mut self) -> Option<i32> {
        self.vec.pop()
    }
    pub fn len(&self) -> usize {
        self.vec.len()
    }
    pub fn set_tag(&mut self, tag: String) {
        self.vec.set_tag(tag);
    }
    pub fn get_tag(&self) -> &str {
        self.vec.get_tag()
    }
    pub fn set_description(&mut self, description: String) {
        self.description = description;
    }
    pub fn get_description(&self) -> &str {
        self.description.as_str()
    }
}

impl CountingDescriptiveVec {
    pub fn new() -> Self {
        Self { vec: DescriptiveVec::new(), push_count: 0, pop_count: 0 }
    }
    pub fn push(&mut self, value: i32) {
        self.vec.push(value);
        self.push_count += 1;
    }
    pub fn pop(&mut self) -> Option<i32> {
        self.pop_count += 1;
        self.vec.pop()
    }
    pub fn len(&self) -> usize {
        self.vec.len()
    }
    pub fn set_tag(&mut self, tag: String) {
        self.vec.set_tag(tag);
    }
    pub fn get_tag(&self) -> &str {
        self.vec.get_tag()
    }
    pub fn set_description(&mut self, description: String) {
        self.vec.set_description(description);
    }
    pub fn get_description(&self) -> &str {
        self.vec.get_description()
    }
    pub fn get_push_count(&self) -> usize {
        self.push_count
    }
    pub fn get_pop_count(&self) -> usize {
        self.pop_count
    }
}