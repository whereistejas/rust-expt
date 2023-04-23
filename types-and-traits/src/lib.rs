use std::collections::BTreeMap;

struct A(Vec<u32>);

struct B {
    a: A,
}

type ABMaps = BTreeMap<u32, B>;

impl ABMaps {}
