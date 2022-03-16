#!/bin/bash
SCRIPT_PATH=$( cd -- "$( dirname -- "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )
BECH32_UTIL="bech32_util/bech32_2_hex.py"
BYTECODE="output/prize.wasm"
ADDRESS=$(erdpy data load --key=address-testnet)
DEPLOY_TRANSACTION=$(erdpy data load --key=deployTransaction-testnet)
PROXY=https://testnet-api.elrond.com
CHAIN=T

# Wallets 
source ./interaction/test_wallets.sh

######################################################################
# SC Management
######################################################################
deploy() {
    erdpy --verbose contract deploy --bytecode ${BYTECODE} --recall-nonce --pem=${OWNER} --gas-limit=500000000 --send --outfile="deploy-testnet.interaction.json" --proxy=${PROXY} --chain=${CHAIN} || return

    TRANSACTION=$(erdpy data parse --file="deploy-testnet.interaction.json" --expression="data['emitted_tx']['hash']")
    ADDRESS=$(erdpy data parse --file="deploy-testnet.interaction.json" --expression="data['emitted_tx']['address']")

    erdpy data store --key=address-testnet --value=${ADDRESS}
    erdpy data store --key=deployTransaction-testnet --value=${TRANSACTION}

    echo ""
    echo "Smart contract address: ${ADDRESS}"
}

upgrade() {
    erdpy --verbose contract upgrade ${ADDRESS} --bytecode ${BYTECODE} --recall-nonce --pem=${OWNER} --gas-limit=500000000 --send --outfile="deploy-testnet.interaction.json" --proxy=${PROXY} --chain=${CHAIN} || return
}

claimDeveloperRewards()
{
    erdpy --verbose contract call ${ADDRESS} --recall-nonce --pem=${OWNER} --gas-limit=50000000 --function="ClaimDeveloperRewards" --send --proxy=${PROXY} --chain=${CHAIN}
}

######################################################################
# Administrator API
######################################################################

distributePrizes() {
    erdpy --verbose contract call ${ADDRESS} --recall-nonce --pem=${OWNER} --gas-limit=500000000 --function="distributePrizes" --send --proxy=${PROXY} --chain=${CHAIN}
}

