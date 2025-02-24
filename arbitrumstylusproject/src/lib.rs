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

//use  ethers::{etherscan::account::{self, AccountBalance}, types::spoof::balance};
use stylus_sdk::{
    alloy_primitives::{Address, U256}, call::transfer_eth, contract::{self, balance}, evm, msg, prelude::*, storage::{StorageAddress, StorageBool, StorageMap, StorageString, StorageU256}
};
use alloy_sol_types::sol;
use stylus_sdk::alloy_sol_types::{SolError, SolType,Panic,Revert};

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
    }
}




sol!{

    event Transfer(address indexed from, address indexed to, uint256 indexed token_id);
    event Approval(address indexed owner, address indexed approved, uint256 indexed token_id);

 }

//  sol_interface! {
    
//  }
// sol! {

//     error AlreadyRegistered();
    
//     error InsufficientPayment(uint256 sent, uint256 required);
//     error NotOrganizer();
//     error NotRegistered();
//     error TicketLimitExceeded();
//     error WithdrawFailed();
    
// }

// #[derive(SolidityError)]
// pub enum EventError {
//     AlreadyRegistered(AlreadyRegistered),
//     InsufficientPayment(InsufficientPayment),
//     NotOrganizer(NotOrganizer),
//     NotRegistered(NotRegistered),
//     TicketLimitExceeded(TicketLimitExceeded),
//     WithdrawFailed(WithdrawFailed),
// }



#[public]
impl Event {
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

        let organizer = msg::sender();
        
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
    pub fn register(&mut self, event_id: U256)  {
        let attendee = msg::sender();
        let price = self.event_prices.get(event_id);

        // Check if already registered

        // self.event_registrations.get(attendee).map(|registrations| registrations.get(attendee)).unwrap_or(false){

        //     return Err(EventError::AlreadyRegistered(AlreadyRegistered {}));
        // }
        if self.event_registrations
        .get(attendee)==true
    {
        // 
        panic!("already registerd");
    }
        // if self.event_registrations
        //     .get(event_id)
        //     .map(|registrations| registrations.get(attendee))
        //     .unwrap_or(false)
        // {  return Err(EventError::AlreadyRegistered(AlreadyRegistered {}));
        // }
        //   

        
        // Validate payment
        let sent = msg::value();
        if sent < price {
            // return Err(EventError::InsufficientPayment(InsufficientPayment {
            //     sent,
            //     required: price,
            // }));

            panic!("insufficientPayment ");
        }

        transfer_eth(contract::address(), price);
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

        
    }

    pub fn refund(&mut self, event_id: U256, attendee: Address)  {
        let registered = self.event_registrations.get(attendee);

        let organizer = self.event_organizers.get(event_id);

        if !registered{
            // EventError::NotOrganizer(NotOrganizer {});

            panic!("not an organizer");
        }
          

        // Only organizer can issue refunds
        if msg::sender() != organizer {
            // return Err(EventError::NotOrganizer(NotOrganizer {}));
            panic!("not an organizer");
        }

        // Check if registered
        if !self.event_registrations
            .get(attendee)==false
        {
            // return Err(EventError::NotRegistered(NotRegistered {}));
            panic!("not registered");
        }

        let price = self.event_prices.get(event_id);
        
        // Send ETH back
        transfer_eth(attendee, price)
            .map_err(|_| panic!(" withdrawal fail"));

        // Update state
        self.event_tickets_sold.insert(event_id, self.event_tickets_sold.get(event_id) - U256::from(1));
        self.event_registrations
            
            .insert(attendee, false);

    
    }

    pub fn withdraw_funds(&mut self, event_id: U256)  {
        let organizer = self.event_organizers.get(event_id);

        // if !organizer{
        //     return Err((EventError::NotOrganizer(NotOrganizer {})));
        // }
           

        if msg::sender() != organizer {
            // return Err(EventError::NotOrganizer(NotOrganizer {}));
            panic!("Not an orgainizer")
        }

       // let balance = balance(contract::address());
        
        let balance= contract::balance();
        transfer_eth(organizer, balance)
            .map_err(|_| panic!("withdrawal failed")/*EventError::WithdrawFailed(WithdrawFailed {})*/);

        
    }
}