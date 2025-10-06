export function LibraryTrackSkeleton() {
  return (
    <div className="flex items-center gap-4 p-3 rounded-lg bg-muted/10 animate-pulse">
      {/* Track number skeleton */}
      <div className="w-6 h-4 bg-muted-foreground/30 rounded flex-shrink-0" />
      
      {/* Album art skeleton */}
      <div className="w-12 h-12 bg-muted-foreground/30 rounded-md flex-shrink-0 relative overflow-hidden">
        {/* Shimmer effect */}
        <div className="absolute inset-0 bg-gradient-to-r from-transparent via-muted-foreground/10 to-transparent animate-pulse" />
      </div>
      
      {/* Track info skeleton */}
      <div className="flex-1 min-w-0 space-y-2">
        {/* Title skeleton with varied widths */}
        <div className="h-4 bg-muted-foreground/30 rounded animate-pulse" style={{ width: `${60 + Math.random() * 30}%` }} />
        {/* Artist skeleton */}
        <div className="h-3 bg-muted-foreground/20 rounded animate-pulse" style={{ width: `${40 + Math.random() * 25}%` }} />
      </div>
      
      {/* Duration skeleton */}
      <div className="w-12 h-4 bg-muted-foreground/30 rounded flex-shrink-0 animate-pulse" />
      
      {/* Action buttons skeleton */}
      <div className="flex items-center gap-2 flex-shrink-0">
        <div className="w-8 h-8 bg-muted-foreground/20 rounded-full animate-pulse" />
        <div className="w-8 h-8 bg-muted-foreground/20 rounded-full animate-pulse" />
      </div>
    </div>
  );
}