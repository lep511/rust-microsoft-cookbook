// Sample test
fn function_1() -> Result(Success, Failure) {
    match operation_that_might_fail() {
        Ok(success) => success,
        Err(failure) => return Err(failure),
    }
}

fn function_2() -> Result(Success, Failure) {
    operation_that_might_fail()?
}