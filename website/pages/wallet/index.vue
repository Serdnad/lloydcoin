<script lang="ts">
    import Wallet from "../../api/wallet"

    export default {
        methods: {
            createWallet() {
                const wallet = new Wallet()
                console.log(wallet)
                localStorage.setItem("wallet-pkey", wallet.getPrivateKey())
                location.reload()
            },
        },
    }
</script>

<template>
    <div>
        <div class="container">
            <h1>lloydcoin wallet</h1>

            <button v-on:click="createWallet">create wallet</button>
        </div>
    </div>
</template>

<style scoped>
    h1 {
        margin-top: 0;
    }
</style>
