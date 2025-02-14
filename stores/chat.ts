// stores/chat.ts
import { defineStore } from 'pinia'

interface ChatMessage {
    user: string
    assistant: string
}

interface Chat {
    id: string
    title: string
    messages: ChatMessage[]
    createdAt: Date
}

export const useChatStore = defineStore('chat', {
    state: () => ({
        chats: [] as Chat[],
        currentChatId: null as string | null
    }),

    getters: {
        currentChat: (state) =>
            state.chats.find(chat => chat.id === state.currentChatId),

        currentMessages: (state) =>
            state.chats.find(chat => chat.id === state.currentChatId)?.messages || []
    },

    actions: {
        initializeStore() {
            // Load chats from localStorage
            const savedChats = localStorage.getItem('chats')
            if (savedChats) {
                this.chats = JSON.parse(savedChats)
            }

            // Create initial chat if none exists
            if (this.chats.length === 0) {
                this.createNewChat()
            }
        },

        createNewChat() {
            const newChat: Chat = {
                id: Date.now().toString(),
                title: 'New Chat',
                messages: [],
                createdAt: new Date()
            }
            this.chats.unshift(newChat)
            this.currentChatId = newChat.id
            this.saveToLocalStorage()
            return newChat
        },

        deleteChat(chatId: string) {
            const index = this.chats.findIndex(chat => chat.id === chatId)
            if (index !== -1) {
                this.chats.splice(index, 1)

                // If we deleted the current chat, select another one
                if (this.currentChatId === chatId) {
                    this.currentChatId = this.chats[0]?.id || null
                }

                this.saveToLocalStorage()
            }
        },

        updateChatMessages(chatId: string, messages: ChatMessage[]) {
            const chat = this.chats.find(c => c.id === chatId)
            if (chat) {
                chat.messages = messages
                // Update chat title based on first message
                chat.title = messages[0]?.user.slice(0, 30) || 'New Chat'
                this.saveToLocalStorage()
            }
        },

        setCurrentChat(chatId: string) {
            this.currentChatId = chatId
        },

        saveToLocalStorage() {
            localStorage.setItem('chats', JSON.stringify(this.chats))
        }
    }
})