import { ec } from "elliptic"

const EC = new ec("secp256k1")

class Wallet {
    keyPair: ec.KeyPair

    constructor(privateKey) {
        console.log(privateKey)
        if(privateKey == undefined) {
            this.keyPair = EC.genKeyPair()
        }
        else {
            this.keyPair = EC.keyFromPrivate(privateKey)
        }
    }

    getPublicKey = () => this.keyPair.getPublic(true, "hex")

    getPrivateKey = () => this.keyPair.getPrivate().toString("hex")
}

export default Wallet
