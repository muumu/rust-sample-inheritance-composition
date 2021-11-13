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

mod private {
    pub trait AsTaggedVec {
        fn as_tagged_vec(&self) -> &super::TaggedVec;
        fn as_mut_tagged_vec(&mut self) -> &mut super::TaggedVec;
    }
    pub trait AsDescriptiveVec {
        fn as_descriptive_vec(&self) -> &super::DescriptiveVec;
        fn as_mut_descriptive_vec(&mut self) -> &mut super::DescriptiveVec;
    }
}

impl private::AsTaggedVec for TaggedVec {
    fn as_tagged_vec(&self) -> &TaggedVec { self }
    fn as_mut_tagged_vec(&mut self) -> &mut TaggedVec { self }
}

impl private::AsTaggedVec for DescriptiveVec {
    fn as_tagged_vec(&self) -> &TaggedVec { &self.vec }
    fn as_mut_tagged_vec(&mut self) -> &mut TaggedVec { &mut self.vec }
}

impl private::AsDescriptiveVec for DescriptiveVec {
    fn as_descriptive_vec(&self) -> &DescriptiveVec { self }
    fn as_mut_descriptive_vec(&mut self) -> &mut DescriptiveVec { self }
}

impl private::AsTaggedVec for CountingDescriptiveVec {
    fn as_tagged_vec(&self) -> &TaggedVec { &self.vec.vec }
    fn as_mut_tagged_vec(&mut self) -> &mut TaggedVec { &mut self.vec.vec }
}

impl private::AsDescriptiveVec for CountingDescriptiveVec {
    fn as_descriptive_vec(&self) -> &DescriptiveVec { &self.vec }
    fn as_mut_descriptive_vec(&mut self) -> &mut DescriptiveVec { &mut self.vec }
}

pub trait TaggedVecTrait: private::AsTaggedVec {
    fn push(&mut self, value: i32) {
        self.as_mut_tagged_vec().vec.push(value);
    }
    fn pop(&mut self) -> Option<i32> {
        self.as_mut_tagged_vec().vec.pop()
    }
    fn len(&self) -> usize {
        self.as_tagged_vec().vec.len()
    }
    fn set_tag(&mut self, tag: String) {
        self.as_mut_tagged_vec().tag = tag;
    }
    fn get_tag(&self) -> &str {
        self.as_tagged_vec().tag.as_str()
    }
}

pub trait DescriptiveVecTrait: TaggedVecTrait + private::AsDescriptiveVec {
    fn set_description(&mut self, description: String) {
        self.as_mut_descriptive_vec().description = description;
    }
    fn get_description(&self) -> &str {
        self.as_descriptive_vec().description.as_str()
    }
}

impl TaggedVecTrait for TaggedVec { }
impl TaggedVecTrait for DescriptiveVec { }
impl TaggedVecTrait for CountingDescriptiveVec { }
impl DescriptiveVecTrait for DescriptiveVec { }
impl DescriptiveVecTrait for CountingDescriptiveVec { }

impl TaggedVec {
    pub fn new() -> Self {
        Self { vec: Vec::<i32>::new(), tag: String::new() }
    }
}

impl DescriptiveVec {
    pub fn new() -> Self {
        Self { vec: TaggedVec::new(), description: String::new() }
    }
}

impl CountingDescriptiveVec {
    pub fn new() -> Self {
        Self { vec: DescriptiveVec::new(), push_count: 0, pop_count: 0 }
    }
    // Override
    pub fn push(&mut self, value: i32) {
        self.vec.push(value);
        self.push_count += 1;
    }
    // Override
    pub fn pop(&mut self) -> Option<i32> {
        self.pop_count += 1;
        self.vec.pop()
    }
    pub fn get_push_count(&self) -> usize {
        self.push_count
    }
    pub fn get_pop_count(&self) -> usize {
        self.pop_count
    }
}


// Rustではコンパイルできない書き方
/*
struct TaggedVec {
    vec: Vec<i32>,
    tag: String,
}

// ！Rustには存在しない継承の文法！
struct DescriptiveVec: TaggedVec {
    description: String,
}

// ！Rustには存在しない継承の文法！
struct CountingDescriptiveVec: DescriptiveVec {
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
    // Override
    pub fn push(&mut self, value: i32) {
        self.vec.push(value);
        self.push_count += 1;
    }
    // Override
    pub fn pop(&mut self) -> Option<i32> {
        self.pop_count += 1;
        self.vec.pop()
    }
    pub fn get_push_count(&self) -> usize {
        self.push_count
    }
    pub fn get_pop_count(&self) -> usize {
        self.pop_count
    }
}
*/


// 参照/可変参照の取得をprivateにしない書き方
/*
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

trait TaggedVecTrait {
    fn as_tagged_vec(&self) -> &TaggedVec;
    fn as_mut_tagged_vec(&mut self) -> &mut TaggedVec;
    fn push(&mut self, value: i32) {
        self.as_mut_tagged_vec().vec.push(value);
    }
    fn pop(&mut self) -> Option<i32> {
        self.as_mut_tagged_vec().vec.pop()
    }
    fn len(&self) -> usize {
        self.as_tagged_vec().vec.len()
    }
    fn set_tag(&mut self, tag: String) {
        self.as_mut_tagged_vec().tag = tag;
    }
    fn get_tag(&self) -> &str {
        self.as_tagged_vec().tag.as_str()
    }
}

trait DescriptiveVecTrait: TaggedVecTrait {
    fn as_descriptive_vec(&self) -> &DescriptiveVec;
    fn as_mut_descriptive_vec(&mut self) -> &mut DescriptiveVec;
    fn set_description(&mut self, description: String) {
        self.as_mut_descriptive_vec().description = description;
    }
    fn get_description(&self) -> &str {
        self.as_descriptive_vec().description.as_str()
    }
}

impl TaggedVecTrait for TaggedVec {
    fn as_tagged_vec(&self) -> &TaggedVec { self }
    fn as_mut_tagged_vec(&mut self) -> &mut TaggedVec { self }
}

impl TaggedVecTrait for DescriptiveVec {
    fn as_tagged_vec(& self) -> &TaggedVec { &self.vec }
    fn as_mut_tagged_vec(&mut self) -> &mut TaggedVec { &mut self.vec }
}

impl DescriptiveVecTrait for DescriptiveVec {
    fn as_descriptive_vec(&self) -> &DescriptiveVec { self }
    fn as_mut_descriptive_vec(&mut self) -> &mut DescriptiveVec { self }
}

impl TaggedVecTrait for CountingDescriptiveVec {
    fn as_tagged_vec(& self) -> &TaggedVec { &self.vec.vec }
    fn as_mut_tagged_vec(&mut self) -> &mut TaggedVec { &mut self.vec.vec }
}

impl DescriptiveVecTrait for CountingDescriptiveVec {
    fn as_descriptive_vec(&self) -> &DescriptiveVec { &self.vec }
    fn as_mut_descriptive_vec(&mut self) -> &mut DescriptiveVec { &mut self.vec }
}

impl CountingDescriptiveVec {
    // Override
    fn push(&mut self, value: i32) {
        self.vec.push(value);
        self.push_count += 1;
    }
    // Override
    fn pop(&mut self) -> Option<i32> {
        self.pop_count += 1;
        self.vec.pop()
    }
    fn get_push_count(&self) -> usize {
        self.push_count
    }
    fn get_pop_count(&self) -> usize {
        self.pop_count
    }
}

pub fn use_inheritance() {
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
 */