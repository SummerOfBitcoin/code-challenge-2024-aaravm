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

````
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
````
<br>
      Then, I, using the public key and signature from the transaction, pass them through Secp256k1 library.
````
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
   ````


<h1>Results and Performance:</h1> Present the results of your solution, and analyze the efficiency of your solution.
<h1>Conclusion: </h1>Discuss any insights gained from solving the problem, and outline potential areas for future improvement or research. Include a list of references or resources consulted during the problem-solving process.
