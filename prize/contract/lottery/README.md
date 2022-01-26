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


