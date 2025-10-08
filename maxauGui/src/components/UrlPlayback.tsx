import { useState } from 'react';
import { Button } from './ui/button';
import { Input } from './ui/input';
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from './ui/card';
import { Alert, AlertDescription } from './ui/alert';
import { Play, Loader2, ExternalLink } from 'lucide-react';
import { tauriApi } from '../lib/tauriApi';

export function UrlPlayback() {
  const [url, setUrl] = useState('');
  const [isLoading, setIsLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const [success, setSuccess] = useState(false);

  const handlePlayUrl = async () => {
    if (!url.trim()) {
      setError('Please enter a valid URL');
      return;
    }

    // URLの基本的な検証
    try {
      const parsedUrl = new URL(url);
      if (parsedUrl.protocol !== 'http:' && parsedUrl.protocol !== 'https:') {
        setError('Only HTTP and HTTPS URLs are supported');
        return;
      }
    } catch {
      setError('Invalid URL format');
      return;
    }

    setIsLoading(true);
    setError(null);
    setSuccess(false);

    try {
      await tauriApi.playUrl(url);
      setSuccess(true);
      setTimeout(() => setSuccess(false), 3000);
    } catch (err) {
      setError(err instanceof Error ? err.message : 'Failed to play URL');
    } finally {
      setIsLoading(false);
    }
  };

  const handleKeyPress = (e: React.KeyboardEvent) => {
    if (e.key === 'Enter' && !isLoading) {
      handlePlayUrl();
    }
  };

  return (
    <Card className="border-accent/20 bg-card/50 backdrop-blur">
      <CardHeader>
        <CardTitle className="flex items-center gap-2">
          <ExternalLink className="w-5 h-5" />
          Play from URL
        </CardTitle>
        <CardDescription>
          Stream audio directly from HTTP/HTTPS URLs
        </CardDescription>
      </CardHeader>
      <CardContent className="space-y-4">
        <div className="flex gap-2">
          <Input
            type="url"
            placeholder="https://example.com/audio.mp3"
            value={url}
            onChange={(e) => setUrl(e.target.value)}
            onKeyPress={handleKeyPress}
            disabled={isLoading}
            className="flex-1"
          />
          <Button
            onClick={handlePlayUrl}
            disabled={isLoading || !url.trim()}
            className="gap-2"
          >
            {isLoading ? (
              <>
                <Loader2 className="w-4 h-4 animate-spin" />
                Loading
              </>
            ) : (
              <>
                <Play className="w-4 h-4" />
                Play
              </>
            )}
          </Button>
        </div>

        {error && (
          <Alert variant="destructive">
            <AlertDescription>{error}</AlertDescription>
          </Alert>
        )}

        {success && (
          <Alert className="border-green-500 bg-green-500/10">
            <AlertDescription className="text-green-700 dark:text-green-400">
              Successfully started playback!
            </AlertDescription>
          </Alert>
        )}

        <div className="text-xs text-muted-foreground space-y-1">
          <p>Supported formats: MP3, FLAC, OGG, WAV, AAC, and more</p>
          <p>Example: https://example.com/song.mp3</p>
        </div>
      </CardContent>
    </Card>
  );
}
