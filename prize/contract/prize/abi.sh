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
sed -E "/elrond_wasm|variant_count/d" src/common_types.rs 
echo ""
