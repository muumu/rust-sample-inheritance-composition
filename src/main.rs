use crate::inheritance::*;

mod inheritance;
mod composition;
mod generics;

fn use_composition() {
    let mut v = composition::CountingDescriptiveVec::new();
    v.set_tag(String::from("foo"));
    v.set_description(String::from("It is foo"));
    v.push(1);
    v.push(2);
    println!("tag: {}", v.get_tag());
    println!("description: {}", v.get_description());
    println!("len: {}", v.len());
    println!("popped: {}", v.pop().unwrap());
    println!("push_count: {}", v.get_push_count()); // 2
    println!("pop_count: {}", v.get_pop_count()); // 1
}

pub fn use_inheritance() {
    let mut v = inheritance::CountingDescriptiveVec::new();
    v.set_tag(String::from("foo"));
    v.set_description(String::from("It is foo"));
    v.push(1);
    v.push(2);
    println!("tag: {}", v.get_tag());
    println!("description: {}", v.get_description());
    println!("len: {}", v.len());
    println!("popped: {}", v.pop().unwrap());
    println!("push_count: {}", v.get_push_count()); // 2
    println!("pop_count: {}", v.get_pop_count()); // 1
    // let _vref = v.as_mut_descriptive_vec(); // エラー: privateなので可変参照は取得できない
}

fn main() {
    use_inheritance();
    use_composition();
    generics::inheritance::use_inheritance();
}
