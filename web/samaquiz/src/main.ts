import { createApp } from 'vue'
import App from './App.vue'
import router from './router'
import { API_HOST, EXEC_ENV, WEB_URL } from './util/config'

console.log('Config:', WEB_URL, API_HOST, EXEC_ENV)

const app = createApp(App).use(router)

app.mount('#app')
