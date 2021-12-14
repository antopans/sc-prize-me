#!/bin/bash
OWNER="../../../wallets/devnet/sc-owner.pem"
BYTECODE="output/egld-loto.wasm"
ADDRESS=$(erdpy data load --key=address-devnet)
DEPLOY_TRANSACTION=$(erdpy data load --key=deployTransaction-devnet)
PROXY=https://devnet-api.elrond.com
CHAIN=D




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
# Administrator API
######################################################################

triggerEndedInstances() {
    erdpy --verbose contract call ${ADDRESS} --recall-nonce --pem=${OWNER} --gas-limit=50000000 --function="triggerEndedInstances" --send --proxy=${PROXY} --chain=${CHAIN}
}

cleanClaimedInstances() {
    erdpy --verbose contract call ${ADDRESS} --recall-nonce --pem=${OWNER} --gas-limit=50000000 --function="cleanClaimedInstances" --send --proxy=${PROXY} --chain=${CHAIN}
}

######################################################################
# DApp endpoints : sponsor API
######################################################################
EGLD_AMOUNT="10000000000000000"     #10000000000000000 => 0.01 EGLD
DURATION_IN_S="30"
PSEUDO="Toto"
URL="https://www.toto.com/"
PICTURE_LINK="https://www.toto.com/global_common_2019/index/images/img-about-other.jpg"
FREE_TEXT="Hello I'm Toto !"

createInstance() {
    #erdpy --verbose contract call ${ADDRESS} --recall-nonce --pem=${OWNER} --gas-limit=50000000 --function="createInstance" --value=${EGLD_AMOUNT} --arguments ${DURATION_IN_S} "${PSEUDO}" "${URL}" "${PICTURE_LINK}" "${FREE_TEXT}" --send --proxy=${PROXY} --chain=${CHAIN}
    erdpy --verbose contract call ${ADDRESS} --recall-nonce --pem=${OWNER} --gas-limit=50000000 --function="createInstance" --value=${EGLD_AMOUNT} --arguments ${DURATION_IN_S} --send --proxy=${PROXY} --chain=${CHAIN}
}

# Param1 : Instance ID
trigger() {
    erdpy --verbose contract call ${ADDRESS} --recall-nonce --pem=${OWNER} --gas-limit=50000000 --function="trigger" --arguments $1 --send --proxy=${PROXY} --chain=${CHAIN}
}

######################################################################
# DApp endpoints : player API
######################################################################

# Param1 : Instance ID
play() {
    erdpy --verbose contract call ${ADDRESS} --recall-nonce --pem=${OWNER} --gas-limit=50000000 --function="play" --arguments $1 --send --proxy=${PROXY} --chain=${CHAIN}
}

# Param1 : Instance ID
claim() {
    erdpy --verbose contract call ${ADDRESS} --recall-nonce --pem=${OWNER} --gas-limit=50000000 --function="claim" --arguments $1 --send --proxy=${PROXY} --chain=${CHAIN}
}

######################################################################
# DApp view API
######################################################################

 getNbInstances() {
    erdpy --verbose contract query ${ADDRESS} --function="getNbInstances" --proxy=${PROXY} 
}

# Param1 : Instance ID
getInstanceStatus() {
    erdpy --verbose contract query ${ADDRESS} --function="getInstanceStatus" --arguments $1 --proxy=${PROXY} 
}

# Param1 : Instance ID
getInstanceInfo() {
    erdpy --verbose contract query ${ADDRESS} --function="getInstanceInfo" --arguments $1 --proxy=${PROXY} 
}

# Param1 : Instance ID
getRemainingTime() {
    erdpy --verbose contract query ${ADDRESS} --function="getRemainingTime" --arguments $1 --proxy=${PROXY} 
}

# Param1 : Instance status
isInstanceWithStatus() {
    erdpy --verbose contract query ${ADDRESS} --function="isInstanceWithStatus" --arguments $1 --proxy=${PROXY} 
}

# Param1 : Instance status
getInstanceIDs() {
    erdpy --verbose contract query ${ADDRESS} --function="getInstanceIDs" --arguments $1 --proxy=${PROXY} 
}

#INSTANCE_SPONSOR="erd1u4kn0mdpnn2geg9fhn7cd0w6m4s7677axy0f6ptw8xz2d4kxyp0sgynsls"
#getSponsorInstances() {
#    erdpy --verbose contract query ${ADDRESS} --function="getSponsorInstances" --arguments $INSTANCE_SPONSOR --proxy=${PROXY} 
#}














######################################################################
# OBSOLETE
######################################################################

cancel() {
    erdpy --verbose contract call ${ADDRESS} --recall-nonce --pem=${OWNER} --gas-limit=50000000 --function="cancel" --send --proxy=${PROXY} --chain=${CHAIN}
}

buy_ticket() {
    #read -p "wallet: " WALLET
    read -p "price: " PRICE
    erdpy --verbose contract call ${ADDRESS} --recall-nonce --pem=${OWNER}  --gas-limit=50000000 --function="buy_ticket" --value=${PRICE} --send --proxy=${PROXY} --chain=${CHAIN}
}


test() {
    erdpy --verbose contract call ${ADDRESS} --r	ecall-nonce --pem=${OWNER} --gas-limit=500000000 --function="test" --arguments "3" --send --proxy=${PROXY} --chain=${CHAIN}
}

test2() {
    erdpy --verbose contract query ${ADDRESS} --function="test2" --arguments "3" --proxy=${PROXY} 
}

test3() {
    erdpy --verbose contract call ${ADDRESS} --recall-nonce --pem=${OWNER} --gas-limit=500000000 --function="test3" --arguments "7" --send --proxy=${PROXY} --chain=${CHAIN}
}


test4() {
    erdpy --verbose contract query ${ADDRESS} --function="test4" --arguments "3" --proxy=${PROXY} 
}

status() {
    erdpy --verbose contract query ${ADDRESS} --function="status" --proxy=${PROXY} 
}

lotoInfo() {
    erdpy --verbose contract query ${ADDRESS} --function="lotoInfo" --proxy=${PROXY} 
}



