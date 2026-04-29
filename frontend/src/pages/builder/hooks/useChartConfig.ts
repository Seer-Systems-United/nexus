import { useMemo } from "react";
import type { SourceDataStructure } from "../../../api/client";
import { extractBaseQuestion, extractTitlePrefix } from "../utils/questionParser";

type CrosstabStructure = Extract<SourceDataStructure, { type: "Crosstab" }>;

type ChartConfig =
  | {
      type: "LineGraph";
      title: string;
      data: Record<string, number | string | null>[];
      keys: string[];
      y_unit: string;
    }
  | {
      type: "BarGraph";
      title: string;
      data: { label: string; value: number }[];
      keys: string[];
      y_unit: string;
    }
  | null;

type UseChartConfigProps = {
  collection: SourceDataStructure[] | null;
  selectedQuestion: string;
  selectedDemographic: string;
  format: string;
};

export function useChartConfig({
  collection,
  selectedQuestion,
  selectedDemographic,
  format,
}: UseChartConfigProps): ChartConfig {
  return useMemo(() => {
    if (!collection || !selectedDemographic) return null;
    if (selectedQuestion === "") return null;

    const matchingCrosstabs = collection.filter(
      (d) =>
        d.type === "Crosstab" &&
        (selectedQuestion === "" ||
          extractBaseQuestion(d.title, d.prompt) === selectedQuestion),
    ) as CrosstabStructure[];

    if (matchingCrosstabs.length === 0) return null;

    const chronological = [...matchingCrosstabs].reverse();
    const y_unit = chronological[0].y_unit || "%";

    if (format === "LineGraph") {
      const lineKeys = new Set<string>();
      const data: Record<string, number | string | null>[] = [];

      chronological.forEach((c, i) => {
        const prefix = extractTitlePrefix(c.title);
        const question = extractBaseQuestion(c.title, c.prompt);
        const label = prefix ? `${prefix} (Poll ${i + 1})` : `Poll ${i + 1}`;

        let row = data.find((d) => d.label === label);
        if (!row) {
          row = { label };
          data.push(row);
        }

        const panel = c.panels[0];
        if (panel) {
          const colIndex = panel.columns.findIndex(
            (col) => col.toLowerCase() === selectedDemographic.toLowerCase(),
          );
          const valIndex = colIndex >= 0 ? colIndex : 0;

          if (panel.rows.length > 0) {
            row[question] = panel.rows[0].values[valIndex] ?? null;
            lineKeys.add(question);
          }
        }
      });

      return {
        type: "LineGraph" as const,
        title: `Trends by Question (${selectedDemographic})`,
        data,
        keys: Array.from(lineKeys),
        y_unit,
      };
    } else {
      const latest = matchingCrosstabs[0];
      const panel = latest.panels[0];

      if (!panel) return null;

      const colIndex = panel.columns.findIndex(
        (col) => col.toLowerCase() === selectedDemographic.toLowerCase(),
      );
      const valIndex = colIndex >= 0 ? colIndex : 0;

      const data = panel.rows.map((r) => ({
        label: r.label,
        value: r.values[valIndex] ?? 0,
      }));

      return {
        type: "BarGraph" as const,
        title: `${extractTitlePrefix(latest.title)}: ${selectedQuestion} (${selectedDemographic})`,
        data,
        keys: ["value"],
        y_unit,
      };
    }
  }, [collection, selectedQuestion, selectedDemographic, format]);
}
