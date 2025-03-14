import { authModule } from './auth-store'
import { miscModule } from './misc-store'
import { participantModule } from './participant-store'
import { userModule } from './user-store'

declare global {
  interface Window {
    store: WebStore
  }
}

export interface WebStore {
  misc: typeof miscModule
  auth: typeof authModule
  user: typeof userModule
  participant: typeof participantModule
}

export const store: WebStore = {
  misc: miscModule,
  auth: authModule,
  user: userModule,
  participant: participantModule,
}

// Attach to window for debugging purposes
window.store = store
