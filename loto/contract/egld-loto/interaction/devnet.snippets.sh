#!/bin/bash
OWNER="../../../wallets/devnet/sc-owner.pem"
BYTECODE="output/egld-loto.wasm"
ADDRESS=$(erdpy data load --key=address-devnet)
DEPLOY_TRANSACTION=$(erdpy data load --key=deployTransaction-devnet)
PROXY=https://devnet-api.elrond.com
CHAIN=D

deploy() {
    erdpy --verbose contract deploy --bytecode ${BYTECODE} --recall-nonce --pem=${OWNER} --gas-limit=50000000 --send --outfile="deploy-devnet.interaction.json" --proxy=${PROXY} --chain=${CHAIN} || return

    TRANSACTION=$(erdpy data parse --file="deploy-devnet.interaction.json" --expression="data['emitted_tx']['hash']")
    ADDRESS=$(erdpy data parse --file="deploy-devnet.interaction.json" --expression="data['emitted_tx']['address']")

    erdpy data store --key=address-devnet --value=${ADDRESS}
    erdpy data store --key=deployTransaction-devnet --value=${TRANSACTION}

    echo ""
    echo "Smart contract address: ${ADDRESS}"
}

upgrade() {
    erdpy --verbose contract upgrade ${ADDRESS} --bytecode ${BYTECODE} --recall-nonce --pem=${OWNER} --gas-limit=50000000 --send --outfile="deploy-devnet.interaction.json" --proxy=${PROXY} --chain=${CHAIN} || return
}

start() {
    #10000000000000000 => 0.01 EGLD
    read -p "Ticket price: " TICKET_PRICE
    read -p "Delay (s): " DEADLINE
    read -p "Fee (%): " FEE_PERCENT
    erdpy --verbose contract call ${ADDRESS} --recall-nonce --pem=${OWNER} --gas-limit=50000000 --function="start" --arguments ${TICKET_PRICE} ${DEADLINE} ${FEE_PERCENT} --send --proxy=${PROXY} --chain=${CHAIN}
}

cancel() {
    erdpy --verbose contract call ${ADDRESS} --recall-nonce --pem=${OWNER} --gas-limit=50000000 --function="cancel" --send --proxy=${PROXY} --chain=${CHAIN}
}

buy_ticket() {
    #read -p "wallet: " WALLET
    read -p "price: " PRICE
    erdpy --verbose contract call ${ADDRESS} --recall-nonce --pem=${OWNER}  --gas-limit=50000000 --function="buy_ticket" --value=${PRICE} --send --proxy=${PROXY} --chain=${CHAIN}
}

trigger() {
    erdpy --verbose contract call ${ADDRESS} --recall-nonce --pem=${OWNER} --gas-limit=500000000 --function="trigger" --send --proxy=${PROXY} --chain=${CHAIN}
}

get_fee() {
    erdpy --verbose contract call ${ADDRESS} --recall-nonce --pem=${OWNER} --gas-limit=50000000 --function="get_fee" --send --proxy=${PROXY} --chain=${CHAIN}
}

status() {
    erdpy --verbose contract query ${ADDRESS} --function="status" --proxy=${PROXY} 
}

lotoInfo() {
    erdpy --verbose contract query ${ADDRESS} --function="lotoInfo" --proxy=${PROXY} 
}



