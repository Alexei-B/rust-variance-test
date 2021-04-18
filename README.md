
This is some code to assert the behaviour of variance over T for some generic types using `PhantomData`.

Specifically, the following should hold true for a type `Type<T>` for variance over `T` where `Type<T>` contains the specified `PhantomData`:

|`Type`                   |Variance Over `T`|
|-------------------------|-----------------|
|`PhantomData<fn() -> T>` |covariant        |
|`PhantomData<fn(T)>`     |contravariant    |
|`PhantomData<fn(T) -> T>`|invariant        |

To use: `cargo doc --open`
