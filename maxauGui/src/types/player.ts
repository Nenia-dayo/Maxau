export interface Track {
  id: number;
  title: string;
  artist: string;
  album: string;
  duration_secs: number;
  path: string; // `file_path`から`path`に変更
}

export type PlaybackStatus = 'Playing' | 'Paused' | 'Stopped';

export interface PlayerState {
  status: PlaybackStatus;
  playlist: Track[];
  currentTrackIndex: number | null;
  currentPositionSecs: number;
  volume: number;
  isInitialized: boolean;
}

export interface PlaybackProgressPayload {
  position_secs: number;
}

export interface TrackChangedPayload {
  index: number;
  track: Track;
}