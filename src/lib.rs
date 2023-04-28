pub mod random_points;
use crate::random_points::run;
use pyo3::prelude::*;

fn init_submodule(module: &PyModule) -> PyResult<()> {
    module.add_function(wrap_pyfunction!(run, module)?)?;
    Ok(())
}

#[pymodule]
fn crab(py: Python, module: &PyModule) -> PyResult<()> {
    let submod = PyModule::new(py, "random_points")?;
    init_submodule(submod)?;
    module.add_submodule(submod)?;
    Ok(())
}
