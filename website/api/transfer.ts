import Wallet from "./wallet"
import { SHA3 } from "sha3"

module Transfer {
    export async function transfer(amount: bigint, to: string) {
        // tODO: MAKE SINGLE WALLET SINGLETON GLOBALLY ACCESSIBLE
        const privKey = localStorage.getItem("wallet-pkey")
        let wallet: Wallet
        if (privKey != null) {
            wallet = new Wallet(privKey)
        } else {
            console.log("UH OH")
        }

        const tx: any = {
            amount: parseInt(amount.toString()),
            receiver_key: to,
            sender_key: wallet.getPublicKey(),
        }

        const hash = new SHA3(256).update(JSON.stringify(tx)).digest("binary")
        const signature = wallet.sign(hash)
        console.log(signature)

        const txSigned: any = {
            data: tx,
            signature: signature.toDER("hex"),
        }

        const payload = {
            typ: "Request",
            action: "transaction",
            data: JSON.stringify(txSigned),
        }

        const socket = connect_to_ip("127.0.0.1", {})

        setTimeout(() => {
            console.log(socket)

            const r = socket.send(JSON.stringify(payload))
            console.log(r)
        }, 2000)
    }
}

export default Transfer

function ip_to_ws(ip) {
    return "ws://" + ip + ":9001"
}

function connect_to_ip(ip, stuff) {
    let url = ip_to_ws(ip)
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
    // let stringify_data = JSON.stringify(data)
    // for (let socket of Object.values(connections)) {
    //     socket.send(stringify_data)
    // }
}
