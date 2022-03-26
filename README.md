

Allows a user to safely extract a Vec<&mut T> out of a Vec<*mut T> provided the vec is empty.


```rust
fn test() {
    let mut v = VecStorage::new();

    {
        let k = &[1usize, 2, 3, 4, 5];
    
        let j = v.as_borrow();

        j.extend(k.iter());

        assert_eq!(*j[0], 1);

        j.clear();
    }

    {
        let k = &mut [5usize, 4, 3, 2, 1];
    
        let j = v.as_borrow_mut();

        j.extend(k.iter_mut());

        assert_eq!(*j[0], 5);
    }
}
```