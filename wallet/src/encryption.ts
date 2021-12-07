import { ec } from 'elliptic'

// Create and initialize EC context
const EC = new ec('secp256k1')

module Encryption {
    export function genKeyPair() {
        return EC.genKeyPair()
    }
}

export default Encryption