# ordered_vec
This little library provides a trait that ensures elements in a vector are pushed in a sorted order.

### Example
```rust
use ordered_vec::OrdVec;
fn main() {
	let mut values: Vec<i32> = Vec::new();
	values.push_ord(5);
	values.push_ord(3);
	values.push_ord(7);
	values.push_ord(1);
	assert_eq!(values, [1, 3, 5, 7]);
}
```

### License
This library is licensed under [MIT](https://opensource.org/licenses/MIT) and [Apache-2.0](https://opensource.org/licenses/Apache-2.0).
