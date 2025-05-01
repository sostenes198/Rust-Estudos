use function_like_macro::sql;

fn main() {
    sql!("SELECT * FROM posts WHERE id=1");
}
