import { Cell, Pie, PieChart, Tooltip, Legend } from "recharts";
import { ResponsiveContainer } from "recharts";
import { CHART_COLORS } from "../../constants/chart";
import { formatTooltipValue } from "../../utils/format";

type PieSlice = {
  label: string;
  value: number;
};

type PieChartProps = {
  data: PieSlice[];
  title: string;
  y_unit?: string;
};

export function PieChartComponent({ data, title, y_unit }: PieChartProps) {
  return (
    <article className="source-card">
      <div className="source-card-header">
        <h3>{title}</h3>
        <p>{y_unit || "Distribution"}</p>
      </div>
      <div className="chart-frame">
        <ResponsiveContainer width="100%" height="100%">
          <PieChart>
            <Pie
              cx="50%"
              cy="50%"
              data={data}
              dataKey="value"
              nameKey="label"
              outerRadius={84}
            >
              {data.map((slice, index) => (
                <Cell
                  key={`${slice.label}-${index}`}
                  fill={CHART_COLORS[index % CHART_COLORS.length]}
                />
              ))}
            </Pie>
            <Tooltip
              formatter={(value) => formatTooltipValue(value, y_unit || "")}
              labelStyle={{ color: "#182235" }}
            />
            <Legend />
          </PieChart>
        </ResponsiveContainer>
      </div>
    </article>
  );
}
