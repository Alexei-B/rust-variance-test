use std::marker::PhantomData;

pub struct Covariant<T> {
    _covariant: PhantomData<fn() -> T>,
}

impl<T> Covariant<T> {
    /// Create new covariant type.
    ///
    /// ```
    /// # use variance::Covariant;
    /// fn is_covariant<'a>(obj: &'a Covariant<&'a str>) -> &'a Covariant<&'a str> { obj }
    /// # fn main() {
    /// // Works because 'static is covariant with 'a, and Covariant is covariant over T.
    /// let covariant = Covariant::<&'static str>::new();
    /// let covariant = is_covariant(&covariant);
    /// let _test = &covariant;
    /// # }
    /// ```
    ///
    /// ```compile_fail
    /// # use variance::Covariant;
    /// fn is_contravariant(obj: Covariant<&'static str>) -> Covariant<&'static str> { obj }
    /// # fn test<'a>() {
    /// // Fails because 'a is contravariant with 'static, and Covariant is covariant over T.
    /// let covariant = Covariant::<&'a str>::new();
    /// let covariant = is_contravariant(covariant);
    /// let _test = &covariant;
    /// # }
    /// # fn main() { test(); }
    /// ```
    ///
    pub fn new() -> Self {
        Covariant::<T> {
            _covariant: PhantomData::<fn() -> T>::default(),
        }
    }
}

pub struct Contravariant<T> {
    _contravariant: PhantomData<fn(T)>,
}

impl<T> Contravariant<T> {
    /// Create new contravariant type.
    ///
    /// ```compile_fail
    /// # use variance::Contravariant;
    /// fn is_covariant<'a>(obj: &'a Contravariant<&'a str>) -> &'a Contravariant<&'a str> { obj }
    /// # fn main() {
    /// // Fails because 'static is covariant with 'a, and Contravariant is contravariant over T.
    /// let contravariant = Contravariant::<&'static str>::new();
    /// let contravariant = is_covariant(&contravariant);
    /// let _test = &contravariant;
    /// # }
    /// ```
    ///
    /// ```
    /// # use variance::Contravariant;
    /// fn is_contravariant(obj: Contravariant<&'static str>) -> Contravariant<&'static str> { obj }
    /// # fn test<'a>() {
    /// // Works because 'a is contravariant with 'static, and Contravariant is contravariant over T.
    /// let contravariant = Contravariant::<&'a str>::new();
    /// let contravariant = is_contravariant(contravariant);
    /// let _test = &contravariant;
    /// # }
    /// # fn main() { test(); }
    /// ```
    ///
    pub fn new() -> Self {
        Contravariant::<T> {
            _contravariant: PhantomData::<fn(T)>::default(),
        }
    }
}

pub struct Invariant<T> {
    _invariant: PhantomData<fn(T) -> T>,
}

impl<T> Invariant<T> {
    /// Create new invariant type.
    ///
    /// ```compile_fail
    /// # use variance::Invariant;
    /// fn is_covariant<'a>(obj: &'a Invariant<&'a str>) -> &'a Invariant<&'a str> { obj }
    /// # fn main() {
    /// // Fails because 'static is covariant with 'a, and Invariant is invariant over T.
    /// let invariant = Invariant::<&'static str>::new();
    /// let invariant = is_covariant(&invariant);
    /// let _test = &invariant;
    /// # }
    /// ```
    ///
    /// ```compile_fail
    /// # use variance::Invariant;
    /// fn is_contravariant(obj: Invariant<&'static str>) -> Invariant<&'static str> { obj }
    /// # fn test<'a>() {
    /// // Fails because 'a is contravariant with 'static, and Invariant is invariant over T.
    /// let invariant = Invariant::<&'a str>::new();
    /// let invariant = is_contravariant(invariant);
    /// let _test = &invariant;
    /// # }
    /// # fn main() { test(); }
    /// ```
    ///
    pub fn new() -> Self {
        Invariant::<T> {
            _invariant: PhantomData::<fn(T) -> T>::default(),
        }
    }
}
