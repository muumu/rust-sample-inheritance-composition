struct TaggedVec {
    vec: Vec<i32>,
    tag: String,
}

struct DescriptiveVec {
    vec: TaggedVec,
    description: String,
}

struct CountingDescriptiveVec {
    vec: DescriptiveVec,
    push_count: usize,
    pop_count: usize
}

impl TaggedVec {
    fn push(&mut self, value: i32) {
        self.vec.push(value);
    }
    fn pop(&mut self) -> Option<i32> {
        self.vec.pop()
    }
    fn len(&self) -> usize {
        self.vec.len()
    }
    fn set_tag(&mut self, tag: String) {
        self.tag = tag;
    }
    fn get_tag(&self) -> &str {
        self.tag.as_str()
    }
}

impl DescriptiveVec {
    fn push(&mut self, value: i32) {
        self.vec.push(value);
    }
    fn pop(&mut self) -> Option<i32> {
        self.vec.pop()
    }
    fn len(&self) -> usize {
        self.vec.len()
    }
    fn set_tag(&mut self, tag: String) {
        self.vec.set_tag(tag);
    }
    fn get_tag(&self) -> &str {
        self.vec.get_tag()
    }
    fn set_description(&mut self, description: String) {
        self.description = description;
    }
    fn get_description(&self) -> &str {
        self.description.as_str()
    }
}

impl CountingDescriptiveVec {
    fn push(&mut self, value: i32) {
        self.vec.push(value);
        self.push_count += 1;
    }
    fn pop(&mut self) -> Option<i32> {
        self.pop_count += 1;
        self.vec.pop()
    }
    fn len(&self) -> usize {
        self.vec.len()
    }
    fn set_tag(&mut self, tag: String) {
        self.vec.set_tag(tag);
    }
    fn get_tag(&self) -> &str {
        self.vec.get_tag()
    }
    fn set_description(&mut self, description: String) {
        self.vec.set_description(description);
    }
    fn get_description(&self) -> &str {
        self.vec.get_description()
    }
    fn get_push_count(&self) -> usize {
        self.push_count
    }
    fn get_pop_count(&self) -> usize {
        self.pop_count
    }
}

pub fn use_composition() {
    let mut v = CountingDescriptiveVec {
        vec: DescriptiveVec {
            vec: TaggedVec { vec: Vec::<i32>::new(), tag: String::new() },
            description: String::new()
        },
        push_count: 0,
        pop_count: 0
    };
    v.set_tag(String::from("foo"));
    v.set_description(String::from("It is foo"));
    v.push(1);
    v.push(2);
    println!("tag: {}", v.get_tag());
    println!("description: {}", v.get_description());
    println!("len: {}", v.len());
    println!("popped: {}", v.pop().unwrap());
    println!("push_count: {}", v.get_push_count());
    println!("pop_count: {}", v.get_pop_count());
}