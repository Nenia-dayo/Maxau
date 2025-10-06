import { Play, Pause, SkipBack, SkipForward } from '@phosphor-icons/react';
import { Button } from '@/components/ui/button';
import { usePlayerStore } from '@/stores/playerStore';
import { tauriApi } from '@/lib/tauriApi';

export function PlaybackControls() {
  const { status, getCurrentTrack, playlist, currentTrackIndex, isInitialized } = usePlayerStore();
  const currentTrack = getCurrentTrack();
  
  const handlePlayPause = async () => {
    if (!currentTrack) {
      // プレイリストの最初の曲を再生
      if (playlist.length > 0) await tauriApi.play(playlist[0].id);
      return;
    }
    
    if (status === 'Playing') {
      await tauriApi.pause();
    } else {
      await tauriApi.resume();
    }
  };
  
  const handlePrevious = async () => {
    if (currentTrackIndex === null || currentTrackIndex <= 0) return;
    const prevTrack = playlist[currentTrackIndex - 1];
    if (prevTrack) {
      await tauriApi.play(prevTrack.id);
    }
  };
  
  const handleNext = async () => {
    if (currentTrackIndex === null || currentTrackIndex >= playlist.length - 1) return;
    const nextTrack = playlist[currentTrackIndex + 1];
    if (nextTrack) {
      await tauriApi.play(nextTrack.id);
    }
  };
  
  const canGoPrevious = currentTrackIndex !== null && currentTrackIndex > 0;
  const canGoNext = currentTrackIndex !== null && currentTrackIndex < playlist.length - 1;

  // Show skeleton loading state while not initialized
  if (!isInitialized) {
    return (
      <div className="flex items-center justify-center space-x-4">
        <div className="w-12 h-12 bg-muted animate-pulse rounded-full" />
        <div className="w-16 h-16 bg-muted animate-pulse rounded-full" />
        <div className="w-12 h-12 bg-muted animate-pulse rounded-full" />
      </div>
    );
  }
  
  return (
    <div className="flex items-center justify-center space-x-4">
      <Button
        variant="ghost"
        size="icon"
        onClick={handlePrevious}
        disabled={!canGoPrevious}
        className="w-12 h-12 rounded-full text-muted-foreground hover:text-foreground hover:bg-accent/20 disabled:opacity-30"
      >
        <SkipBack size={24} weight="fill" />
      </Button>
      
      <Button
        variant="default"
        size="icon"
        onClick={handlePlayPause}
        disabled={!currentTrack && playlist.length === 0}
        className="w-16 h-16 rounded-full bg-accent hover:bg-accent/90 text-accent-foreground shadow-lg hover:shadow-xl transition-all duration-200 hover:scale-105"
      >
        {status === 'Playing' ? (
          <Pause size={32} weight="fill" />
        ) : (
          <Play size={32} weight="fill" />
        )}
      </Button>
      
      <Button
        variant="ghost"
        size="icon"
        onClick={handleNext}
        disabled={!canGoNext}
        className="w-12 h-12 rounded-full text-muted-foreground hover:text-foreground hover:bg-accent/20 disabled:opacity-30"
      >
        <SkipForward size={24} weight="fill" />
      </Button>
    </div>
  );
}