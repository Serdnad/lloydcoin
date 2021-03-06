import pkg from 'elliptic';

const WORD_LIST = [
    "lloyd",
    "lloydie",
    "cool",
    "eom",
    "inferno",
    "tropic",
    "vi",
    "valhalla",
    "purple",
    "lsd",
    "crack",
    "fingals",
    "gold",
    "chill",
    "house",
    "lazy",
    "tennis",
    "pingpong",
    "smash",
    "couch",
    "eom",
    "flag",
    ""
]

class Wallet {
    static EC = new pkg.ec("secp256k1")

    keyPair: pkg.ec.KeyPair

    constructor(privateKey?: string) {
        console.log("OLD")
        console.log(privateKey)
        if (privateKey != undefined) {
            this.keyPair = Wallet.EC.keyFromPrivate(privateKey)
        } else {
            this.keyPair = Wallet.EC.genKeyPair()
            console.log(this.getPrivateKey())
        }
        console.log("PUBLOC", this.getPublicKey())
    }

    getPublicKey = () => this.keyPair.getPublic(true, "hex")

    getPrivateKey = () => this.keyPair.getPrivate().toString("hex")

    sign = (hash) => {
        return this.keyPair.sign(hash)
    }
}

export default Wallet
