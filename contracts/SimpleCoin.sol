// SPDX-License-Identifier: MIT
pragma solidity 0.8.15;

contract SimpleCoin {
        mapping (address => uint) balances;

        struct Brick {
                int brick;
        }

        struct Things {
                Brick[] things;
        }

        constructor() {
                balances[tx.origin] = 10000;
        }

        function sendCoin(address receiver, uint amount) public returns(bool sufficient) {
                if (balances[msg.sender] < amount) return false;
                balances[msg.sender] -= amount;
                balances[receiver] += amount;
                return true;
        }

        function getBalanceInEth(address addr) public returns(uint){
                return getBalance(addr) * 2;
        }

        function getBalance(address addr) public payable returns(uint) {
                return balances[addr];
        }

        function read_things(Things memory params) public returns(bool) {
                return true;
        }
}
