#[test]
fn test() {
    let v: Vec<*mut usize> = Vec::new();

    assert_eq!(v.capacity(), 0);

    let k = &mut [5usize, 4, 3, 2, 1];

    let mut v2: Vec<&usize> = convert_empty_vec(v);
    v2.extend(k.iter());

    assert_eq!(*v2[0], 5);

    v2.clear();
    assert!(v2.capacity() >= 5);

    let mut v3: Vec<&mut usize> = convert_empty_vec(v2);
    assert!(v3.capacity() >= 5);

    let k = &mut [5usize, 4, 3, 2, 1];

    v3.extend(k.iter_mut());

    assert_eq!(*v3[0], 5);
}

///
/// Convert an empty vec conserving capacity.
/// The memory size and alignment of A and B must be equal.
/// Panics if they are not.
/// Panics if the specified vec is not empty.
///
pub fn convert_empty_vec<A, B>(vec: Vec<A>) -> Vec<B> {
    
    const{ assert!(std::mem::size_of::<A>() == std::mem::size_of::<B>())};
    const{ assert!(std::mem::align_of::<A>() == std::mem::align_of::<B>())};
    assert!(vec.is_empty());

    let mut v_clone = std::mem::ManuallyDrop::new(vec);
    unsafe {
        Vec::from_raw_parts(
            v_clone.as_mut_ptr() as *mut B,
            v_clone.len(),
            v_clone.capacity(),
        )
    }
}
