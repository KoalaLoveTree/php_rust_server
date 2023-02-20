mod errors;

use crate::errors::PhpRustServerError::InternalServerError;
use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Request, Response, Server};
use phper::errors::Throwable;
use phper::{functions::Argument, modules::Module, php_get_module, values::ZVal, Error};

static mut CLOSURE: Option<ZVal> = None;

#[tokio::main]
async fn start_server(arguments: &mut [ZVal]) -> phper::Result<()> {
    pretty_env_logger::init();

    let php_closure = arguments[0].clone();
    unsafe {
        CLOSURE = Some(php_closure);
    }

    let make_svc = make_service_fn(|_conn| async {
        Ok::<_, Box<dyn std::error::Error + Send + Sync>>(service_fn(create_closure_service))
    });

    let addr = ([127, 0, 0, 1], 3000).into();

    let server = Server::bind(&addr).serve(make_svc);

    println!("Listening on http://{}", addr);

    server.await.map_err(|err| Error::Boxed(Box::new(err)))?;

    Ok(())
}

async fn create_closure_service(
    request: Request<Body>,
) -> Result<Response<Body>, Box<dyn std::error::Error + Send + Sync>> {
    let query_string = String::from(request.uri().query().unwrap_or(""));
    let mut php_closure;
    unsafe {
        php_closure = CLOSURE.clone().unwrap();
    }

    let response = php_closure
        .call(&mut [ZVal::from(query_string)])
        .map_err(|err| {
            Box::new(InternalServerError {
                message: err
                    .get_message()
                    .unwrap_or("unidentified error".to_string())
                    .to_string(),
            })
        })?
        .expect_z_str()
        .map_err(|err| {
            Box::new(InternalServerError {
                message: err
                    .get_message()
                    .unwrap_or("unidentified error".to_string())
                    .to_string(),
            })
        })?
        .to_str()
        .map_err(|err| {
            Box::new(InternalServerError {
                message: err.to_string(),
            })
        })?
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
