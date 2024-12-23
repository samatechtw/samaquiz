export interface IDropdownMenuItem {
  label?: string
  labelKey?: string
  class?: string
  to?: unknown
  linkTarget?: string
  loading?: boolean
  click?: () => void
}
