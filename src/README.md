

Allows a user to safely extract a Vec<&mut T> out of a Vec<*mut T> provided the vec is empty.


```rust
fn test() {
    let k = &mut [1usize, 2, 3, 4, 5];
    let mut v = VecStorage::new();

    {
        let j = v.as_borrow();

        j.extend(k.iter());

        assert_eq!(j[2], &3);

        j.clear();
    }

    {
        let j = v.as_borrow_mut();

        j.extend(k.iter_mut());

        assert_eq!(j[2], &3);
    }
}
```