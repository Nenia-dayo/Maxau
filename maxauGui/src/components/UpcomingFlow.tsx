import { LibrarySyncStatus } from './LibrarySyncStatus';
import { UpcomingTrackCard } from './UpcomingTrackCard';
import { usePlayerStore } from '@/stores/playerStore';

export function UpcomingFlow() {
  const { getUpcomingTracks, isInitialized } = usePlayerStore();
  const upcomingTracks = getUpcomingTracks();
  
  // Show skeleton loading state while initializing
  if (!isInitialized) {
    return (
      <div className="bg-card/30 backdrop-blur-sm rounded-2xl p-6 border border-border/30">
        {/* Header skeleton */}
        <div className="flex items-center justify-between">
          <div>
            <div className="h-6 w-32 bg-muted animate-pulse rounded mb-2" />
            <div className="h-4 w-48 bg-muted/70 animate-pulse rounded" />
          </div>
          <div className="h-8 w-28 bg-muted animate-pulse rounded" />
        </div>
        
        {/* Track list skeleton */}
        <div className="mt-6 space-y-3">
          {Array.from({ length: 4 }).map((_, index) => (
            <div key={index} className="bg-muted/50 animate-pulse rounded-xl p-4 flex items-center space-x-4">
              <div className="w-12 h-12 bg-muted animate-pulse rounded-lg" />
              <div className="flex-1 space-y-2">
                <div className="h-4 w-3/4 bg-muted animate-pulse rounded" />
                <div className="h-3 w-1/2 bg-muted/70 animate-pulse rounded" />
              </div>
              <div className="h-8 w-8 bg-muted animate-pulse rounded-full" />
            </div>
          ))}
        </div>
      </div>
    );
  }
  
  return (
    <div className="bg-card/30 backdrop-blur-sm rounded-2xl p-6 border border-border/30">
      <LibrarySyncStatus />
      
      <div className="mt-6 perspective-1000">
        <div className="preserve-3d space-y-3 max-h-96 overflow-y-auto pr-2 scrollbar-thin scrollbar-thumb-accent/50 scrollbar-track-transparent">
          {upcomingTracks.length > 0 ? (
            upcomingTracks.map((track, index) => (
              <UpcomingTrackCard
                key={track.id}
                track={track}
                depth={Math.min(index, 4)} // Visual depth based on upcoming position
                isCurrentTrack={false} // These are all upcoming tracks
              />
            ))
          ) : (
            <div className="text-center py-8">
              <p className="text-muted-foreground">
                No upcoming tracks
              </p>
              <p className="text-muted-foreground/70 text-sm mt-1">
                This is the last track in your queue
              </p>
            </div>
          )}
        </div>
        
        <div className="mt-4 text-center">
          <p className="text-sm text-muted-foreground">
            {upcomingTracks.length} upcoming track{upcomingTracks.length !== 1 ? 's' : ''}
          </p>
        </div>
      </div>
    </div>
  );
}