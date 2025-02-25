

#![cfg_attr(not(any(test, feature = "export-abi")), no_main)]
extern crate alloc;

use std::result::Result::Ok;
//use  ethers::{etherscan::account::{self, AccountBalance}, types::spoof::balance};
use stylus_sdk::{
    alloy_primitives::{Address, U256}, call::transfer_eth, contract::{self, balance}, evm, msg, prelude::*, storage::{StorageAddress, StorageBool, StorageMap, StorageString, StorageU256}
};
use alloy_sol_types::{sol, SolEventInterface};
use stylus_sdk::alloy_sol_types::{SolError, SolType,Panic,Revert};

pub trait ERC721Params{

    const NAME : &'static str;
    const SYMBOL: &'static str;

    /// The NFT's Uniform Resource Identifier.
    fn token_uri(token_id: U256) -> String;
}

sol_storage! {
    #[entrypoint]
    pub struct Event {
        StorageU256 next_event_id;
        StorageMap<U256, StorageAddress> event_organizers;
        StorageString event_names;
        StorageMap<U256, StorageU256> event_dates;
        StorageString event_locations;
        StorageMap<U256, StorageU256> event_prices;
        StorageMap<U256, StorageU256> event_total_tickets;
        StorageMap<U256,   StorageU256> event_tickets_sold;
        StorageMap<Address, StorageBool> event_registrations;


        // this implementation is for the NFTS
        mapping(uint256 => address) owners;
    
        mapping(address => uint256) balances;
        /// Token id to approved user map
        mapping(uint256 => address) token_approvals;
    
        mapping(address => mapping(address => bool)) operator_approvals;
        /// Total supply  of NFT 
        uint256 total_supply;

        
    }

    
}

sol!{

    event Transfer(address indexed from, address indexed to, uint256 indexed token_id);
    event Approval(address indexed owner, address indexed approved, uint256 indexed token_id);

 }


 sol! {
    error AlreadyRegistered();
    error ZeroAddressNotAllowed();
    error NotEnoughAmount();
    error NotOrganizer();
    error NotRegistered();

 }

 sol_interface! {
    /// Allows calls to the `onERC721Received` method of other contracts implementing `IERC721TokenReceiver`.
    interface IERC721TokenReceiver {
        function onERC721Received(address operator, address from, uint256 token_id, bytes data) external returns(bytes4);
    }
}

sol! {
    error InvalidTokenId(uint256 token_id);
    // The specified address is not the owner of the specified token id
    error NotOwner(address from, uint256 token_id, address real_owner);
    // The specified address does not have allowance to spend the specified token id
    error NotApproved(address owner, address spender, uint256 token_id);
    // Attempt to transfer token id to the Zero address
    error TransferToZero(uint256 token_id);
    // The receiver address refused to receive the specified token id
    error ReceiverRefused(address receiver, uint256 token_id, bytes4 returned);
}



#[derive(SolidityError)]
pub enum Erc721Error {
    InvalidTokenId(InvalidTokenId),
    NotOwner(NotOwner),
    NotApproved(NotApproved),
    TransferToZero(TransferToZero),
    ReceiverRefused(ReceiverRefused),
}


 #[derive(SolidityError)]

 pub enum EventError{
    AlreadyRegistered(AlreadyRegistered),
    ZeroAddressNotAllowed(ZeroAddressNotAllowed),
    NotEnoughAmount(NotEnoughAmount),
    NotOrganizer(NotOrganizer),
    NotRegistered(NotRegistered)
 }

#[public]
impl Event{
    pub fn create_event(
        &mut self,
        name: String,
        date: U256,
        location: String,
        price: U256,
        total_tickets: U256,
    ) -> U256 {
        let event_id = self.next_event_id.get();
        self.next_event_id.set(event_id + U256::from(1));

        let organizer = self.vm().msg_sender();
        
        self.event_organizers.insert(event_id, organizer);
        self.event_names.set_str(name);
        self.event_dates.insert(event_id, date);
        self.event_locations.set_str(location);
        self.event_prices.insert(event_id, price);
        self.event_total_tickets.insert(event_id, total_tickets);
        self.event_tickets_sold.insert(event_id, U256::ZERO);

        event_id
    }

