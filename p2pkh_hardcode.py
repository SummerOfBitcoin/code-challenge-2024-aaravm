import json

# Load the JSON data from the file


def hex_to_little_endian(hex_number):
    # Convert hex string to bytes
    hex_bytes = bytes.fromhex(hex_number)

    # Reverse the byte order (little endian)
    little_endian_bytes = hex_bytes[::-1]

    # Convert bytes back to hex string
    little_endian_hex = little_endian_bytes.hex()

    return little_endian_hex


with open("mempool/0ac528562a1626863c0cb912eb725530c54e786e6485380c16633e4b9bce1720.json") as f:
    data = json.load(f)

raw_transaction = ""

# Example usage
version = hex(data["version"])[2:]
version = version.zfill(8)
raw_transaction = raw_transaction + (hex_to_little_endian(version))

input_count = hex(len(data["vin"]))[2:]
input_count = input_count.zfill(2)
raw_transaction = raw_transaction + hex_to_little_endian(input_count)

for i in range(len(data["vin"])):
    prev_txid = data["vin"][0]["txid"]
    raw_transaction = raw_transaction + hex_to_little_endian(prev_txid)

    prev_index = data["vin"][0]["vout"]
    prev_index = hex(prev_index)[2:]
    prev_index = prev_index.zfill(8)
    raw_transaction = raw_transaction + hex_to_little_endian(prev_index)

    # script_sig_length = len(data["vin"][0]["scriptsig"])
    # script_sig_length = int(script_sig_length / 2)
    # script_sig_length = hex(script_sig_length)[2:]
    # script_sig_length = script_sig_length.zfill(2)
    # raw_transaction = raw_transaction + script_sig_length

    script_pubkey_length = len(data["vin"][0]["prevout"]["scriptpubkey"])
    script_pubkey_length = int(script_pubkey_length / 2)
    script_pubkey_length = hex(script_pubkey_length)[2:]
    script_pubkey_length = script_pubkey_length.zfill(2)
    raw_transaction = raw_transaction + script_pubkey_length

    script_pubkey = data["vin"][0]["prevout"]["scriptpubkey"]
    raw_transaction = raw_transaction + script_pubkey

    raw_transaction = raw_transaction + "ffffffff"


output_count = hex(len(data["vout"]))[2:]
output_count = output_count.zfill(2)
raw_transaction = raw_transaction + hex_to_little_endian(output_count)

for i in range(len(data["vout"])):
    value = data["vout"][0]["value"]
    value = hex(value)[2:]
    value = value.zfill(16)
    raw_transaction = raw_transaction + hex_to_little_endian(value)

    script_length = len(data["vout"][0]["scriptpubkey"])
    script_length = int(script_length / 2)
    script_length = hex(script_length)[2:]
    script_length = script_length.zfill(2)
    raw_transaction = raw_transaction + script_length

    script_pubkey = data["vout"][0]["scriptpubkey"]
    raw_transaction = raw_transaction + script_pubkey

locktime = hex(data["locktime"])[2:]
locktime = locktime.zfill(8)
raw_transaction = raw_transaction + hex_to_little_endian(locktime)

sighash_all= "01000000"
raw_transaction = raw_transaction + sighash_all

# print(raw_transaction)

import hashlib
import ecdsa
from binascii import unhexlify


signature=data["vin"][0]["scriptsig_asm"].split()[1]
signature=signature[:-2]
pubKey=data["vin"][0]["scriptsig_asm"].split()[3]

data1 = unhexlify(raw_transaction)
sha256_hash1 = hashlib.sha256(data1).hexdigest()

data2 = unhexlify(sha256_hash1)
sha256_hash = hashlib.sha256(data2).hexdigest()
