extern crate cpython;

use cpython::{Python, PyResult, py_module_initializer, py_fn};

py_module_initializer!(myrustlib, |py, m | {
    m.add(py, "__doc__", "Rust implementation")?;
    m.add(py, "count_doubles", py_fn!(py, count_doubles(chars: &str)))?;
    Ok(())
});


fn count_doubles(_py: Python, chars: &str) -> PyResult<u64> {
    let mut total = 0u64;

    for (char_1, char_2) in chars.chars().zip(chars.chars().skip(1)) {
        if char_1 == char_2 {
            total += 1;
        }
    }

    Ok(total)
}
