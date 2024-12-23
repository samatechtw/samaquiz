import { UserType } from '@frontend/types'
import jwt from 'jsonwebtoken'
import { testConfig } from '../test.config'

export const adminAuthHeader = (adminId?: string) => {
  const id = adminId ?? '083fb9af-a2fd-41b8-bfa2-4747cc87b604'
  const authToken = generateAuthToken(id, UserType.Admin)
  return `Bearer ${authToken}`
}

export const userAuthHeader = (userId: string) => {
  const authToken = generateAuthToken(userId, UserType.User)
  return `Bearer ${authToken}`
}

export const generateAuthToken = (
  userId: string,
  userType: UserType,
  exp?: string,
): string => {
  const secret = Buffer.from(testConfig.get('appAuthSecret'), 'base64')
  const tokenExpiresIn = testConfig.get('authExpiresIn')
  const token = jwt.sign({ sub: userId, user_type: userType }, secret, {
    expiresIn: exp ?? tokenExpiresIn,
  })
  return token
}

export const generateConfirmToken = (userId: string, exp?: string): string => {
  const confirmSecret = Buffer.from(testConfig.get('confirmSharedSecret'), 'base64')
  return jwt.sign({ sub: userId }, confirmSecret, { expiresIn: exp ?? '1d' })
}

export const expiredAdminToken =
  'Bearer eyJ0eXAiOiJKV1QiLCJhbGciOiJSUzI1NiJ9.eyJzdWIiOiJiOGQ0ODQzZS00YjgzLTQzNDAtOTEwNC01YjIyNWFlNTUxZDUiLCJ1c2VyX3R5cGUiOiJBZG1pbiIsImV4cCI6MTY5MTExNDc5Mn0.HxbbyVUmzMdrxLDudcpUrtdmkazOLNU0tUMIxYntfntkjoScFNiHHOfS8AOlukapP5u3-1Ep-XrwrlvqhERv7_XiwSwFlqfskc8GRwn-AZNk_PR6Ag8A36g9Pm-5GOM4HJyruF6mWiXuFUdGWBSYq7Mv7KG5sWFW81ZiBLq1TUeAwTe82tYGtb_ZA-ydPeM_0ChQPEXKTPEzvUW5JkfGZ7gxGc255tOLrKDE6NimPsXGwRQkHecY9w7EMGnpJbfsGQThzI9dtpfyxwUtycJo7TtLjiwAH8iHahBGqWDZl-1k7YcvB0vDdDAG051xOuH_eLxEEvYsnmLRNRZzj2JzBQeGevwq6VLL-LvvQXii8DZemGBQE9Fq8_ugSbCpDmS6lmzfMwuMK_yYKYTFPhomoxeK8q6arzPtwTvoWthQLUsBJYSgOvhrAQTNz799xspYgEd61eMOYNvFhPaMXWP_L-BMkFm-i9kZlQkvADFaDg96Y1m6l0D0mQiWFaKhhFZH-z6AzSaQuiGhwh3lXBAWg-fgdpLBNZTzsCmvnf3Rljx7cgI_hcNmT9RQpnF62Nyg2eNFkpoLPVLrOVOTZCBjmbnAAeK_pdfRukuX167Inuf3ul8w4tvsh5wDRuTugnwSP16Mma3ZupGtAOaFstniP0OeB7pjQoU1NTx6GejLVk8'

export const expiredUser1Token =
  'Bearer eyJ0eXAiOiJKV1QiLCJhbGciOiJSUzI1NiJ9.eyJzdWIiOiI5MDNiM2MyOC1kZWFhLTQ1ZGMtYTQzZi01MTFmZTk2NWQzNGUiLCJ1c2VyX3R5cGUiOiJPd25lciIsImV4cCI6MTY5NDMyNjEzNX0.G3bbXGcMuI_rXTSCFLm1GEj06S8b1yt47HrzL9XNlh4l9ih1w9-yPJQQfKw5oxy5G5vmELbgQ5YFZz94Vn0QPGEZtfz1Pgh6sz6tbh5xLPFBJlvdLH5z9hDQ6OsLb55GhFOf57Ui4ADizcD7qSmJ1fWVSjdmx7bubGnX_DFtB6o0apEMCK3OLyKMCLqKFD7tOwTpathvJhbTBpfjelg8kUV3NILYiDnAotHuBj2vk7X5SSmX1KTob2AM0LZ6lPmeRsJWeKAkjJ1xG6CTr3MTOVg85tsLholkyvbG5ENLIWRc6IvMXZ0FUIsGkWMHotZ_vQqDoKiVVco_QTrPFSmLtGkdTgCZbOWF0uQ8Q__sKyzQ8C_1vS3qJQXGFQ7ME-ELlPhq902OcJ9kCaSyv_dgsi40H9sgOkFtxm2-AvR_vJKAL3XJH1OLmt0FjoAoF8R-iGpXzIm6nIwOGQGaMEem6uK6hcJ1cj_kQqP7UAz45FOTYzyYfh1coslm6iPYFiIzct8XwE5p_FqhivgoMAVinTgC2VVecinvybINAwSVwgx-T-SuGxxP8IhGFF_E-5ivgEYSYapS5d-GlKufVXclyxlmhk6N6MZg801k2sokBKgYPX9hr987hLLrOS_KxK_0UlKuWFDFnYkw4DMn9s1yt-E7AUE4rMjmlr9oEJShsEA'
