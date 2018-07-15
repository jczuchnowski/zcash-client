
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate serde_json;
extern crate chrono;
extern crate data_encoding;
extern crate base64;
extern crate futures;
extern crate hyper;

pub mod model;
pub mod client;
pub mod clientx;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
