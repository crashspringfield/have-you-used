use on_trees_lib::*;

fn main() {
    let difficulty = 0x000fffffffffffffffffffffffffffff;

    // add genesis block
    let mut genesis_block = Block::new(
        0,
        now(),
        vec![0; 32],
        vec![
            Transaction {
                inputs: vec![],
                outputs: vec![
                    transaction::Output {
                        to_addr: "Treebeard".to_owned(),
                        value: 50
                    },
                    transaction::Output {
                        to_addr: "Finglas".to_owned(),
                        value: 7
                    },
                ],
            }
        ],
        difficulty
    );

    genesis_block.mine();
    println!("Mined genesis block {:?}", &genesis_block);

    let mut last_hash = genesis_block.hash.clone();
    let mut blockchain = Blockchain::new();
    blockchain.update_with_block(genesis_block).expect("Failed to add genesis block");

    // add another block
    let mut block2 = Block::new(
        1,
        now(),
        last_hash,
        vec![
            Transaction {
                inputs: vec![ ],
                outputs: vec![
                    transaction::Output {
                        to_addr: "Fladrif".to_owned(),
                        value: 200
                    },
                ],
            },
            Transaction {
                inputs: vec![
                    blockchain.blocks[0].transactions[0].outputs[0].clone(),
                ],
                outputs: vec![
                    transaction::Output {
                        to_addr: "Treebeard".to_owned(),
                        value: 36
                    },
                    transaction::Output {
                        to_addr: "Finglas".to_owned(),
                        value: 12
                    },
                ],
            }
        ],
        difficulty
    );

    block2.mine();
    println!("Mined next block {:?}", &block2);

    last_hash = block2.hash.clone();
    blockchain.update_with_block(block2).expect("Failed to add next block");

}
