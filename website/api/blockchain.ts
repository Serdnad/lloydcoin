module Blockchain {
    const NODE_URL = ""

    // {
    //     amount: Number.parseInt(amount.value),
    //     receiver_key: receiver.value,
    //     sender_key: wallet.value.getPublicKey(),
    // }

    export interface Transaction {
        amount: number,
        receiver_key: string,
        sender_key: string
    }

    async function sendTransaction(tx: Transaction) {

    }
}

export default Blockchain