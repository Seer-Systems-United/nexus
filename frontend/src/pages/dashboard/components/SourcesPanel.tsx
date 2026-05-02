import { SourceStructure } from "./SourceStructure";
import { collectionSummary, sourceSummary } from "../utils/sourceSummary";
import type { SourceRow } from "../utils/dataLoader";

type SourcesPanelProps = {
  sources: SourceRow[];
};

export function SourcesPanel({ sources }: SourcesPanelProps) {
  return (
    <section className="table-panel source-panel" aria-label="Polling sources">
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
                  {row.collection.data.map((structure, index) => (
                    <SourceStructure
                      key={`${row.id}-${index}`}
                      structure={structure}
                    />
                  ))}
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
  );
}
