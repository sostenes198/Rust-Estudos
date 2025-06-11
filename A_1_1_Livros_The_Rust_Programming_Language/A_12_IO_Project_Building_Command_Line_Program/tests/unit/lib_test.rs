use A_12_IO_Project_Building_Command_Line_Program::Lib;

#[cfg(test)]
pub mod lib_test {
    use super::*;
    use mockall;
    use mockall::predicate;
    use std::io::{Error, ErrorKind};
    use A_12_IO_Project_Building_Command_Line_Program::core::traits::file_reader::MockFileReader;

    #[test]
    fn execute_lib_successfully() {
        // arrange
        let file_path = String::from("file.txt");

        let args_vec = vec![
            String::from("binary"),
            String::from("hello"),
            file_path.to_string(),
        ];
        
        let args = args_vec.into_iter();

        let mut mock_file_reader = MockFileReader::new();
        mock_file_reader
            .expect_read_to_string()
            .with(predicate::eq(file_path))
            .times(1)
            .returning(|_| {
                Ok("\
        Test file content
        hello world
        "
                .to_string())
            });

        let lib = Lib::new(mock_file_reader);

        // act
        let result = lib.run(args);

        // assert
        assert!(result.is_ok());
    }

    #[test]
    pub fn execute_lib_failure_when_failed_to_read_file() {
        // arrange
        let file_path = String::from("file.txt");

        let args_vec = vec![
            String::from("binary"),
            String::from("hello"),
            file_path.to_string(),
        ];
        
        let args = args_vec.into_iter();

        let mut mock_file_reader = MockFileReader::new();
        mock_file_reader
            .expect_read_to_string()
            .with(predicate::eq(file_path))
            .times(1)
            .returning(|_| Err(Error::new(ErrorKind::Other, "FAIL")));

        let lib = Lib::new(mock_file_reader);

        // act
        let result = lib.run(args);

        // assert
        assert!(result.is_err());

        let error = result.unwrap_err();
        let io_error = error.downcast_ref::<Error>().unwrap();

        assert_eq!(io_error.kind(), ErrorKind::Other);
        assert_eq!(io_error.to_string(), "FAIL");
    }
}
