import {
  CartesianGrid,
  Legend,
  Line,
  LineChart,
  Tooltip,
  XAxis,
  YAxis,
} from "recharts";
import { ResponsiveContainer } from "recharts";
import { CHART_COLORS } from "../../constants/chart";
import { formatTooltipValue } from "../../utils/format";

type LineChartDataPoint = {
  label: string;
  [key: string]: number | string | null;
};

type LineChartSeries = {
  label: string;
  values: (number | null)[];
};

type LineChartProps = {
  data: LineChartDataPoint[];
  title: string;
  keys: string[];
  y_unit?: string;
};

export function LineChartComponent({
  data,
  title,
  keys,
  y_unit,
}: LineChartProps) {
  return (
    <article className="source-card">
      <div className="source-card-header">
        <h3>{title}</h3>
        <p>{y_unit || "Trend"}</p>
      </div>
      <div className="chart-frame">
        <ResponsiveContainer width="100%" height="100%">
          <LineChart
            data={data}
            margin={{ top: 8, right: 8, bottom: 20, left: 0 }}
          >
            <CartesianGrid stroke="#e7edf4" vertical={false} />
            <XAxis dataKey="label" tick={{ fontSize: 11 }} minTickGap={24} />
            <YAxis tick={{ fontSize: 11 }} width={52} />
            <Tooltip
              formatter={(value) => formatTooltipValue(value, y_unit || "")}
              labelStyle={{ color: "#182235" }}
            />
            <Legend />
            {keys.map((key, index) => (
              <Line
                key={key}
                dataKey={key}
                dot={true}
                stroke={CHART_COLORS[index % CHART_COLORS.length]}
                strokeWidth={2}
                type="monotone"
              />
            ))}
          </LineChart>
        </ResponsiveContainer>
      </div>
    </article>
  );
}
