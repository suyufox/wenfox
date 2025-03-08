import { defineStore } from 'pinia'

export const useApiStore = defineStore('api', () => {
  async function fetchUser(name) {
    const response = await fetch(`/api/user/${name}`)
    return response.text()
  }

  return { fetchUser }
})
