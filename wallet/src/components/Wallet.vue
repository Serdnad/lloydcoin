<template>
  <div class="wallet">
    <div v-for="ip in ips" :key="ip">
        {{ ip }}
    </div>
    <input type="button" value="Connect to ips" v-on:click="connect"/>    
  </div>
</template>

<script>
export default {
  name: 'Wallet',
  props: {
      ips: Array
  },
  methods: {
    connect: function() {
        this.$emit('connect')
    }
  }
}
</script>

<style scoped>
h3 {
  margin: 40px 0 0;
}
ul {
  list-style-type: none;
  padding: 0;
}
li {
  display: inline-block;
  margin: 0 10px;
}
a {
  color: #42b983;
}
</style>
