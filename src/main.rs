use hex;
use bitcoin::consensus::encode::deserialize;
use bitcoin::Transaction;

fn main() {
    // Example Bitcoin transaction (raw hex)
    let tx_hex = "01000000000101b6e447e3730b6c22a4312c51e98a013b8e1514ebe592a75767349b659dd1eb4b0000000000ffffffff020000000000000000536a4c50000d2ab10002ce909734abc6014d89e07b7d1d5aa1d324eb6af71e2860a470d612483853e078120e105d3ea910720edbb89fc9025e3b4d8e0701e44510686281d5484fbb48444129251371047bf8ad5b5fb9010000000000160014841996f8ff255c875c4f8875a7bd036bf64209210246304302203c5ef41b9f17525714ab840dbd1716c2baae14e14db84a18716f97b5d1c3aa3c021f6be45f733d3ce5094b470385d997e797ffab976610c015833b395197be586601210380a033803cdcfae4dda162741774cbf38af31ebdd11e9bba414590d7fe36835400000000";


    // Decode the transaction from hex
    let tx_bytes = hex::decode(tx_hex).expect("Invalid hex");
    let tx: Transaction = deserialize(&tx_bytes).expect("Failed to deserialize transaction");

    // Print the transaction details
    println!("Transaction ID: {}", tx.compute_txid());
    println!("Version: {}", tx.version);
    println!("Lock Time: {}", tx.lock_time);
    println!("Input Count: {}", tx.input.len());
    println!("Output Count: {}", tx.output.len());

    // Print inputs
    for (i, input) in tx.input.iter().enumerate() {
        println!("Input {}:", i);
        println!("  Previous Output: {}", input.previous_output);
        println!("  Total Size {}", input.total_size());
    }

    // Print outputs
    for (i, output) in tx.output.iter().enumerate() {
        println!("Output {}:", i);
        println!("  Value: {} satoshis", output.value);
        println!("  Script PubKey - locking script {} ", output.script_pubkey);

    }
    println!("{}", "\n**OP_RETURN is an opcode in Bitcoin's scripting language. It marks the output as unspendable and is typically used to embed data in the blockchain. ");
}
