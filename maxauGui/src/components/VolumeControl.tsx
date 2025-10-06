import { useState } from 'react';
import { SpeakerHigh, SpeakerLow, SpeakerNone } from '@phosphor-icons/react';
import { Button } from '@/components/ui/button';
import { Popover, PopoverContent, PopoverTrigger } from '@/components/ui/popover';
import { usePlayerStore } from '@/stores/playerStore';
import { tauriApi } from '@/lib/tauriApi';

export function VolumeControl() {
  const { volume, isInitialized } = usePlayerStore();
  const [isOpen, setIsOpen] = useState(false);
  
  const handleVolumeChange = async (e: React.ChangeEvent<HTMLInputElement>) => {
    const newVolume = parseFloat(e.target.value);
    await tauriApi.setVolume(newVolume);
    // Update store state
    usePlayerStore.getState().setVolume(newVolume);
  };
  
  const getVolumeIcon = () => {
    if (volume === 0) return <SpeakerNone size={20} />;
    if (volume < 0.5) return <SpeakerLow size={20} />;
    return <SpeakerHigh size={20} />;
  };
  
  const volumePercentage = Math.round(volume * 100);
  
  // Show skeleton loading state while not initialized
  if (!isInitialized) {
    return (
      <div className="w-8 h-8 bg-muted animate-pulse rounded" />
    );
  }
  
  return (
    <Popover open={isOpen} onOpenChange={setIsOpen}>
      <PopoverTrigger asChild>
        <Button
          variant="ghost"
          size="sm"
          className="text-muted-foreground hover:text-foreground hover:bg-accent/20"
        >
          {getVolumeIcon()}
        </Button>
      </PopoverTrigger>
      <PopoverContent className="w-40 p-3" side="bottom" align="center">
        <div className="space-y-3">
          <div className="text-sm font-medium text-center">
            Volume: {volumePercentage}%
          </div>
          <div className="relative">
            <input
              type="range"
              min={0}
              max={1}
              step={0.01}
              value={volume}
              onChange={handleVolumeChange}
              className="w-full h-2 bg-secondary rounded-lg appearance-none cursor-pointer 
                         [&::-webkit-slider-thumb]:appearance-none 
                         [&::-webkit-slider-thumb]:w-4 
                         [&::-webkit-slider-thumb]:h-4 
                         [&::-webkit-slider-thumb]:rounded-full 
                         [&::-webkit-slider-thumb]:bg-accent 
                         [&::-webkit-slider-thumb]:cursor-pointer 
                         [&::-webkit-slider-thumb]:border-2 
                         [&::-webkit-slider-thumb]:border-white 
                         [&::-webkit-slider-thumb]:shadow-md
                         [&::-moz-range-thumb]:w-4 
                         [&::-moz-range-thumb]:h-4 
                         [&::-moz-range-thumb]:rounded-full 
                         [&::-moz-range-thumb]:bg-accent 
                         [&::-moz-range-thumb]:cursor-pointer 
                         [&::-moz-range-thumb]:border-2 
                         [&::-moz-range-thumb]:border-white 
                         [&::-moz-range-thumb]:shadow-md
                         [&::-moz-range-thumb]:border-none"
              style={{
                background: `linear-gradient(to right, oklch(0.65 0.2 285) 0%, oklch(0.65 0.2 285) ${volumePercentage}%, oklch(0.25 0.08 280) ${volumePercentage}%, oklch(0.25 0.08 280) 100%)`
              }}
            />
          </div>
        </div>
      </PopoverContent>
    </Popover>
  );
}