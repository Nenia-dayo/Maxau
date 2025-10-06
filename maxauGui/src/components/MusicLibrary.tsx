import { useState, useEffect } from 'react';
import { usePlayerStore } from '@/stores/playerStore';
import { LibraryTrackItem } from './LibraryTrackItem';
import { LibraryTrackSkeleton } from './LibraryTrackSkeleton';
import { Card, CardContent, CardHeader, CardTitle } from '@/components/ui/card';
import { Button } from '@/components/ui/button';
import { Input } from '@/components/ui/input';
import { MagnifyingGlass, Shuffle } from '@phosphor-icons/react';

export function MusicLibrary() {
  const { playlist, isInitialized } = usePlayerStore();
  const [searchQuery, setSearchQuery] = useState('');
  const [isLoading, setIsLoading] = useState(true);

  // Simulate API loading delay for demonstration
  useEffect(() => {
    if (isInitialized) {
      const timer = setTimeout(() => {
        setIsLoading(false);
      }, 1500); // Simulate 1.5s loading time
      
      return () => clearTimeout(timer);
    }
  }, [isInitialized]);

  const filteredTracks = playlist.filter(track =>
    track.title.toLowerCase().includes(searchQuery.toLowerCase()) ||
    track.artist.toLowerCase().includes(searchQuery.toLowerCase())
  );

  return (
    <Card className="bg-card/80 backdrop-blur-sm border-border/50 max-h-[600px] flex flex-col">
      <CardHeader className="space-y-4 flex-shrink-0">
        <div className="flex items-center justify-between">
          <CardTitle className="text-2xl font-bold text-foreground">
            Music Library
          </CardTitle>
          <div className="flex items-center gap-2">
            <Button
              variant="outline"
              size="sm"
              className="gap-2"
              disabled={isLoading}
            >
              <Shuffle className="w-4 h-4" />
              Shuffle All
            </Button>
          </div>
        </div>
        
        <div className="relative">
          <MagnifyingGlass className="absolute left-3 top-1/2 transform -translate-y-1/2 w-4 h-4 text-muted-foreground" />
          <Input
            placeholder="Search tracks..."
            value={searchQuery}
            onChange={(e) => setSearchQuery(e.target.value)}
            className="pl-10"
            disabled={isLoading}
          />
        </div>
      </CardHeader>
      
      <CardContent className="space-y-2 flex-1 overflow-hidden">
        {/* Show skeleton loading state */}
        {isLoading ? (
          <div className="space-y-2 overflow-y-auto max-h-full pr-2 scrollbar-thin scrollbar-thumb-accent/50 scrollbar-track-transparent">
            {Array.from({ length: 8 }).map((_, index) => (
              <div
                key={index}
                style={{
                  animationDelay: `${index * 50}ms`,
                  animationDuration: '1.5s'
                }}
                className="animate-pulse"
              >
                <LibraryTrackSkeleton />
              </div>
            ))}
          </div>
        ) : (
          <div className="space-y-2 overflow-y-auto max-h-full pr-2 scrollbar-thin scrollbar-thumb-accent/50 scrollbar-track-transparent">
            {/* Track count header */}
            <div className="text-sm text-muted-foreground mb-4 sticky top-0 bg-card/90 backdrop-blur-sm py-2">
              {filteredTracks.length} track{filteredTracks.length !== 1 ? 's' : ''}
              {searchQuery && ` matching "${searchQuery}"`}
            </div>
            
            {/* Track list */}
            <div className="space-y-1">
              {filteredTracks.length > 0 ? (
                filteredTracks.map((track, index) => (
                  <LibraryTrackItem 
                    key={track.id} 
                    track={track} 
                    index={index}
                    originalIndex={playlist.findIndex(t => t.id === track.id)}
                  />
                ))
              ) : (
                <div className="text-center py-12 text-muted-foreground">
                  <MagnifyingGlass className="w-12 h-12 mx-auto mb-4 opacity-50" />
                  <p>No tracks found</p>
                  {searchQuery && (
                    <p className="text-sm mt-1">Try adjusting your search terms</p>
                  )}
                </div>
              )}
            </div>
          </div>
        )}
      </CardContent>
    </Card>
  );
}