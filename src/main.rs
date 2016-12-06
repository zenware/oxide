#[macro_use] extern crate log;
extern crate daemonize;

use daemonize::{Daemonize};

fn main() {
    let daemonize = Daemonize::new()
        .pid_file("/tmp/oxide.pid")
        .chown_pid_file(true)
        .working_directory("/tmp")
        .user("nobody")
        .group("daemon")
        .umask(0o777)
        .privileged_action(|| "Executed before drop privileges"); // Setup a listening socket.

     match daemonize.start() {
         Ok(_) => info!("Success, daemonized"),
         Err(e) => error!("{}", e),
     }
 }
