#[cfg(test)]
mod tests {
    uniffi::build_foreign_language_testcases!("tests/test_networking.swift",);
    uniffi::build_foreign_language_testcases!("tests/test_async_stream.swift",);
    uniffi::build_foreign_language_testcases!("tests/test_file_io.swift",);
}
