export function formatMetricValue(value: number, unit: string): string {
  const formatter = Number.isInteger(value)
    ? new Intl.NumberFormat("en-US", { maximumFractionDigits: 0 })
    : new Intl.NumberFormat("en-US", { maximumFractionDigits: 1 });

  const formatted = formatter.format(value);
  return unit === "%"
    ? `${formatted}%`
    : `${formatted}${unit ? ` ${unit}` : ""}`;
}

export function formatTooltipValue(
  value: number | string | ReadonlyArray<number | string> | undefined,
  unit: string,
): string {
  if (typeof value === "number") {
    return formatMetricValue(value, unit);
  }

  if (typeof value === "string") {
    return value;
  }

  if (Array.isArray(value)) {
    return value.join(", ");
  }

  return "";
}
