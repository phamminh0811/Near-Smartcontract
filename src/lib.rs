use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{env, near_bindgen,PanicOnDefault};
use near_sdk::json_types::U128;
use near_sdk::collections::{UnorderedMap};
use serde::{Deserialize, Serialize};


#[global_allocator]
static ALLOC: near_sdk::wee_alloc::WeeAlloc = near_sdk::wee_alloc::WeeAlloc::INIT;


#[derive(BorshDeserialize, BorshSerialize,Deserialize, Serialize,PartialEq,Debug)]
pub struct Student{ 
    name: String,
    age: u8,
    math_point:f64,
    physics_point:f64,
    chemistry_point:f64,
}

#[derive(BorshDeserialize, BorshSerialize)]
pub enum Subject {
    Math,
    Physics,
    Chelmistry,
}

#[near_bindgen]
#[derive(PanicOnDefault, BorshDeserialize, BorshSerialize)]
pub struct Contract{
    pub student: UnorderedMap<u128, Student>
}
#[near_bindgen]
impl Contract{
    #[init] 
    pub fn new() -> Self{
        // Useful snippet to copy/paste, making sure state isn't already initialized
        assert!(env::state_read::<Self>().is_none(), "Already initialized");
        // Note this is an implicit "return" here
        Self {
            student: UnorderedMap::new(b"s".to_vec()),
        }
    }

    pub fn get_student_info(&self, id:U128) {
        let stored_student = self.get_student(id);
        let log_message = format!("{} is {} years old. He has {} point on math, {} on physics and {} on chemistry"
                                                , stored_student.name
                                                ,stored_student.age
                                                ,stored_student.math_point
                                                ,stored_student.physics_point
                                                ,stored_student.chemistry_point);   
        env::log(log_message.as_bytes());
    }

    pub fn add_student(&mut self,id: U128,
                        name:String,
                        age:u8,
                        math_point:f64,
                        physics_point:f64,
                        chemistry_point:f64,)
    {
        

        let existed_student: Option<Student> = self.student.get(&id.into());
        if existed_student.is_some() {
            env::panic(b"Sorry, already added this student.")
        }
        if math_point<0.0  || physics_point<0.0 || chemistry_point<0.0 {
            env::panic(b"Point must below 0")
        } else if math_point>10.0|| physics_point>10.0  || chemistry_point>10.0 {
            env::panic(b"Point must above 10")
        }
        let student = Student{name: name, age: age,math_point:math_point, physics_point: physics_point,chemistry_point:chemistry_point};
        self.student.insert(&id.into(),&student);
    }

    pub fn delete_student(&mut self, id: U128){
        let existed_student: Option<Student> = self.student.get(&id.into());
        if existed_student.is_none() {
            env::panic(b"Sorry, No student found")
        }

        self.student.remove(&id.into());
    }

    pub fn reset_all(&mut self){
        assert_eq!(env::current_account_id()
                    , env::predecessor_account_id()
                    , "To reset all the students, this method must be called by the contract owner.");
        self.student.clear();
        env::log(b"All the students have been deleted.");
    }

    pub fn get_avg_point(&self, subject: String) -> f64{
        let points = self.student.values();
        let mut s : f64 = 0.0;
        
        
        if subject== "math".to_string()  {
            for point in points {
                s += point.math_point;
            }        
        }else  if subject== "physics".to_string(){
            for point in points {
                s += point.physics_point;
            }
        } else if subject== "chemistry".to_string(){
            for point in points {
                s += point.chemistry_point;
            }
        } else {
            env::panic(b"Invalid input, only one of this three type: math,physics,chemistry")
        }
             
        
        s / (self.student.len() as f64)
    }
}


impl Contract{
    pub fn get_student(&self, id:U128) -> Student {
        match self.student.get(&id.into()){
            Some(stored_student) => {
                
                stored_student
            },
            None => env::panic(b"No student found"),
        }
    }
}



#[cfg(test)]
mod tests {
    use super::*;
    use near_sdk::MockedBlockchain;
    use near_sdk::{testing_env, VMContext, AccountId};
    // part of writing unit tests is setting up a mock context
    // in this example, this is only needed for env::log in the contract
    // this is also a useful list to peek at when wondering what's available in env::*
    fn get_context(input: Vec<u8>, is_view: bool, predecessor: AccountId) -> VMContext {
        VMContext {
            current_account_id: "alice.testnet".to_string(),
            signer_account_id: "robert.testnet".to_string(),
            signer_account_pk: vec![0, 1, 2],
            predecessor_account_id: predecessor,
            input,
            block_index: 0,
            block_timestamp: 0,
            account_balance: 0,
            account_locked_balance: 0,
            storage_usage: 0,
            attached_deposit: 0,
            prepaid_gas: 10u64.pow(18),
            random_seed: vec![0, 1, 2],
            is_view,
            output_data_receivers: vec![],
            epoch_height: 19,
        }
    }

    // mark individual unit tests with #[test] for them to be registered and fired
    #[test]
    fn add() {
        let context = get_context(vec![],false,"bob.testnet".to_string());
        testing_env!(context);
        let mut contract = Contract::new();
        let id_1 = U128(679508051007679500);
        let student_1 = Student{
                                            name:"A".to_string(),
                                            age: 20,
                                            math_point: 9.5, 
                                            physics_point: 8.5,
                                            chemistry_point: 5.0,
                                        };
        contract.add_student(id_1, "A".to_string(), 20, 9.5, 8.5, 5.0);
        assert_eq!(contract.get_student(id_1.into()),student_1);

        // let id_2 = U128(679508051007679500);
        // contract.add_student(id_2, "B".to_string(), 18, 7.5, 8.5, 7.0);

    }


    #[test]
    fn delete() {
        let context = get_context(vec![],false,"bob.testnet".to_string());
        testing_env!(context);
        let mut contract = Contract::new();
        let id = U128(679508051007679500);
        contract.add_student(id, "Minh".to_string(), 20, 9.5, 8.5, 5.0);
        contract.delete_student(id);
        contract.get_student(id);
    }

    #[test]
    fn get_avg_point(){
        let context = get_context(vec![],false,"bob.testnet".to_string());
        testing_env!(context);
        let mut contract = Contract::new();
        let id_1 = U128(679508051007679500);
        contract.add_student(id_1, "A".to_string(), 20, 9.5, 8.5, 5.0);

        let id_2 = U128(679508051007679600);
        contract.add_student(id_2, "B".to_string(), 18, 7.5, 8.5, 7.0);

        
        assert_eq!(contract.get_avg_point("math".to_string()),8.5);
    }
}