cleanClaimed() {
    erdpy --verbose contract call ${ADDRESS} --recall-nonce --pem=${OWNER} --gas-limit=500000000 --function="cleanClaimed" --send --proxy=${PROXY} --chain=${CHAIN}
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

getCharityPool() {
    erdpy --verbose contract query ${ADDRESS} --function="getCharityPool" --proxy=${PROXY} 
}

claimDonations() {
    erdpy --verbose contract call ${ADDRESS} --recall-nonce --pem=${OWNER} --gas-limit=50000000 --function="claimDonations" --send --proxy=${PROXY} --chain=${CHAIN}
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

# Param1 : max length
setParamSponsorInfoMaxLength() {
    erdpy --verbose contract call ${ADDRESS} --recall-nonce --pem=${OWNER} --gas-limit=50000000 --function="setParamSponsorInfoMaxLength" --arguments $1 --send --proxy=${PROXY} --chain=${CHAIN}
}

getParamSponsorInfoMaxLength() {
    erdpy --verbose contract query ${ADDRESS} --function="getParamSponsorInfoMaxLength" --proxy=${PROXY} 
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

# ELROND
# Param #1 : duration in seconds
# Param #2 : pem wallet
createEgld() {
    EGLD_AMOUNT="50000000000000000"     #10000000000000000 => 0.01 EGLD
    PSEUDO="0x$(xxd -pu -c 256 <<< "Elrond")"
    URL1="0x$(xxd -pu -c 256  <<< "https://elrond.com/")"
    URL2="0x$(xxd -pu -c 256  <<< "https://twitter.com/ElrondNetwork")"
    URL3="0x$(xxd -pu -c 256  <<< "")"
    RESERVED="0x$(xxd -pu -c 256  <<< "")"
    GRAPHIC="0x$(xxd -pu -c 256  <<< "cover")"
    LOGO_LINK="0x$(xxd -pu -c 256  <<< "https://image.shutterstock.com/z/stock-vector-elrond-egld-token-coin-symbol-with-crypto-currency-themed-background-design-modern-blue-neon-color-1912925707.jpg")"
    FREE_TEXT="0x$(xxd -pu -c 256  <<< "The Internet Scale Blockchain Is Live!")"
    PREMIUM="0"
    CHARITY="0"

    erdpy --verbose contract call ${ADDRESS} --recall-nonce --pem=$2 --gas-limit=500000000 --function="create" --value=${EGLD_AMOUNT} --arguments $1 ${PSEUDO} ${URL1} ${URL2} ${URL3} ${RESERVED} ${GRAPHIC} ${LOGO_LINK} ${FREE_TEXT} ${PREMIUM} ${CHARITY} --send --proxy=${PROXY} --chain=${CHAIN}
}

# CUPSHE
createEgld_2() {
    EGLD_AMOUNT="10000000000000000"     #10000000000000000 => 0.01 EGLD
    PSEUDO="0x$(xxd -pu -c 256 <<< "CUPSHE")"
    URL1="0x$(xxd -pu -c 256  <<< "https://fr.cupshe.com/")"
    URL2="0x$(xxd -pu -c 256  <<< "")"
    URL3="0x$(xxd -pu -c 256  <<< "")"
    RESERVED="0x$(xxd -pu -c 256  <<< "")"
    GRAPHIC="0x$(xxd -pu -c 256  <<< "cover")"
    LOGO_LINK="0x$(xxd -pu -c 256  <<< "https://cdn-review.cupshe.com/cmc-admin/20210712/97e1b5540f894b96b7f84e71814e69fe13636053499957")"
    FREE_TEXT="0x$(xxd -pu -c 256  <<< "Let'have a look to the new collection !")"
    PREMIUM="0"
    CHARITY="0"

    erdpy --verbose contract call ${ADDRESS} --recall-nonce --pem=$2 --gas-limit=500000000 --function="create" --value=${EGLD_AMOUNT} --arguments $1 ${PSEUDO} ${URL1} ${URL2} ${URL3} ${RESERVED} ${GRAPHIC} ${LOGO_LINK} ${FREE_TEXT}  ${PREMIUM} ${CHARITY} --send --proxy=${PROXY} --chain=${CHAIN}
}

# Jeux video
createEgld_1() {
    EGLD_AMOUNT="20000000000000000"     #10000000000000000 => 0.01 EGLD
    PSEUDO="0x$(xxd -pu -c 256 <<< "Jeux Video")"
    URL1="0x$(xxd -pu -c 256  <<< "https://www.jeuxvideo.com/")"
    URL2="0x$(xxd -pu -c 256  <<< "")"
    URL3="0x$(xxd -pu -c 256  <<< "")"
    RESERVED="0x$(xxd -pu -c 256  <<< "")"
    GRAPHIC="0x$(xxd -pu -c 256  <<< "cover")"
    LOGO_LINK="0x$(xxd -pu -c 256  <<< "https://image.jeuxvideo.com/medias-md/163967/1639665893-3649-card.png")"
    FREE_TEXT="0x$(xxd -pu -c 256  <<< "Play 2 earn =)")"
    PREMIUM="0"
    CHARITY="0"

    erdpy --verbose contract call ${ADDRESS} --recall-nonce --pem=$2 --gas-limit=500000000 --function="create" --value=${EGLD_AMOUNT} --arguments $1 ${PSEUDO} ${URL1} ${URL2} ${URL3} ${RESERVED} ${GRAPHIC} ${LOGO_LINK} ${FREE_TEXT} ${PREMIUM} ${CHARITY} --send --proxy=${PROXY} --chain=${CHAIN}
}

# McDo
createEgld_3() {
    EGLD_AMOUNT="20000000000000000"     #10000000000000000 => 0.01 EGLD
    PSEUDO="0x$(xxd -pu -c 256 <<< "Mc Donalds")"
    URL1="0x$(xxd -pu -c 256  <<< "https://www.mcdonalds.fr/")"
    URL2="0x$(xxd -pu -c 256  <<< "")"
    URL3="0x$(xxd -pu -c 256  <<< "")"
    RESERVED="0x$(xxd -pu -c 256  <<< "")"
    GRAPHIC="0x$(xxd -pu -c 256  <<< "cover")"
    LOGO_LINK="0x$(xxd -pu -c 256  <<< "https://eu-images.contentstack.com/v3/assets/blt5004e64d3579c43f/blt6243759afdfd4588/61e5b45c35e87a3ac8bc4840/Logo_France_Mcdo.png")"
    FREE_TEXT="0x$(xxd -pu -c 256  <<< "Play 2 earn and come to eat =)")"
    PREMIUM="0"
    CHARITY="0"

    erdpy --verbose contract call ${ADDRESS} --recall-nonce --pem=$2 --gas-limit=500000000 --function="create" --value=${EGLD_AMOUNT} --arguments $1 ${PSEUDO} ${URL1} ${URL2} ${URL3} ${RESERVED} ${GRAPHIC} ${LOGO_LINK} ${FREE_TEXT} ${PREMIUM} ${CHARITY} --send --proxy=${PROXY} --chain=${CHAIN}
}

# e-toro
createEgld_4() {
    EGLD_AMOUNT="20000000000000000"     #10000000000000000 => 0.01 EGLD
    PSEUDO="0x$(xxd -pu -c 256 <<< "e-toro")"
    URL1="0x$(xxd -pu -c 256  <<< "https://stocks.etoro.com/")"
    URL2="0x$(xxd -pu -c 256  <<< "")"
    URL3="0x$(xxd -pu -c 256  <<< "")"
    RESERVED="0x$(xxd -pu -c 256  <<< "")"
    GRAPHIC="0x$(xxd -pu -c 256  <<< "cover")"
    LOGO_LINK="0x$(xxd -pu -c 256  <<< "https://www.etoro.com/wp-content/uploads/2018/05/eToro-share-img.png")"
    FREE_TEXT="0x$(xxd -pu -c 256  <<< "Play 2 share =)")"
    PREMIUM="0"
    CHARITY="0"

    erdpy --verbose contract call ${ADDRESS} --recall-nonce --pem=$2 --gas-limit=500000000 --function="create" --value=${EGLD_AMOUNT} --arguments $1 ${PSEUDO} ${URL1} ${URL2} ${URL3} ${RESERVED} ${GRAPHIC} ${LOGO_LINK} ${FREE_TEXT} ${PREMIUM} ${CHARITY} --send --proxy=${PROXY} --chain=${CHAIN}
}

