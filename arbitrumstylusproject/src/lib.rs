// // // //!
// // // //! Stylus Hello World
// // // //!
// // // //! The following contract implements the Counter example from Foundry.
// // // //!
// // // //! ```solidity
// // // //! contract Counter {
// // // //!     uint256 public number;
// // // //!     function setNumber(uint256 newNumber) public {
// // // //!         number = newNumber;
// // // //!     }
// // // //!     function increment() public {
// // // //!         number++;
// // // //!     }
// // // //! }
// // // //! ```
// // // //!
// // // //! The program is ABI-equivalent with Solidity, which means you can call it from both Solidity and Rust.
// // // //! To do this, run `cargo stylus export-abi`.
// // // //!
// // // //! Note: this code is a template-only and has not been audited.
// // // //!
// // // // Allow `cargo stylus export-abi` to generate a main function.
// // // #![cfg_attr(not(any(test, feature = "export-abi")), no_main)]
// // // extern crate alloc;

// // // /// Import items from the SDK. The prelude contains common traits and macros.
// // // use stylus_sdk::{alloy_primitives::U256, prelude::*};

// // // // Define some persistent storage using the Solidity ABI.
// // // // `Counter` will be the entrypoint.
// // // sol_storage! {
// // //     #[entrypoint]
// // //     pub struct Counter {
// // //         uint256 number;
        
// // //     }
// // // }

// // // /// Declare that `Counter` is a contract with the following external methods.
// // // #[public]
// // // impl Counter {
// // //     /// Gets the number from storage.
// // //     pub fn number(&self) -> U256 {
// // //         self.number.get()
// // //     }

// // //     /// Sets a number in storage to a user-specified value.
// // //     pub fn set_number(&mut self, new_number: U256) {
// // //         self.number.set(new_number);
// // //     }

// // //     /// Sets a number in storage to a user-specified value.
// // //     pub fn mul_number(&mut self, new_number: U256) {
// // //         self.number.set(new_number * self.number.get());
// // //     }

// // //     /// Sets a number in storage to a user-specified value.
// // //     pub fn add_number(&mut self, new_number: U256) {
// // //         self.number.set(new_number + self.number.get());
// // //     }

// // //     /// Increments `number` and updates its value in storage.
// // //     pub fn increment(&mut self) {
// // //         let number = self.number.get();
// // //         self.set_number(number + U256::from(1));
// // //     }

// // //     /// Adds the wei value from msg_value to the number in storage.
// // //     #[payable]
// // //     pub fn add_from_msg_value(&mut self) {
// // //         let number = self.number.get();
// // //         self.set_number(number + self.vm().msg_value());
// // //     }
// // // }

// // // #[cfg(test)]
// // // mod test {
// // //     use super::*;

// // //     #[test]
// // //     fn test_counter() {
// // //         use stylus_sdk::testing::*;
// // //         let vm = TestVM::default();
// // //         let mut contract = Counter::from(&vm);

// // //         assert_eq!(U256::ZERO, contract.number());

// // //         contract.increment();
// // //         assert_eq!(U256::from(1), contract.number());

// // //         contract.add_number(U256::from(3));
// // //         assert_eq!(U256::from(4), contract.number());

// // //         contract.mul_number(U256::from(2));
// // //         assert_eq!(U256::from(8), contract.number());

// // //         contract.set_number(U256::from(100));
// // //         assert_eq!(U256::from(100), contract.number());

// // //         // Override the msg value for future contract method invocations.
// // //         vm.set_value(U256::from(2));

// // //         contract.add_from_msg_value();
// // //         assert_eq!(U256::from(102), contract.number());
// // //     }
// // // }

// // #![cfg_attr(not(any(test, feature = "export-abi")), no_main)]
// // extern crate alloc;

// // use alloc::string::String;
// // //use openzeppelin_stylus_proc::interface_id;

// // use stylus_sdk::{
// //     abi::Bytes,
// //     call::{Call, MethodError},
// //     evm, function_selector, msg,
// //     prelude::*,
// //     storage::{StorageAddress, StorageBool, StorageMap, StorageU256},
// //     alloy_primitives::U256,
// // };
// // use alloy_sol_types::sol;

// // sol_storage! {
// //     #[entrypoint]
// //     pub struct Event {
// //         // Basic event data
// //         // attendee:
// //          address attendee;
// //          address Organizer;
// //         uint256 id;
// //         string evenntName;
// //         uint256 date;
// //         string location;
// //         uint256 price;
// //         uint256 tickets_sold;
// //         uint256 total_tickets;

// //         // Mappings to track organizers, attendees, and registration
// //          mapping(uint256 => address) eventOrganizers;
// //          mapping(uint256 => address) eventAttendees;
// //          mapping(address => bool) already_register;

// //         // NFT fields for minting NFT after the event
// //          uint256 nft_next_token;
// //         mapping(uint256 => address)  nft_owners;
// //     }

// //     // pub struct Organizer {
// //     //     walletAddress: address,
// //     //     id: uint256,
// //     //     name: string,
// //     //     isVerifed: bool,
// //     //     eventCreated: uint256[]
// //     // }

// //     // pub struct Attendee {
// //     //     walletAddress: address,
// //     //     id: uint256,
// //     //     name: string,
// //     //     isVerifed: bool
// //     // }
// // }

// // sol! {
// //     error Already_paid();
// //     error Address_zero_not_allowed();
// //     error Amount_must_be_greater_than_zero(uint256 balance, uint256 amount);
// //     error Did_not_register(address attendee);
// // }

