extern crate iron;
extern crate router;
extern crate sudoku;
use sudoku::sudoku_action::*;

use iron::prelude::*;
use iron::{BeforeMiddleware, AfterMiddleware, typemap};
use router::Router;
use iron::status;
use iron::error::Error;
use iron::request::Body;
use std::io::Read;
use std::io::Write;
use iron::response::WriteBody;


fn main() {
    let mut router = Router::new();
    router.get("/",handler,"index");
    router.get("/:query",handler,"query");
    router.post("/sudoku",sudoku_handler,"sudoku");
    Iron::new(router).http("192.168.1.5:9000").unwrap();

    fn handler(req: &mut Request) -> IronResult<Response> {
        let ref query = req.extensions.get::<Router>().unwrap().find("query").unwrap_or("/");
        Ok(Response::with((status::Ok,*query)))
    }

    fn sudoku_handler(req: &mut Request) -> IronResult<Response> {
        let mut data_buf : Vec<u8> = Vec::new();
        if let Ok(size) = req.body.read(data_buf.as_mut()) {
            println!("bodysize : {}", size);
        }
        let mut data_str = String::new();
        data_str.write_body(&mut data_buf);
        let mut sdk = Sudoku::from_json_new(&data_str);
        let mut rel_vec = Vec::new();
        sdk.generate_sudoku(&mut rel_vec);

        let ret = format!("{}",rel_vec[0]);

        Ok(Response::with((status::Ok,ret)))
    }
}