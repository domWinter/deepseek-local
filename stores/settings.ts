import { defineStore } from 'pinia'

interface SystemPrompt {
    role: 'system'
    content: string
}

export const useSettingsStore = defineStore('settings', {
    state: () => ({
        systemPrompt: {
            role: 'system' as const,
            content: 'You are a nice AI assistant. Answer precise and short.'
        } as SystemPrompt
    }),

    actions: {
        initializeStore() {
            // Load settings from localStorage
            const savedSettings = localStorage.getItem('settings')
            if (savedSettings) {
                const parsed = JSON.parse(savedSettings)
                this.systemPrompt = parsed.systemPrompt
            }
        },

        updateSystemPrompt(newPrompt: SystemPrompt) {
            this.systemPrompt = newPrompt
            this.saveToLocalStorage()
        },

        saveToLocalStorage() {
            localStorage.setItem('settings', JSON.stringify({
                systemPrompt: this.systemPrompt
            }))
        }
    }
})