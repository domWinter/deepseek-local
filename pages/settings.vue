<template>
    <div class="relative flex-col items-start gap-8 flex max-h-screen">
      <form class="grid w-full items-start gap-6" @submit.prevent="saveSettings">
        <fieldset class="grid gap-6 rounded-lg border p-4">
          <legend class="-ml-1 px-1 text-sm font-medium">Messages</legend>
          <div class="grid gap-3">
            <Label for="role">Role</Label>
            <Select v-model="prompt.role" default-value="system">
              <SelectTrigger>
                <SelectValue placeholder="Select a role" />
              </SelectTrigger>
              <SelectContent class="bg-white">
                <SelectItem value="system"> System </SelectItem>
              </SelectContent>
            </Select>
          </div>
          <div class="grid gap-3">
            <Label for="content">Content</Label>
            <Textarea
              id="content"
              v-model="prompt.content"
              placeholder="You are a nice AI assistant. Answer precise and short."
              class="min-h-[7rem]"
            />
          </div>
          <div class="ml-auto">
            <Button type="submit">Save</Button>
          </div>
        </fieldset>
      </form>
    </div>
  </template>
  
  <script setup lang="ts">
  import { ref, onMounted } from 'vue'
  import { useToast } from '@/components/ui/toast/use-toast'

  const { toast } = useToast()

  const settingsStore = useSettingsStore()
  
  const prompt = ref({
      role: 'system',
      content: ''
  })
  
  onMounted(() => {
      settingsStore.initializeStore()
      prompt.value = { ...settingsStore.systemPrompt }
  })
  
  const saveSettings = () => {
      settingsStore.updateSystemPrompt(prompt.value)
      toast({
        title: 'Settings: Updated',
        description: 'The settings were updated successfully',
      });
  }
  </script>