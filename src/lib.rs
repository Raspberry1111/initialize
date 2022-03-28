use std::ops::Index;

/// Initialize an object with a static size from a function
///
/// Example:
/// ```rust
/// # use initialize::*;
/// ##[derive(PartialEq, Eq)]
/// struct MyStruct {
///     some_data: usize    
/// }
///
/// let my_array: [MyStruct; 2] = InitWithIndex::init_with(|index| MyStruct {some_data: index * 3});
///   
///# assert!(
/// my_array == [MyStruct {some_data: 0}, MyStruct {some_data: 3}] // true
///# );
/// ```
pub trait InitWithIndex<N, T, Func: Fn(N) -> T>
where
    Self: Index<N>,
{
    fn init_with(f: Func) -> Self;
}

/// Initialize an object with a dynamic size from a function
///
/// Example
/// ``` rust
/// # use initialize::*;
/// ##[derive(PartialEq, Eq)]
/// struct MyStruct {
///     some_data: usize    
/// }
/// let my_vec = Vec::<MyStruct>::init_with_size(20, |index| MyStruct {some_data: index * 3});
///# assert!(
/// my_vec[16] == MyStruct {some_data: 48} // true
///# );
///```
pub trait InitWithDynamicIndex<N, T, Func: Fn(N) -> T>
where
    Self: Index<N>,
{
    fn init_with_size(size: N, f: Func) -> Self;
}

#[cfg(feature = "unsafe")]
impl<const SIZE: usize, T, Func: Fn(usize) -> T> InitWithIndex<usize, T, Func> for [T; SIZE] {
    fn init_with(f: Func) -> Self {
        let mut arr: std::mem::MaybeUninit<[T; SIZE]> = std::mem::MaybeUninit::zeroed();

        let ptr = arr.as_mut_ptr();
        for index in 0..SIZE {
            unsafe { std::ptr::write((ptr as *mut T).add(index), f(index)) }
            /*
            Safety:
            We just created the zeroed out the chunk of memory with MaybeUninit, so it is safe to write to
            We are aligning by the size of t (add does it automatically) so the ptr is aligned correctly
            */
        }

        unsafe { arr.assume_init() } // We just initialized every value in the array
    }
}

impl<T, Func: Fn(usize) -> T> InitWithDynamicIndex<usize, T, Func> for Vec<T> {
    fn init_with_size(size: usize, f: Func) -> Self {
        let mut vec = Vec::with_capacity(size);
        for index in 0..size {
            vec.push(f(index));
        }
        vec
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(feature = "unsafe")]
    #[test]
    fn array_initialize_with_index() {
        #[derive(PartialEq, Eq, Debug)]
        struct MyData {
            _data: usize,
        }

        let array: [MyData; 3] = InitWithIndex::init_with(|index| MyData { _data: index * 2 });

        assert_eq!(
            array,
            [
                MyData { _data: 0 },
                MyData { _data: 2 },
                MyData { _data: 4 }
            ]
        )
    }
    #[test]
    fn vec_initialize_with_dynamic_index() {
        #[derive(PartialEq, Eq, Debug)]
        struct MyData {
            _data: usize,
        }

        let array = Vec::<MyData>::init_with_size(100, |index| MyData { _data: index * 10 });

        assert_eq!(array[99], MyData { _data: 990 })
    }
}
