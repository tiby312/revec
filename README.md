

Allows a user to safely convert between `Vec<A>` and `Vec<B>` provided the vec is empty is A and B have the same memory size and alignment.


```rust
fn test() {
    let v:Vec<*mut usize> = Vec::new();

    assert_eq!(v.capacity(),0);
    
    let k = &mut [5usize, 4, 3, 2, 1];

    let mut v2:Vec<&usize>=convert_empty_vec(v);
    v2.extend(k.iter());

    assert_eq!(*v2[0], 5);

    v2.clear();
    assert!(v2.capacity()>=5);


    let mut v3:Vec<&mut usize>=convert_empty_vec(v2);
    assert!(v3.capacity()>=5);


    let k = &mut [5usize, 4, 3, 2, 1];

    v3.extend(k.iter_mut());

    assert_eq!(*v3[0], 5);

    
}
```