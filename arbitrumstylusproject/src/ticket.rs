// #![cfg_attr(not(any(test, feature = "export-abi")), no_main)]
// extern crate alloc;

// use openzeppelin_stylus_proc::interface_id;

// use stylus_sdk::{
//     abi::Bytes,
//     call::{self, Call, MethodError},
//     evm, function_selector, msg,
//     prelude::*,
//     storage::{StorageAddress, StorageBool, StorageMap, StorageU256},
//     alloy_primitives::U256,
// };
// use alloy_sol_types::sol;

// sol_storage!{
//     #[entrypoint]

//     pub struct Event{
//         address attendee;
//         address organizer;
//         uint256 id;
//         string eventName;
//         uint256 date;
//         string location;
//         uint256 price;
//         uint256 ticketSold;
//         uint256 totalTickets;


//         mapping(uint256=>address) eventOrganizers;

//         mapping(uint256=>address) eventAttendees;

//         mapping(address=>bool) already_register 

//     }

//     pub struct Organizer{
//         address walletAddress;
//         uint256 id;
//         string name;
//         bool isVerifed;
//         uint256[] eventCreated;

//     }


//     pub struct Attendee{
//         address walletAddress;
//         uint256 id;
//         string name;
//         bool isVerifed;



//     }
// }

// sol!{

//     error Already_paid();

//     error Address_zero_not_allowed();

//     error Amount_must_be_greater_than_zero(uint256 balance, uint256 amount);
//     error Did_not_register(address attendee)
// }

// [derive(SolidityError)];

// sol_interface!{

//     interface ERC20{

//         fn balanceOf(address owner) external return(uint256);
//         fn trasnsferFrom(address owner, address receiver, uint256 amount);
//     }


//     interface ERC721{

//         event Transfer(address indexed from, address indexed to, uint256 indexed tokenId);

   
//     event Approval(address indexed owner, address indexed approved, uint256 indexed tokenId);

//     event ApprovalForAll(address indexed owner, address indexed operator, bool approved);

   
//     function balanceOf(address owner) external view returns (uint256 balance);

    
//      */
//     function ownerOf(uint256 tokenId) external view returns (address owner);

//     }
// }

// pub enum EventError{


//     Already_paid(Already_paid),
//     Address_zero_not_allowed(Address_zero_not_allowed),
//     Amount_must_be_greater_than_zero(Amount_must_be_greater_than_zero),
//     Did_not_register(Did_not_register)



// }

// impl Event{

//     fn purchase_ticket(& mut self, amount: uint256)->Result<(),EventError>{

//         let mut  ticketID = 0;
//         let purchaser_address=self.attendee.get();
    // let purchaser_Address = msg::Sender();
//         let account_balance= ERC20(purchaser_address);

//         if account_balance<0 {

//             return Err(EventError::Amount_must_be_greater_than_zero(
//                 Amount_must_be_greater_than_zero{
//                     account_balance,
//                     amount
//                 }
//             ));
//         }

//         account_balance-=amount;

//         account_balance.trasnsferFrom(purchaser_address,self.vm().address(), amount);

//         eventAttendees[ticketID+=1]=self.attendee;

//         already_register[purchaser_address]=true;
//         // I need to map the id to the user 


//         Ok(())

//     }

//     fn create_event(&mut self, organizer_address: address, name: string)->Result<string,EventError>{

//         self.eventName= name;

//         Ok((name));
//     }

    
//     fn get_event_details()->Self{
//         Self::new()
//     }
    
//     fn only_organizer(&self){
//         self.organizer;
//     }

//     fn refund_amount(&mut self, attendee: address, amount: uint256)->Result<(),EventError>{
//         only_organizer();

//         if already_register[attendee]==false{
          
//           return Err(EventError::Did_not_register(Did_not_register{attendee}));
//         }
        
//         self.vm().address().trasnsferFrom(self.vm.address(),attendee,amount )

