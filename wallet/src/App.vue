<template>
    <div id="app">
        <Wallet :ips="ips" v-on:connect="connect" />
        <div>
            <input type="button" value="Generate private & public key" v-on:click="generate" />
        </div>
        <div>
            <input v-model="amount" placeholder="Transaction amount" />
            <input v-model="receiver" placeholder="Receiver" />
            <input type="button" value="Submit transaction" v-on:click="submit" />
        </div>
        <input v-model="mnemonic" placeholder="10 words passphrase" />
        <div id="keys">
            <p>Address: {{ this.address }}</p>
            <p>Public key: {{ this.publicKeyPem }}</p>
            <p>Private key: {{ this.privateKeyPem }}</p>
        </div>
    </div>
</template>

<script>
import Wallet from "./components/Wallet.vue"
import Encryption from "./encryption"
const keccak256 = require("keccak256")

//const cryptico = require("cryptico")
//const bip39 = require("bip39")
//const crypto = require("crypto")
const forge = require("node-forge")

function ip_to_url(ip) {
    return "ws://" + ip + ":9001"
}

function connect_to_ip(ip, stuff) {
    let url = ip_to_url(ip)
    console.log("the url: " + url)
    let socket = new WebSocket(url)

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
            let ips = JSON.parse(response.data)

            for (let ip of ips) {
                if (!stuff.ips.includes(ip)) {
                    stuff.ips.push(ip)
                }
            }
        }
    }

    return socket
}

function createVertebra(amount, receiver_id, sender_id) {
    return { amount, receiver_id, sender_id }
}

function broadcast(data, connections) {
    let stringify_data = JSON.stringify(data)

    for (let socket of Object.values(connections)) {
        socket.send(stringify_data)
    }
}

export default {
    name: "App",
    components: {
        Wallet,
    },
    data() {
        return {
            connections: {},
            ips: ["192.168.80.1"],
            address: "",
            publicKey: 0,
            publicKeyPem: "",
            privateKey: 0,
            privateKeyPem: "",
            mnemonic: "",
            message: "",
            amount: null,
            receiver: null,
        }
    },
    methods: {
        connect: function() {
            for (let ip of this.ips) {
                console.log("the ip: " + ip)
                let connection = connect_to_ip(ip, this)
                this.connections[ip] = connection
            }
        },
        async generate() {
            let rsa = forge.pki.rsa

            rsa.generateKeyPair({ bits: 1024, workers: -1 }, (err, keypair) => {
                Encryption.genKeyPair()

                this.privateKey = keypair.privateKey
                this.privateKeyPem = forge.pki.privateKeyToPem(this.privateKey)

                this.publicKey = keypair.publicKey
                this.publicKeyPem = forge.pki.publicKeyToPem(this.publicKey)

                console.log(this.publicKey)
                console.log(this.publicKeyPem)
                this.address = keccak256(this.publicKeyPem).toString("hex")
            })
        },
        submit() {
            let amount = 10
            let receiver = this.receiver
            let sender = this.address

            let vertebra = createVertebra(amount, receiver, sender)

            let md = forge.md.sha256.create()
            md.update(JSON.stringify(vertebra), "utf8")
            let signature = this.privateKey.sign(md)

            vertebra.signature = signature

            let data = {
                typ: "Request",
                action: "transaction",
                data: JSON.stringify(vertebra),
            }

            broadcast(data, this.connections)
        },
    },
    mount() {},
}
</script>

<style>
#app {
    font-family: Avenir, Helvetica, Arial, sans-serif;
    -webkit-font-smoothing: antialiased;
    -moz-osx-font-smoothing: grayscale;
    text-align: center;
    color: #2c3e50;
    margin-top: 60px;
    display: flex;
    flex-direction: column;
}
</style>