# Lambo
createEgld_5() {
    EGLD_AMOUNT="20000000000000000"     #10000000000000000 => 0.01 EGLD
    PSEUDO="0x$(xxd -pu -c 256 <<< "lamborghini")"
    URL1="0x$(xxd -pu -c 256  <<< "https://www.lamborghini.com/")"
    URL2="0x$(xxd -pu -c 256  <<< "")"
    URL3="0x$(xxd -pu -c 256  <<< "")"
    RESERVED="0x$(xxd -pu -c 256  <<< "")"
    GRAPHIC="0x$(xxd -pu -c 256  <<< "fill")"
    LOGO_LINK="0x$(xxd -pu -c 256  <<< "https://logo-marque.com/wp-content/uploads/2021/03/Lamborghini-Logo.png")"
    FREE_TEXT="0x$(xxd -pu -c 256  <<< "Play 2 drive beautiful car in the world =)")"
    PREMIUM="0"
    CHARITY="0"

    erdpy --verbose contract call ${ADDRESS} --recall-nonce --pem=$2 --gas-limit=500000000 --function="create" --value=${EGLD_AMOUNT} --arguments $1 ${PSEUDO} ${URL1} ${URL2} ${URL3} ${RESERVED} ${GRAPHIC} ${LOGO_LINK} ${FREE_TEXT} ${PREMIUM} ${CHARITY} --send --proxy=${PROXY} --chain=${CHAIN}
}


