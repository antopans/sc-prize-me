#!/bin/python3

import sys
from erdpy.wallet import bech32

bech32_string = sys.argv[1]
hrp, value_bytes = bech32.bech32_decode(bech32_string)
decoded_bytes = bech32.convertbits(value_bytes, 5, 8, False)
print(bytearray(decoded_bytes).hex())


