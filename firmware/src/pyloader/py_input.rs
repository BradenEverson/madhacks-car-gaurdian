//! Trait for Converting a primative type to a command line argument

/// Trait defining a type that can be converted to a command line argument as an input to a python
/// script
pub trait PyArg {
    /// Convert type to an appropriate argument
    fn to_cli_arg(&self) -> String;
}

/// Creates a default PyArg implementation that just uses a types to_string() implementation
macro_rules! default_py_arg_impl {
    ($($dtype:ty),*) => {
        $(
        impl PyArg for $dtype {
            fn to_cli_arg(&self) -> String {
                self.to_string()
            }
        }
        )*
    };
}

default_py_arg_impl!(u8, u16, u32, u64, usize, i8, i16, i32, i64, isize, f32, f64, String, &str);

impl<const N: usize, ARG: PyArg> PyArg for [ARG; N] {
    fn to_cli_arg(&self) -> String {
        let mut res = String::new();

        for arg in self {
            res += &format!("{} ", arg.to_cli_arg());
        }

        res
    }
}

#[cfg(test)]
mod tests {
    use super::PyArg;

    #[test]
    fn pyarg_on_array() {
        assert_eq!("10 20 30 ", [10, 20, 30].to_cli_arg())
    }
}
