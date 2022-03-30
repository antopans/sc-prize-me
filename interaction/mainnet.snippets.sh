#!/bin/bash
BYTECODE="output/prize.wasm"
ADDRESS=$(erdpy data load --key=address-mainnet)
DEPLOY_TRANSACTION=$(erdpy data load --key=deployTransaction-mainnet)
PROXY=https://api.elrond.com
CHAIN=1

KEY_FILE="../wallets/wallet_owner_prize-me.json"
PASS_FILE="../wallets/wallet_owner_pass_prize-me"

######################################################################
# SC Management
######################################################################
deploy() {
    erdpy --verbose contract deploy --bytecode ${BYTECODE} --recall-nonce --keyfile=${KEY_FILE} --passfile=${PASS_FILE} --gas-limit=150000000 --send --outfile="deploy-mainnet.interaction.json" --proxy=${PROXY} --chain=${CHAIN} || return

    TRANSACTION=$(erdpy data parse --file="deploy-mainnet.interaction.json" --expression="data['emitted_tx']['hash']")
    ADDRESS=$(erdpy data parse --file="deploy-mainnet.interaction.json" --expression="data['emitted_tx']['address']")

    erdpy data store --key=address-mainnet --value=${ADDRESS}
    erdpy data store --key=deployTransaction-mainnet --value=${TRANSACTION}

    echo ""
    echo "Smart contract address: ${ADDRESS}"
}

upgrade() {
    erdpy --verbose contract upgrade ${ADDRESS} --bytecode ${BYTECODE} --recall-nonce --keyfile=${KEY_FILE} --passfile=${PASS_FILE} --gas-limit=150000000 --send --outfile="deploy-mainnet.interaction.json" --proxy=${PROXY} --chain=${CHAIN} || return
}
