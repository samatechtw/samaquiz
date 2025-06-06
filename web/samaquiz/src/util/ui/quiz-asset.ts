import { IQuestionViewModel, IQuizViewModel } from '@frontend/types'

export const quizBackgroundUrl = (
  quiz: IQuizViewModel | undefined,
): string | undefined => {
  const bg = quiz?.intro_background_url
  if (!bg) {
    return undefined
  }
  return `url(${bg})`
}

export const questionAssetUrl = (
  question: IQuestionViewModel | undefined,
): string | undefined => {
  const url = question?.asset_url
  if (!url) {
    return undefined
  }
  return `url(${url})`
}
