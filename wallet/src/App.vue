<template>
  <div id="app">
    <Wallet :ips="ips" v-on:connect="connect" />
    <input type="button" value="Generate private & public key" v-on:click="generate"/>
    {{ this.message }}
    <div id="keys">
        Mnemnoic: {{ this.mnemonic }}
        Public key: {{ this.publicKey }}
        Private key: {{ this.privateKey }}
    </div>
  </div>
</template>

<script>
import Wallet from './components/Wallet.vue'
const cryptico = require("cryptico")
const bip39 = require("bip39")

function ip_to_url(ip) {
    return "ws://"+ip+":9001"
}

function connect_to_ip(ip, stuff) {
    let url = ip_to_url(ip)
    console.log("the url: " + url)
    let socket = new WebSocket(url)

    socket.onopen = () => {
        let request = JSON.stringify({
            typ: "Request",
            action: "get_nodes",
            data: null
        })
        socket.send(request)
    }

    socket.onmessage = (response_object) => {
        let response = JSON.parse(response_object.data)

        console.log(response)

        if(response.typ == "Response" && response.action == "get_nodes") {
           let ips = JSON.parse(response.data)

           for(let ip of ips) {
                if(!stuff.ips.includes(ip)) {
                    stuff.ips.push(ip)
                }
            }
        }
    }

    return socket
}

export default {
  name: 'App',
  components: {
    Wallet
  },
  data() {
    return {
      connections: {},
      ips: ["10.8.57.232"],
      publicKey: 0,
      privateKey: 0,
      mnemonic: "",
      message: "",
    }
  },
  methods: {
    connect: function() {
        for(let ip of this.ips) {
            console.log("the ip: " + ip)
            let connection = connect_to_ip(ip, this) 
            this.connections[ip] = connection
        }
    },
    async generate () {
        const mnemonic = bip39.generateMnemonic()
        this.mnemonic = mnemonic

        const seed = await bip39.mnemonicToSeed(mnemonic)
        const bits = 1024
        const privateKey = cryptico.generateRSAKey(seed, bits)
        const publicKey = cryptico.publicKeyString(privateKey)

        this.privateKey = privateKey
        this.publicKey = publicKey
    }
  }
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
}
</style>
