import { defineStore } from 'pinia';

export const useUserStore = defineStore('user', {
  state: () => ({
    name: 'Guest'
  }),
  actions: {
    setName(name: string) {
      this.name = name;
    }
  }
});
