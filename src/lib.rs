use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Request, Response, Server};
use phper::{functions::Argument, modules::Module, php_get_module, values::ZVal};
use std::convert::Infallible;

static mut CLOSURE: Option<ZVal> = None;

#[tokio::main]
async fn start_server(arguments: &mut [ZVal]) -> phper::Result<()> {
    pretty_env_logger::init();

    let php_closure = arguments[0].clone();
    unsafe {
        CLOSURE = Some(php_closure);
    }

    let make_svc =
        make_service_fn(|_conn| async { Ok::<_, Infallible>(service_fn(create_closure_service)) });

    let addr = ([127, 0, 0, 1], 3000).into();

    let server = Server::bind(&addr).serve(make_svc);

    println!("Listening on http://{}", addr);

    server.await.unwrap();

    Ok(())
}

async fn create_closure_service(request: Request<Body>) -> Result<Response<Body>, Infallible> {
    let query_string = String::from(request.uri().query().unwrap());
    let mut php_closure;
    unsafe {
        php_closure = CLOSURE.clone().expect("Missed closure");
    }

    let response = php_closure
        .call(&mut [ZVal::from(query_string)])
        .unwrap()
        .expect_z_str()
        .unwrap()
        .to_str()
        .unwrap()
        .to_string();

    Ok(Response::new(Body::from(response)))
}

#[php_get_module]
pub fn get_module() -> Module {
    let mut module = Module::new(
        env!("CARGO_CRATE_NAME"),
        env!("CARGO_PKG_VERSION"),
        env!("CARGO_PKG_AUTHORS"),
    );

    module
        .add_function("start_server", start_server)
        .argument(Argument::by_val("closure"));

    module
}
