type LandingPageProps = {
  onNavigate: (href: string, page: "landing" | "dashboard" | "login") => void;
};

function LandingPage({ onNavigate }: LandingPageProps) {
  return (
    <section className="landing-page page-grid" aria-labelledby="landing-title">
      <div className="intro-panel">
        <p className="eyebrow">Public Polling Federation System</p>
        <h1 id="landing-title">Trusted polling across independent networks.</h1>
        <p className="summary">
          Nexus coordinates polls, federation nodes, and voting status in one
          quiet workspace built for operators and auditors.
        </p>
        <div className="action-row">
          <a
            className="button primary"
            href="/dashboard"
            onClick={(event) => {
              event.preventDefault();
              onNavigate("/dashboard", "dashboard");
            }}
          >
            View Dashboard
          </a>
          <a
            className="button secondary"
            href="/login"
            onClick={(event) => {
              event.preventDefault();
              onNavigate("/login", "login");
            }}
          >
            Operator Login
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
