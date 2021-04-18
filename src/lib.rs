use std::marker::PhantomData;

pub struct DzCovariant<T> {
    _covariant: PhantomData<fn() -> T>
}

impl<T> DzCovariant<T> {
    /// Create new covariant type.
    ///
    /// ```
    /// # use variance::DzCovariant;
    /// fn check_covariant<'a>(obj: &'a DzCovariant<&'a str>) -> &'a DzCovariant<&'a str> { obj }
    /// # fn main() {
    /// let covariant = DzCovariant::<&'static str>::new();
    /// let covariant = check_covariant(&covariant);
    /// let _test = &covariant;
    /// # }
    /// ```
    ///
    /// ```compile_fail
    /// # use variance::DzCovariant;
    /// fn check_contravariant<'a>(obj: DzCovariant<&'static str>) -> DzCovariant<&'static str> { obj }
    /// # fn test<'a>() {
    /// let covariant = DzCovariant::<&'a str>::new();
    /// let covariant = check_contravariant(covariant);
    /// let _test = &covariant;
    /// # }
    /// # fn main() { test(); }
    /// ```
    ///
    pub fn new() -> Self {
        DzCovariant::<T> {
            _covariant: PhantomData::<fn() -> T>::default()
        }
    }
}

pub struct DzContravariant<T> {
    _contravariant: PhantomData<fn(T)>
}

impl<T> DzContravariant<T> {
    /// Create new contravariant type.
    ///
    /// ```compile_fail
    /// # use variance::DzContravariant;
    /// fn check_covariant<'a>(obj: &'a DzContravariant<&'a str>) -> &'a DzContravariant<&'a str> { obj }
    /// # fn main() {
    /// let contravariant = DzContravariant::<&'static str>::new();
    /// let contravariant = check_covariant(&contravariant);
    /// let _test = &contravariant;
    /// # }
    /// ```
    ///
    /// ```
    /// # use variance::DzContravariant;
    /// fn check_contravariant<'a>(obj: DzContravariant<&'static str>) -> DzContravariant<&'static str> { obj }
    /// # fn test<'a>() {
    /// let contravariant = DzContravariant::<&'a str>::new();
    /// let contravariant = check_contravariant(contravariant);
    /// let _test = &contravariant;
    /// # }
    /// # fn main() { test(); }
    /// ```
    ///
    pub fn new() -> Self {
        DzContravariant::<T> {
            _contravariant: PhantomData::<fn(T)>::default()
        }
    }
}

pub struct DzInvariant<T> {
    _invariant: PhantomData<fn(T) -> T>
}

impl<T> DzInvariant<T> {
    /// Create new invariant type.
    ///
    /// ```compile_fail
    /// # use variance::DzInvariant;
    /// fn check_covariant<'a>(obj: &'a DzInvariant<&'a str>) -> &'a DzInvariant<&'a str> { obj }
    /// # fn main() {
    /// let invariant = DzInvariant::<&'static str>::new();
    /// let invariant = check_covariant(&invariant);
    /// let _test = &invariant;
    /// # }
    /// ```
    ///
    /// ```compile_fail
    /// # use variance::DzInvariant;
    /// fn check_contravariant<'a>(obj: DzInvariant<&'static str>) -> DzInvariant<&'static str> { obj }
    /// # fn test<'a>() {
    /// let invariant = DzInvariant::<&'a str>::new();
    /// let invariant = check_contravariant(invariant);
    /// let _test = &invariant;
    /// # }
    /// # fn main() { test(); }
    /// ```
    ///
    pub fn new() -> Self {
        DzInvariant::<T> {
            _invariant: PhantomData::<fn(T) -> T>::default()
        }
    }
}
