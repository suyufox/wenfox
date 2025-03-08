<script setup lang="ts">
import TheWelcome from '../components/TheWelcome.vue'

import { ref } from 'vue'
import { useApiStore } from '@/stores/api'

const username = ref('')
const apiStore = useApiStore()

async function fetchUser() {
  const result = await apiStore.fetchUser(username.value)
  alert(result)
}
</script>

<template>
  <main>
    <TheWelcome />
  </main>
  <div>
    <input v-model="username" placeholder="输入用户名">
    <button @click="fetchUser">获取用户信息</button>
  </div>
</template>
