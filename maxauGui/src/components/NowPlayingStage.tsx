import { StageAlbumArt } from './StageAlbumArt';
import { StageTrackInfo } from './StageTrackInfo';
import { StagePlayerControls } from './StagePlayerControls';

export function NowPlayingStage() {
  return (
    <div className="bg-card/50 backdrop-blur-md rounded-3xl p-8 shadow-2xl border border-border/50">
      <div className="grid grid-cols-1 lg:grid-cols-3 gap-8 items-center">
        <div className="flex justify-center">
          <StageAlbumArt />
        </div>
        
        <div className="flex justify-center lg:col-span-1">
          <StageTrackInfo />
        </div>
        
        <div className="flex justify-center">
          <StagePlayerControls />
        </div>
      </div>
    </div>
  );
}