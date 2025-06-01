import { IQuizSessionViewModel, QuizSessionStatus } from '@frontend/types'

const statusOrder: Record<QuizSessionStatus, number> = {
  Ready: 3,
  Active: 4,
  Complete: 2,
  Canceled: 1,
}

export const sortQuizSessions = (sessions: IQuizSessionViewModel[] | undefined) => {
  return (sessions || []).sort((a, b) => {
    if (a.status !== b.status) {
      return statusOrder[b.status] - statusOrder[a.status]
    }
    return (b.start_time ?? 0) - (a.start_time ?? 0)
  })
}
