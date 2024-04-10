import os
import json
import hashlib
from binascii import unhexlify

def get_scriptpubkey_asm(json_data):
    return json_data["vin"][0]["prevout"]["scriptpubkey_asm"]

def get_scriptsig_asm(json_data):
    return json_data["vin"][0]["scriptsig_asm"]

folder_path = "p2pkh"

for filename in os.listdir(folder_path):
    # print(filename)
    file_path = os.path.join(folder_path, filename)
    with open(file_path, "r") as file:
        try:
            json_data = json.load(file)
            scriptpubkey = get_scriptpubkey_asm(json_data).split()[3]
            scriptsig= get_scriptsig_asm(json_data).split()[3]
            # print(scriptsig, scriptpubkey)
            data1 = unhexlify(scriptsig)

            sha256_hash = hashlib.sha256(data1).hexdigest()
            # print(sha256_hash)

            data2 = unhexlify(sha256_hash)
            # data = hashlib.sha256(data2).hexdigest()
            ripemd_hash = hashlib.new('ripemd160', data2).hexdigest()
            if ripemd_hash == scriptpubkey:
                print("True")
            # print(scriptpubkey_type)
        except Exception as e:
            print(f"Error loading file {filename}: {e}")