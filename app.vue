<template>
  <SidebarProvider>
    <Sidebar>
      <SidebarHeader>
        <SidebarMenu>
          <SidebarMenuItem>
            <h2 class="mx-2 my-2">Local Deepseek</h2>
          </SidebarMenuItem>
          <SidebarMenuItem>
            <Button
              class="w-full justify-start"
              variant="outline"
              @click="createNewChat"
            >
              <PlusIcon class="mr-2 h-4 w-4" />
              New Chat
            </Button>
          </SidebarMenuItem>
        </SidebarMenu>
      </SidebarHeader>
      <SidebarContent>
        <SidebarGroup>
          <SidebarGroupLabel>Chat History</SidebarGroupLabel>
          <SidebarGroupContent>
            <SidebarMenu>
              <SidebarMenuItem v-for="chat in chatStore.chats" :key="chat.id">
                <div class="flex items-center w-full group">
                  <SidebarMenuButton
                    class="flex-grow"
                    :class="{
                      'bg-slate-100 text-slate-900':
                        chatStore.currentChatId === chat.id,
                      'hover:bg-slate-100 hover:text-slate-900':
                        chatStore.currentChatId !== chat.id,
                    }"
                    @click="chatStore.setCurrentChat(chat.id)"
                  >
                    {{ chat.title }}
                  </SidebarMenuButton>
                  <Button
                    variant="ghost"
                    size="icon"
                    class="opacity-0 group-hover:opacity-100 transition-opacity"
                    @click="chatStore.deleteChat(chat.id)"
                  >
                    <Trash2Icon class="h-4 w-4" />
                  </Button>
                </div>
              </SidebarMenuItem>
            </SidebarMenu>
          </SidebarGroupContent>
        </SidebarGroup>
      </SidebarContent>
      <SidebarRail />
    </Sidebar>
    <SidebarInset>
      <header class="flex h-16 shrink-0 items-center gap-2 border-b px-4">
        <SidebarTrigger class="-ml-1" />
        <Separator orientation="vertical" class="mr-2 h-4" />
        <Breadcrumb>
          <BreadcrumbList>
            <BreadcrumbItem class="hidden md:block">
              <BreadcrumbLink href="#">
                {{ route.name === "settings" ? "Settings" : "Chat" }}
              </BreadcrumbLink>
            </BreadcrumbItem>
          </BreadcrumbList>
        </Breadcrumb>
        <Button
          v-if="route.name !== 'settings'"
          class="ml-auto"
          variant="outline"
          size="icon"
          @click="navigateTo('/settings')"
        >
          <Settings class="w-4 h-4" />
        </Button>
        <Button
          v-else
          class="ml-auto"
          variant="outline"
          size="icon"
          @click="navigateTo('/')"
        >
          <MessageCircle class="w-4 h-4" />
        </Button>
      </header>
      <div class="flex-1 overflow-y-auto h-[calc(100vh-4rem)]">
        <div class="flex flex-col gap-4 p-4 pb-24">
          <NuxtPage />
        </div>
      </div>
    </SidebarInset>
    <Toaster class="bg-white" />
  </SidebarProvider>
</template>

<script setup lang="ts">
import { PlusIcon, Trash2Icon, Settings, MessageCircle } from "lucide-vue-next";
const chatStore = useChatStore();
const route = useRoute();

function createNewChat() {
  chatStore.createNewChat();
}
</script>

