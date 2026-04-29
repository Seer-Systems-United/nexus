import { getSource, listSources, type SourceCollection } from "../../../api/client";

export type SourceRow = {
  id: string;
  name: string;
  status: string;
  tone: "online" | "review";
  collection: SourceCollection | null;
  error: string | null;
};

export async function loadSourceRows(): Promise<SourceRow[]> {
  const sourceSummaries = await listSources();
  const sourceResults = await Promise.allSettled(
    sourceSummaries.map((source) => getSource(source.id, "latest")),
  );

  return sourceSummaries.map((source, index) => {
    const result = sourceResults[index];

    if (result.status === "fulfilled") {
      return {
        id: source.id,
        name: source.name,
        status: "Synced",
        tone: "online" as const,
        collection: result.value,
        error: null,
      };
    }

    return {
      id: source.id,
      name: source.name,
      status: "Unavailable",
      tone: "review" as const,
      collection: null,
      error:
        result.reason instanceof Error
          ? result.reason.message
          : "Source data is unavailable.",
    };
  });
}
