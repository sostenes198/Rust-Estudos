use std::alloc::{alloc, dealloc, Layout};
use std::process;
use std::thread;
use std::time::Duration;

fn spin(seconds: u64) {
    thread::sleep(Duration::from_secs(seconds));
}

fn sample_using_alloc(){
    unsafe {
        // 1. Define o layout da alocação (um i32)
        let layout = Layout::new::<i32>();

        // 2. Aloca memória para um i32 (como malloc)
        let p: *mut i32 = alloc(layout) as *mut i32;

        // 3. Verifica se a alocação falhou
        if p.is_null() {
            panic!("Falha na alocação");
        }

        // 4. Inicializa o valor apontado para 0
        *p = 0;

        // 5. Imprime o endereço e o valor inicial
        println!(
            "({}) endereço apontado por p: {:p}, valor: {}",
            process::id(),
            p,
            *p
        );

        // 6. Loop que incrementa o valor e imprime
        loop {
            spin(1);
            *p += 1;
            println!("({}) valor de p: {}", process::id(), *p);
        }

        // Nunca chega aqui, mas se fosse sair, desalocaria:
        dealloc(p as *mut u8, layout);
    }
}

fn main() {
    let mut p = Box::new(42);

    let pid = process::id();
    let ptr_addr = &*p as *const i32;

    println!("p      = {:?}", p);         // Box<i32>
    println!("*p     = {}", *p);          // desreferencia: 42
    println!("&p     = {:p}", &p);        // endereço da variável `p` (na stack)
    println!("&*p    = {:p}", &*p);       // endereço do valor `42` (na heap)
    unsafe {
        println!("value p = {:?}", *ptr_addr);
    }


    println!("({}) address pointed to by p: {:p}", pid, ptr_addr);

    // Loop infinito
    loop {
        spin(1);
        *p += 1;
        println!("({}) p: {}", pid, *p);
    }
}
