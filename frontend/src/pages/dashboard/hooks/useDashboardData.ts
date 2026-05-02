import { useCallback, useEffect, useState } from "react";
import { getDashboard, type DashboardResponse } from "../../../api/client";
import { loadSourceRows, type SourceRow } from "../utils/dataLoader";

export function useDashboardData(
  token: string | null,
  onUnauthorized: () => void,
) {
  const [dashboard, setDashboard] = useState<DashboardResponse | null>(null);
  const [sources, setSources] = useState<SourceRow[]>([]);
  const [error, setError] = useState<string | null>(null);
  const [isLoading, setIsLoading] = useState(false);

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
    } else if (isUnauthorized(dashboardResult.reason)) {
      onUnauthorized();
      setDashboard(null);
      setError("Your session expired. Sign in again to continue.");
      setIsLoading(false);
      return;
    } else {
      setError(errorMessage(dashboardResult.reason));
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

  return { dashboard, sources, error, isLoading, loadDashboard };
}

function isUnauthorized(error: unknown): boolean {
  return error instanceof Error && "status" in error && error.status === 401;
}

function errorMessage(error: unknown): string {
  return error instanceof Error ? error.message : "Unable to load dashboard data";
}
