import type { SourceDataStructure } from "../../../api/client";
import { BarChartComponent } from "../../../components/charts";
import { LineChartComponent } from "../../../components/charts";
import { PieChartComponent } from "../../../components/charts";
import { CrosstabTable } from "../../../components/charts";

type SourceStructureProps = {
  structure: SourceDataStructure;
};

export function SourceStructure({ structure }: SourceStructureProps) {
  switch (structure.type) {
    case "BarGraph":
      return (
        <BarChartComponent
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
          data={structure.slices}
          title={structure.title}
          y_unit={structure.y_unit}
        />
      );
    case "Crosstab":
      return (
        <article className="source-card source-card-table">
          <div className="source-card-header">
            <h3>{structure.title}</h3>
            <p>{structure.prompt}</p>
          </div>
          <div className="crosstab-stack">
            {structure.panels.map((panel, index) => (
              <div className="crosstab-frame" key={`${structure.title}-${index}`}>
                <CrosstabTable panel={{ title: structure.title, ...panel }} unit={structure.y_unit} />
              </div>
            ))}
          </div>
        </article>
      );
    case "Unstructured":
      return (
        <article className="source-card source-card-text">
          <div className="source-card-header">
            <h3>Source Notes</h3>
          </div>
          <p className="source-note">{structure.data}</p>
        </article>
      );
  }
}
