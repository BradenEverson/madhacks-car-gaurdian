//! Helper structs for binding to a Python file and contextually storing its result

use std::{
    marker::PhantomData,
    path::PathBuf,
    process::{Command, Stdio},
    str::FromStr,
};

use py_input::PyArg;

pub mod py_input;

/// A struct responsible for holding onto a python script and returning it's execution
#[derive(Default, Debug, Clone)]
pub struct PyLoader<INPUT: PyArg, OUTPUT: FromStr> {
    /// The filepath to the python script
    script: PathBuf,
    /// The current input being fed to the python script
    input: PhantomData<INPUT>,
    output: PhantomData<OUTPUT>,
}

impl<INPUT: Default + PyArg, OUTPUT: Default + FromStr> PyLoader<INPUT, OUTPUT> {
    /// Creates a new PyLoader builder object
    pub fn builder() -> Builder<INPUT, OUTPUT> {
        Builder::default()
    }

    /// Runs a python script with a certain input and returns an appropriate output
    pub fn run(&self, input: INPUT) -> Option<OUTPUT> {
        let mut output = Command::new("python3");
        let output = output.arg(&self.script);

        for arg in input.to_cli_arg().trim().split(' ') {
            output.arg(arg);
        }
        let output = output.stdout(Stdio::piped()).output().ok()?;

        if output.status.success() {
            let stdout_str = String::from_utf8(output.stdout).ok()?;
            let result = OUTPUT::from_str(stdout_str.trim()).ok()?;
            Some(result)
        } else {
            None
        }
    }
}

/// A PyLoader builder
#[derive(Default)]
pub struct Builder<INPUT, OUTPUT> {
    /// Script Path
    script: Option<PathBuf>,
    /// Input
    input: PhantomData<INPUT>,
    /// Output
    output: PhantomData<OUTPUT>,
}

impl<INPUT: PyArg, OUTPUT: FromStr> Builder<INPUT, OUTPUT> {
    /// Loads a python script
    pub fn with_script<P: Into<PathBuf>>(mut self, path: P) -> Self {
        self.script = Some(path.into());
        self
    }

    /// Builds a PyLoader with respect to this builder object
    pub fn build(self) -> Option<PyLoader<INPUT, OUTPUT>> {
        Some(PyLoader {
            script: self.script?,
            input: PhantomData,
            output: PhantomData,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::PyLoader;

    #[test]
    fn builder_behaves_properly() {
        let script_runner = PyLoader::<f32, f32>::builder()
            .with_script("foo/bar.py")
            .build();
        assert!(script_runner.is_some())
    }

    #[test]
    fn one_plus_one_from_python() {
        let one_plus_one = PyLoader::<[i32; 2], i32>::builder()
            .with_script("test_scripts/add_two_nums.py")
            .build()
            .expect("Get PyLoader");

        let result = one_plus_one.run([1, 1]).expect("1 + 1 = ?");
        assert_eq!(result, 2)
    }
}
