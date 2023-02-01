use std::error::Error;
use std::net::{TcpListener, TcpStream};
use std::thread;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::Duration;

let running = Arc::new(AtomicBool::new(true));

// Function to handle incoming connections
fn handle_connection(stream: TcpStream) {
    // Your code to handle the incoming stream
}

// Function to listen for `Ctrl + C`
fn listen_for_ctrl_c(running: Arc<AtomicBool>) {
    ctrlc::set_handler(move || {
        println!("Received SIGINT, shutting down the server");
        running.store(false, Ordering::SeqCst);
    }).expect("Error setting Ctrl-C handler");
}

fn main() -> Result<(), Box<dyn Error>> {
    // Listen for `Ctrl + C`
    let r = running.clone();
    listen_for_ctrl_c(r);

    // Start the server
    let listener =
        TcpListener::bind("0.0.0.0:8080")?;
    println!("Server running on 0.0.0.0:8080");

    while running.load(Ordering::SeqCst) {
        match listener.accept() {
            Ok((stream, _)) => {
                let
                    r =
                    running.clone();
                thread::spawn(move
                              ||
                              {
                                  handle_connection(stream);
                              });
            }
            Err(e)
                =>
                {
                    if
                        e.kind()
                            ==
                            std::io::ErrorKind::WouldBlock
                            {
                                std::thread::sleep(Duration::from_millis(100));
                            }
                    else
                    {
                        return
                            Err(Box::new(e));
                    }
                }
        }
    }

    Ok(())
}

