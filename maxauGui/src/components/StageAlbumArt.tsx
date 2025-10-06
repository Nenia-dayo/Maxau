import React, { useState, useEffect } from 'react';
import { usePlayerStore } from '@/stores/playerStore';
import { tauriApi } from '@/lib/tauriApi';
import { arrayBufferToUrl } from '@/lib/arrayBufferToUrl';

export function StageAlbumArt() {
  const { getCurrentTrack, status, isInitialized } = usePlayerStore();
  const currentTrack = getCurrentTrack();
  const [albumArtUrl, setAlbumArtUrl] = useState<string | null>(null);
  const [isLoading, setIsLoading] = useState(false);

  useEffect(() => {
    if (!currentTrack?.id || !isInitialized) {
      setAlbumArtUrl(null);
      return;
    }

    setIsLoading(true);
    tauriApi.getAlbumArt(currentTrack.id)
      .then(buffer => {
        if (buffer) {
          const url = arrayBufferToUrl(buffer);
          setAlbumArtUrl(url);
        } else {
          setAlbumArtUrl(null);
        }
      })
      .catch(error => {
        console.error('Failed to load album art:', error);
        setAlbumArtUrl(null);
      })
      .finally(() => {
        setIsLoading(false);
      });

    // Cleanup old URL when component unmounts or track changes
    return () => {
      if (albumArtUrl) {
        URL.revokeObjectURL(albumArtUrl);
      }
    };
  }, [currentTrack?.id, isInitialized]);

  // Show skeleton loading state while not initialized or loading
  if (!isInitialized || isLoading) {
    return (
      <div className="w-48 h-48 bg-muted animate-pulse rounded-3xl shadow-lg" />
    );
  }

  if (!currentTrack || !albumArtUrl) {
    return (
      <div className="w-48 h-48 bg-muted rounded-3xl shadow-lg flex items-center justify-center">
        <div className="text-muted-foreground text-center">
          <div className="w-16 h-16 mx-auto mb-2 bg-muted-foreground/20 rounded-lg flex items-center justify-center">
            <svg className="w-8 h-8" fill="currentColor" viewBox="0 0 24 24">
              <path d="M12 3v10.55c-.59-.34-1.27-.55-2-.55-2.21 0-4 1.79-4 4s1.79 4 4 4 4-1.79 4-4V7h4V3h-6z"/>
            </svg>
          </div>
          <p className="text-sm">No album art</p>
        </div>
      </div>
    );
  }

  return (
    <div className="relative">
      <img
        src={albumArtUrl}
        alt={`${currentTrack.album} - ${currentTrack.artist}`}
        className={`w-48 h-48 object-cover rounded-3xl shadow-lg transition-transform duration-300 ${
          status === 'Playing' ? 'animate-spin-slow' : ''
        }`}
      />
      
      {/* Overlay with subtle glow effect when playing */}
      {status === 'Playing' && (
        <div className="absolute inset-0 rounded-3xl bg-accent/10 ring-2 ring-accent/30 animate-pulse" />
      )}
    </div>
  );
}