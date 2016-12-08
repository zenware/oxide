//use std::any;
use std::any::TypeId;
use std::collections::HashMap;
/// A value is a typed, singular unit of data.
/// A value can be a relation.
#[derive(PartialEq, Eq, Hash)]
struct Value<T> {
    data: T
}

/// An attribute is a pair containing an attribute name along
/// with an attribute type.
///
/// ("Address", "Text")
///
#[derive(PartialEq, Eq, Hash)]
pub struct Attribute {
    pub attr_type: TypeId, //::of::<T>()
    pub attr_name: String,
}

impl Attribute {
    /// &smut self and modify self.
    pub fn rename(&self, new_name: String) {
        //self.attrName = newName;
        println!("{}", new_name);
    }
}

#[test]
fn test_attribute() {
    assert!(false);
}

/// A tuple is a set of attribute names mapped to values.
/// I'm not 100% sure I need to store attributes the way I am bc of that
struct Tuple<T> {
    attributes: HashMap<Attribute, Value<T>>,
    //values: Vec<Value<T>>
}

struct Relation<T> {
    set: Tuple<T>
}

#[test]
fn test_relation() {
    let my_value: Value<String> = Value::<String> { data: String::from("Jay Looney") };
    let my_attribute = Attribute {
        attr_name: String::from("Name"),
        attr_type: TypeId::of::<String>()
    };
    let mut my_hashmap: HashMap<Attribute, Value<String>> = HashMap::new();
    my_hashmap.insert(my_attribute, my_value);
    let my_tuple: Tuple<String> = Tuple::<String> { attributes: my_hashmap };
    let my_relation: Relation<String> = Relation::<String> { set: my_tuple };

    assert!(false);
}
