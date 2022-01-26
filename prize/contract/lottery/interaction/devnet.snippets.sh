#!/bin/bash
BYTECODE="output/lottery.wasm"
ADDRESS=$(erdpy data load --key=address-devnet)
DEPLOY_TRANSACTION=$(erdpy data load --key=deployTransaction-devnet)
PROXY=https://devnet-api.elrond.com
CHAIN=D

# Wallets 
source ./interaction/devnet.wallets.sh

######################################################################
# SC Management
######################################################################
deploy() {
    erdpy --verbose contract deploy --bytecode ${BYTECODE} --recall-nonce --pem=${OWNER} --gas-limit=500000000 --send --outfile="deploy-devnet.interaction.json" --proxy=${PROXY} --chain=${CHAIN} || return

    TRANSACTION=$(erdpy data parse --file="deploy-devnet.interaction.json" --expression="data['emitted_tx']['hash']")
    ADDRESS=$(erdpy data parse --file="deploy-devnet.interaction.json" --expression="data['emitted_tx']['address']")

    erdpy data store --key=address-devnet --value=${ADDRESS}
    erdpy data store --key=deployTransaction-devnet --value=${TRANSACTION}

    echo ""
    echo "Smart contract address: ${ADDRESS}"
}

upgrade() {
    erdpy --verbose contract upgrade ${ADDRESS} --bytecode ${BYTECODE} --recall-nonce --pem=${OWNER} --gas-limit=500000000 --send --outfile="deploy-devnet.interaction.json" --proxy=${PROXY} --chain=${CHAIN} || return
}

######################################################################
# Debug API
######################################################################
# Param1 : Instance ID
getInfoTmp() {
    erdpy --verbose contract query ${ADDRESS} --function="getInfoTmp" --arguments $1 --proxy=${PROXY} 
}

######################################################################
# Administrator API
######################################################################

triggerEnded() {
    erdpy --verbose contract call ${ADDRESS} --recall-nonce --pem=${OWNER} --gas-limit=50000000 --function="triggerEnded" --send --proxy=${PROXY} --chain=${CHAIN}
}

cleanClaimed() {
    erdpy --verbose contract call ${ADDRESS} --recall-nonce --pem=${OWNER} --gas-limit=50000000 --function="cleanClaimed" --send --proxy=${PROXY} --chain=${CHAIN}
}

# Param1 : fees amount
setFees() {
    erdpy --verbose contract call ${ADDRESS} --recall-nonce --pem=${OWNER} --gas-limit=50000000 --function="setFees" --arguments $1 --send --proxy=${PROXY} --chain=${CHAIN}
}

claimFees() {
    erdpy --verbose contract call ${ADDRESS} --recall-nonce --pem=${OWNER} --gas-limit=50000000 --function="claimFees" --send --proxy=${PROXY} --chain=${CHAIN}
}

 getFeePool() {
    erdpy --verbose contract query ${ADDRESS} --function="getFeePool" --proxy=${PROXY} 
}

######################################################################
# DApp endpoints : sponsor API
######################################################################
EGLD_AMOUNT="10000000000000000"     #10000000000000000 => 0.01 EGLD
PSEUDO="0x$(xxd -pu -c 256 <<< "E-MOON")"
URL="0x$(xxd -pu -c 256  <<< "https://emoon.space/")"
PICTURE_LINK="0x$(xxd -pu -c 256  <<< "https://media.heartlandtv.com/images/HARVEST+MOON+SD.jpg")"
FREE_TEXT="0x$(xxd -pu -c 256  <<< "Buy & sell NFTs !!!")"

# Param #1 : duration in seconds
# Param #2 : pem wallet
create() {
    erdpy --verbose contract call ${ADDRESS} --recall-nonce --pem=$2 --gas-limit=50000000 --function="create" --value=${EGLD_AMOUNT} --arguments $1 ${PSEUDO} ${URL} ${PICTURE_LINK} ${FREE_TEXT} --send --proxy=${PROXY} --chain=${CHAIN}
}

# Param1 : Instance ID
# Param2 : pem wallet
trigger() {
    erdpy --verbose contract call ${ADDRESS} --recall-nonce --pem=$2 --gas-limit=50000000 --function="trigger" --arguments $1 --send --proxy=${PROXY} --chain=${CHAIN}
}

######################################################################
# DApp endpoints : player API
######################################################################

# Param1 : Instance ID
# Param2 : pem wallet
# Param3 : fees : #1000000000000000 => 0.001 EGLD
play() {
    erdpy --verbose contract call ${ADDRESS} --recall-nonce --pem=$2 --gas-limit=50000000 --function="play" --value=$3 --arguments $1 --send --proxy=${PROXY} --chain=${CHAIN}
}

