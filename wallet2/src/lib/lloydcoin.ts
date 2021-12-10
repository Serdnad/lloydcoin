export interface TransactionData {
    amount: Number
    receiver_key: string
    sender_key: string
}

export interface Transaction {
    data: TransactionData
    signature: string
}

export interface Block {
    prev_hash: string
    tx: {
        data: {
            amount: number
            receiver_key: string
            sender_key: string
        }
        signature: string
    }
}
