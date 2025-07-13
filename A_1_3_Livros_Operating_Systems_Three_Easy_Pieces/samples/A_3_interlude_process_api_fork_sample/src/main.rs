use nix::sys::wait::wait;
use nix::unistd::{ForkResult, execvp, fork, getpid};
use std::ffi::CString;
use std::process::exit;

fn sample_fork() {
    println!("hello world (pid:{})", getpid());

    match unsafe { fork() } {
        Ok(ForkResult::Child) => {
            println!("hello, I am child (pid:{})", getpid());
        }
        Ok(ForkResult::Parent { child }) => {
            println!("hello, I am parent of {} (pid:{})", child, getpid());
        }
        Err(_) => {
            eprintln!("fork failed");
            exit(1);
        }
    }
}

fn sample_with_wait_child_process() {
    println!("hello world (pid:{})", getpid());

    let rc = unsafe { fork() };
    match rc {
        Ok(ForkResult::Child) => {
            println!("hello, I am child (pid:{})", getpid());
        }
        Ok(ForkResult::Parent { child }) => {
            let rc_wait = wait().expect("wait failed");
            println!("hello, I am parent of {:?} (pid:{})", rc_wait, getpid());
        }
        Err(_) => {
            eprintln!("fork failed");
            exit(1);
        }
    }
}

fn sample_with_child_and_execvp() {
    println!("hello world (pid:{})", getpid());

    match unsafe { fork() } {
        Ok(ForkResult::Child) => {
            println!("hello, I am child (pid:{})", getpid());

            let prog = CString::new("wc").unwrap();
            let arg1 = CString::new("p3.c").unwrap();

            let args = &[prog.clone(), arg1.clone()];

            match execvp(&prog, args) {
                Ok(_) => {}
                Err(_) => {
                    eprintln!("execvp failed");
                    exit(1)
                }
            }

            // Esta linha nunca será executada se execvp funcionar
            println!("this shouldn't print out");
        }

        Ok(ForkResult::Parent { child }) => {
            let rc_wait = wait().expect("wait failed");
            println!("hello, I am parent of {:?} (pid:{})", rc_wait, getpid());
        }

        Err(_) => {
            eprintln!("fork failed");
            exit(1);
        }
    }
}

fn main() {
    sample_with_child_and_execvp();
}
