import { usePlayerStore } from '@/stores/playerStore';

export function StageTrackInfo() {
  const { getCurrentTrack, isInitialized } = usePlayerStore();
  const currentTrack = getCurrentTrack();
  
  // Show skeleton loading state while not initialized
  if (!isInitialized) {
    return (
      <div className="flex flex-col items-center justify-center text-center space-y-3">
        <div className="space-y-3">
          {/* Skeleton loading for title */}
          <div className="h-9 w-64 bg-muted animate-pulse rounded-md" />
          {/* Skeleton loading for artist */}
          <div className="h-6 w-48 bg-muted animate-pulse rounded-md mx-auto" />
          {/* Skeleton loading for album */}
          <div className="h-5 w-56 bg-muted animate-pulse rounded-md mx-auto" />
        </div>
        {/* Skeleton equalizer bars */}
        <div className="flex space-x-1 items-end h-8 pt-4">
          {[...Array(5)].map((_, i) => (
            <div
              key={i}
              className="w-1 bg-muted animate-pulse rounded-full"
              style={{
                height: `${20 + Math.sin(i) * 10}px`
              }}
            />
          ))}
        </div>
      </div>
    );
  }
  
  if (!currentTrack) {
    return (
      <div className="flex flex-col items-center justify-center text-center space-y-3">
        <div className="space-y-3">
          {/* Skeleton loading for title */}
          <div className="h-9 w-64 bg-muted animate-pulse rounded-md" />
          {/* Skeleton loading for artist */}
          <div className="h-6 w-48 bg-muted animate-pulse rounded-md mx-auto" />
          {/* Skeleton loading for album */}
          <div className="h-5 w-56 bg-muted animate-pulse rounded-md mx-auto" />
        </div>
      </div>
    );
  }
  
  return (
    <div className="flex flex-col items-center justify-center text-center space-y-3 max-w-md">
      <div className="space-y-2">
        <h1 className="text-3xl font-bold text-foreground leading-tight">
          {currentTrack.title}
        </h1>
        <h2 className="text-xl font-medium text-accent">
          {currentTrack.artist}
        </h2>
        <h3 className="text-lg text-muted-foreground">
          {currentTrack.album}
        </h3>
      </div>
      
      {/* Animated equalizer bars when playing */}
      <div className="flex space-x-1 items-end h-8 pt-4">
        {[...Array(5)].map((_, i) => (
          <div
            key={i}
            className="w-1 bg-gradient-to-t from-accent to-primary rounded-full animate-pulse"
            style={{
              height: `${20 + Math.sin(i) * 10}px`,
              animationDelay: `${i * 0.1}s`,
              animationDuration: '1.5s'
            }}
          />
        ))}
      </div>
    </div>
  );
}