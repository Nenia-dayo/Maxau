import { create } from 'zustand';
import { Track, PlaybackStatus, PlayerState } from '../types/player';
import { tauriApi } from '../lib/tauriApi';

interface PlayerStore extends PlayerState {
  initialize: () => Promise<void>;
  getCurrentTrack: () => Track | null;
  getUpcomingTracks: () => Track[];
  setVolume: (volume: number) => Promise<void>;
  cleanup: (() => void) | null;
  
  // 内部アクション（コンポーネントからは直接呼ばない）
  _setStatus: (status: PlaybackStatus) => void;
  _setPlaylist: (playlist: Track[]) => void;
  _setCurrentTrack: (payload: { index: number; track: Track }) => void;
  _setPosition: (position: number) => void;
}

export const usePlayerStore = create<PlayerStore>((set, get) => ({
  status: 'Stopped',
  playlist: [],
  currentTrackIndex: null,
  currentPositionSecs: 0,
  volume: 0.8,
  isInitialized: false,
  cleanup: null,

  initialize: async () => {
    if (get().isInitialized) return;
    try {
      const cleanup = await tauriApi.setupEventListeners();
      const initialState = await tauriApi.initializeLibrary();
      set({ ...initialState, cleanup, isInitialized: true });
    } catch (error) {
      console.error('Failed to initialize player:', error);
      set({ isInitialized: true });
    }
  },
  
  _setStatus: (status) => set({ status }),
  _setPlaylist: (playlist) => set({ playlist }),
  _setCurrentTrack: (payload) => set({ currentTrackIndex: payload.index }),
  _setPosition: (position) => set({ currentPositionSecs: position }),
  
  setVolume: async (volume) => {
    try {
      await tauriApi.setVolume(volume);
      set({ volume });
    } catch (error) {
      console.error('Failed to set volume:', error);
    }
  },
  
  getCurrentTrack: () => {
    const { playlist, currentTrackIndex } = get();
    if (currentTrackIndex === null || !playlist.length) {
      return null;
    }
    return playlist[currentTrackIndex] || null;
  },
  
  getUpcomingTracks: () => {
    const { playlist, currentTrackIndex } = get();
    if (currentTrackIndex === null) {
      return playlist;
    }
    return playlist.slice(currentTrackIndex + 1);
  },
}));