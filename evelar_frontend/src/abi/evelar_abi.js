export const Contract_ABI =


[
    {
      "inputs": [
        { "internalType": "string", "name": "name", "type": "string" },
        { "internalType": "uint256", "name": "date", "type": "uint256" },
        { "internalType": "string", "name": "location", "type": "string" },
        { "internalType": "uint256", "name": "price", "type": "uint256" },
        { "internalType": "uint256", "name": "total_tickets", "type": "uint256" }
      ],
      "name": "createEvent",
      "outputs": [{ "internalType": "uint256", "name": "", "type": "uint256" }],
      "stateMutability": "nonpayable",
      "type": "function"
    },
    {
      "inputs": [{ "internalType": "uint256", "name": "event_id", "type": "uint256" }],
      "name": "register",
      "outputs": [],
      "stateMutability": "payable",
      "type": "function"
    },
    {
      "inputs": [
        { "internalType": "uint256", "name": "event_id", "type": "uint256" },
        { "internalType": "address", "name": "attendee", "type": "address" }
      ],
      "name": "refund",
      "outputs": [],
      "stateMutability": "nonpayable",
      "type": "function"
    },
    {
      "inputs": [{ "internalType": "uint256", "name": "event_id", "type": "uint256" }],
      "name": "withdrawFunds",
      "outputs": [],
      "stateMutability": "nonpayable",
      "type": "function"
    },
    {
      "inputs": [
        { "internalType": "uint256", "name": "token_id", "type": "uint256" },
        { "internalType": "address", "name": "from", "type": "address" },
        { "internalType": "address", "name": "to", "type": "address" }
      ],
      "name": "transfer",
      "outputs": [],
      "stateMutability": "nonpayable",
      "type": "function"
    },
    {
      "inputs": [{ "internalType": "address", "name": "to", "type": "address" }],
      "name": "mint",
      "outputs": [],
      "stateMutability": "nonpayable",
      "type": "function"
    },
    {
      "inputs": [],
      "name": "AlreadyRegistered",
      "type": "error"
    },
    {
      "inputs": [],
      "name": "ZeroAddressNotAllowed",
      "type": "error"
    },
    {
      "inputs": [],
      "name": "NotEnoughAmount",
      "type": "error"
    },
    {
      "inputs": [],
      "name": "NotOrganizer",
      "type": "error"
    },
    {
      "inputs": [],
      "name": "NotRegistered",
      "type": "error"
    },
    {
      "inputs": [{ "internalType": "uint256", "name": "", "type": "uint256" }],
      "name": "InvalidTokenId",
      "type": "error"
    },
    {
      "inputs": [
        { "internalType": "address", "name": "", "type": "address" },
        { "internalType": "uint256", "name": "", "type": "uint256" },
        { "internalType": "address", "name": "", "type": "address" }
      ],
      "name": "NotOwner",
      "type": "error"
    },
    {
      "inputs": [
        { "internalType": "address", "name": "", "type": "address" },
        { "internalType": "address", "name": "", "type": "address" },
        { "internalType": "uint256", "name": "", "type": "uint256" }
      ],
      "name": "NotApproved",
      "type": "error"
    },
    {
      "inputs": [{ "internalType": "uint256", "name": "", "type": "uint256" }],
      "name": "TransferToZero",
      "type": "error"
    },
    {
      "inputs": [
        { "internalType": "address", "name": "", "type": "address" },
        { "internalType": "uint256", "name": "", "type": "uint256" },
        { "internalType": "bytes4", "name": "", "type": "bytes4" }
      ],
      "name": "ReceiverRefused",
      "type": "error"
    }
  ]

  export const Contract_Address = "0x7ecb793f1de94aea690262409481e8e724876bf0";