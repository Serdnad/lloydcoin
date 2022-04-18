<script lang="ts">
    // import Transfer from "../../api/transfer"

    // let amount, receiver

    // async function transfer() {
    //     Transfer.transfer(amount, receiver)
    // }

    import Transfer from "../../api/transfer"

    export default {
        data() {
            return {
                amount: 0,
                recipient: "",
                ips: [],
            }
        },
        methods: {
            transfer() {
                Transfer.transfer(this.amount, this.recipient)
            },
        },
    }
</script>

<template>
    <div class="content">
        <div class="container">
            <div class="input-row">
                <input v-model="amount" placeholder="how much" />
            </div>

            <div class="input-row">
                <input v-model="recipient" placeholder="to whom" />
            </div>

            <button v-on:click="transfer">transfer</button>
        </div>
    </div>
</template>

<style scoped>
    .input-row {
        margin: 16px 0;
    }

    input {
        padding: 8px;

        border-radius: 8px;
        border: 2px solid black;
    }
</style>
