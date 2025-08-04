import "./App.css";

function NotFound() {
  const goHome = () => {
    window.location.href = "/";
  };

  return (
    <div className="url-shortener">
      <h1>404 - Link Not Found</h1>
      <p className="description">
        The shortened link you're looking for doesn't exist or may have expired.
      </p>
      <div className="not-found-actions">
        <button onClick={goHome} className="shorten-button">
          Go to Main Page
        </button>
      </div>
    </div>
  );
}

export default NotFound;