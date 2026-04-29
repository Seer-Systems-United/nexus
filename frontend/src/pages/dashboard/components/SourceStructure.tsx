import type { SourceCollection, SourceDataStructure } from "../../../api/client";
import { BarChartComponent } from "../../../components/charts";
import { LineChartComponent } from "../../../components/charts";
import { PieChartComponent } from "../../../components/charts";
import { CrosstabTable } from "../../../components/charts";

function isVisualStructure(structure: SourceDataStructure): boolean {
  return structure.type !== "Unstructured";
}

function sourceSummary(source: {
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

function collectionSummary(collection: SourceCollection): string {
  const graphCount = collection.data.filter(isVisualStructure).length;
  const graphLabel = graphCount === 1 ? "graph" : "graphs";

  if (collection.subtitle) {
    return `${collection.subtitle} - ${graphCount} ${graphLabel}`;
  }

  return `${graphCount} ${graphLabel} in latest source response`;
}

type SourceStructureProps = {
  structure: SourceDataStructure;
  key: string;
};

export function SourceStructure({ structure, key }: SourceStructureProps) {
  switch (structure.type) {
    case "BarGraph":
      return (
        <BarChartComponent
          key={key}
          data={structure.x.map((label, index) => ({
            label,
            value: structure.y[index] ?? 0,
          }))}
          title={structure.title}
          y_unit={structure.y_unit}
        />
      );
    case "LineGraph":
      return (
        <LineChartComponent
          key={key}
          data={structure.x.map((label, index) => {
            const row: { label: string; [key: string]: number | string | null } = { label };
            for (const series of structure.series) {
              row[series.label] = series.values[index] ?? null;
            }
            return row as any;
          })}
          title={structure.title}
          keys={structure.series.map((s) => s.label)}
          y_unit={structure.y_unit}
        />
      );
    case "PieChart":
      return (
        <PieChartComponent
          key={key}
          data={structure.slices}
          title={structure.title}
          y_unit={structure.y_unit}
        />
      );
    case "Crosstab":
      return (
        <article className="source-card source-card-table" key={key}>
          <div className="source-card-header">
            <h3>{structure.title}</h3>
            <p>{structure.prompt}</p>
          </div>
          <div className="crosstab-stack">
            {structure.panels.map((panel, index) => (
              <div className="crosstab-frame" key={`${key}-panel-${index}`}>
                <CrosstabTable panel={{ title: structure.title, ...panel }} unit={structure.y_unit} />
              </div>
            ))}
          </div>
        </article>
      );
    case "Unstructured":
      return (
        <article className="source-card source-card-text" key={key}>
          <div className="source-card-header">
            <h3>Source Notes</h3>
          </div>
          <p className="source-note">{structure.data}</p>
        </article>
      );
  }
}

export { sourceSummary, collectionSummary, isVisualStructure };
