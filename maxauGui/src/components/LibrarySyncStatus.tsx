import { Button } from '@/components/ui/button';
import { ArrowsClockwise } from '@phosphor-icons/react';
import { tauriApi } from '@/lib/tauriApi';
import { useState } from 'react';
import { usePlayerStore } from '@/stores/playerStore';

export function LibrarySyncStatus() {
  const [isScanning, setIsScanning] = useState(false);
  const { isInitialized } = usePlayerStore();
  
  const handleRescan = async () => {
    setIsScanning(true);
    try {
      await tauriApi.rescanLibrary();
    } finally {
      setTimeout(() => setIsScanning(false), 1000); // Simulate scan time
    }
  };
  
  // Show skeleton while not initialized
  if (!isInitialized) {
    return (
      <div className="flex items-center justify-between">
        <div>
          <div className="h-6 w-32 bg-muted animate-pulse rounded mb-2" />
          <div className="h-4 w-48 bg-muted/70 animate-pulse rounded" />
        </div>
        <div className="h-8 w-28 bg-muted animate-pulse rounded" />
      </div>
    );
  }
  
  return (
    <div className="flex items-center justify-between">
      <div>
        <h2 className="text-xl font-semibold text-foreground">Up Next</h2>
        <p className="text-sm text-muted-foreground">
          {isScanning ? 'Scanning library...' : 'Upcoming tracks in your queue'}
        </p>
      </div>
      
      <Button
        variant="outline"
        size="sm"
        onClick={handleRescan}
        disabled={isScanning}
        className="border-accent/30 hover:border-accent hover:bg-accent/10 text-accent hover:text-accent"
      >
        <ArrowsClockwise 
          size={16} 
          className={isScanning ? 'animate-spin' : ''} 
        />
        {isScanning ? 'Syncing...' : 'Sync Library'}
      </Button>
    </div>
  );
}