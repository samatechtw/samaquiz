export interface ISessionLeader {
  name: string
  avatar: string
  points: number
}

export interface IGetSessionLeadersApiResponse {
  leaders: ISessionLeader[]
}
