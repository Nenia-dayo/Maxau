import { usePlayerStore } from '@/stores/playerStore';
import { formatDuration } from '@/lib/utils';
import { tauriApi } from '@/lib/tauriApi';

export function Seekbar() {
  const { getCurrentTrack, currentPositionSecs, isInitialized } = usePlayerStore();
  const currentTrack = getCurrentTrack();
  
  // Show skeleton loading state while not initialized
  if (!isInitialized) {
    return (
      <div className="w-full space-y-2">
        <div className="w-full h-2 bg-muted animate-pulse rounded-lg" />
        <div className="flex justify-between">
          <div className="h-4 w-10 bg-muted animate-pulse rounded" />
          <div className="h-4 w-10 bg-muted animate-pulse rounded" />
        </div>
      </div>
    );
  }
  
  if (!currentTrack) return null;
  
  const handleSeek = (e: React.ChangeEvent<HTMLInputElement>) => {
    const newPosition = parseFloat(e.target.value);
    tauriApi.seek(newPosition);
  };
  
  const progress = (currentPositionSecs / currentTrack.duration_secs) * 100;
  
  return (
    <div className="w-full space-y-2">
      <div className="relative">
        <input
          type="range"
          min={0}
          max={currentTrack.duration_secs}
          value={currentPositionSecs}
          onChange={handleSeek}
          className="w-full h-2 bg-secondary rounded-lg appearance-none cursor-pointer 
                     [&::-webkit-slider-thumb]:appearance-none 
                     [&::-webkit-slider-thumb]:w-4 
                     [&::-webkit-slider-thumb]:h-4 
                     [&::-webkit-slider-thumb]:rounded-full 
                     [&::-webkit-slider-thumb]:bg-accent 
                     [&::-webkit-slider-thumb]:cursor-pointer 
                     [&::-webkit-slider-thumb]:border-2 
                     [&::-webkit-slider-thumb]:border-white 
                     [&::-webkit-slider-thumb]:shadow-lg
                     [&::-moz-range-thumb]:w-4 
                     [&::-moz-range-thumb]:h-4 
                     [&::-moz-range-thumb]:rounded-full 
                     [&::-moz-range-thumb]:bg-accent 
                     [&::-moz-range-thumb]:cursor-pointer 
                     [&::-moz-range-thumb]:border-2 
                     [&::-moz-range-thumb]:border-white 
                     [&::-moz-range-thumb]:shadow-lg
                     [&::-moz-range-thumb]:border-none"
          style={{
            background: `linear-gradient(to right, oklch(0.65 0.2 285) 0%, oklch(0.65 0.2 285) ${progress}%, oklch(0.25 0.08 280) ${progress}%, oklch(0.25 0.08 280) 100%)`
          }}
        />
      </div>
      
      <div className="flex justify-between text-sm text-muted-foreground font-mono">
        <span>{formatDuration(currentPositionSecs)}</span>
        <span>{formatDuration(currentTrack.duration_secs)}</span>
      </div>
    </div>
  );
}