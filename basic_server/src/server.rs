//modules are private by default
//crate = root
use crate::http::{Request, Response, StatusCode, ParseError};
use::std::net::TcpListener;
use::std::convert::TryFrom;
use::std::io::{Read, Write};


pub trait Handler {
    fn handle_request(&mut self, request: &Request) -> Response;
    fn handle_bad_request(&mut self, e: &ParseError) -> Response {
        println!("Failed to parse request {}", e);
        Response::new(StatusCode::BadRequest, None)
    }

}
pub struct Server {
    address: String,
}

impl Server {

    pub fn new(address: String) -> Self {

        Self {
            address
        }
    }

    //this func takes ownership beacuse we don't pass a reference
    pub fn run(self, mut handler: impl Handler) {
        println!("Listening on {}", self.address);

        //unwrap will terminate the program in case of error
        let listener = TcpListener::bind(&self.address).unwrap();

        /*
        //loop labeling
        'outer: loop {
            loop {
                //breaks outer loop
                break 'outer;
            }
        }
        */

        loop {
            match listener.accept() {
                Ok((mut stream, sock_addr)) => {
                    //do something
                    println!("Receiving stream of bytes on socket: {}", sock_addr);
                    //buffer for storing incoming bytes
                    let mut buffer = [0; 1024];
                    match stream.read(&mut buffer){
                        Ok(_) => {

                            println!("Received a request: {}", String::from_utf8_lossy(&mut buffer));

                            //Request::try_from(&buffer as &[u8]);
                           let response =  match Request::try_from(&buffer[..]) {  //same thing as above

                                //let res: &Result<Request, _> = &buffer[::].try_into();
                                Ok(request) => {
                                    dbg!(request);
                                    Response::new(
                                        StatusCode::Ok, 
                                        Some("<h1> IT WORKS!</h1>".to_string()))

                                },
                                Err(e) => {
                                    println!("Failed to parse  a request: {}", e);
                                    Response::new(StatusCode::BadRequest,None)
                                }

                            };

                            if let Err(e) = response.send(&mut stream) {
                                println!("Failed to send response: {}", e);
                            }
                        

                        }
                        Err(e) => println!("Failed to read from connection: {}", e),
                    }

                },

                Err(e) => println!("Failed to establish connection: {}", e),

                //default case
                //_ => {handle default case}
            }

            
        }
    }
}