//         already_register[attendee]==false

//     }
//     //fn initialize()

//     fn initialize(&self)->Self{

//     }
// }
//     //fn mint(){
//     // mint to the contract and transfer to the whitelisted address
//   //  }
//     // minting an erc721 token to a registered address
//     // fn withdraw()

/* *//* */
#![cfg_attr(not(any(test, feature = "export-abi")), no_main)]
extern crate alloc;

use alloc::string::String;
use openzeppelin_stylus_proc::interface_id;

use stylus_sdk::{
    abi::Bytes,
    call::{Call, MethodError},
    evm, function_selector, msg,
    prelude::*,
    storage::{StorageAddress, StorageBool, StorageMap, StorageU256},
    alloy_primitives::U256,
};
use alloy_sol_types::sol;

sol_storage! {
    #[entrypoint]
    pub struct Event {
        // Basic event data
        attendee: address,
        organizer: address,
        id: uint256,
        eventName: string,
        date: uint256,
        location: string,
        price: uint256,
        ticketSold: uint256,
        totalTickets: uint256,

        // Mappings to track organizers, attendees, and registration
        eventOrganizers: mapping(uint256 => address),
        eventAttendees: mapping(uint256 => address),
        already_register: mapping(address => bool),

        // NFT fields for minting NFT after the event
        nft_next_token: uint256,
        nft_owners: mapping(uint256 => address)
    }

    // pub struct Organizer {
    //     walletAddress: address,
    //     id: uint256,
    //     name: string,
    //     isVerifed: bool,
    //     eventCreated: uint256[]
    // }

    // pub struct Attendee {
    //     walletAddress: address,
    //     id: uint256,
    //     name: string,
    //     isVerifed: bool
    // }
}

sol! {
    error Already_paid();
    error Address_zero_not_allowed();
    error Amount_must_be_greater_than_zero(uint256 balance, uint256 amount);
    error Did_not_register(address attendee);
}

#[derive(SolidityError)]
pub enum EventError {
    Already_paid(Already_paid),
    Address_zero_not_allowed(Address_zero_not_allowed),
    Amount_must_be_greater_than_zero(Amount_must_be_greater_than_zero),
    Did_not_register(Did_not_register)
}

sol_interface! {
    interface ERC20 {
        fn balanceOf(owner: address) external returns (uint256);
        fn transferFrom(owner: address, receiver: address, amount: uint256);
    }

    interface ERC721 {
        event Transfer(from: address, to: address, tokenId: uint256);
        event Approval(owner: address, approved: address, tokenId: uint256);
        event ApprovalForAll(owner: address, operator: address, approved: bool);

        fn balanceOf(owner: address) external view returns (uint256);
        fn ownerOf(tokenId: uint256) external view returns (address);
    }
}

impl Event {
    /// Purchase a ticket for this event.
    fn purchase_ticket(&mut self, amount: uint256) -> Result<(), EventError> {
        // Get the purchaser’s address.
        let purchaser_address = msg::sender();
        
        // Check the purchaser’s ERC20 balance.
        let account_balance: uint256 = ERC20::balanceOf(purchaser_address);
        if account_balance < amount {
            return Err(EventError::Amount_must_be_greater_than_zero(
                Amount_must_be_greater_than_zero { balance: account_balance, amount: amount }
            ));
        }
        
        // Transfer funds from the purchaser to this contract.
        ERC20::transferFrom(purchaser_address, msg::address(), amount);
        
        // Increment ticketSold and register the attendee.
        self.ticketSold = self.ticketSold + 1;
        self.eventAttendees.insert(self.ticketSold, purchaser_address);
        self.already_register.insert(purchaser_address, true);
        
        Ok(())
    }
    