# CUPRA
createEgld_10() {
    EGLD_AMOUNT="10000000000000000"     #10000000000000000 => 0.01 EGLD
    PSEUDO="0x$(xxd -pu -c 256 <<< "CUPRA")"
    URL1="0x$(xxd -pu -c 256  <<< "https://www.cupraofficial.fr")"
    URL2="0x$(xxd -pu -c 256  <<< "")"
    URL3="0x$(xxd -pu -c 256  <<< "")"
    RESERVED="0x$(xxd -pu -c 256  <<< "")"
    GRAPHIC="0x$(xxd -pu -c 256  <<< "cover")"
    LOGO_LINK="0x$(xxd -pu -c 256  <<< "https://www.cupraofficial.fr/content/countries/fr/cupra-website/fr/notre-adn/garage/cupra-urbanrebel-concept-car/_jcr_content/article/richtextwithfloating/singlevideoimage.resizedViewPort.scale.assetRootXL.jpg")"
    FREE_TEXT="0x$(xxd -pu -c 256  <<< "Play 2 drive beautiful car in Spain =)")"
    PREMIUM="0"
    CHARITY="0"

    erdpy --verbose contract call ${ADDRESS} --recall-nonce --pem=$2 --gas-limit=500000000 --function="create" --value=${EGLD_AMOUNT} --arguments $1 ${PSEUDO} ${URL1} ${URL2} ${URL3} ${RESERVED} ${GRAPHIC} ${LOGO_LINK} ${FREE_TEXT} ${PREMIUM} ${CHARITY} --send --proxy=${PROXY} --chain=${CHAIN}
}



# CCI 
createEgld_20() {
    EGLD_AMOUNT="10000000000000000"     #10000000000000000 => 0.01 EGLD
    PSEUDO="0x$(xxd -pu -c 256 <<< "La ruche numérique")"
    URL1="0x$(xxd -pu -c 256  <<< "https://www.laruchenumerique.com/")"
    URL2="0x$(xxd -pu -c 256  <<< "")"
    URL3="0x$(xxd -pu -c 256  <<< "")"
    RESERVED="0x$(xxd -pu -c 256  <<< "")"
    GRAPHIC="0x$(xxd -pu -c 256  <<< "fill")"
    LOGO_LINK="0x$(xxd -pu -c 256  <<< "https://mlmo8pz9isqm.i.optimole.com/Y-n1_Ko-11RC-jz7/w:auto/h:auto/q:auto/https://www.laruchenumerique.com/wp-content/uploads/elementor/thumbs/ruche-numerique-capsule-house-of-code-ohpwg6f3j88lc3xwj1is5k1hroj2j96txaikh3opmk.jpg")"
    FREE_TEXT="0x$(xxd -pu -c 256  <<< "Play 2 learn")"
    PREMIUM="0"
    CHARITY="0"

    erdpy --verbose contract call ${ADDRESS} --recall-nonce --pem=$2 --gas-limit=500000000 --function="create" --value=${EGLD_AMOUNT} --arguments $1 ${PSEUDO} ${URL1} ${URL2} ${URL3} ${RESERVED} ${GRAPHIC} ${LOGO_LINK} ${FREE_TEXT} ${PREMIUM} ${CHARITY} --send --proxy=${PROXY} --chain=${CHAIN}
}

