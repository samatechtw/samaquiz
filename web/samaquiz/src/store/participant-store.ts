// Quiz Participant state
import { IQuizParticipant } from '@frontend/types'
import { LocalStoragePlugin, useModule } from '@samatech/vue-store'

export interface IParticipantState {
  sessions: Record<string, IQuizParticipant | undefined>
}

const getters = (_state: IParticipantState) => ({})

const mutations = (state: IParticipantState) => ({
  setParticipant(sessionId: string, participant: IQuizParticipant) {
    state.sessions[sessionId] = participant
  },
  clearParticipant(sessionId: string) {
    state.sessions[sessionId] = {
      name: '',
      avatar: '',
      participantId: '',
    }
    delete state.sessions[sessionId]
  },
})

export const participantModule = useModule<
  IParticipantState,
  ReturnType<typeof getters>,
  ReturnType<typeof mutations>
>({
  name: 'participant-store',
  version: 2,
  stateInit: () => ({
    sessions: {},
  }),
  getters,
  mutations,
  plugins: [LocalStoragePlugin],
})
