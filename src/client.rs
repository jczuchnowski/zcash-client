extern crate serde_json;

use base64::encode;
use futures::{Future, Stream};
use hyper::{self, Body, Client, Method, Request, Uri};
use hyper::client::HttpConnector;
use hyper::header::{Authorization, ContentLength, ContentType};
use serde_json::Value;
use std::io;

use model::{Args, Balance, BlockchainInfo, Transaction, RawZTransaction, ZTransaction};

pub struct ZcashNodeClient<'a> {
    uri: Uri,
    auth: String,
    client: &'a Client<HttpConnector, Body>,
}

impl<'a> ZcashNodeClient<'a> {

    pub fn new(uri: String, username: String, password: String, client: &Client<HttpConnector, Body>) -> ZcashNodeClient {
        let uri = uri.parse().expect("Error when parsing the URI");
        let creds = encode(&format!("{}:{}", username, password));
        let auth = format!("{}{}", "Basic ", creds);

        ZcashNodeClient {
            uri: uri,
            auth: auth,
            client: client
        }
    }

    pub fn from_args(args: &Args, client: &'a Client<HttpConnector, Body>) -> ZcashNodeClient<'a> {
        let node_url = if args.network == "testnet" {
            "http://127.0.0.1:18232/"
        } else if args.network == "mainnet" {
            "http://127.0.0.1:8232/"
        } else {
            panic!("Unknown network parameter. Allowed values are: 'testnet' or 'mainnet'.");
        };

        if args.verbose {
            println!("connecting to {} at {}", args.network, node_url);
            println!("username {}", args.username);
            println!("password {}", args.password);
        }

        let zcash_client = ZcashNodeClient::new(
            node_url.parse().unwrap(), 
            String::from(args.username.clone()), 
            String::from(args.password.clone()), 
            &client);

        zcash_client
    }

    ///////////////////////////////////
    //                               //
    // Bitcoin inherited API         //
    //                               //
    ///////////////////////////////////


    pub fn getblockchaininfo(&self) -> Box<Future<Item = BlockchainInfo, Error = hyper::Error> + 'a> {

        let json = r#"{"jsonrpc": "1.0", "id":"curltest", "method": "getblockchaininfo" }"#;

        let mut req = Request::new(Method::Post, self.uri.clone());
        req.headers_mut().set(Authorization(self.auth.to_string()));
        req.headers_mut().set(ContentType::json());
        req.headers_mut().set(ContentLength(json.len() as u64));
        req.set_body(json);

        let post = self.client.request(req).map(|res| {
        
            res.body().concat2().map(|body| {
                let v: Value = serde_json::from_slice(&body).map_err(|e| {
                    io::Error::new(
                        io::ErrorKind::Other,
                        e
                    )
                }).unwrap();

                let raw_val = v["result"].clone();

                let b: BlockchainInfo = serde_json::from_value(raw_val).unwrap();
                b
            })

        }).flatten();

        Box::new(post)
    }

    pub fn listtransactions(&self) -> Box<Future<Item = Vec<Transaction>, Error = hyper::Error> + 'a> {

        let json = json!({
            "jsonrpc": "1.0",
            "id": "test",
            "method": "listtransactions",
            "params": []
        }).to_string();

        let mut req = Request::new(Method::Post, self.uri.clone());
        req.headers_mut().set(Authorization(self.auth.to_string()));
        req.headers_mut().set(ContentType::json());
        req.headers_mut().set(ContentLength(json.len() as u64));
        req.set_body(json);

        let post = self.client.request(req).map(|res| {
        
            res.body().concat2().map(|body| {
                let v: Value = serde_json::from_slice(&body).map_err(|e| {
                    io::Error::new(
                        io::ErrorKind::Other,
                        e
                    )
                }).unwrap();

                let raw_val = v["result"].clone();

                let b: Vec<Transaction> = serde_json::from_value(raw_val).unwrap();
                b
            })

        }).flatten();

