import { useCallback, useEffect, useState } from "react";
import {
  ApiRequestError,
  getDashboard,
  type DashboardMetric,
  type DashboardResponse
} from "../../api/client";

const federationRows = [
  { name: "Northeast Civic", status: "Synced", polls: 18, tone: "online" },
  { name: "Central County", status: "Review", polls: 7, tone: "review" },
  { name: "Western Municipal", status: "Synced", polls: 11, tone: "online" }
];

type DashboardPageProps = {
  onNavigate: (
    href: string,
    page: "landing" | "dashboard" | "login" | "signup"
  ) => void;
  onUnauthorized: () => void;
  token: string | null;
};

function DashboardPage({
  onNavigate,
  onUnauthorized,
  token
}: DashboardPageProps) {
  const [dashboard, setDashboard] = useState<DashboardResponse | null>(null);
  const [error, setError] = useState<string | null>(null);
  const [isLoading, setIsLoading] = useState(false);

  const loadDashboard = useCallback(async () => {
    if (!token) {
      setDashboard(null);
      setError(null);
      setIsLoading(false);
      return;
    }

    setError(null);
    setIsLoading(true);

    try {
      const nextDashboard = await getDashboard(token);
      setDashboard(nextDashboard);
    } catch (requestError) {
      if (
        requestError instanceof ApiRequestError &&
        requestError.status === 401
      ) {
        onUnauthorized();
        setDashboard(null);
        setError("Your session expired. Sign in again to continue.");
      } else if (requestError instanceof ApiRequestError) {
        setError(requestError.message);
      } else {
        setError("Unable to load dashboard data");
      }
    } finally {
      setIsLoading(false);
    }
  }, [onUnauthorized, token]);

  useEffect(() => {
    void loadDashboard();
  }, [loadDashboard]);

  if (!token) {
    return (
      <section className="dashboard-page" aria-labelledby="dashboard-title">
        <div className="dashboard-state">
          <p className="eyebrow">Authentication Required</p>
          <h1 id="dashboard-title">Sign in to view dashboard access.</h1>
          <div className="action-row">
            <a
              className="button primary"
              href="/login"
              onClick={(event) => {
                event.preventDefault();
                onNavigate("/login", "login");
              }}
            >
              Sign In
            </a>
            <a
              className="button secondary"
              href="/signup"
              onClick={(event) => {
                event.preventDefault();
                onNavigate("/signup", "signup");
              }}
            >
              Create Account
            </a>
          </div>
        </div>
      </section>
    );
  }

  return (
    <section className="dashboard-page" aria-labelledby="dashboard-title">
      <div className="dashboard-toolbar">
        <div className="page-heading">
          <p className="eyebrow">Operations</p>
          <h1 id="dashboard-title">Federation Dashboard</h1>
          {dashboard && (
            <p className="account-line">
              {dashboard.user.name} - {dashboard.user.email}
            </p>
          )}
        </div>
        <button
          className="button secondary"
          disabled={isLoading}
          onClick={() => {
            void loadDashboard();
          }}
          type="button"
        >
          {isLoading ? "Refreshing..." : "Refresh"}
        </button>
      </div>

      {error && (
        <div className="dashboard-state compact" role="alert">
          <p>{error}</p>
        </div>
      )}

      <div className="dashboard-grid">
        {metricsForDisplay(dashboard, isLoading).map((metric, index) => (
          <article
            className={`metric-card ${metricTone(metric, index)}`}
            key={metric.label}
          >
            <span className="metric-value">{metric.value}</span>
            <span className="metric-label">{metric.label}</span>
            <span className={`metric-status ${metric.status}`}>
              {metric.status}
            </span>
          </article>
        ))}
      </div>

      <section className="table-panel" aria-label="Federation status">
        <div className="table-header">
          <h2>Federations</h2>
          <span>Live status</span>
        </div>
        <div className="federation-list">
          {federationRows.map((row) => (
            <div className="federation-row" key={row.name}>
              <div>
                <strong>{row.name}</strong>
                <span>{row.polls} active polls</span>
              </div>
              <span className={`status-chip ${row.tone}`}>{row.status}</span>
            </div>
          ))}
        </div>
      </section>
    </section>
  );
}

function metricsForDisplay(
  dashboard: DashboardResponse | null,
  isLoading: boolean
): DashboardMetric[] {
  if (dashboard) {
    return dashboard.metrics;
  }

  if (isLoading) {
    return [
      { label: "Active federations", value: "...", status: "loading" },
      { label: "Node availability", value: "...", status: "loading" },
      { label: "Ballots synchronized", value: "...", status: "loading" }
    ];
  }

  return [
    { label: "Active federations", value: "0", status: "offline" },
    { label: "Node availability", value: "0%", status: "offline" },
    { label: "Ballots synchronized", value: "0", status: "offline" }
  ];
}

function metricTone(metric: DashboardMetric, index: number): string {
  if (metric.status === "review") {
    return "danger";
  }

  if (index === 0 && metric.status !== "loading") {
    return "strong";
  }

  return "";
}

export default DashboardPage;
