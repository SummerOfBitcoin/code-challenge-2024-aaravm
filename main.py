file_path = "output.txt"
file_path1= "final_p2pkh.txt"

        
with open(file_path1, "r") as file: 
    for line in file:
        with open(file_path, "a") as file:
            try:
                file.write(line.strip() + "\n")
            except Exception as e:
                print(f"Error writing to file {file_path}: {e}")
