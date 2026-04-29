import { useCallback, useEffect, useState } from "react";
import {
  getDashboard,
  type ApiUser,
  type DashboardResponse,
} from "../../api/client";
import { loadSourceRows, type SourceRow } from "./utils/dataLoader";
import { accountIdentifier } from "./utils/user";
import { DashboardMetrics, metricsForDisplay } from "./components/DashboardMetrics";
import { SourceStructure, sourceSummary, collectionSummary } from "./components/SourceStructure";

type DashboardPageProps = {
  onNavigate: (
    href: string,
    page: "landing" | "dashboard" | "login" | "signup",
  ) => void;
  onUnauthorized: () => void;
  token: string | null;
  user: ApiUser | null;
};

function DashboardPage({
  onNavigate,
  onUnauthorized,
  token,
  user,
}: DashboardPageProps) {
  const [dashboard, setDashboard] = useState<DashboardResponse | null>(null);
  const [sources, setSources] = useState<SourceRow[]>([]);
  const [error, setError] = useState<string | null>(null);
  const [isLoading, setIsLoading] = useState(false);
  const displayUser = dashboard?.user ?? user;

  const loadDashboard = useCallback(async () => {
    if (!token) {
      setDashboard(null);
      setSources([]);
      setError(null);
      setIsLoading(false);
      return;
    }

    setError(null);
    setIsLoading(true);

    const [dashboardResult, sourcesResult] = await Promise.allSettled([
      getDashboard(token),
      loadSourceRows(),
    ]);

    if (dashboardResult.status === "fulfilled") {
      setDashboard(dashboardResult.value);
    } else {
      const requestError = dashboardResult.reason;

      if (
        requestError instanceof Error &&
        'status' in requestError &&
        requestError.status === 401
      ) {
        onUnauthorized();
        setDashboard(null);
        setIsLoading(false);
        setError("Your session expired. Sign in again to continue.");
        return;
      }

      if (requestError instanceof Error) {
        setError(requestError.message);
      } else {
        setError("Unable to load dashboard data");
      }
    }

    if (sourcesResult.status === "fulfilled") {
      setSources(sourcesResult.value);
    } else {
      setSources([]);
      console.error("Failed to load sources", sourcesResult.reason);
    }

    setIsLoading(false);
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
          {displayUser && (
            <p className="account-line">
              {displayUser.name} - {accountIdentifier(displayUser)}
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

      <DashboardMetrics
        metrics={metricsForDisplay(dashboard, isLoading)}
        isLoading={isLoading}
      />

      <section
        className="table-panel source-panel"
        aria-label="Polling sources"
      >
        <div className="table-header">
          <h2>Polling Sources</h2>
          <span>{sources.length} connected</span>
        </div>
        <div className="federation-list">
          {sources.map((row) => (
            <article className="source-section" key={row.id}>
              <div className="federation-row source-row">
                <div>
                  <strong>{row.name}</strong>
                  <span>{sourceSummary(row)}</span>
                </div>
                <span className={`status-chip ${row.tone}`}>{row.status}</span>
              </div>

              {row.collection ? (
                <div className="source-body">
                  <div className="source-copy">
                    <h3>{row.collection.title}</h3>
                    <p>{collectionSummary(row.collection)}</p>
                  </div>
                  <div className="source-grid">
                    {row.collection.data.map((structure, index) =>
                      SourceStructure({ structure, key: `${row.id}-${index}` }),
                    )}
                  </div>
                </div>
              ) : (
                <div className="source-body">
                  <p className="source-message">
                    {row.error ?? "Source data is unavailable."}
                  </p>
                </div>
              )}
            </article>
          ))}
        </div>
      </section>
    </section>
  );
}

export default DashboardPage;
