use A_12_IO_Project_Building_Command_Line_Program::core::config as cfg;
#[cfg(test)]
pub mod config_test {
    use super::*;

    #[test]
    fn create_config_successfully() {
        let cases = vec![
            (
                vec![
                    String::from("binary"),
                    String::from("hello"),
                    String::from("file.txt"),
                ],
                cfg::Config {
                    query: String::from("hello"),
                    file_path: String::from("file.txt"),
                },
            ),
            (
                vec![
                    String::from("binary_2"),
                    String::from("hello_2"),
                    String::from("file.txt_2"),
                    String::from("4"),
                    String::from("5"),
                ],
                cfg::Config {
                    query: String::from("hello_2"),
                    file_path: String::from("file.txt_2"),
                },
            ),
        ];

        for (args, expected) in cases {
            // arrange - act
            let result = cfg::Config::new(&args);

            // assert
            assert!(result.is_ok());
            assert_eq!(result.unwrap(), expected);
        }
    }

    #[test]
    fn return_error_on_invalid_input() {
        // arrange
        let args = vec![String::from("binary")];

        // act
        let result = cfg::Config::new(&args);

        // assert
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "not enough arguments");
    }
}