# Param1 : Instance ID
# Param2 : pem wallet
claimPrize() {
    erdpy --verbose contract call ${ADDRESS} --recall-nonce --pem=$2 --gas-limit=50000000 --function="claimPrize" --arguments $1 --send --proxy=${PROXY} --chain=${CHAIN}
}

######################################################################
# DApp view API
######################################################################

 getFeePol() {
    erdpy --verbose contract query ${ADDRESS} --function="getFeePol" --proxy=${PROXY} 
}

 getNb() {
    erdpy --verbose contract query ${ADDRESS} --function="getNb" --proxy=${PROXY} 
}

# Param1 : Instance ID
getStatus() {
    erdpy --verbose contract query ${ADDRESS} --function="getStatus" --arguments $1 --proxy=${PROXY} 
}

# Param1 : Instance ID
getInfo() {
    erdpy --verbose contract query ${ADDRESS} --function="getInfo" --arguments $1 --proxy=${PROXY} 
}

# Var params : Instance status filter (from 1 to 5 status can be provided)
getAllInfo() {
    erdpy --verbose contract query ${ADDRESS} --function="getAllInfo" --arguments $* --proxy=${PROXY} 
}

# Param1 : Instance ID
getRemainingTime() {
    erdpy --verbose contract query ${ADDRESS} --function="getRemainingTime" --arguments $1 --proxy=${PROXY} 
}

# Param1 : Instance status
hasStatus() {
    erdpy --verbose contract query ${ADDRESS} --function="hasStatus" --arguments $1 --proxy=${PROXY} 
}

# Var params : Instance status filter (from 1 to 5 status can be provided)
getIDs() {
    erdpy --verbose contract query ${ADDRESS} --function="getIDs" --arguments $* --proxy=${PROXY} 
}

# Param1 : Sponsor address in HEX format (not string2hex but Bech32 to hex => http://207.244.241.38/elrond-converters/#bech32-to-hex ; advise: use AddressValue class in erdjs)
getSponsorIDs() {
    # Example : "0xe56d37eda19cd48ca0a9bcfd86bddadd61ed7bdd311e9d056e3984a6d6c6205f" to pass bech32 address erd1u4kn0mdpnn2geg9fhn7cd0w6m4s7677axy0f6ptw8xz2d4kxyp0sgynsls
    erdpy --verbose contract query ${ADDRESS} --function="getSponsorIDs" --arguments $1 --proxy=${PROXY} 
}

# Param1 : Player address in HEX format (not string2hex but Bech32 to hex => http://207.244.241.38/elrond-converters/#bech32-to-hex ; advise: use AddressValue class in erdjs)
getPlayerIDs() {
    # Example : "0xe56d37eda19cd48ca0a9bcfd86bddadd61ed7bdd311e9d056e3984a6d6c6205f" to pass bech32 address erd1u4kn0mdpnn2geg9fhn7cd0w6m4s7677axy0f6ptw8xz2d4kxyp0sgynsls
    erdpy --verbose contract query ${ADDRESS} --function="getPlayerIDs" --arguments $1 --proxy=${PROXY} 
}

# Param1 : Instance ID
# Param2 : Player address in HEX format (not string2hex but Bech32 to hex => http://207.244.241.38/elrond-converters/#bech32-to-hex ; advise: use AddressValue class in erdjs)
hasPlayed() {
    # Example : "0xe56d37eda19cd48ca0a9bcfd86bddadd61ed7bdd311e9d056e3984a6d6c6205f" to pass bech32 address erd1u4kn0mdpnn2geg9fhn7cd0w6m4s7677axy0f6ptw8xz2d4kxyp0sgynsls
    erdpy --verbose contract query ${ADDRESS} --function="hasPlayed" --arguments $1 $2 --proxy=${PROXY} 
}

# Param1 : Instance ID
# Param2 : Player address in HEX format (not string2hex but Bech32 to hex => http://207.244.241.38/elrond-converters/#bech32-to-hex ; advise: use AddressValue class in erdjs)
hasWon() {
    # Example : "0xe56d37eda19cd48ca0a9bcfd86bddadd61ed7bdd311e9d056e3984a6d6c6205f" to pass bech32 address erd1u4kn0mdpnn2geg9fhn7cd0w6m4s7677axy0f6ptw8xz2d4kxyp0sgynsls
    erdpy --verbose contract query ${ADDRESS} --function="hasWon" --arguments $1 $2 --proxy=${PROXY} 
}
