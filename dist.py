import os
import json
import shutil

# Function to extract scriptpubkey_type from vin
def get_scriptpubkey_type(json_data):
    return json_data.get("vin", [{}])[0].get("prevout", {}).get("scriptpubkey_type", "")

folder_path = "mempool"
sorted_files = {"p2pkh", "p2sh"}

for filename in os.listdir(folder_path):
    print(filename)
    # filename = filename.split("_")[1]
    file_path = os.path.join(folder_path, filename)
    with open(file_path, "r") as file:
        try:
            json_data = json.load(file)
            scriptpubkey_type = get_scriptpubkey_type(json_data)
            # print(scriptpubkey_type)
            sorted_files.add(scriptpubkey_type)
            shutil.copy(file_path, f"{scriptpubkey_type}/{filename}")


        except Exception as e:
            # print(f"Error loading file {filename}: {e}")
            continue