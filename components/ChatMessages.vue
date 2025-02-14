<script setup lang="ts">
interface Message {
  user: string
  assistant?: string
}

interface Props {
  messages: Message[]
}

const props = defineProps<Props>()
</script>

<template>
  <div class="flex flex-col gap-2 pb-20">
    <div v-for="(msg, index) in messages" :key="index">
      <div 
        class="w-[70%] max-w-[100%] border border-gray-300 bg-gray-100 rounded-md px-4 py-4 shadow-md"
      >
        {{ msg.user }}
      </div>
      <div 
        v-if="msg.assistant"
        class="max-w-[100%] border border-gray-300 mt-2 rounded-md px-4 py-4 shadow-md whitespace-pre-line"
      >
        <MDC :value="msg.assistant"></MDC>
    </div>
    </div>
  </div>
</template>