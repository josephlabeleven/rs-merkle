import {Keypair, PublicKey} from "@solana/web3.js";
import * as fs from "fs";
import BN from 'bn.js';
import {MerkleTree} from "./merkleTree";

function getRandomInt(min: number, max: number) {
    min = Math.ceil(min);
    max = Math.floor(max);
    return Math.floor(Math.random() * (max - min) + min);
}

interface WhiteListItem {
    handle: string,
    amount: number
}

function generate_white_list_json(num: number) {
    let whitelist: WhiteListItem[] = [];

    for (let i = 0; i < num; i++) {
        whitelist.push(
            {
                'handle': Keypair.generate().publicKey.toBase58(),
                'amount': getRandomInt(1, 20)
            })
    }

    fs.writeFileSync('src/whitelist-test.json', JSON.stringify(whitelist));
}

async function main() {
    generate_white_list_json(100);

    let whitelistJson = fs.readFileSync('src/whitelist-test.json');
    let whitelist: WhiteListItem[] = JSON.parse(whitelistJson.toString());

    // build merkle tree
    let leaves = []
    for (let item of whitelist) {
        leaves.push(
            Buffer.from([
                ...new PublicKey(item.handle).toBuffer(),
                ...new BN(item.amount).toArray('le', 8),
            ])
        );
    }

    let tree = new MerkleTree(leaves);

    console.log(`root hex: ${tree.getHexRoot()}`)
}

main()