use client::ZcashNodeClient;
use futures::{Future};
use model::{ZTransaction};
use hyper::{self};
use futures::future::join_all;

pub struct ZcashNodeClientx<'a> {
    client: &'a ZcashNodeClient<'a>,
}

impl<'a> ZcashNodeClientx<'a> {

    pub fn new(client: &'a ZcashNodeClient<'a>) -> ZcashNodeClientx<'a> {
        ZcashNodeClientx {
            client: client
        }
    }

    pub fn z_transactions(&self, z_addresses: &Vec<String>) -> Box<Future<Item = Vec<ZTransaction>, Error = hyper::Error> + 'a> {
        let tx_z_fut: Vec<Box<Future<Item = Vec<ZTransaction>, Error = hyper::Error>>> = 
            z_addresses.clone().iter().map(|zaddr| self.client.z_listreceivedbyaddress(zaddr)).collect();
        let joined_tx_z_fut = join_all(tx_z_fut);
        let result = joined_tx_z_fut.map(|v| v.iter().flat_map(|txs| txs).map(|v| v.clone()).collect());
        Box::new(result)
    }

    // pub fn z_address_with_amount(amount: f32) -> Box<Future<Item = String, Error = hyper::Error> + 'a> {
    //     let balances_fut = self.client.z_listaddresses().flat_map(|v| 
    //         v.iter().map(|a| self.client.z_getbalance(a).map(|b| (b a)))
    //         .filter(|&v| v.0 < amount)
    //     );
    // }

}
