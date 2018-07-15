extern crate chrono;

use chrono::naive::NaiveDateTime;
use data_encoding::HEXUPPER_PERMISSIVE;

pub struct Args {
    pub verbose: bool,
    pub network: String,
    pub username: String,
    pub password: String,    
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Balance {
    pub transparent: String,
    pub private: String,
    pub total: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct BlockchainInfo {
    pub chain: String,
    pub blocks: u32,
    pub difficulty: f64,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct RawZTransaction {
    pub txid: String,
    pub amount: f32,
    pub memo: String,
}

#[derive(Debug)]
pub struct ZTransaction {
    pub txid: String,
    pub amount: f32,
    pub memo: String,
}

impl Clone for ZTransaction {
    fn clone(&self) -> ZTransaction { 
        ZTransaction {
            txid: self.txid.clone(), 
            amount: self.amount, 
            memo: self.memo.clone(),
        }
    }
}

impl ZTransaction {

    pub fn new(txid: &str, amount: f32, memo: &str) -> ZTransaction {
        let decoded: Vec<u8> = HEXUPPER_PERMISSIVE.decode(memo.as_bytes()).unwrap();
        let cleaned: Vec<u8> = ZTransaction::remove_trailing_zeroes(decoded);
        let a: u8 = 246;
        let s = if cleaned.len() == 1 && cleaned.get(0).unwrap() == &a {
            String::new()
        } else {
            String::from_utf8(cleaned).unwrap()
        };

        ZTransaction {
            txid: txid.to_owned(),
            amount: amount,
            memo: s
        }
    }

    fn remove_trailing_zeroes(mut s: Vec<u8>) -> Vec<u8> {
        let ex: u8 = 0;
        let index = s.iter().enumerate().find(|&r| r.1 == &ex).map(|x| x.0);
    
        if index.is_some() {
            s.truncate(index.unwrap())
        }

        s
    }

}

#[derive(Debug, Deserialize, Serialize)]
pub struct Transaction {
    pub address: String,
    pub category: String,
    pub amount: f32,
    pub txid: String,
    time: i64,
    pub timereceived: u32,
}

impl Transaction {

    pub fn get_date_time(&self) -> String {
        let dt = NaiveDateTime::from_timestamp(self.time, 0);
        dt.format("%Y-%m-%d %H:%M:%S").to_string()
    }

    pub fn get_date(&self) -> String {
        let dt = NaiveDateTime::from_timestamp(self.time, 0);
        dt.format("%b %d").to_string()
    }

}
