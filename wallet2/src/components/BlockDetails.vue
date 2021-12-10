<template>
    <h1>{{ asd }}</h1>
    <div v-if="block.tx != null">
        <h3>Block Details</h3>
        <div>
            <p>Previous block: {{ block.prev_hash }}</p>
            <br />

            <p>Sender: {{ block.tx.data.sender_key }}</p>
            <p>Receiver: {{ block.tx.data.receiver_key }}</p>
            <p>Amount: {{ block.tx.data.amount }}</p>
        </div>
    </div>
</template>

<script lang="ts" setup>
const props = defineProps(["asd", "block"])
</script>

<style></style>
