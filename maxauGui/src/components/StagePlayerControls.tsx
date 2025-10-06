import { Seekbar } from './Seekbar';
import { PlaybackControls } from './PlaybackControls';
import { VolumeControl } from './VolumeControl';

export function StagePlayerControls() {
  return (
    <div className="flex flex-col items-center space-y-6 w-full max-w-md">
      <Seekbar />
      <PlaybackControls />
      <VolumeControl />
    </div>
  );
}