# CCI 
createEgld_21() {
    EGLD_AMOUNT="10000000000000000"     #10000000000000000 => 0.01 EGLD
    PSEUDO="0x$(xxd -pu -c 256 <<< "La ruche numérique")"
    URL1="0x$(xxd -pu -c 256  <<< "https://www.laruchenumerique.com/")"
    URL2="0x$(xxd -pu -c 256  <<< "")"
    URL3="0x$(xxd -pu -c 256  <<< "")"
    RESERVED="0x$(xxd -pu -c 256  <<< "")"
    GRAPHIC="0x$(xxd -pu -c 256  <<< "fill")"
    LOGO_LINK="0x$(xxd -pu -c 256  <<< "https://mlmo8pz9isqm.i.optimole.com/Y-n1_Ko-zcIbW14D/w:auto/h:auto/q:auto/https://www.laruchenumerique.com/wp-content/uploads/2019/07/ruche-numerique-logo-white.png")"
    FREE_TEXT="0x$(xxd -pu -c 256  <<< "Play 2 learn")"
    PREMIUM="0"
    CHARITY="0"

    erdpy --verbose contract call ${ADDRESS} --recall-nonce --pem=$2 --gas-limit=500000000 --function="create" --value=${EGLD_AMOUNT} --arguments $1 ${PSEUDO} ${URL1} ${URL2} ${URL3} ${RESERVED} ${GRAPHIC} ${LOGO_LINK} ${FREE_TEXT} ${PREMIUM} ${CHARITY} --send --proxy=${PROXY} --chain=${CHAIN}
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
    RESERVED="$(xxd -pu -c 256  <<< "")"
    GRAPHIC="$(xxd -pu -c 256  <<< "")"
    LOGO_LINK="$(xxd -pu -c 256  <<< "https://img2.storyblok.com/1440x0/smart/filters:format(webp)/f/113424/1920x1080/af849350b2/ride-token.png")"
    FREE_TEXT="$(xxd -pu -c 256  <<< "Win our new wonderful token !")"
    PREMIUM="00"
    CHARITY="00"
    TX_SC_CREATE_DATA="${SC_FUNCTION::-2}@${DURATION}@${PSEUDO}@${URL1}@${URL2}@${URL3}@${RESERVED}@${GRAPHIC}@${LOGO_LINK}@${FREE_TEXT}@${PREMIUM}@${CHARITY}"
    
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
    RESERVED="$(xxd -pu -c 256  <<< "")"
    GRAPHIC="$(xxd -pu -c 256  <<< "")"
    LOGO_LINK="$(xxd -pu -c 256  <<< "https://media.heartlandtv.com/images/HARVEST+MOON+SD.jpg")"
    FREE_TEXT="$(xxd -pu -c 256  <<< "Buy & sell NFTs !!!")"
    PREMIUM="00"
    CHARITY="00"
    TX_SC_CREATE_DATA="${SC_FUNCTION::-2}@${DURATION}@${PSEUDO}@${URL1}@${URL2}@${URL3}@${RESERVED}@${GRAPHIC}@${LOGO_LINK}@${FREE_TEXT}@${PREMIUM}@${CHARITY}"
    
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

# Var params : optional instance status filter (from 1 to 5 status can be provided).
 getNb() {
    if [ $# == 0 ]; then
        erdpy --verbose contract query ${ADDRESS} --function="getNb" --proxy=${PROXY} 
    else
        erdpy --verbose contract query ${ADDRESS} --function="getNb"  --arguments $* --proxy=${PROXY} 
    fi

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

# Param1 : player pem wallet or '0'
# Param2 : start iid
# Param3 : max number of instances to return
# Var params : Instance status filter (from 1 to 5 status can be provided)
getAllInfoFrag() {
    if [ $1 == "0" ]; then
        PLAYER_HEX_ADDRESS=$ADDR_ZERO
    else
        BECH32_PEM_WALLET=`grep -o -m 1 "erd[0-9a-z]*" $1`    
        PLAYER_HEX_ADDRESS=`${SCRIPT_PATH}/${BECH32_UTIL} $BECH32_PEM_WALLET`
        PLAYER_HEX_ADDRESS="0x${PLAYER_HEX_ADDRESS}"
    fi
    
    # replace arg1 with hex address
    set -- $PLAYER_HEX_ADDRESS "${@:2}"

    erdpy --verbose contract query ${ADDRESS} --function="getAllInfoFrag" --arguments $* --proxy=${PROXY} 
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
    PLAYER_HEX_ADDRESS=`${SCRIPT_PATH}/${BECH32_UTIL} $BECH32_PEM_WALLET`

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