// // #[derive(SolidityError)]
// // pub enum EventError {
// //     Already_paid(Already_paid),
// //     Address_zero_not_allowed(Address_zero_not_allowed),
// //     Amount_must_be_greater_than_zero(Amount_must_be_greater_than_zero),
// //     Did_not_register(Did_not_register)
// // }

// // sol_interface! {
// //     interface ERC20 {
// //         fn balanceOf(owner: address) external returns (uint256);
// //         fn transferFrom(owner: address, receiver: address, amount: uint256);
// //     }

// //     interface ERC721 {
// //         event Transfer(from: address, to: address, tokenId: uint256);
// //         event Approval(owner: address, approved: address, tokenId: uint256);
// //         event ApprovalForAll(owner: address, operator: address, approved: bool);

// //         fn balanceOf(owner: address) external view returns (uint256);
// //         fn ownerOf(tokenId: uint256) external view returns (address);
// //     }
// // }

// // #[public]
// // impl Event {
// //     /// Purchase a ticket for this event.
// //     fn purchase_ticket(&mut self, amount: U256) -> Result<(), EventError> {
// //         // Get the purchaser’s address.
// //         let purchaser_address = msg::sender();
        
// //         // Check the purchaser’s ERC20 balance.
// //         let account_balance: U256 = ERC20(purchaser_address).balanceOf();
// //         if account_balance < amount {
// //             return Err(EventError::Amount_must_be_greater_than_zero(
// //                 Amount_must_be_greater_than_zero { balance: account_balance, amount: amount }
// //             ));
// //         }
        
// //         // Transfer funds from the purchaser to this contract.
// //         ERC20::transferFrom(purchaser_address, msg::address(), amount);
        
// //         // Increment ticketSold and register the attendee.
// //         self.ticketSold = self.ticketSold + 1;
// //         self.eventAttendees.insert(self.ticketSold, purchaser_address);
// //         self.already_register.insert(purchaser_address, true);
        
// //         Ok(())
// //     }
    
// //     /// Create an event and assign the organizer.
// //     fn create_event(&mut self, organizer_address: address, name: string) -> Result<string, EventError> {
// //         if organizer_address == address(0) {
// //             return Err(EventError::Address_zero_not_allowed(Address_zero_not_allowed()));
// //         }
// //         self.organizer = organizer_address;
// //         self.eventName = name.clone();
// //         Ok(name)
// //     }
    

// //     fn get_event_details(&self) -> (address, address, uint256, string, uint256, string, uint256, uint256, uint256) {
// //         (
// //             self.attendee,
// //             self.organizer,
// //             self.id,
// //             self.eventName.clone(),
// //             self.date,
// //             self.location.clone(),
// //             self.price,
// //             self.ticketSold,
// //             self.totalTickets
// //         )
// //     }
    
// //     /// Ensure that only the organizer can perform certain actions.
// //     fn only_organizer(&self) {
// //         if msg::sender() != self.organizer {
// //             panic!("Only organizer allowed");
// //         }
// //     }
    
// //     /// Refund an amount to an attendee.
// //     fn refund_amount(&mut self, attendee: address, amount: uint256) -> Result<(), EventError> {
// //         self.only_organizer();
        
// //         if !self.already_register.get(attendee).unwrap_or(false) {
// //             return Err(EventError::Did_not_register(Did_not_register { attendee: attendee }));
// //         }
        
// //         // Transfer funds from this contract to the attendee.
// //         ERC20::transferFrom(msg::address(), attendee, amount);
// //         self.already_register.insert(attendee, false);
        
// //         Ok(())
// //     }
    
// //     /// Mint an NFT ticket for a registered attendee.
// //     fn mint_nft(&mut self, to: address) -> Result<(), EventError> {
// //         // Ensure the recipient is registered.
// //         if !self.already_register.get(to).unwrap_or(false) {
// //             return Err(EventError::Did_not_register(Did_not_register { attendee: to }));
// //         }
// //         let token_id = self.nft_next_token;
// //         self.nft_next_token = self.nft_next_token + 1;
// //         self.nft_owners.insert(token_id, to);
        

// //         emit!(ERC721::Transfer(address(0), to, token_id));
// //         Ok(())
// //     }
    
// //     /// Withdraw funds from the contract to a designated address.
// //     fn withdraw(&mut self, to: address, amount: uint256) -> Result<(), EventError> {
// //         self.only_organizer();

// //         // Transfer funds from this contract to the designated address.
// //         // (Assuming evm::transfer returns a Result<(), MethodError>)
// //         evm::transfer(to, amount).map_err(|_| EventError::Already_paid(Already_paid()))?;
// //         Ok(())
// //     }
    
// //     /// Initialize event data.
// //     fn initialize(&mut self, id: uint256, eventName: string, date: uint256, location: string, price: uint256, totalTickets: uint256) {
// //         self.id = id;
// //         self.eventName = eventName;
// //         self.date = date;
// //         self.location = location;
// //         self.price = price;
// //         self.totalTickets = totalTickets;
// //         self.ticketSold = 0;
// //         self.nft_next_token = 1;
// //     }
// // }


// #![cfg_attr(not(any(test, feature = "export-abi")), no_main)]
// extern crate alloc;

// //use openzeppelin_stylus_proc::interface_id;

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
        /// User to operator map (the operator can manage all NFTs of the owner)
        mapping(address => mapping(address => bool)) operator_approvals;
        /// Total supply
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
        // self.vm().Transfer(organizer, event_id, balance);

        Ok(())
          
        
    }

    // the function to transfer nft 
    // the token_id is for 
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