import type { DashboardMetric } from "../../../api/client";

type DashboardMetricsProps = {
  metrics: DashboardMetric[];
  isLoading: boolean;
};

function metricTone(metric: DashboardMetric, index: number): string {
  if (metric.status === "review") {
    return "danger";
  }

  if (index === 0 && metric.status !== "loading") {
    return "strong";
  }

  return "";
}

export function DashboardMetrics({
  metrics,
  isLoading,
}: DashboardMetricsProps) {
  return (
    <div className="dashboard-grid">
      {metrics.map((metric, index) => (
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
  );
}

export function metricsForDisplay(
  dashboard: { metrics: DashboardMetric[] } | null,
  isLoading: boolean,
): DashboardMetric[] {
  if (dashboard) {
    return dashboard.metrics;
  }

  if (isLoading) {
    return [
      { label: "Active federations", value: "...", status: "loading" },
      { label: "Node availability", value: "...", status: "loading" },
      { label: "Ballots synchronized", value: "...", status: "loading" },
    ];
  }

  return [
    { label: "Active federations", value: "0", status: "offline" },
    { label: "Node availability", value: "0%", status: "offline" },
    { label: "Ballots synchronized", value: "0", status: "offline" },
  ];
}
