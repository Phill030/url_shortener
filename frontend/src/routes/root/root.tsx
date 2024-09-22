import { useState } from "react";
import IconButton, { IconPosition } from "../../components/button/icon-button.tsx";
import IconInput from "../../components/input/icon-input.tsx";
import style from "./root.module.scss";

export default function Root() {
  const [isLoading, setLoading] = useState(false);
  const [url, setUrl] = useState("");
  const [error, setError] = useState("");
  const [shortUrl, setShortUrl] = useState("");

  const shorten = async () => {
    setLoading(true);
    setError("");
    setShortUrl("");

    try {
      const response = await fetch("https://s.phill030.de/shorten", {
        method: "POST",
        headers: {
          "Content-Type": "application/json",
        },
        body: JSON.stringify({ url }),
      });

      if (!response.ok) {
        switch (response.status) {
          case 400:
            throw new Error("Malformed URL. Please try again");
          case 429:
            throw new Error("Rate limit exceeded. Please try again later.");
          case 500:
            throw new Error("Internal server error. Please try again later.");
          default:
            throw new Error("Failed to shorten URL. Please try again.");
        }
      }

      const data = await response.json();
      setShortUrl(data.short_url);
      setUrl("");
    } catch (err: any) {
      setError(err.message);
    } finally {
      setLoading(false);
    }
  };

  const handleInputChange = (e: React.ChangeEvent<HTMLInputElement>) => {
    setUrl(e.target.value);
  };

  return (
    <div className={style.wrapper}>
      <h1>URL Shortener</h1>
      <div className={style.box}>
        <form>
          <IconInput
            icon="material-symbols:link"
            type="text"
            className={style.urlInput}
            value={url}
            onChange={handleInputChange}
            placeholder="https://example.com"
            required
          />
          <IconButton
            icon={isLoading ? "eos-icons:three-dots-loading" : "majesticons:rocket-3-start"}
            text={isLoading ? "Shortening..." : "Shorten"}
            iconPosition={IconPosition.LEFT}
            className={style.shortenButton}
            onClick={shorten}
            disabled={isLoading || !url}
          />
        </form>
      </div>

      {error && <p className={style.error}>{error}</p>}
      {shortUrl && (
        <div className={style.result}>
          <p>Shortened URL:</p>
          <a href={shortUrl} target="_blank" rel="noopener noreferrer">
            {shortUrl}
          </a>
        </div>
      )}
    </div>
  );
}
