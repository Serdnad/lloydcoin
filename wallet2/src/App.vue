<template>
    <div>
        <h1>Lloyd Coin Wallet</h1>

        <button @click="generateWallet">Generate Wallet</button>

        <div>
            <p>Public Key: {{ wallet?.getPublicKey() }}</p>
            <p>Private Key: {{ wallet?.getPrivateKey() }}</p>
        </div>

        <hr />

        <div class="form">
            <input v-model="receiver" placeholder="receiver" />
            <input v-model="amount" placeholder="amount" />
        </div>

        <button @click="submitTransaction">Submit Transaction</button>
    </div>
</template>

<script lang="ts" setup>
import { Ref, ref } from "vue"
import { SHA3 } from "sha3"
import Wallet from "./lib/wallet"

let wallet: Ref<Wallet> = ref()

let receiver = ref("")
let amount = ref("")

function generateWallet() {
    wallet.value = new Wallet()
}

function submitTransaction() {
    let keypair = wallet.value.keyPair
    let transaction = {
        amount: Number.parseInt(amount.value),
        receiver_key: receiver.value,
        sender_key: wallet.value.getPublicKey(),
    }
    console.log(transaction)

    const hash = new SHA3(256).update(JSON.stringify(transaction)).digest("binary")
    console.log(hash)
    const signature = keypair.sign(hash)
    transaction.signature = signature.toDER("hex")

    console.log(transaction)
    console.log(signature.r.toString("hex"))
    console.log(signature.s.toString("hex"))
    console.log(signature.recoveryParam)

    let q = signature.r.toString("hex") + signature.s.toString("hex") + signature.recoveryParam
    console.log(q)

    broadcast(transaction)
}

const URL = "ws://192.168.80.1:9001"
let connections: WebSocket[] = []
function initWebsocket() {
    let socket = new WebSocket(URL)

    socket.onopen = () => {
        let request = JSON.stringify({
            typ: "Request",
            action: "get_nodes",
            data: null,
        })
        socket.send(request)
    }

    socket.onmessage = (response_object) => {
        let response = JSON.parse(response_object.data)

        console.log(response)

        if (response.typ == "Response" && response.action == "get_nodes") {
            let ips = JSON.parse(response["data"])

            for (let ip of ips) {
                if (connections.includes(ip)) {
                    connections.push(ip)
                }
            }
        }
    }

    connections = [socket]
    return socket
}

async function broadcast(transaction) {
    const data = {
        typ: "Request",
        action: "transaction",
        data: JSON.stringify(transaction),
    }

    console.log(JSON.stringify(data))
    connections[0].send(JSON.stringify(data))
}

initWebsocket()
</script>

<style lang="scss" scoped>
#app {
    font-family: Avenir, Helvetica, Arial, sans-serif;
    -webkit-font-smoothing: antialiased;
    -moz-osx-font-smoothing: grayscale;
    text-align: center;
    color: #2c3e50;
    margin-top: 60px;
}
</style>
