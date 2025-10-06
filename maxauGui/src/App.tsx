import { useEffect } from 'react';
import { usePlayerStore } from './stores/playerStore';
import { NowPlayingStage } from './components/NowPlayingStage';
import { MusicLibrary } from './components/MusicLibrary';

function App() {
  const { initialize, isInitialized } = usePlayerStore();
  
  useEffect(() => {
    initialize();
    
    return () => {
      const cleanup = usePlayerStore.getState().cleanup;
      if (cleanup) {
        cleanup();
      }
    };
  }, [initialize]);
  
  if (!isInitialized) {
    return (
      <div className="min-h-screen bg-background flex items-center justify-center">
        <div className="text-center space-y-4">
          <div className="w-16 h-16 border-4 border-accent border-t-transparent rounded-full animate-spin mx-auto" />
          <div className="space-y-2">
            <h1 className="text-2xl font-bold text-foreground">Maxau</h1>
            <p className="text-muted-foreground">Initializing your music library...</p>
          </div>
        </div>
      </div>
    );
  }
  
  return (
    <div className="min-h-screen bg-background text-foreground">
      <div className="fixed inset-0 bg-gradient-to-br from-background via-primary/5 to-accent/10 pointer-events-none" />
      
      <div className="relative z-10 p-4 sm:p-6 space-y-8 max-w-7xl mx-auto">
        <header className="text-center py-4 sm:py-8">
          <h1 className="text-4xl font-bold text-foreground mb-2">Maxau</h1>
          <p className="text-muted-foreground">Focus Flow Music Experience</p>
        </header>
        
        <main className="space-y-8">
          <section>
            <NowPlayingStage />
          </section>
          
          <section>
            <MusicLibrary />
          </section>
        </main>
      </div>
    </div>
  );
}

export default App;
