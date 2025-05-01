Executar programa: `cargo run -- hello poem.txt`

Executar testes unitários: `cargo --tests --test unit_test`

Executar testes unitários com coverage:
`cargo llvm-cov --html --test unit_test  --ignore-filename-regex 'main\.rs$|mod\.rs$'`

Executar testes na IDE: `test --package A_12_IO_Project_Building_Command_Line_Program --test unit_test`