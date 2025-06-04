use clap::Parser;
use std::env;
use std::process::Command;
use nix::unistd::{fork, ForkResult, chroot};
use nix::sched::{unshare, CloneFlags};
use nix::sys::wait::waitpid;

#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Args {
    #[arg()]
    command: String,

    #[arg(trailing_var_arg = true)]
    args: Vec<String>,
}

fn main() {
    let args = Args::parse();

    println!("==> [CLI] Launching {:?}", args);

    unshare(CloneFlags::CLONE_NEWUTS | CloneFlags::CLONE_NEWPID)
        .expect("Failed to unshare");

    match unsafe { fork() } {
        Ok(ForkResult::Child) => {
            println!("==> [Child] Switching root...");
            chroot("/tmp/rootfs").expect("chroot failed");
            env::set_current_dir("/").expect("chdir failed");

            println!("==> [Child] Executing command...");
            let err = Command::new(&args.command)
                .args(&args.args)
                .status()
                .expect("failed to exec");
            std::process::exit(err.code().unwrap_or(1));
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

