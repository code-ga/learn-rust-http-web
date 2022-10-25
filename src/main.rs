use rust_http_web_lib::App::HttpHandler;
use rust_http_web_lib::Request::get_http_data::HeaderData;
use rust_http_web_lib::Response::ResponseTool;
use rust_http_web_lib::RouteManager::RouterManager;

use std::net::TcpListener;

fn main() {
    let mut handle = HttpHandler::new();
    handle.turn_threading();
    let port = match std::env::var("PORT") {
        Ok(e) => e.parse::<i32>().unwrap(),
        Err(_) => 8000,
    };

    handle.get("/".to_owned(), home_handler);
    handle.all(log_request);
    let mut new_route = HttpHandler::new();
    new_route.get("/".to_owned(), home_handler);

    handle.route("/sub".to_string(), &new_route);
    let listener = TcpListener::bind(format!("127.0.0.1:{port}"));
    match listener {
        Ok(listener) => {
            println!(
                "máy chủ bật rồi đó ở cổng {} link đi đến đó nè http://{}",
                port,
                listener.local_addr().unwrap()
            );
            handle.handle_http_request(listener);
        }
        Err(err) => {
            panic!("{}", err);
        }
    }
}
fn home_handler(_request: &mut HeaderData, response: &mut ResponseTool, _: &mut RouterManager) {
    response.send("<h1>hello world</h1>".to_string(), true);
}
fn log_request(request: &mut HeaderData, _response: &mut ResponseTool, _: &mut RouterManager) {
    println!("method: {}, path: {}", request.method, request.path);
}
