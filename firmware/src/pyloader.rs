//! Helper structs for binding to a Python file and contextually storing its result

use std::{marker::PhantomData, path::PathBuf};

/// A struct responsible for holding onto a python script and returning it's execution
#[derive(Default, Debug, Clone)]
pub struct PyLoader<INPUT, OUTPUT> {
    /// The filepath to the python script
    script: PathBuf,
    /// The current input being fed to the python script
    input: PhantomData<INPUT>,
    output: PhantomData<OUTPUT>,
}

impl<INPUT: Default, OUTPUT: Default> PyLoader<INPUT, OUTPUT> {
    /// Creates a new PyLoader builder object
    pub fn builder() -> Builder<INPUT, OUTPUT> {
        Builder::default()
    }

    /// Runs a python script with a certain input and returns an appropriate output
    pub fn run(_input: INPUT) -> OUTPUT {
        todo!()
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

impl<INPUT, OUTPUT> Builder<INPUT, OUTPUT> {
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
}
