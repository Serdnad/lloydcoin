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

        <hr />
        <input v-model="blockHash" placeholder="block hash" />

        <button @click="getBlock">Get BLOCK</button>

        <!-- <div v-if="block"> -->
        <BlockDetails asd="asdq" :block="block" />
        <!-- </div> -->
    </div>
</template>

<script lang="ts" setup>
import { Ref, ref } from "vue"
import { SHA3 } from "sha3"
import Wallet from "./lib/wallet"
import { Block, Transaction, TransactionData } from "./lib/lloydcoin"
import BlockDetails from "./components/BlockDetails.vue"

let wallet: Ref<Wallet> = ref()

let receiver = ref("")
let amount = ref("")

let blockHash = ref("")
let block = ref({} as Block)

function generateWallet() {
    wallet.value = new Wallet()
}

function getBlock() {
    let request = {
        id: "123",
        typ: "Request",
        action: "get_block",
        data: blockHash.value,
    }

    connections[0].send(JSON.stringify(request))
}

function submitTransaction() {
    let keypair = wallet.value.keyPair
    let txData: TransactionData = {
        amount: Number.parseInt(amount.value),
        receiver_key: receiver.value,
        sender_key: wallet.value.getPublicKey(),
    }
    console.log(txData)

    const hash = new SHA3(256).update(JSON.stringify(txData)).digest("binary")
    const signature = keypair.sign(hash)

    const tx: Transaction = {
        data: txData,
        signature: signature.toDER("hex"),
    }

    broadcast(tx)
}

const URL = "ws://10.8.4.155:9001"
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

        if (response.typ == "Response") {
            switch (response.action) {
                case "get_nodes":
                    let ips = JSON.parse(response["data"])

                    for (let ip of ips) {
                        if (connections.includes(ip)) {
                            connections.push(ip)
                        }
                    }
                    break

                case "get_block":
                    console.log(response)
                    block.value = JSON.parse(response.data)
                    console.log(block.value)
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

<style lang="scss">
#app {
    font-family: Avenir, Helvetica, Arial, sans-serif;
    -webkit-font-smoothing: antialiased;
    -moz-osx-font-smoothing: grayscale;
    text-align: center;
    color: #2c3e50;
    margin-top: 60px;
}
</style>
