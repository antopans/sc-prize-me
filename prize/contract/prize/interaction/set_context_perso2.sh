#!/bin/bash

# Wallets 
source ./interaction/devnet.wallets.sh

#Create 1 instance (duration: 5 hours)
source ./interaction/devnet.snippets.sh; createEgld_1 18000 $SPONSOR1
sleep 10

#Create 1 instance (duration: 5 hours)
source ./interaction/devnet.snippets.sh; createEgld_2 18000 $SPONSOR2
sleep 10

#Create 1 instance (duration: 5 hours)
source ./interaction/devnet.snippets.sh; createEgld_3 18000 $SPONSOR3
sleep 10

#Create 1 instance (duration: 5 hours)
source ./interaction/devnet.snippets.sh; createEgld_4 18000 $SPONSOR4
sleep 10

#Create 1 instance (duration: 5 hours)
source ./interaction/devnet.snippets.sh; createEgld_5 18000 $SPONSOR1
sleep 10