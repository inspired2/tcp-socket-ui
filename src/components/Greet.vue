<script setup>
import { ref } from "vue";
import { invoke } from "@tauri-apps/api/tauri";

const greetMsg = ref("");
const address = ref("127.0.0.1:8282");

async function turn_on() {
  greetMsg.value = await invoke("turn_on");
}
async function turn_off() {
  greetMsg.value = await invoke("turn_off");
}
async function get_state() {
  greetMsg.value = await invoke("status");
}
async function connect() {
  greetMsg.value = await invoke("connect_client", { addr: address.value});
}
</script>

<template>
  <div class="card">
    <input id="greet-input" v-model="address" placeholder="Enter Tcp Smart Socket Address" />
    <button type="button" @click="connect()">Connect</button>
    <button type="button" @click="turn_on()">Turn On</button>
    <button type="button" @click="turn_off()">Turn Off</button>
    <button type="button" @click="get_state()">Get State</button>

  </div>

  <p>{{ greetMsg }}</p>
</template>
