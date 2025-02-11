<script setup lang="ts">
import { ref } from "vue";
import WebSocket from '@tauri-apps/plugin-websocket';

const msg = ref("");
const response = ref("");
const ws = await WebSocket.connect('ws://127.0.0.1:8080');

ws.addListener((msg: any) => {
    console.log("Mew msg!")
    console.log(msg)
    response.value += msg.data
});

async function send_req() {
    response.value = ""
    await ws.send(msg.value);
}

</script>

<template>
    <div class="mt-24 ml-auto mr-auto w-[70%] flex flex-row items-center gap-2">
        <input class="w-[95%]" id="chat-input" v-model="msg" placeholder="Enter a question..." /> 
        <button @click="send_req" >Submit</button>
    </div>
    

    <textarea disabled class="bg-white rounded-xl ml-auto mr-auto mt-12 w-[70%] shadow-md px-2 py-2" id="chat-response" name="response" rows="5" cols="33" v-model="response">
    </textarea>

</template>