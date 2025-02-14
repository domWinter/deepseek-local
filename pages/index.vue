<script setup lang="ts">
import { ref, computed, inject, onUnmounted } from "vue";
import { useMediaQuery } from "@vueuse/core";
import WebSocket from "@tauri-apps/plugin-websocket";
import { SIDEBAR_STATE_KEY } from "../components/ui/sidebar/utils";

// State
const loading = ref(false);

// Chat store
const chatStore = useChatStore();
const messages = computed(() => chatStore.currentMessages);
const settingsStore = useSettingsStore();

// Initialize store if not already
settingsStore.initializeStore();
chatStore.initializeStore();

let ws = undefined;

onMounted(async () => {
  // Websocket setup
  ws = await WebSocket.connect("ws://127.0.0.1:8080");
  // Websocket listener
  ws.addListener((msg: any) => {
    if (!chatStore.currentChatId) return;
    if (msg.data === "<END>") {
      loading.value = false;
    } else {
      const newMessages = [...messages.value];
      const lastMessage = newMessages[newMessages.length - 1];
      if (lastMessage) {
        if (typeof msg.data === 'string') {
          const formattedData = msg.data.replace(/\\n/g, '\n')
          lastMessage.assistant += formattedData;
          chatStore.updateChatMessages(chatStore.currentChatId, newMessages);
        }
      }
    }
  });
});

// Sidebar and responsive state
const isMobile = useMediaQuery("(max-width: 768px)");
const sidebarContext = inject(SIDEBAR_STATE_KEY);
const state = computed(() => sidebarContext?.state.value);

// Computed
const chatPosition = computed(() => {
  if (isMobile.value || state.value === "collapsed") {
    return "calc(var(--sidebar-width-icon) - 3rem)";
  }
  return "var(--sidebar-width)";
});

async function send_req(message: string) {
  if (!message.trim()) return;

  // If no chat is selected, create a new one
  if (!chatStore.currentChatId) {
    chatStore.createNewChat();
  }

  loading.value = true;
  const newMessages = [...messages.value, { user: message, assistant: "" }];
  chatStore.updateChatMessages(chatStore.currentChatId!, newMessages);

  // Convert messages to the format expected by the backend
  const messageHistory = newMessages.flatMap((msg) => [
    ("user", ["user", msg.user]),
    ...(msg.assistant ? [("assistant", ["assistant", msg.assistant])] : []),
  ]);

  let req = JSON.stringify({
    messages: messageHistory.slice(-6),
    system_prompt:
      "You are an AI assistant helping the user and always answering in german!",
  });

  // Send as JSON
  await ws.send(req);
}

onDeactivated(() =>{
  alert("asd")
})

// Cleanup websocket on component unmount
onUnmounted(() => {
  ws.disconnect();
});
</script>

<template>
  <div class="h-full relative">
    <!-- Messages -->
    <ChatMessages :messages="messages" />

    <ChatInputBar
      :chat-position="chatPosition"
      :loading="loading"
      @send-message="send_req"
    />
  </div>
</template>