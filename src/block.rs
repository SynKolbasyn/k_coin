use std::cmp::Ordering::Equal;

use rsa::sha2::{Sha512_256, Digest};
use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};
use anyhow::Result;
use rand::random;


#[derive(Serialize, Deserialize, Debug)]
pub struct Block {
    prev_block: Option<Box<Block>>,

    from: String,
    to: String,
    amount: f64,
    miner: String,
    miner_amount: f64,

    #[serde(skip_serializing_if = "Option::is_none")]
    proof_of_work: Option<u128>,

    #[serde(skip_serializing_if = "Option::is_none")]
    created_time: Option<DateTime<Utc>>,
}


impl Block {
    pub fn new<T: ToString>(block: Option<Block>, from: T, to: T, amount: f64) -> Block {
        Block {
            prev_block: match block {
                Some(b) => Some(Box::from(b)),
                None => None,
            },
            from: from.to_string(),
            to: to.to_string(),
            amount,
            miner: String::new(),
            miner_amount: 10.0,
            proof_of_work: None,
            created_time: None,
        }
    }

    pub fn get_money_count(&self, user: String) -> f64 {
        let mut money: f64 = 0.0;

        if self.from.cmp(&user) == Equal {
            money -= self.amount;
        }

        if self.to.cmp(&user) == Equal {
            money += self.amount;
        }

        if self.miner.cmp(&user) == Equal {
            money += self.amount;
        }

        return match &self.prev_block {
            Some(block) => money + block.get_money_count(user),
            None => money,
        };
    }

    fn pow_to_string(proof_of_work: u128) -> String {
        format!("{:0>256}", format!("{:b}", proof_of_work))
    }

    fn check_proof_of_work(&self, proof_of_work: u128) -> Result<bool> {
        Ok(self.get_bin_hash(proof_of_work)?.starts_with("0000000000000000"))
    }

    pub fn check(&self) -> Result<bool> {
        if self.miner_amount != 10.0 {
            return Ok(false);
        }

        if self.amount < 0.0 {
            return Ok(false);
        }

        if self.get_money_count(self.from.clone()) < self.amount {
            return Ok(false);
        }

        let proof_of_work_validation: bool = match self.proof_of_work {
            Some(proof_of_work) => self.check_proof_of_work(proof_of_work)?,
            None => true,
        };

        if !proof_of_work_validation {
            return Ok(false);
        }

        return match &self.prev_block {
            Some(block) => block.check(),
            None => Ok(true),
        };
    }

    pub fn verify<T: ToString>(&mut self, miner: T) -> Result<()> {
        // TODO: Rewrite the selection to a graphics accelerator
        
        let mut result: u128 = random::<u128>();

        self.miner = miner.to_string();
    
        while !self.check_proof_of_work(result)? {
            result = random::<u128>();
        }
    
        self.proof_of_work = Some(result);
        self.created_time = Some(Utc::now());
    
        Ok(())
    }

    pub fn get_hex_hash(&self, proof_of_work: u128) -> Result<String> {
        let hash: Vec<u8> = Sha512_256::digest(serde_json::to_string::<Block>(self)? + &Self::pow_to_string(proof_of_work)).to_vec();
        Ok(base16ct::lower::encode_string(&hash))
    }
    
    pub fn get_bin_hash(&self, proof_of_work: u128) -> Result<String> {
        let mut result: String = String::new();
        let hash: Vec<u8> = Sha512_256::digest(serde_json::to_string::<Block>(self)? + &Self::pow_to_string(proof_of_work)).to_vec();
        
        for i in hash {
            result += &format!("{:0>8}", format!("{:b}", i));
        }
        
        Ok(result)
    }
}