        Box::new(post)
    }

    ///////////////////////////////////
    //                               //
    // Zcash API                     //
    //                               //
    ///////////////////////////////////

    pub fn z_getbalance(&self, addr: &String) -> Box<Future<Item = String, Error = hyper::Error> + 'a> {

        let json = json!({
            "jsonrpc": "1.0",
            "id": "test",
            "method": "z_getbalance",
            "params": [addr]
        }).to_string();

        let mut req = Request::new(Method::Post, self.uri.clone());
        req.headers_mut().set(Authorization(self.auth.to_string()));
        req.headers_mut().set(ContentType::json());
        req.headers_mut().set(ContentLength(json.len() as u64));
        req.set_body(json);

        let post = self.client.request(req).map(|res| {

            res.body().concat2().map(|body| {
                let v: Value = serde_json::from_slice(&body).map_err(|e| {
                    io::Error::new(
                        io::ErrorKind::Other,
                        e
                    )
                }).unwrap();

                let raw_val = v["result"].clone();

                let b: String = serde_json::from_value(raw_val).unwrap();
                b
            })

        }).flatten();

        Box::new(post)
    }

    pub fn z_gettotalbalance(&self) -> Box<Future<Item = Balance, Error = hyper::Error> + 'a> {

        let json = r#"{"jsonrpc": "1.0", "id":"curltest", "method": "z_gettotalbalance" }"#;

        let mut req = Request::new(Method::Post, self.uri.clone());
        req.headers_mut().set(Authorization(self.auth.to_string()));
        req.headers_mut().set(ContentType::json());
        req.headers_mut().set(ContentLength(json.len() as u64));
        req.set_body(json);

        let post = self.client.request(req).map(|res| {

            res.body().concat2().map(|body| {
                let v: Value = serde_json::from_slice(&body).map_err(|e| {
                    io::Error::new(
                        io::ErrorKind::Other,
                        e
                    )
                }).unwrap();

                let raw_val = v["result"].clone();

                let b: Balance = serde_json::from_value(raw_val).unwrap();
                b
            })

        }).flatten();

        Box::new(post)
    }

    pub fn z_listaddresses(&self) -> Box<Future<Item = Vec<String>, Error = hyper::Error> + 'a> {

        let json = r#"{"jsonrpc": "1.0", "id":"curltest", "method": "z_listaddresses" }"#;

        let mut req = Request::new(Method::Post, self.uri.clone());
        req.headers_mut().set(Authorization(self.auth.to_string()));
        req.headers_mut().set(ContentType::json());
        req.headers_mut().set(ContentLength(json.len() as u64));
        req.set_body(json);

        let post = self.client.request(req).map(|res| {
        
            res.body().concat2().map(|body| {
                let v: Value = serde_json::from_slice(&body).map_err(|e| {
                    io::Error::new(
                        io::ErrorKind::Other,
                        e
                    )
                }).unwrap();

                let raw_val = v["result"].clone();

                let b: Vec<String> = serde_json::from_value(raw_val).unwrap();
                b
            })

        }).flatten();

        Box::new(post)
    }

    pub fn z_listreceivedbyaddress(&self, zaddr: &String) -> Box<Future<Item = Vec<ZTransaction>, Error = hyper::Error> + 'a> {

        let json = json!({
            "jsonrpc": "1.0",
            "id": "test",
            "method": "z_listreceivedbyaddress",
            "params": [zaddr]
        }).to_string();

        let mut req = Request::new(Method::Post, self.uri.clone());
        req.headers_mut().set(Authorization(self.auth.to_string()));
        req.headers_mut().set(ContentType::json());
        req.headers_mut().set(ContentLength(json.len() as u64));
        req.set_body(json);

        let post = self.client.request(req).map(|res| {
        
            res.body().concat2().map(|body| {
                let v: Value = serde_json::from_slice(&body).map_err(|e| {
                    io::Error::new(
                        io::ErrorKind::Other,
                        e
                    )
                }).unwrap();

                let raw_val = v["result"].clone();

                let b: Vec<RawZTransaction> = serde_json::from_value(raw_val).unwrap();
                let c: Vec<ZTransaction> = b.iter().map(|x| ZTransaction::new(&x.txid, x.amount, &x.memo)).collect();
                c
            })

        }).flatten();

        Box::new(post)
    }

    pub fn z_sendmany(&self, from: &str, to: &str, amount: f32) -> Box<Future<Item = Option<String>, Error = hyper::Error> + 'a> {

        let json = json!({
            "jsonrpc": "1.0",
            "id": "test",
            "method": "z_sendmany",
            "params": [from, [{"address": to, "amount": 0.01}]]
        }).to_string();

        println!("request: {}", json.clone());

        let mut req = Request::new(Method::Post, self.uri.clone());
        req.headers_mut().set(Authorization(self.auth.to_string()));
        req.headers_mut().set(ContentType::json());
        req.headers_mut().set(ContentLength(json.len() as u64));
        req.set_body(json);

        let post = self.client.request(req).map(|res| {
        
            res.body().concat2().map(|body| {
                let v: Value = serde_json::from_slice(&body).map_err(|e| {
                    io::Error::new(
                        io::ErrorKind::Other,
                        e
                    )
                }).unwrap();

                let raw_val = v["result"].clone();
                let b: Option<String> = serde_json::from_value(raw_val).unwrap();
                let b2 = b.clone().unwrap_or(String::from("None"));
                println!("response: {}", b2);
                b
            })

        }).flatten();

        Box::new(post)
    }

}
