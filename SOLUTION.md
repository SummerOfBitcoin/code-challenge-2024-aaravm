<h1>Introduction</h1>
In this challenge, I was tasked with the simulation of mining process of a block, which includes validating and including transactions from a given set of transactions. The repository contains a folder mempool which contains JSON files. These files represent individual transactions, some of which may be invalid. My goal is to successfully mine a block by including only the valid transactions.
<h1>High level overview</h1>
I have done this project, mainly by taking of learnmeabitcoin.com. I was able to grasp the basics from there, like what is a raw transaction, the different types of transactions like legacy, segwit and taproot. There are a lot of different fields, and I figured out that to create a valid block, I will first need to verify the transactions, and if valid, create a txid out of it and add it to the block.
<h1>Design Approach: </h1> In essence, there are 3 components of a Block:
<ul>
  <li><b> Block Header:</b>
    The block header is a small amount of metadata the summarizes all the data inside the block. It contains six different fields (version, previous block, merkle root, time, bits, nonce) This is the first line of the block. 
  </li>
  <li> <b> Coinbase Transaction:</b> 
    A coinbase transaction is the first transaction in a block. It's placed there by the me so that when I construct the candidate block I can claim the block reward (block subsidy + fees) if I am successful in mining the block. It's a special type of transaction that has a single blank input. So in other words, the outputs from coinbase transactions are the source of new bitcoins. This is the second line of the block. 
  </li>
  <li><b> Transactions:</b> 
    I have made list of transactions that I have validated and filled the candidate block with the highest-fee transactions to maximize the amount I can claim from the block reward. They are in the block from the 3rd line onwards, till the end of the file, with one txid in each line.
 </li>
</ul>
<h1>Implementation Details: </h1>

![home](https://github.com/SummerOfBitcoin/code-challenge-2024-aaravm/assets/32593731/07705727-6d9e-4ddc-8a65-9260c7d9de15)

This is the main structure of the program. 
There are 2 major folders: create_transactions and create_txid.
<ul>
  <li>
    <b>create_transactions:</b> This folder has 2 files: p2pkh.rs and p2wpkh.rs. This is responsible for creating a raw transaction from each transaction in the mempool for p2pkh and p2wpkh types respectively, which can be passed into the function Secp256k1::new().verify(&message, &signature, &pubkey), which can verify whether the transaction is correct or not using secp256k1 library
  </li>
  <li>
    <b>create_txid:</b> This folder has 2 files: p2pkh.rs and p2wpkh.rs. They is responsible for creating a txid from the transaction. A particular transaction is given to this function only when is verified, so that it can be mined into the block
    <br>
    It also has w_p2wpkh.rs file, which is used to create the wtxid of the transaction. It is required when we need to create the coinbase transaction. Note that the wtxid and txid for a p2pkh is the same
    <br>
    coinbase.rs and header.rs are used to create the coinbase transaction and the header of the block respectively.
  </li>
</ul>
Here is what the main.rs file does:

Firstly, for all files in the mempool, I put a loop through them, and in each loop, firstly I check the type of inputs. If all inputs are p2pkh or p2wpkh, I create a raw transaction according to its type:

```
    for i in 0..input_count {
        let script_sigtype = data["vin"][i]["prevout"]["scriptpubkey_type"].as_str().unwrap(); 
        let hex = if script_sigtype == "p2pkh" {
            create_transaction::p2pkh::create_transaction_p2pkh(data.clone(), i)
            } else if script_sigtype == "v0_p2wpkh" {
                create_transaction::p2wpkh::create_transaction_p2wpkh(data.clone(), i)
            } else {
                flag = true;
                break;
        };
```
<br>
      Then, I, using the public key and signature from the transaction, pass them through Secp256k1 library.
<br>

```
   let secp = Secp256k1::new();
   match secp.verify(&message, &signature, &pubkey) {
                  Ok(_) => {
                      println!("Signature is valid!");
                      continue;
                  },
                  Err(Error::IncorrectSignature) =>
                  { 
                      println!("Signature is invalid!");
                      flag = true;
                      break;
                  },
                  _ => println!("Failed to verify signature!"),
              }
   ```
 
   Then, if the transaction comes out to be valid, I make the txids and wtxids, and store them in a text file. 
   After looping through all the transactions, I make a merkle root from the wtxids to create the coinbase transaction
     <br>
```
 let contents = fs::read_to_string(file_path)
    .expect("Should have been able to read the file");
    
    let txids: Vec<String> = contents.lines().map(String::from).collect();

    let mut txids: Vec<String> = txids
        .iter()
        .map(|x| x.chars().collect::<Vec<char>>().chunks(2).map(|c| c.iter().collect::<String>()).collect::<Vec<String>>().iter().rev().map(|s| s.to_string()).collect::<String>())
        .collect();

    let result = merkle_root::merkleroot(&mut txids);
  ```

Through the generated result, I then create the coinbase transaction, and add it to the block.
Then I create the merkle root of the txids, and use that to generate the header.
For that, I add the necessary parameters, and for the nonce, I start from 1, and keep incrementing the nonce by 1 till I get below the target difficulty.
```
loop {
        // Hash the block header
        let mut attempt = header.clone();
        attempt.extend_from_slice(&field(nonce, 4));

        let result = hash256(&attempt);
        let result_reversed = reverse_bytes(&result);
        
        // End if we get a block hash below the target
        if u256_from_bytes_be(&result_reversed) < u256_from_bytes_be(&target) {
            println!("{}",nonce);
            println!("{}",hex::encode(&attempt));

            break;
        }

        // Increment the nonce and try again
        nonce += 1;
    }
  ```

<h1>Results and Performance:</h1> I am getting a good score of around 95/100, which indicates that I am able to use the block space efficiently. I think the code can be improved by adding support for other types of transactions since I only implement p2pkh and p2wpkh. I can also add support for when there are different kinds of transactions in the input.
<h1>Conclusion: </h1> I found this task very interesting, and I feel there are a lot of future potential in improving the mining procedure of the bitcoin. for example:
<ol>
  <li> I feel that we can automate some part of the block, such that there is no need to include them in the block, and the system automatically does it for you, like adding the UNIX time in the block header</li>
  <li>I honestly don't know why it is required that there are 2 txids, one with the raw transaction, and one generated later while making the final block</li>
  <li>I think finding the nonce is a waste of computation power, and there has to be a better way of using this power, like a committee can be formed like the bitcoin maintainers to discuss potential ways we can use the computation power for the good humanity, and the compute power being used towards that problem instead.. </li>
</ol>
<br>
<b>References:</b>
<ul>
  <li>
    BIP 143: https://github.com/bitcoin/bips/blob/master/bip-0143.mediawiki
  </li>
  <li>
    Learn me a bitcoin https://learnmeabitcoin.com/technical/block/
  </li>
  
</ul>
