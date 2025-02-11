<template>
  <Main>
    <div>
      <div
        class="mt-24 ml-auto mr-auto w-[70%] flex flex-row items-center gap-2"
      >
        <Input type="text" v-model="msg" placeholder="Enter a question..." />
        <Button @click="send_req">Submit</Button>
      </div>

      <Textarea
        class="w-[80%] ml-auto mr-auto mt-12"
        v-model="response"
        disabled
      />
    </div>
  </Main>
</template>



<script setup lang="ts">
import { ref } from "vue";
import WebSocket from "@tauri-apps/plugin-websocket";

const msg = ref("");
const response = ref("");
const ws = await WebSocket.connect("ws://127.0.0.1:8080");

ws.addListener((msg: any) => {
  console.log("Mew msg!");
  console.log(msg);
  response.value += msg.data;
});

async function send_req() {
  response.value = "";
  await ws.send(msg.value);
}
</script>