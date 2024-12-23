export interface IUpdateUserApiRequest {
  email?: string
  name?: string
  description?: string
  link?: string
  location?: string
  old_password?: string
  new_password?: string
  user_type?: string
  user_status?: string
}
