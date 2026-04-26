type LandingPageProps = {
  isAuthenticated: boolean;
  onNavigate: (
    href: string,
    page: "landing" | "dashboard" | "login" | "signup"
  ) => void;
  userName?: string;
};

function LandingPage({
  isAuthenticated,
  onNavigate,
  userName
}: LandingPageProps) {
  return (
    <section className="landing-page page-grid" aria-labelledby="landing-title">
      <div className="intro-panel">
        <p className="eyebrow">
          {isAuthenticated && userName
            ? `Welcome back, ${userName}`
            : "Public Polling Federation System"}
        </p>
        <h1 id="landing-title">Trusted polling across independent networks.</h1>
        <p className="summary">
          Nexus coordinates polls, federation nodes, and voting status in one
          quiet workspace built for operators and auditors.
        </p>
        <div className="action-row">
          <a
            className="button primary"
            href={isAuthenticated ? "/dashboard" : "/signup"}
            onClick={(event) => {
              event.preventDefault();
              onNavigate(
                isAuthenticated ? "/dashboard" : "/signup",
                isAuthenticated ? "dashboard" : "signup"
              );
            }}
          >
            {isAuthenticated ? "View Dashboard" : "Create Account"}
          </a>
          <a
            className="button secondary"
            href={isAuthenticated ? "/dashboard" : "/login"}
            onClick={(event) => {
              event.preventDefault();
              onNavigate(
                isAuthenticated ? "/dashboard" : "/login",
                isAuthenticated ? "dashboard" : "login"
              );
            }}
          >
            {isAuthenticated ? "Open Workspace" : "Operator Login"}
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
