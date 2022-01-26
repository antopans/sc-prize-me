#!/bin/bash
echo "***************"
echo "* Init"
echo "***************"
sed -n '/#\[init/p' src/lib.rs
echo ""

echo "***************"
echo "* Endpoints"
echo "***************"
sed -n '/#\[endpoint/p' src/lib.rs
echo ""

echo "***************"
echo "* View"
echo "***************"
sed -n '/#\[view/p' src/lib.rs
echo ""

echo "***************"
echo "* Detailed API"
echo "***************"
sed -n '/^    fn .*{/p' src/lib.rs
echo ""

echo "***************"
echo "* Structures"
echo "***************"
sed -E "/elrond_wasm/d" src/instance_info.rs 
sed -E "/elrond_wasm/d" src/fee_policy.rs 
echo ""

echo "***************"
echo "* Enums"
echo "***************"
sed -E "/elrond_wasm|variant_count/d" src/instance_status.rs 
echo ""
