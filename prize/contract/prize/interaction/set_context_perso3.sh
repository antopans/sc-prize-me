#!/bin/bash

# Wallets 
source ./interaction/devnet.wallets.sh

#Create 1 instance (duration: 5 hours)
source ./interaction/devnet.snippets.sh; createEgld_10 1800 $SPONSOR2
sleep 10

#Create 1 instance (duration: 5 hours)
source ./interaction/devnet.snippets.sh; createEgld_20 1800 $SPONSOR2
sleep 10

#Create 1 instance (duration: 5 hours)
source ./interaction/devnet.snippets.sh; createEgld_21 1800 $SPONSOR2
sleep 10
