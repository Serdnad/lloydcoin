import { ec } from "elliptic"

const EC = new ec("secp256k1")

class Wallet {
    keyPair: ec.KeyPair

    constructor() {
        this.keyPair = EC.genKeyPair()
    }

    getPublicKey = () => this.keyPair.getPublic(true, "hex")

    getPrivateKey = () => this.keyPair.getPrivate().toString("hex")
}

export default Wallet