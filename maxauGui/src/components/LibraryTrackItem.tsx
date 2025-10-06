import { useState } from 'react';
import { Track } from '@/types/player';
import { usePlayerStore } from '@/stores/playerStore';
import { Button } from '@/components/ui/button';
import { Play, Pause, Heart, DotsThree } from '@phosphor-icons/react';
import { formatDuration } from '@/lib/utils';
import { tauriApi } from '@/lib/tauriApi';

interface LibraryTrackItemProps {
  track: Track;
  index: number;
  originalIndex?: number;
}

export function LibraryTrackItem({ track, index }: LibraryTrackItemProps) {
  const { status, getCurrentTrack } = usePlayerStore();
  const [isHovered, setIsHovered] = useState(false);
  const [isLiked, setIsLiked] = useState(false);
  
  const currentTrack = getCurrentTrack();
  const isCurrentTrack = currentTrack?.id === track.id;
  const isPlaying = status === 'Playing' && isCurrentTrack;
  
  const handlePlayPause = async () => {
    if (isPlaying) {
      await tauriApi.pause();
    } else if (isCurrentTrack) {
      await tauriApi.resume();
    } else {
      await tauriApi.play(track.id);
    }
  };

  const handleLike = () => {
    setIsLiked(!isLiked);
    // TODO: Implement like/unlike logic with backend
    console.log('Like track:', track.title);
  };

  const handleMore = () => {
    // TODO: Implement more options menu (add to playlist, show in folder, etc.)
    console.log('More options for track:', track.title);
  };

  return (
    <div 
      className={`group flex items-center gap-4 p-3 rounded-lg transition-all duration-200 hover:bg-muted/30 ${
        isCurrentTrack ? 'bg-accent/10 border border-accent/20' : ''
      }`}
      onMouseEnter={() => setIsHovered(true)}
      onMouseLeave={() => setIsHovered(false)}
      onDoubleClick={() => tauriApi.play(track.id)}
    >
      {/* Track number / Play button */}
      <div className="w-6 flex items-center justify-center flex-shrink-0">
        {isHovered || isCurrentTrack ? (
          <Button
            variant="ghost"
            size="sm"
            className="w-8 h-8 p-0 hover:bg-accent/20"
            onClick={handlePlayPause}
          >
            {isPlaying ? (
              <Pause className="w-4 h-4" />
            ) : (
              <Play className="w-4 h-4" />
            )}
          </Button>
        ) : (
          <span className={`text-sm ${isCurrentTrack ? 'text-accent' : 'text-muted-foreground'}`}>
            {index + 1}
          </span>
        )}
      </div>
      
      {/* Album art placeholder */}
      <div className="w-12 h-12 bg-muted rounded-md flex items-center justify-center flex-shrink-0">
        <div className="w-6 h-6 bg-muted-foreground/30 rounded" />
      </div>
      
      {/* Track info */}
      <div className="flex-1 min-w-0">
        <h4 className={`font-medium truncate ${isCurrentTrack ? 'text-accent' : 'text-foreground'}`}>
          {track.title || 'Unknown Title'}
        </h4>
        <p className="text-sm text-muted-foreground truncate">
          {track.artist || 'Unknown Artist'}
        </p>
      </div>
      
      {/* Duration */}
      <div className="text-sm text-muted-foreground flex-shrink-0">
        {formatDuration(track.duration_secs)}
      </div>
      
      {/* Action buttons */}
      <div className={`flex items-center gap-1 flex-shrink-0 transition-opacity duration-200 ${
        isHovered || isCurrentTrack ? 'opacity-100' : 'opacity-0'
      }`}>
        <Button
          variant="ghost"
          size="sm"
          className={`w-8 h-8 p-0 hover:bg-accent/20 ${isLiked ? 'text-red-500' : ''}`}
          onClick={handleLike}
        >
          <Heart className="w-4 h-4" weight={isLiked ? 'fill' : 'regular'} />
        </Button>
        <Button
          variant="ghost"
          size="sm"
          className="w-8 h-8 p-0 hover:bg-accent/20"
          onClick={handleMore}
        >
          <DotsThree className="w-4 h-4" />
        </Button>
      </div>
    </div>
  );
}