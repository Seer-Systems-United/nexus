import type { ApiUser } from "../../api/client";
import { accountIdentifier } from "./utils/user";
import { DashboardMetrics, metricsForDisplay } from "./components/DashboardMetrics";
import { DashboardAuthRequired } from "./components/DashboardAuthRequired";
import { SourcesPanel } from "./components/SourcesPanel";
import { useDashboardData } from "./hooks/useDashboardData";

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
  const { dashboard, sources, error, isLoading, loadDashboard } =
    useDashboardData(token, onUnauthorized);
  const displayUser = dashboard?.user ?? user;

  if (!token) {
    return <DashboardAuthRequired onNavigate={onNavigate} />;
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

      <SourcesPanel sources={sources} />
    </section>
  );
}

export default DashboardPage;
