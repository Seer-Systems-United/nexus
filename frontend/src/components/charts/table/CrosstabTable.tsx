import { formatMetricValue } from "../../utils/format";

type SourceDataPanel = {
  title: string;
  columns: string[];
  groups: Array<{
    title: string;
    labels: string[];
  }>;
  rows: Array<{
    label: string;
    values: number[];
  }>;
};

type CrosstabTableProps = {
  panel: SourceDataPanel;
  unit: string;
};

export function CrosstabTable({ panel, unit }: CrosstabTableProps) {
  const groupedColumnsCount = panel.groups.reduce(
    (acc, group) => acc + Math.max(group.labels.length, 1),
    0,
  );
  const emptySpan = Math.max(0, panel.columns.length - groupedColumnsCount);
  const tableMinWidth = `max(100%, ${180 + panel.columns.length * 132}px)`;

  return (
    <div className="crosstab-scroll">
      <table className="crosstab-table" style={{ minWidth: tableMinWidth }}>
        <colgroup>
          <col className="crosstab-response-column" />
          {panel.columns.map((column, index) => (
            <col
              className="crosstab-value-column"
              key={`${column}-col-${index}`}
            />
          ))}
        </colgroup>
        <thead>
          {panel.groups.length > 0 ? (
            <>
              <tr>
                <th
                  className="crosstab-response-heading"
                  scope="col"
                  rowSpan={2}
                >
                  Response
                </th>
                {emptySpan > 0 && (
                  <th
                    aria-label="Ungrouped columns"
                    className="crosstab-group-heading"
                    colSpan={emptySpan}
                    scope="colgroup"
                  />
                )}
                {panel.groups.map((group, index) => (
                  <th
                    className="crosstab-group-heading"
                    colSpan={Math.max(group.labels.length, 1)}
                    key={`${group.title}-${index}`}
                    scope="colgroup"
                  >
                    {group.title}
                  </th>
                ))}
              </tr>
              <tr>
                {panel.columns.map((column, index) => (
                  <th
                    className="crosstab-column-heading"
                    key={`${column}-${index}`}
                    scope="col"
                  >
                    {column}
                  </th>
                ))}
              </tr>
            </>
          ) : (
            <tr>
              <th className="crosstab-response-heading" scope="col">
                Response
              </th>
              {panel.columns.map((column, index) => (
                <th
                  className="crosstab-column-heading"
                  key={`${column}-${index}`}
                  scope="col"
                >
                  {column}
                </th>
              ))}
            </tr>
          )}
        </thead>
        <tbody>
          {panel.rows.map((row, rowIndex) => (
            <tr key={`${row.label}-${rowIndex}`}>
              <th className="crosstab-row-heading" scope="row">
                {row.label}
              </th>
              {panel.columns.map((_column, index) => {
                const value = row.values[index];

                return (
                  <td key={`${row.label}-${index}`}>
                    {typeof value === "number"
                      ? formatMetricValue(value, unit)
                      : ""}
                  </td>
                );
              })}
            </tr>
          ))}
        </tbody>
      </table>
    </div>
  );
}
