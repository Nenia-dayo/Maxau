import { invoke } from '@tauri-apps/api/core';
import { listen } from '@tauri-apps/api/event';
import { PlayerState, PlaybackStatus, Track } from '../types/player';
import { usePlayerStore } from '../stores/playerStore';

export const tauriApi = {
  /**
   * バックエンドにライブラリの初期化を要求し、プレイヤーの初期状態を取得します。
   */
  initializeLibrary: (): Promise<PlayerState> => {
    return invoke('initialize_library');
  },

  /**
   * バックエンドに音楽ライブラリの再スキャンを要求します。
   */
  rescanLibrary: (): Promise<PlayerState> => {
    return invoke('rescan_library');
  },

  /**
   * 指定されたIDのトラックの再生を開始または再開するようにバックエンドに要求します。
   * @param trackId 再生するトラックのID
   */
  play: (trackId: number): Promise<void> => {
    return invoke('play', { trackId });
  },

  /**
   * 指定されたURLから音楽を再生するようにバックエンドに要求します。
   * @param url 再生するHTTP/HTTPSのURL
   */
  playUrl: (url: string): Promise<void> => {
    return invoke('play_url', { url });
  },

  /**
   * 現在の再生を一時停止するようにバックエンドに要求します。
   */
  pause: (): Promise<void> => {
    return invoke('pause');
  },

  /**
   * 一時停止中の再生を再開するようにバックエンドに要求します。
   */
  resume: (): Promise<void> => {
    return invoke('resume');
  },

  /**
   * 再生位置を指定された秒数に移動するようにバックエンドに要求します。
   * @param positionSecs 移動先の秒数
   */
  seek: (positionSecs: number): Promise<void> => {
    return invoke('seek', { positionSecs });
  },

  /**
   * 音量を設定するようにバックエンドに要求します。
   * @param volume 0.0から1.0の範囲の音量
   */
  setVolume: (volume: number): Promise<void> => {
    return invoke('set_volume', { volume });
  },

  /**
   * 指定されたIDのトラックのアルバムアート（画像データ）をバックエンドから取得します。
   * @param trackId アルバムアートを取得したいトラックのID
   * @returns 画像データ(Uint8Array)またはnull
   */
  getAlbumArt: (trackId: number): Promise<Uint8Array | null> => {
    return invoke('get_album_art', { trackId });
  },

  /**
   * バックエンドから送信されるすべてのイベントをリッスンし、
   * グローバルストア（Zustand）の状態を更新するように設定します。
   * @returns すべてのイベントリスナーを解除するためのクリーンアップ関数
   */
  setupEventListeners: async (): Promise<() => void> => {
    const store = usePlayerStore.getState();

    const unlistenPlaybackState = await listen<PlaybackStatus>('playback-state-changed', (event) => {
      store._setStatus(event.payload);
    });

    const unlistenTrackChanged = await listen<{ index: number; track: Track }>('track-changed', (event) => {
      store._setCurrentTrack(event.payload);
    });

    const unlistenPlaybackProgress = await listen<{ position_secs: number }>('playback-progress', (event) => {
      store._setPosition(event.payload.position_secs);
    });

    const unlistenPlaylistUpdated = await listen<Track[]>('playlist-updated', (event) => {
      store._setPlaylist(event.payload);
    });

    // すべてのリスナーを解除する単一の関数を返します
    return () => {
      unlistenPlaybackState();
      unlistenTrackChanged();
      unlistenPlaybackProgress();
      unlistenPlaylistUpdated();
    };
  }
};