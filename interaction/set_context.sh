#!/bin/bash

# Wallets 
source ./interaction/test_wallets.sh

#Create 4 instances
source ./interaction/devnet.snippets.sh; createEgld 60 $SPONSOR1
sleep 5

source ./interaction/devnet.snippets.sh; createEgld 60 $SPONSOR2
sleep 5

source ./interaction/devnet.snippets.sh; createEgld 60 $SPONSOR3
sleep 5

# 24h
source ./interaction/devnet.snippets.sh; createEgld 86400 $SPONSOR4
sleep 5


#Play

#Instance 1 => 2 players
source ./interaction/devnet.snippets.sh; play "1" $PLAYER1 0
sleep 5
source ./interaction/devnet.snippets.sh; play "1" $PLAYER2 0
sleep 5

#Instance 2 => 3 players
source ./interaction/devnet.snippets.sh; play "2" $PLAYER1 0
sleep 5
source ./interaction/devnet.snippets.sh; play "2" $PLAYER2 0
sleep 5
source ./interaction/devnet.snippets.sh; play "2" $PLAYER3 0
sleep 5

#Instance 3 => 6 players
source ./interaction/devnet.snippets.sh; play "3" $PLAYER1 0
sleep 5
source ./interaction/devnet.snippets.sh; play "3" $PLAYER2 0
sleep 5
source ./interaction/devnet.snippets.sh; play "3" $PLAYER3 0
sleep 5
source ./interaction/devnet.snippets.sh; play "3" $PLAYER4 0
sleep 5
source ./interaction/devnet.snippets.sh; play "3" $PLAYER5 0
sleep 5
source ./interaction/devnet.snippets.sh; play "3" $PLAYER6 0
sleep 5

#Instance 4 => 0 player

