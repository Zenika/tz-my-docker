use nix::sched::{unshare, CloneFlags};
use nix::unistd::{fork, ForkResult};
use std::process::Command;

fn main() {
    println!("==> Unshare namespaces (UTS + PID)");

    unshare(CloneFlags::CLONE_NEWUTS | CloneFlags::CLONE_NEWPID)
        .expect("Failed to unshare namespaces");

    match unsafe { fork() } {
        Ok(ForkResult::Child) => {
            println!("==> In child process. Launching bash...");
            Command::new("/bin/bash")
                .status()
                .expect("Failed to exec bash");
        }
        Ok(ForkResult::Parent { child }) => {
            println!("==> Parent process, child pid: {}", child);
        }
        Err(e) => {
            eprintln!("Fork failed: {}", e);
        }
    }
}