    /// Create an event and assign the organizer.
    fn create_event(&mut self, organizer_address: address, name: string) -> Result<string, EventError> {
        if organizer_address == address(0) {
            return Err(EventError::Address_zero_not_allowed(Address_zero_not_allowed()));
        }
        self.organizer = organizer_address;
        self.eventName = name.clone();
        Ok(name)
    }
    

    fn get_event_details(&self) -> (address, address, uint256, string, uint256, string, uint256, uint256, uint256) {
        (
            self.attendee,
            self.organizer,
            self.id,
            self.eventName.clone(),
            self.date,
            self.location.clone(),
            self.price,
            self.ticketSold,
            self.totalTickets
        )
    }
    
    /// Ensure that only the organizer can perform certain actions.
    fn only_organizer(&self) {
        if msg::sender() != self.organizer {
            panic!("Only organizer allowed");
        }
    }
    
    /// Refund an amount to an attendee.
    fn refund_amount(&mut self, attendee: address, amount: uint256) -> Result<(), EventError> {
        self.only_organizer();
        
        if !self.already_register.get(attendee).unwrap_or(false) {
            return Err(EventError::Did_not_register(Did_not_register { attendee: attendee }));
        }
        
        // Transfer funds from this contract to the attendee.
        ERC20::transferFrom(msg::address(), attendee, amount);
        self.already_register.insert(attendee, false);
        
        Ok(())
    }
    
    /// Mint an NFT ticket for a registered attendee.
    fn mint_nft(&mut self, to: address) -> Result<(), EventError> {
        // Ensure the recipient is registered.
        if !self.already_register.get(to).unwrap_or(false) {
            return Err(EventError::Did_not_register(Did_not_register { attendee: to }));
        }
        let token_id = self.nft_next_token;
        self.nft_next_token = self.nft_next_token + 1;
        self.nft_owners.insert(token_id, to);
        

        emit!(ERC721::Transfer(address(0), to, token_id));
        Ok(())
    }
    
    /// Withdraw funds from the contract to a designated address.
    fn withdraw(&mut self, to: address, amount: uint256) -> Result<(), EventError> {
        self.only_organizer();

        // Transfer funds from this contract to the designated address.
        // (Assuming evm::transfer returns a Result<(), MethodError>)
        evm::transfer(to, amount).map_err(|_| EventError::Already_paid(Already_paid()))?;
        Ok(())
    }
    
    /// Initialize event data.
    fn initialize(&mut self, id: uint256, eventName: string, date: uint256, location: string, price: uint256, totalTickets: uint256) {
        self.id = id;
        self.eventName = eventName;
        self.date = date;
        self.location = location;
        self.price = price;
        self.totalTickets = totalTickets;
        self.ticketSold = 0;
        self.nft_next_token = 1;
    }
}


/* */

