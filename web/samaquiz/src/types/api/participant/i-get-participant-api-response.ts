export interface IGetParticipantApiResponse {
  id: string
  session_id: string
  user_id?: string
  name: string
  avatar: string
  created_at: Date
}
