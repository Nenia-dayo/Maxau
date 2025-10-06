export function arrayBufferToUrl(buffer: ArrayBuffer | Uint8Array, mimeType: string = 'image/jpeg'): string {
  const uint8Array = buffer instanceof Uint8Array ? buffer : new Uint8Array(buffer);
  const blob = new Blob([uint8Array as any], { type: mimeType });
  return URL.createObjectURL(blob);
}
