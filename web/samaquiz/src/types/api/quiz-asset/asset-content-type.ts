export enum AssetContentType {
  Jpeg = 'image/jpeg',
  Png = 'image/png',
  Webp = 'image/webp',
  Svg = 'image/svg+xml',
  Gif = 'image/gif',
  Mp4 = 'video/mp4',
}

export const ALL_CONTENT_TYPES = Object.values(AssetContentType)

export const MEDIA_CONTENT_TYPES = [
  AssetContentType.Jpeg,
  AssetContentType.Png,
  AssetContentType.Webp,
  AssetContentType.Svg,
  AssetContentType.Gif,
  AssetContentType.Mp4,
]

export const CONTENT_TYPE_EXTS = {
  [AssetContentType.Jpeg]: 'jpg',
  [AssetContentType.Png]: 'png',
  [AssetContentType.Webp]: 'webp',
  [AssetContentType.Svg]: 'svg',
  [AssetContentType.Gif]: 'gif',
  [AssetContentType.Mp4]: 'mp4',
}

export const extFromContentType = (contentType: string): string | undefined => {
  return {
    'image/jpeg': 'jpg',
    'image/png': 'png',
    'image/webp': 'webp',
    'image/svg+xml': 'svg',
    'image/gif': 'gif',
    'video/mp4': 'mp4',
  }[contentType]
}