    #[payable]
    pub fn register(&mut self, event_id: U256)->Result<(), EventError> {
        let attendee = self.vm().msg_sender();
        let price = self.event_prices.get(event_id);

            // address zero check
            if attendee.is_zero(){
                return Err(EventError::NotEnoughAmount(NotEnoughAmount {  }))
            }
        
// check if the person has registered before 
        if self.event_registrations
        .get(attendee)==true
    {
        // 
       return Err(EventError::AlreadyRegistered(AlreadyRegistered{}))
    }


        // Validate payment
        let sent = self.vm().msg_value();
        if sent < price {
            
            return Err(EventError::NotEnoughAmount(NotEnoughAmount{}));
        }

       
        // Check ticket availability
        let tickets_sold = self.event_tickets_sold.get(event_id);
        let total_tickets = self.event_total_tickets.get(event_id);
        
        if tickets_sold >= total_tickets {
            // return Err(EventError::TicketLimitExceeded(TicketLimitExceeded {}));
            panic!("Ticket_limit Exceeded");
        }

        // Update registration
        self.event_tickets_sold.insert(event_id, tickets_sold + U256::from(1));
        self.event_registrations
            .insert(attendee, true);

            Ok(())
        
    }

    pub fn refund(&mut self, event_id: U256, attendee: Address)->Result<(), EventError> {
        let registered = self.event_registrations.get(attendee);

        let organizer = self.event_organizers.get(event_id);

        if !registered{
            return Err( EventError::NotOrganizer(NotOrganizer {}))
        }
          

        // Only organizer can issue refunds
        if self.vm().msg_sender() != organizer {
             return Err(EventError::NotOrganizer(NotOrganizer {}));
               
        }

        // Check if registered
        if !self.event_registrations
            .get(attendee)==false
        {
             return Err(EventError::NotRegistered(NotRegistered {}));
           
        }

        let price = self.event_prices.get(event_id);
        
        // Send ETH back
       let _= self.vm().transfer_eth(attendee, price);
       

        // Update state
        self.event_tickets_sold.insert(event_id, self.event_tickets_sold.get(event_id) - U256::from(1));
        self.event_registrations
            
            .insert(attendee, false);

            // emit a transfer event
        
    
        // vm().log({Transfer(attendee, event_id, price)});
        

        Ok(())
        
    
    }

    pub fn withdraw_funds(&mut self, event_id: U256) ->Result<(), EventError>  {
        let organizer = self.event_organizers.get(event_id);

        // if !organizer{
        //     return Err((EventError::NotOrganizer(NotOrganizer {})));
        // }
           

        if self.vm().msg_sender() != organizer {
             return Err(EventError::NotOrganizer(NotOrganizer {}));
            
        }

       // let balance = balance(contract::address());
        
        let contract_address= self.vm().contract_address();
        let balance= self.vm().balance(contract_address);
      let _ =  self.vm().transfer_eth(organizer, balance);

        // to log event 
        // self.vm(), Loog{Transfer(organizer, event_id, balance)};

        Ok(())
          
        
    }

    the function to transfer nft 
    the token_id is for 
    pub fn transfer(&mut self, token_id: U256, from: Address, to: Address) -> Result<(), Erc721Error> {
        let mut owner = self.owners.setter(token_id);
        let previous_owner = owner.get();
        if previous_owner != from {
            return Err(Erc721Error::NotOwner(NotOwner {
                from,
                token_id,
                real_owner: previous_owner,
            }));
        }
        owner.set(to);

        // right now working with storage can be verbose, but this will change upcoming version of the Stylus SDK
        let mut from_balance = self.balances.setter(from);
        let balance = from_balance.get() - U256::from(1);
        from_balance.set(balance);

        let mut to_balance = self.balances.setter(to);
        let balance = to_balance.get() + U256::from(1);
        to_balance.set(balance);

        // cleaning app the approved mapping for this token
        self.token_approvals.delete(token_id);
        
       
       // i need to log this event 
       // self.vm().log(Transfer { from, to, token_id });

    //    log(Transfer{from,to,token_id});
        Ok(())
    }
    // function to mint nft after the even t
    pub fn mint(&mut self, to: Address) -> Result<(), Erc721Error> {
        let new_token_id = self.total_supply.get();
        self.total_supply.set(new_token_id + U256::from(1u8));
        self.transfer(new_token_id, Address::default(), to)?;
        Ok(())
    }
}