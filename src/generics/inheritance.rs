struct TaggedVec<T> {
    vec: Vec<T>,
    tag: String,
}

struct DescriptiveVec<T> {
    vec: TaggedVec<T>,
    description: String,
}

trait TaggedVecTrait<'a, T: 'a> {
    fn as_tagged_vec(&'a self) -> &'a TaggedVec<T>;
    fn as_mut_tagged_vec(&'a mut self) -> &'a mut TaggedVec<T>;
    fn push(&'a mut self, value: T) {
        self.as_mut_tagged_vec().vec.push(value);
    }
    fn pop(&'a mut self) -> Option<T> {
        self.as_mut_tagged_vec().vec.pop()
    }
    fn len(&'a self) -> usize {
        self.as_tagged_vec().vec.len()
    }
    fn set_tag(&'a mut self, tag: String) {
        self.as_mut_tagged_vec().tag = tag;
    }
    fn get_tag(&'a self) -> &'a str {
        self.as_tagged_vec().tag.as_str()
    }
}

trait DescriptiveVecTrait<'a, T: 'a>: TaggedVecTrait<'a, T> {
    fn as_descriptive_vec(&'a self) -> &'a DescriptiveVec<T>;
    fn as_mut_descriptive_vec(&'a mut self) -> &'a mut DescriptiveVec<T>;
    fn set_description(&'a mut self, description: String) {
        self.as_mut_descriptive_vec().description = description;
    }
    fn get_description(&'a self) -> &'a str {
        self.as_descriptive_vec().description.as_str()
    }
}

impl<'a, T: 'a> TaggedVecTrait<'a, T> for TaggedVec<T> {
    fn as_tagged_vec(&'a self) -> &'a TaggedVec<T> { self }
    fn as_mut_tagged_vec(&'a mut self) -> &'a mut TaggedVec<T> { self }
}

impl<'a, T: 'a> TaggedVecTrait<'a, T> for DescriptiveVec<T> {
    fn as_tagged_vec(&'a self) -> &'a TaggedVec<T> { &self.vec }
    fn as_mut_tagged_vec(&'a mut self) -> &'a mut TaggedVec<T> { &mut self.vec }
}

impl<'a, T: 'a> DescriptiveVecTrait<'a, T> for DescriptiveVec<T> {
    fn as_descriptive_vec(&'a self) -> &'a DescriptiveVec<T> { self }
    fn as_mut_descriptive_vec(&'a mut self) -> &'a mut DescriptiveVec<T> { self }
}

pub fn use_inheritance() {
    let mut descriptive_vec = DescriptiveVec::<i32> {
        vec: TaggedVec::<i32> { vec: Vec::<i32>::new(), tag: String::new() },
        description: String::new()
    };
    descriptive_vec.set_tag(String::from("foo"));
    descriptive_vec.set_description(String::from("It is foo"));
    descriptive_vec.push(1);
    descriptive_vec.push(2);
    println!("tag: {}", descriptive_vec.get_tag());
    println!("description: {}", descriptive_vec.get_description());
    println!("len: {}", descriptive_vec.len());
    println!("popped: {}", descriptive_vec.pop().unwrap());
}