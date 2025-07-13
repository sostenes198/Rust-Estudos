#[cfg(test)]
pub mod file_reader_tests {
    use A_12_IO_Project_Building_Command_Line_Program::core::file_reader_system::FileReaderSystem;
    use A_12_IO_Project_Building_Command_Line_Program::core::traits::file_reader::FileReader;
    use std::io::{ErrorKind, Write};
    use tempfile::{Builder, NamedTempFile};

    const FILE_NAME: &'static str = "file_reader_system_test";
    const FILE_EXTENSION: &'static str = ".txt";

    fn prepare_to_test() -> NamedTempFile {
        let temp_file = Builder::new()
            .prefix(FILE_NAME)
            .suffix(FILE_EXTENSION)
            .tempfile()
            .expect("Failed to create temp file");

        let mut file = temp_file.as_file();
        writeln!(file, "This is a test file content").expect("Failed to write to temp file");

        temp_file
    }

    #[test]
    pub fn read_file_content_successfully() {
        // arrange
        let temp_file = prepare_to_test();

        let file_reader_system = FileReaderSystem {};

        // act
        let result =
            file_reader_system.read_to_string(&temp_file.path().to_str().unwrap().to_string());

        // assert
        assert!(result.is_ok());

        let content = result.unwrap();
        assert_eq!(content, "This is a test file content\n");
    }

    #[test]
    pub fn return_error_when_not_found_file_to_read() {
        // arrange
        let file_reader_system = FileReaderSystem {};

        // act
        let result = file_reader_system.read_to_string(&"invalid_file_path".to_string());

        // assert
        assert!(result.is_err());

        let err = result.unwrap_err();
        assert_eq!(err.kind(), ErrorKind::NotFound);
    }
}
