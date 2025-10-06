import { motion } from 'framer-motion';
import { Track } from '@/types/player';
import { formatSeconds } from '@/utils/formatSeconds';
import { tauriApi } from '@/lib/tauriApi';

interface UpcomingTrackCardProps {
  track: Track;
  depth: number;
  isCurrentTrack?: boolean;
}

export function UpcomingTrackCard({ track, depth, isCurrentTrack = false }: UpcomingTrackCardProps) {
  const handleDoubleClick = () => {
    tauriApi.play(track.id);
  };
  
  const scale = 1 - (depth * 0.05);
  const translateZ = -depth * 50;
  const opacity = 1 - (depth * 0.1);
  
  return (
    <motion.div
      layoutId={`track-${track.id}`}
      onClick={handleDoubleClick}
      className={`backdrop-blur-sm rounded-xl p-4 border cursor-pointer 
                 transition-all duration-300 hover:shadow-lg ${
        isCurrentTrack 
          ? 'bg-accent/20 border-accent/50 hover:bg-accent/30' 
          : 'bg-card/80 border-border/30 hover:bg-card hover:border-accent/50'
      }`}
      style={{
        transform: `translateZ(${translateZ}px) scale(${scale})`,
        opacity: Math.max(0.3, opacity)
      }}
      whileHover={{ 
        scale: scale * 1.02,
        transition: { duration: 0.2 }
      }}
      whileTap={{ scale: scale * 0.98 }}
    >
      <div className="flex items-center space-x-3">
        <div 
          className="w-12 h-12 rounded-lg bg-gradient-to-br flex-shrink-0 relative"
          style={{
            background: `linear-gradient(135deg, 
              hsl(${(track.id * 60) % 360}, 70%, 60%), 
              hsl(${(track.id * 60 + 60) % 360}, 50%, 30%))`
          }}
        >
          {isCurrentTrack && (
            <div className="absolute inset-0 bg-accent/30 rounded-lg animate-pulse" />
          )}
        </div>
        
        <div className="flex-1 min-w-0">
          <h3 className={`font-medium truncate ${
            isCurrentTrack ? 'text-accent' : 'text-foreground'
          }`}>
            {track.title}
          </h3>
          <p className="text-sm text-muted-foreground truncate">
            {track.artist}
          </p>
        </div>
        
        <div className="text-sm text-muted-foreground font-mono">
          {formatSeconds(track.duration_secs)}
        </div>
      </div>
    </motion.div>
  );
}