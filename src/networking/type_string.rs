use std::intrinsics;

pub fn print_type_of<T>(_: &T) {
    println!("{}", unsafe { std::intrinsics::type_name::<T>() });
}

pub fn type_name_of<T>(_: &T) -> &str {
    unsafe { intrinsics::type_name::<T>() }
}

//pub trait TypeInfo {
//    fn type_name() -> String;
//    fn type_of(&self) -> String;
//}

/*
macro_rules! impl_type_info {
    ($($name:ident$(<$($T:ident),+>)*),*) => {
        $(impl_type_info_single!($name$(<$($T),*>)*);)*
    };
}

macro_rules! mut_if {
    ($name:ident = $value:expr, $($any:expr)+) => (let mut $name = $value;);
    ($name:ident = $value:expr,) => (let $name = $value;);
}

macro_rules! impl_type_info_single {
    ($name:ident$(<$($T:ident),+>)*) => {
        impl$(<$($T: TypeInfo),*>)* TypeInfo for $name$(<$($T),*>)* {
            fn type_name() -> String {
                mut_if!(res = String::from(stringify!($name)), $($($T)*)*);
                $(
                    res.push('<');
                    $(
                        res.push_str(&$T::type_name());
                        res.push(',');
                    )*
                    res.pop();
                    res.push('>');
                )*
                res
            }
            fn type_of(&self) -> String {
                $name$(::<$($T),*>)*::type_name()
            }
        }
    }
}

impl<'a, T: TypeInfo + ?Sized> TypeInfo for &'a T {
    fn type_name() -> String {
        let mut res = String::from("&");
        res.push_str(&T::type_name());
        res
    }
    fn type_of(&self) -> String {
        <&T>::type_name()
    }
}

macro_rules! type_of {
    ($x:expr) => { (&$x).type_of() };
}

impl_type_info!(u8, u16, u32, u64, usize, i8, i16, i32, i64, f32, f64);
impl_type_info!(str, String, Vec<T>, Result<T,S>);
*/