/* */ /* */
/*#![cfg_attr(not(any(test, feature = "export-abi")), no_main)]
extern crate alloc;

use openzeppelin_stylus_proc::interface_id;

use stylus_sdk::{
    abi::Bytes,
    call::{self, Call, MethodError},
    evm, function_selector, msg,
    prelude::*,
    storage::{StorageAddress, StorageBool, StorageMap, StorageString, StorageU256},
    alloy_primitives::{U256, Address},
};
use alloy_sol_types::sol;

sol_storage! {
    #[entrypoint]
    pub struct Event {
        StorageAddress organizer;
        StorageU256 id;
        StorageString event_name;
        StorageU256 date;
        StorageString location;
        StorageU256 price;
        StorageU256 tickets_sold;
        StorageU256 total_tickets;
        StorageAddress erc20_token;

        StorageMap<U256, StorageAddress> event_organizers;
        StorageMap<U256, StorageAddress> event_attendees;
        StorageMap<Address, StorageBool> already_registered;
    }
}

sol! {
    error AlreadyPaid();
    error AddressZeroNotAllowed();
    error AmountMustBeGreaterThanZero(uint256 balance, uint256 amount);
    error DidNotRegister(address attendee);
    error Unauthorized();
}

#[derive(SolidityError)]
pub enum EventError {
    AlreadyPaid(AlreadyPaid),
    AddressZeroNotAllowed(AddressZeroNotAllowed),
    AmountMustBeGreaterThanZero(AmountMustBeGreaterThanZero),
    DidNotRegister(DidNotRegister),
    Unauthorized(Unauthorized),
}

sol_interface! {
    interface ERC20 {
        fn balanceOf(address owner) external returns (uint256);
        fn transferFrom(address from, address to, uint256 amount) external;
    }
}

impl Event {
    fn only_organizer(&self) -> Result<(), EventError> {
        if msg::sender() != self.organizer.get() {
            return Err(EventError::Unauthorized(Unauthorized {}));
        }
        Ok(())
    }

    pub fn initialize(&mut self, erc20_token: Address, organizer: Address) {
        self.erc20_token.set(erc20_token);
        self.organizer.set(organizer);
    }

    pub fn create_event(
        &mut self,
        name: String,
        date: U256,
        location: String,
        price: U256,
        total_tickets: U256,
    ) -> Result<U256, EventError> {
        let event_id = self.id.get();
        self.id.set(event_id + U256::from(1));

        self.event_organizers.insert(event_id, StorageAddress::from(msg::sender()));
        self.event_name.set_str(&name);
        self.date.set(date);
        self.location.set_str(&location);
        self.price.set(price);
        self.total_tickets.set(total_tickets);
        self.tickets_sold.set(U256::ZERO);

        Ok(event_id)
    }

    pub fn purchase_ticket(&mut self, event_id: U256, amount: U256) -> Result<(), EventError> {
        let purchaser = msg::sender();

        if self.already_registered.get(purchaser) {
            return Err(EventError::AlreadyPaid(AlreadyPaid {}));
        }

        let erc20 = ERC20::new(self.erc20_token.get());
        let balance = erc20.balance_of(purchaser)
            .call()
            .map_err(|_| EventError::AmountMustBeGreaterThanZero(AmountMustBeGreaterThanZero {
                balance: U256::ZERO,
                amount,
            }))?;

        if balance < amount {
            return Err(EventError::AmountMustBeGreaterThanZero(AmountMustBeGreaterThanZero { balance, amount }));
        }

        erc20.transfer_from(purchaser, self.vm().address(), amount)
            .call()
            .map_err(|_| EventError::AmountMustBeGreaterThanZero(AmountMustBeGreaterThanZero { balance, amount }))?;

        let tickets_sold = self.tickets_sold.get() + U256::from(1);
        self.tickets_sold.set(tickets_sold);
        self.event_attendees.insert(tickets_sold, StorageAddress::from(purchaser));
        self.already_registered.insert(purchaser, StorageBool::from(true));

        Ok(())
    }

    pub fn refund_ticket(
        &mut self,
        event_id: U256,
        attendee: Address,
        amount: U256,
    ) -> Result<(), EventError> {
        self.only_organizer()?;

        if !self.already_registered.get(attendee) {
            return Err(EventError::DidNotRegister(DidNotRegister { attendee }));
        }

        let erc20 = ERC20::new(self.erc20_token.get());
        erc20.transfer_from(self.vm().address(), attendee, amount)
            .call()
            .map_err(|_| EventError::AmountMustBeGreaterThanZero(AmountMustBeGreaterThanZero {
                balance: U256::ZERO,
                amount,
            }))?;

        let tickets_sold = self.tickets_sold.get() - U256::from(1);
        self.tickets_sold.set(tickets_sold);
        self.already_registered.insert(attendee, StorageBool::from(false));

        Ok(())
    }

    pub fn get_event_details(&self, event_id: U256) -> (String, U256, String, U256, U256) {
        (
            self.event_name.get_str(),
            self.date.get(),
            self.location.get_str(),
            self.price.get(),
            self.total_tickets.get(),
        )
    }
}
    */