mod errors;
use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Request, Response, Server};
use phper::{Error, functions::Argument, modules::Module, php_get_module, values::ZVal};
use crate::errors::PhpRustServerError;

static mut CLOSURE: Option<ZVal> = None;

#[tokio::main]
async fn start_server(arguments: &mut [ZVal]) -> phper::Result<()> {
    pretty_env_logger::init();

    let php_closure = arguments[0].clone();
    unsafe {
        CLOSURE = Some(php_closure);
    }

    let make_svc = make_service_fn(|_conn| async {
        Ok::<_, PhpRustServerError>(service_fn(create_closure_service))
    });

    let addr = ([127, 0, 0, 1], 3000).into();

    let server = Server::bind(&addr).serve(make_svc);

    println!("Listening on http://{}", addr);


    server.await.unwrap_or_else(|err| {
        let error_message = err.to_string();
        return Err(Error::from(error_message);
    });

    Ok(())
}

async fn create_closure_service(
    request: Request<Body>,
) -> Result<Response<Body>, PhpRustServerError> {
    let query_string = String::from(request.uri().query().unwrap_or(""));
    let mut php_closure;
    unsafe {
        php_closure = CLOSURE.clone().unwrap();
    }

    let response = match php_closure.call(&mut [ZVal::from(query_string)]) {
        Ok(response) => match response.expect_z_str() {
            Ok(z_str) => match z_str.to_str() {
                Ok(str_value) => str_value.to_string(),
                Err(_) => return Err(PhpRustServerError::ConvertingToStringSliceError()),
            },
            Err(_) => {
                return Err(PhpRustServerError::InvalidZTypeError{ expected: "ZStr".to_string() });
            }
        },
        Err(_) => return Err(PhpRustServerError::InvalidClosureError()),
    };

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
