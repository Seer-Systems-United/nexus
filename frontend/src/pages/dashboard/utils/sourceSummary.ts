import type { SourceCollection, SourceDataStructure } from "../../../api/client";

export function isVisualStructure(structure: SourceDataStructure): boolean {
  return structure.type !== "Unstructured";
}

export function sourceSummary(source: {
  collection: SourceCollection | null;
  error: string | null;
}): string {
  if (!source.collection) {
    return source.error ?? "Source data is unavailable.";
  }

  const structureCount = source.collection.data.length;
  const structureLabel = structureCount === 1 ? "view" : "views";

  if (source.collection.subtitle) {
    return `${structureCount} ${structureLabel} - ${source.collection.subtitle}`;
  }

  return `${structureCount} ${structureLabel} available`;
}

export function collectionSummary(collection: SourceCollection): string {
  const graphCount = collection.data.filter(isVisualStructure).length;
  const graphLabel = graphCount === 1 ? "graph" : "graphs";

  if (collection.subtitle) {
    return `${collection.subtitle} - ${graphCount} ${graphLabel}`;
  }

  return `${graphCount} ${graphLabel} in latest source response`;
}
