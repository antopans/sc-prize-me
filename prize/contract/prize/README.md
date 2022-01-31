# Build contract
erdpy contract build

# Clean 
erdpy contract clean

# Deploy contract 
source ./interaction/devnet.snippets.sh; deploy

# Upgrade contract
source ./interaction/devnet.snippets.sh; upgrade

# Interact with the contract : <api>        
# see functions in ./interaction/devnet.snippets.sh
source ./interaction/devnet.snippets.sh; <api>

# NFT prize creation
source ./interaction/devnet.snippets.sh; createNft 256 $SPONSOR1 MNT-8e7d64 1 1

# ESDT prize creation 
source ./interaction/devnet.snippets.sh; createEsdt 256 $SPONSOR1 EOL-12652c 1000






