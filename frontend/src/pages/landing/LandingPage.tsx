type LandingPageProps = {
  isAuthenticated: boolean;
  onNavigate: (
    href: string,
    page: "landing" | "dashboard" | "login" | "signup",
  ) => void;
  userName?: string;
};

function LandingPage({
  isAuthenticated,
  onNavigate,
  userName,
}: LandingPageProps) {
  return (
    <section className="landing-page page-grid" aria-labelledby="landing-title">
      <div className="intro-panel">
        <p className="eyebrow">
          {isAuthenticated && userName
            ? `Welcome back, ${userName}`
            : "Public Polling Federation System"}
        </p>
        <h1 id="landing-title">All the polls, one location</h1>
        <p className="summary">
          Nexus centralizes all the most popular pollsters data into one
          easy-to-use API. No account required.
        </p>
        <div className="action-row">
          {!isAuthenticated && (
            <a
              className="button primary"
              href="/signup"
              onClick={(event) => {
                event.preventDefault();
                onNavigate("/signup", "signup");
              }}
            >
              Create Account
            </a>
          )}
          <a
            className={isAuthenticated ? "button primary" : "button secondary"}
            href="/docs"
          >
            API Docs
          </a>
        </div>
      </div>

      <div className="signal-panel" aria-label="Federation snapshot">
        <div className="metric-card strong">
          <span className="metric-value">24</span>
          <span className="metric-label">Active federations</span>
        </div>
        <div className="metric-card">
          <span className="metric-value">98.7%</span>
          <span className="metric-label">Node availability</span>
        </div>
        <div className="metric-card">
          <span className="metric-value">12k</span>
          <span className="metric-label">Ballots synchronized</span>
        </div>
      </div>
    </section>
  );
}

export default LandingPage;
