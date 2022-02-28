/*
                                                                                                                                
                                                                                                                               
                                                                      
                                          #@@@@(                      
                                    *@@@@@@@@@@@@@@@@%                
                                 ,@@@@@@@@@@@@@@@@@@@@@@@             
                               (@@@@@@@@@@@@@@@@@@      @ &@@@@@@     
                             .@@@@@@@@@@@@@@@@@@          @@@@@@@.    
                   .((/.    @@@@@@@@@@@@@@@@@@@(           %@@@@      
               @@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@*                      
            #@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@&                      
           @@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@.                     
          @@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@&              
         @@@@@@@@@&@@@@,  .&@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@           
        #@@(           *@@@@@@/  .@@@@@@@@@@@@@@@@@@@@@@@@@@@         
        @@            @@@#  @@@@@@@@   @@@@@@@@@@@@@@@@@@@@@@@        
       *@/            @@@(  @@@@@@@@@@ @@@@@@@@@@@@@@@@@@@@@@@&       
    #@@@@@           @@@@@@@@@@@@@@@@& @@@@@@@@@    .@@@@@@@@@@       
   @@@@@@@@         @@@@@@@@@@@@@@@@@.#@@@@@@@@@        @@@@@@,       
    @@@@@@,           @@@@@@@@@@@@@@@ @@@@@@@@@.         @@@@@        
                       @@@@@@*@@@@@@ @@@@@@@@@.          *@@@         
                       %@@@@@@@@@@@ @@@@@@@@@            %@@          
                       &@@@@@@@@@ @@@@@@@@@@@@@&     ,@@, @           
                        &@@@@( %@@@@@@@@            @@@@@@            
                             @@@@@@@@                @@@@             
                            &@@@@@%                                   
                           &@@@@                                      
                          @@@&                                        
                        @@#                                           
                     .%                                                        
 */

// To conserve gas, efficient serialization is achieved through Borsh (http://borsh.io/)
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::LookupMap;
use near_sdk::{env, near_bindgen, setup_alloc, AccountId, Promise};

setup_alloc!();


#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct SnapCash {
    memo: LookupMap<String, Vec<String>>,
}

impl Default for SnapCash {
    fn default() -> Self {
        Self {
            memo: LookupMap::new(b"memo".to_vec()),
        }
    }
}

#[near_bindgen]
impl SnapCash {
    //change method adding info/changing on blockchain

    pub fn add_memo(&mut self, memo_text: String, price: String) {
        let account_id = env::signer_account_id();
        let contains_user = self.memo.contains_key(&account_id);

        if contains_user {
            let mut temporary_list = match self.memo.get(&account_id) {
                Some(y) => y, //y is the amount/vector of memos
                None => vec![],
            };

            temporary_list.push(memo_text + " || " + &price + "NEAR");

            self.memo.insert(&account_id, &temporary_list);
        } else {
            let new_vec = vec![memo_text + " || " + &price + "NEAR"];
            self.memo.insert(&account_id, &new_vec);
        }
    }
    //sending the near to the account
    pub fn transfer_near(&mut self, account_id: AccountId, amount: f64) {
        Promise::new(account_id).transfer(amount as u128);
    }

    //view methods

    pub fn get_memos(self, user: String) -> Vec<String> {
        match self.memo.get(&user) {
            Some(y) => y, //vector that contains all of the user memo example ['test_memo','test_memo2']
            None => vec![], //else this will return an empty vector
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use near_sdk::MockedBlockchain;
    use near_sdk::{testing_env, VMContext};

    // mock the context for testing, notice "signer_account_id" that was accessed above from env::
    fn get_context(input: Vec<u8>, is_view: bool) -> VMContext {
        VMContext {
            current_account_id: "alice_near".to_string(),
            signer_account_id: "bob_near".to_string(),
            signer_account_pk: vec![0, 1, 2],
            predecessor_account_id: "carol_near".to_string(),
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

    #[test]
    fn set_then_get_memo() {
        let context = get_context(vec![], false);
        testing_env!(context);
        let mut contract = SnapCash::default();
        contract.get_memos("Sup".to_string());
        assert_eq!(
            "howdy".to_string(),
            contract.add_memo("bob_near".to_string(), "123".to_string())
        );
    }

    #[test]
    fn get_default_memo() {
        let context = get_context(vec![], true);
        testing_env!(context);
        let contract = SnapCash::default();
        // this test did not call set_greeting so should return the default "Hello" greeting
        assert_eq!(
            "Hello".to_string(),
            contract.add_memo("francis.near".to_string(), "321".to_string())
        );
    }
}
