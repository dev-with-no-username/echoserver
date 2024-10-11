// we need to put this macro here, cause it's here where we define the
// module and if we don't do this, but we put this macro inside the file,
// Rust complains about 'unused import' even if we use that import
// and even if the test running, cause the test runs in any case
#[cfg(test)]
pub mod test;