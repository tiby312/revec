pub struct VecStorage<T> {
    inner: Vec<*mut T>,
}

#[test]
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

impl<T> Default for VecStorage<T> {
    #[inline(always)]
    #[must_use]
    fn default() -> Self {
        VecStorage::new()
    }
}
impl<T> VecStorage<T> {
    #[inline(always)]
    #[must_use]
    pub fn from_vec(inner: Vec<*mut T>) -> Self {
        VecStorage { inner }
    }

    #[inline(always)]
    #[must_use]
    pub fn new() -> Self {
        Self::from_vec(vec![])
    }

    #[inline(always)]
    #[must_use]
    pub fn as_borrow<'a, 'b>(&'a mut self) -> &'a mut Vec<&'b T> {
        assert!(self.inner.is_empty());
        unsafe { &mut *((&mut self.inner) as *mut _ as *mut _) }
    }

    #[inline(always)]
    #[must_use]
    pub fn as_borrow_mut<'a, 'b>(&'a mut self) -> &'a mut Vec<&'b mut T> {
        assert!(self.inner.is_empty());
        unsafe { &mut *((&mut self.inner) as *mut _ as *mut _) }
    }

    #[inline(always)]
    #[must_use]
    pub fn as_borrow_custom_ptr<K>(&mut self) -> &mut Vec<K> {
        //Make const one day https://blog.rust-lang.org/2021/12/02/Rust-1.57.0.html
        assert!(std::mem::size_of::<K>() == std::mem::size_of::<*mut T>());
        assert!(std::mem::align_of::<K>() == std::mem::align_of::<*mut T>());
        assert!(self.inner.is_empty());
        unsafe { &mut *((&mut self.inner) as *mut _ as *mut _) }
    }

    #[inline(always)]
    #[must_use]
    pub fn as_inner(&mut self) -> &mut Vec<*mut T> {
        &mut self.inner
    }

    #[inline(always)]
    #[must_use]
    pub fn into_inner(self) -> Vec<*mut T> {
        self.inner
    }
}
