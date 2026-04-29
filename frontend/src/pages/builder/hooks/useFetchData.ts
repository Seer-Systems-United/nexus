import { useState, useCallback } from "react";
import { getSource, type SourceCollection } from "../../../api/client";

type FetchDataParams = {
  source: string;
  scope: string;
  count: number;
  keyword: string;
};

type UseFetchDataReturn = {
  collection: SourceCollection | null;
  isLoading: boolean;
  error: string | null;
  fetchData: (params: FetchDataParams) => Promise<void>;
};

export function useFetchData(): UseFetchDataReturn {
  const [collection, setCollection] = useState<SourceCollection | null>(null);
  const [isLoading, setIsLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);

  const fetchData = useCallback(async (params: FetchDataParams) => {
    setIsLoading(true);
    setError(null);
    setCollection(null);

    try {
      const result = await getSource(
        params.source,
        params.scope,
        params.count,
        params.keyword,
      );
      if (!result.data.some((d) => d.type === "Crosstab")) {
        setError("No crosstab data found for this keyword/source combination.");
      } else {
        setCollection(result);
      }
    } catch (err: unknown) {
      const error = err instanceof Error ? err : new Error(String(err));
      setError(error.message || "Failed to load data.");
    } finally {
      setIsLoading(false);
    }
  }, []);

  return { collection, isLoading, error, fetchData };
}
