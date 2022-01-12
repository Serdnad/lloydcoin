<script lang="ts">
    import Wallet from "../../api/wallet"

    let wallet: Wallet

    export default {
        data: () => ({
            wallet: null,
        }),
        mounted() {
            const privKey = localStorage.getItem("wallet-pkey")
            if (privKey != null) {
                this.wallet = new Wallet(privKey)
            }
        },
    }
</script>

<template>
    <header>
        <div class="content">
            <nav>
                <NuxtLink title="don't be shy, click me" style="cursor: nw-resize" to="/wallet">wallet</NuxtLink>
                <NuxtLink title="or click me" style="cursor: ne-resize" to="/transfer">transfer</NuxtLink>
            </nav>

            <div>
                <p v-if="wallet">{{ "0x" + wallet.getPublicKey().substring(0, 8) }}</p>
                <p v-else>no wallet :(</p>
            </div>
        </div>
    </header>
</template>

<style scoped>
    header {
        background: #222222;
        padding: 16px;
    }

    .content {
        max-width: 1080px;
        margin: auto;

        display: flex;
        justify-content: space-between;
    }

    a {
        color: white;
        text-decoration: none;
        margin: 0 16px;
        font-weight: 600;
    }

    p {
        color: white;
        margin: 0px;
    }
</style>
