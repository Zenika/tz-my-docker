use std::env;
use std::process::Command;
use nix::unistd::{chroot, fork, ForkResult};
use nix::sys::wait::waitpid;


fn main() {
    match unsafe { fork() } {
        Ok(ForkResult::Child) => {
            println!("==> [Child] Switching root...");
            chroot("/tmp/rootfs").expect("chroot failed");
            env::set_current_dir("/").expect("chdir failed");

            println!("==> [Child] Launching /bin/sh in chroot");
            Command::new("/bin/sh")
                .status()
                .expect("failed to exec /bin/sh");
        }
        Ok(ForkResult::Parent { child }) => {
            println!("==> [Parent] Container launched with PID {}", child);
            waitpid(child, None).expect("waitpid failed");

        }
        Err(e) => {
            eprintln!("Fork failed: {}", e);
        }
    }
}
