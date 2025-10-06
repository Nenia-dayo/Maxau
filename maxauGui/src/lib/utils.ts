import { type ClassValue, clsx } from "clsx";
import { twMerge } from "tailwind-merge";

export function cn(...inputs: ClassValue[]) {
  return twMerge(clsx(inputs));
}

export function formatDuration(seconds: number): string {
  if (isNaN(seconds) || seconds < 0) {
    return "0:00";
  }
  const minutes = Math.floor(seconds / 60);
  const remainingSeconds = Math.floor(seconds % 60);
  return `${minutes}:${remainingSeconds.toString().padStart(2, '0')}`;
}

export function arrayBufferToUrl(buffer: Uint8Array): string | null {
  if (!buffer || buffer.length === 0) return null;
  const blob = new Blob([buffer as BlobPart], { type: 'image/jpeg' });
  return URL.createObjectURL(blob);
}
