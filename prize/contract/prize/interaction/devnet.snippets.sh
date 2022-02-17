#!/bin/bash
SCRIPT_PATH=$( cd -- "$( dirname -- "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )
BECH32_UTIL="bech32_util/bech32_2_hex.py"
BYTECODE="output/prize.wasm"
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

claimDeveloperRewards()
{
    erdpy --verbose contract call ${ADDRESS} --recall-nonce --pem=${OWNER} --gas-limit=50000000 --function="ClaimDeveloperRewards" --send --proxy=${PROXY} --chain=${CHAIN}
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

# Param1 : Instance ID
# Param2 : premium status
setPremium() {
    erdpy --verbose contract call ${ADDRESS} --recall-nonce --pem=${OWNER} --gas-limit=50000000 --function="setPremium" --arguments $1 $2 --send --proxy=${PROXY} --chain=${CHAIN}
}

# Param1 : Instance ID
# Param2 : disable status
disable() {
    erdpy --verbose contract call ${ADDRESS} --recall-nonce --pem=${OWNER} --gas-limit=50000000 --function="disable" --arguments $1 $2 --send --proxy=${PROXY} --chain=${CHAIN}
}

# Param1 : fees amount in EGLD
# Param2 : sponsor reward in percent
setFeePol() {
    erdpy --verbose contract call ${ADDRESS} --recall-nonce --pem=${OWNER} --gas-limit=50000000 --function="setFeePol" --arguments $1 $2 --send --proxy=${PROXY} --chain=${CHAIN}
}

getFeePol() {
    erdpy --verbose contract query ${ADDRESS} --function="getFeePol" --proxy=${PROXY} 
}

getFeePool() {
    erdpy --verbose contract query ${ADDRESS} --function="getFeePool" --proxy=${PROXY} 
}

claimFees() {
    erdpy --verbose contract call ${ADDRESS} --recall-nonce --pem=${OWNER} --gas-limit=50000000 --function="claimFees" --send --proxy=${PROXY} --chain=${CHAIN}
}

# Param1 : manual claim enable status
setParamManClaim() {
    erdpy --verbose contract call ${ADDRESS} --recall-nonce --pem=${OWNER} --gas-limit=50000000 --function="setParamManClaim" --arguments $1 --send --proxy=${PROXY} --chain=${CHAIN}
}

getParamManClaim() {
    erdpy --verbose contract query ${ADDRESS} --function="getParamManClaim" --proxy=${PROXY} 
}

# Param1 : max instances per sponsor
setParamNbMaxInstancesPerSponsor() {
    erdpy --verbose contract call ${ADDRESS} --recall-nonce --pem=${OWNER} --gas-limit=50000000 --function="setParamNbMaxInstancesPerSponsor" --arguments $1 --send --proxy=${PROXY} --chain=${CHAIN}
}

getParamNbMaxInstancesPerSponsor() {
    erdpy --verbose contract query ${ADDRESS} --function="getParamNbMaxInstancesPerSponsor" --proxy=${PROXY} 
}

# Param1 : min duration
# Param2 : max duration
setParamDuration() {
    erdpy --verbose contract call ${ADDRESS} --recall-nonce --pem=${OWNER} --gas-limit=50000000 --function="setParamDuration" --arguments $1 $2 --send --proxy=${PROXY} --chain=${CHAIN}
}

getParamDuration() {
    erdpy --verbose contract query ${ADDRESS} --function="getParamDuration" --proxy=${PROXY} 
}

getAddrBlacklist() {
    erdpy --verbose contract query ${ADDRESS} --function="getAddrBlacklist" --proxy=${PROXY} 
}

# Param1 : address to blacklist
addAddrBlacklist() {
    BECH32_PEM_WALLET=`grep -o -m 1 "erd[0-9a-z]*" $1`    
    HEX_ADDRESS=`${SCRIPT_PATH}/${BECH32_UTIL} $BECH32_PEM_WALLET`
    erdpy --verbose contract call ${ADDRESS} --recall-nonce --pem=${OWNER} --gas-limit=50000000 --function="addAddrBlacklist" --arguments "0x${HEX_ADDRESS}" --send --proxy=${PROXY} --chain=${CHAIN}
}

# Param1 : address to blacklist
rmAddrBlacklist() {
    BECH32_PEM_WALLET=`grep -o -m 1 "erd[0-9a-z]*" $1`    
    HEX_ADDRESS=`${SCRIPT_PATH}/${BECH32_UTIL} $BECH32_PEM_WALLET`
    erdpy --verbose contract call ${ADDRESS} --recall-nonce --pem=${OWNER} --gas-limit=50000000 --function="rmAddrBlacklist" --arguments "0x${HEX_ADDRESS}" --send --proxy=${PROXY} --chain=${CHAIN}
}

# Param1 : log enable status
setLogEnableStatus() {
    erdpy --verbose contract call ${ADDRESS} --recall-nonce --pem=${OWNER} --gas-limit=50000000 --function="setLogEnableStatus" --arguments $1 --send --proxy=${PROXY} --chain=${CHAIN}
}

getLogEnableStatus() {
    erdpy --verbose contract query ${ADDRESS} --function="getLogEnableStatus" --proxy=${PROXY} 
}

######################################################################
# DApp endpoints : sponsor API
######################################################################

# Param #1 : duration in seconds
# Param #2 : pem wallet
createEgld() {
    EGLD_AMOUNT="10000000000000000"     #10000000000000000 => 0.01 EGLD
    PSEUDO="0x$(xxd -pu -c 256 <<< "Elrond")"
    URL1="0x$(xxd -pu -c 256  <<< "https://elrond.com/")"
    URL2="0x$(xxd -pu -c 256  <<< "https://twitter.com/ElrondNetwork")"
    URL3="0x$(xxd -pu -c 256  <<< "")"
    URL4="0x$(xxd -pu -c 256  <<< "")"
    URL5="0x$(xxd -pu -c 256  <<< "")"
    LOGO_LINK="0x$(xxd -pu -c 256  <<< "https://www.shutterstock.com/fr/image-vector/elrond-egld-token-coin-symbol-crypto-1912925707")"
    FREE_TEXT="0x$(xxd -pu -c 256  <<< "The Internet Scale Blockchain Is Live!")"

    erdpy --verbose contract call ${ADDRESS} --recall-nonce --pem=$2 --gas-limit=50000000 --function="create" --value=${EGLD_AMOUNT} --arguments $1 ${PSEUDO} ${URL1} ${URL2} ${URL3} ${URL4} ${URL5} ${LOGO_LINK} ${FREE_TEXT} --send --proxy=${PROXY} --chain=${CHAIN}
}

# Param #1 : duration in seconds
# Param #2 : pem wallet
# Param #3 : Token ID
# Param #4 : Token amount
createEsdt() {
    SC_FUNCTION="$(xxd -pu -c 256 <<< "create")"
    DURATION=`printf "%02X" $1`; if [ $(expr ${#DURATION} % 2) != "0" ]; then DURATION="0${DURATION}"; fi
    PSEUDO="$(xxd -pu -c 256 <<< "Holoride")"
    URL1="$(xxd -pu -c 256  <<< "https://www.holoride.com/")"
    URL2="$(xxd -pu -c 256  <<< "https://twitter.com/holoride")"
    URL3="$(xxd -pu -c 256  <<< "https://www.instagram.com/holoride")"
    URL4="$(xxd -pu -c 256  <<< "")"
    URL5="$(xxd -pu -c 256  <<< "")"
    LOGO_LINK="$(xxd -pu -c 256  <<< "https://img2.storyblok.com/1440x0/smart/filters:format(webp)/f/113424/1920x1080/af849350b2/ride-token.png")"
    FREE_TEXT="$(xxd -pu -c 256  <<< "Win our new wonderful token !")"
    TX_SC_CREATE_DATA="${SC_FUNCTION::-2}@${DURATION}@${PSEUDO}@${URL1}@${URL2}@${URL3}@${URL4}@${URL5}@${LOGO_LINK}@${FREE_TEXT}"
    
    TOKEN_ID="$(xxd -pu -c 256  <<< $3)"
    TOKEN_AMOUNT=`printf "%02X" $4`; if [ $(expr ${#TOKEN_AMOUNT} % 2) != "0" ]; then TOKEN_AMOUNT="0${TOKEN_AMOUNT}"; fi

    TX_TOKEN_DATA="ESDTTransfer@${TOKEN_ID::-2}@${TOKEN_AMOUNT}"    

    TX_DATA="${TX_TOKEN_DATA}@${TX_SC_CREATE_DATA}"

    erdpy --verbose tx new --receiver=$ADDRESS --recall-nonce --pem=$2 --gas-limit=50000000 --data=${TX_DATA} --send --proxy=${PROXY} --chain=${CHAIN}
}

# Param #1 : duration in seconds
# Param #2 : pem wallet
# Param #3 : Token ID
# Param #4 : Token nonce
# Param #5 : Token amount
createNft() {
    BECH32_PEM_WALLET=`grep -o -m 1 "erd[0-9a-z]*" $2`    
    SC_HEX_ADDRESS=`${SCRIPT_PATH}/${BECH32_UTIL} $ADDRESS`

    SC_FUNCTION="$(xxd -pu -c 256 <<< "create")"
    DURATION=`printf "%02X" $1`; if [ $(expr ${#DURATION} % 2) != "0" ]; then DURATION="0${DURATION}"; fi
    PSEUDO="$(xxd -pu -c 256 <<< "E-MOON")"
    URL1="$(xxd -pu -c 256  <<< "https://emoon.space/")"
    URL2="$(xxd -pu -c 256  <<< "")"
    URL3="$(xxd -pu -c 256  <<< "")"
    URL4="$(xxd -pu -c 256  <<< "")"
    URL5="$(xxd -pu -c 256  <<< "")"
    LOGO_LINK="$(xxd -pu -c 256  <<< "https://media.heartlandtv.com/images/HARVEST+MOON+SD.jpg")"
    FREE_TEXT="$(xxd -pu -c 256  <<< "Buy & sell NFTs !!!")"
    TX_SC_CREATE_DATA="${SC_FUNCTION::-2}@${DURATION}@${PSEUDO}@${URL1}@${URL2}@${URL3}@${URL4}@${URL5}@${LOGO_LINK}@${FREE_TEXT}"
    
    TOKEN_ID="$(xxd -pu -c 256  <<< $3)"
    TOKEN_NONCE=`printf "%02X" $4`
    TOKEN_AMOUNT=`printf "%02X" $5`; if [ $(expr ${#TOKEN_AMOUNT} % 2) != "0" ]; then TOKEN_AMOUNT="0${TOKEN_AMOUNT}"; fi
    TX_TOKEN_DATA="ESDTNFTTransfer@${TOKEN_ID::-2}@${TOKEN_NONCE}@${TOKEN_AMOUNT}@${SC_HEX_ADDRESS}"    

    TX_DATA="${TX_TOKEN_DATA}@${TX_SC_CREATE_DATA}"  

    erdpy --verbose tx new --receiver=${BECH32_PEM_WALLET} --recall-nonce --pem=$2 --gas-limit=50000000 --data=${TX_DATA} --send --proxy=${PROXY} --chain=${CHAIN}
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

 getNb() {
    erdpy --verbose contract query ${ADDRESS} --function="getNb" --proxy=${PROXY} 
}

# Param1 : Instance ID
getStatus() {
    erdpy --verbose contract query ${ADDRESS} --function="getStatus" --arguments $1 --proxy=${PROXY} 
}

# Param1 : Instance ID
# Param2 : player pem wallet or empty string ""
getInfo() {
    if [[ $2 = "" ]]; then
        PLAYER_HEX_ADDRESS=$ADDR_ZERO
    else
        BECH32_PEM_WALLET=`grep -o -m 1 "erd[0-9a-z]*" $2`    
        PLAYER_HEX_ADDRESS=`${SCRIPT_PATH}/${BECH32_UTIL} $BECH32_PEM_WALLET`
        PLAYER_HEX_ADDRESS="0x${PLAYER_HEX_ADDRESS}"
    fi
    erdpy --verbose contract query ${ADDRESS} --function="getInfo" --arguments $1 $PLAYER_HEX_ADDRESS --proxy=${PROXY} 
}

# Param1 : player pem wallet or '0'
# Var params : Instance status filter (from 1 to 5 status can be provided)
getAllInfo() {
    if [ $1 == "0" ]; then
        PLAYER_HEX_ADDRESS=$ADDR_ZERO
    else
        BECH32_PEM_WALLET=`grep -o -m 1 "erd[0-9a-z]*" $1`    
        PLAYER_HEX_ADDRESS=`${SCRIPT_PATH}/${BECH32_UTIL} $BECH32_PEM_WALLET`
        PLAYER_HEX_ADDRESS="0x${PLAYER_HEX_ADDRESS}"
    fi
    
    # replace arg1 with hex address
    set -- $PLAYER_HEX_ADDRESS "${@:2}"

    erdpy --verbose contract query ${ADDRESS} --function="getAllInfo" --arguments $* --proxy=${PROXY} 
}

# Param1 : Instance ID
getInfoLegacy() {
    erdpy --verbose contract query ${ADDRESS} --function="getInfoLegacy" --arguments $1 --proxy=${PROXY} 
}

# Var params : Instance status filter (from 1 to 5 status can be provided)
getAllInfoLegacy() {
    erdpy --verbose contract query ${ADDRESS} --function="getAllInfoLegacy" --arguments $* --proxy=${PROXY} 
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

# Param1 : Sponsor pem wallet 
getSponsorIDs() {
    BECH32_PEM_WALLET=`grep -o -m 1 "erd[0-9a-z]*" $1`    
    SPONSOR_HEX_ADDRESS=`${SCRIPT_PATH}/${BECH32_UTIL} $BECH32_PEM_WALLET`
    
    erdpy --verbose contract query ${ADDRESS} --function="getSponsorIDs" --arguments "0x${SPONSOR_HEX_ADDRESS}" --proxy=${PROXY} 
}

# Param1 : Player pem wallet
getPlayerIDs() {
    BECH32_PEM_WALLET=`grep -o -m 1 "erd[0-9a-z]*" $1`    
    PLAYER_HEX_ADDRESS=`${SCRIPT_PATH}/${BECH32_UTIL}$BECH32_PEM_WALLET`

    erdpy --verbose contract query ${ADDRESS} --function="getPlayerIDs" --arguments "0x${PLAYER_HEX_ADDRESS}" --proxy=${PROXY} 
}

# Param1 : Instance ID
# Param2 : Player pem wallet
hasPlayed() {
    BECH32_PEM_WALLET=`grep -o -m 1 "erd[0-9a-z]*" $2`    
    PLAYER_HEX_ADDRESS=`${SCRIPT_PATH}/${BECH32_UTIL} $BECH32_PEM_WALLET`

    erdpy --verbose contract query ${ADDRESS} --function="hasPlayed" --arguments $1 "0x${PLAYER_HEX_ADDRESS}" --proxy=${PROXY} 
}

# Param1 : Instance ID
# Param2 : Player pem wallet
hasWon() {
    BECH32_PEM_WALLET=`grep -o -m 1 "erd[0-9a-z]*" $2`    
    PLAYER_HEX_ADDRESS=`${SCRIPT_PATH}/${BECH32_UTIL} $BECH32_PEM_WALLET`

    erdpy --verbose contract query ${ADDRESS} --function="hasWon" --arguments $1 "0x${PLAYER_HEX_ADDRESS}" --proxy=${PROXY} 
}
