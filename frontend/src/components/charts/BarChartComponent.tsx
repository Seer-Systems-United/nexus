import { Bar, BarChart, CartesianGrid, Tooltip, XAxis, YAxis } from "recharts";
import { ResponsiveContainer } from "recharts";
import { CHART_COLORS } from "../../constants/chart";
import { formatTooltipValue } from "../../utils/format";

type BarChartDataPoint = {
  label: string;
  value: number;
};

type BarGraphStructure = {
  title: string;
  y_unit?: string;
  x: string[];
  y: number[];
  data: BarChartDataPoint[];
};

type BarChartProps = {
  data: BarChartDataPoint[];
  title: string;
  y_unit?: string;
  color?: string;
};

export function BarChartComponent({ data, title, y_unit, color }: BarChartProps) {
  return (
    <article className="source-card">
      <div className="source-card-header">
        <h3>{title}</h3>
        <p>{y_unit || "Values"}</p>
      </div>
      <div className="chart-frame">
        <ResponsiveContainer width="100%" height="100%">
          <BarChart
            data={data}
            margin={{ top: 8, right: 8, bottom: 28, left: 0 }}
          >
            <CartesianGrid stroke="#e7edf4" vertical={false} />
            <XAxis
              angle={-35}
              dataKey="label"
              height={72}
              interval={0}
              textAnchor="end"
              tick={{ fontSize: 11 }}
            />
            <YAxis tick={{ fontSize: 11 }} width={52} />
            <Tooltip
              formatter={(value) => formatTooltipValue(value, y_unit || "")}
              labelStyle={{ color: "#182235" }}
              cursor={{ fill: "transparent" }}
            />
            <Bar
              dataKey="value"
              fill={color || CHART_COLORS[0]}
              radius={[6, 6, 0, 0]}
            />
          </BarChart>
        </ResponsiveContainer>
      </div>
    </article>
  );
}
