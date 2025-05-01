use attribute_like_macro::route;

#[route(GET)]
fn home() {
    println!("Página inicial");
}

#[route("POST")]
fn submit() {
    println!("Enviando formulário...");
}

fn main() {
    home();
    submit();
}