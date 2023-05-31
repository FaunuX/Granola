use pyo3::prelude::*; 

use std::convert::Infallible;
use std::net::SocketAddr;

use http_body_util::Full;
use hyper::body::Bytes;
use hyper::server::conn::http1;
use hyper::service::service_fn;
use hyper::{Request, Response};
use tokio::net::TcpListener;

async fn hello(_: Request<hyper::body::Incoming>) -> Result<Response<Full<Bytes>>, Infallible> {
    Ok(Response::new(Full::new(Bytes::from("Hello, World!"))))
}

async fn run() -> PyResult<()> {
    println!("OOK ACK");
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    // We create a TcpListener and bind it to 127.0.0.1:3000
    let listener = TcpListener::bind(addr).await?;

    // We start a loop to continuously accept incoming connections
    loop {
        let (stream, _) = listener.accept().await?;

        // Spawn a tokio task to serve multiple connections concurrently
        tokio::task::spawn(async move {
            // Finally, we bind the incoming connection to our `hello` service
            if let Err(err) = http1::Builder::new()
                // `service_fn` converts our function in a `Service`
                .serve_connection(stream, service_fn(hello))
                    .await
                    {
                        println!("Error serving connection: {:?}", err);
                    }
        });
    }
}

#[pyfunction]
fn serve<'a>(py: Python<'a>, port: u16, app: &'a PyAny) -> PyResult<&'a PyAny> {
    pyo3_asyncio::tokio::future_into_py(py, async {
        run().await;
        Ok(Python::with_gil(|py| py.None()))
    })
        // ---------------------------------------------------------------- |
        // ---------------------------------------------------------------- |
        // let running = Arc::new(AtomicBool::new(true));
        // let r = running.clone();
        // let listener = Listener::connect(format!("127.0.0.1:{}", port));
        // ctrlc::set_handler(move || {
        //     r.store(false, Ordering::SeqCst);
        // }).expect("Error setting Ctrl-C handler");
        // println!("TCP server started on port {}, Ctrl+C to quit", port);
        // loop {
        //     match run_server(&listener, &running) {
        //         ListenerResult::RequestFound(stream) => handle_connection(stream, app),
        //         ListenerResult::RequestNotFound => continue,
        //         ListenerResult::KeyboardInterrupt => break
        //     };
}

#[pymodule]
#[pyo3(name="granola")]
fn granola(py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(serve, m)?)?;
    Ok(())
}
