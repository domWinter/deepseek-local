<script setup lang="ts">
import { ref } from 'vue'
import { Loader2 } from 'lucide-vue-next'

interface Props {
  chatPosition: string
  loading: boolean
}

const props = defineProps<Props>()
const emit = defineEmits<{
  'send-message': [message: string]
}>()

const msg = ref('')

function handleSubmit(event: Event) {
  event.preventDefault()
  if (!props.loading) {
    sendMessage()
  }
}

function handleClick() {
  if (!props.loading) {
    sendMessage()
  }
}

function handleKeyPress(event: KeyboardEvent) {
  if (msg.value && event.key === 'Enter' && !props.loading) {
    event.preventDefault()
    sendMessage()
  }
}

function sendMessage() {
  if (!msg.value.trim()) return
  emit('send-message', msg.value)
  msg.value = ''
}
</script>

<template>
  <div
    class="fixed bottom-0 right-0 bg-white p-4 border-t transition-all duration-200"
    :style="{ left: chatPosition, margin: 0 }"
  >
    <form @submit.prevent="handleSubmit" class="flex flex-row items-center gap-2">
      <Input 
        v-model="msg"
        type="text"
        placeholder="Enter a question..."
        @keyup="handleKeyPress"
      />
      <Button 
        type="button"
        :disabled="loading" 
        @click="handleClick"
        class="w-36"
      >
        <Loader2 
          v-show="loading" 
          class="w-4 h-4 mr-2 animate-spin" 
        />
        {{ loading ? "Loading" : "Submit" }}
      </Button>
    </form>
  </div>
</template>