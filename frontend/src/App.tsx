import { useState } from "react";
import "./App.css";
import { API_ENDPOINTS } from "./config";

function App() {
  const [url, setUrl] = useState("");
  const [shortUrl, setShortUrl] = useState("");
  const [isLoading, setIsLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const [copied, setCopied] = useState(false);

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();

    // Reset states
    setError(null);
    setShortUrl("");
    setCopied(false);

    // Validate URL
    if (!url) {
      setError("Please enter a URL");
      return;
    }

    try {
      setIsLoading(true);

      const response = await fetch(API_ENDPOINTS.shorten, {
        method: "POST",
        headers: {
          "Content-Type": "application/json",
        },
        body: JSON.stringify({ url }),
      });

      if (!response.ok) {
        throw new Error(`Error: ${response.status}`);
      }

      const data = await response.json();
      setShortUrl(data.short_url);
    } catch (err) {
      setError(
        err instanceof Error
          ? err.message
          : "An error occurred while shortening the URL",
      );
    } finally {
      setIsLoading(false);
    }
  };

  const copyToClipboard = async () => {
    try {
      await navigator.clipboard.writeText(shortUrl);
      setCopied(true);
      setTimeout(() => setCopied(false), 2000);
      // eslint-disable-next-line @typescript-eslint/no-unused-vars
    } catch (_err) {
      setError("Failed to copy to clipboard");
    }
  };

  return (
    <div className="url-shortener">
      <h1>URL Shortener</h1>
      <p className="description">
        Enter a long URL below to get a shorter, more manageable link.
      </p>

      <form onSubmit={handleSubmit} className="shortener-form">
        <input
          type="url"
          value={url}
          onChange={(e) => setUrl(e.target.value)}
          placeholder="https://example.com/long-url-to-shorten"
          disabled={isLoading}
          className="url-input"
        />
        <button type="submit" disabled={isLoading} className="shorten-button">
          {isLoading ? "Shortening..." : "Shorten"}
        </button>
      </form>

      {error && <div className="error-message">{error}</div>}

      {shortUrl && (
        <div className="result">
          <h2>Your shortened URL:</h2>
          <div className="shortened-url-container">
            <a
              href={shortUrl}
              target="_blank"
              rel="noopener noreferrer"
              className="shortened-url"
            >
              {shortUrl}
            </a>
            <button onClick={copyToClipboard} className="copy-button">
              {copied ? "Copied!" : "Copy"}
            </button>
          </div>
        </div>
      )}
    </div>
  );
}

export default App;
