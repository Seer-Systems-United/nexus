type FetchDataFormProps = {
  source: string;
  scope: string;
  count: number;
  keyword: string;
  isLoading: boolean;
  onSourceChange: (value: string) => void;
  onScopeChange: (value: string) => void;
  onCountChange: (value: number) => void;
  onKeywordChange: (value: string) => void;
  onFetch: () => void;
};

export function FetchDataForm({
  source,
  scope,
  count,
  keyword,
  isLoading,
  onSourceChange,
  onScopeChange,
  onCountChange,
  onKeywordChange,
  onFetch,
}: FetchDataFormProps) {
  return (
    <div
      className="dashboard-state"
      style={{ width: "100%", marginBottom: "16px" }}
    >
      <h2>1. Fetch Data</h2>
      <div
        style={{
          display: "grid",
          gridTemplateColumns: "repeat(auto-fit, minmax(200px, 1fr))",
          gap: "16px",
        }}
      >
        <label>
          Source
          <select value={source} onChange={(e) => onSourceChange(e.target.value)}>
            <option value="yougov">YouGov</option>
            <option value="emerson">Emerson</option>
            <option value="gallup">Gallup</option>
          </select>
        </label>
        <label>
          Scope
          <select value={scope} onChange={(e) => onScopeChange(e.target.value)}>
            <option value="last_years">Last N Years</option>
            <option value="last_months">Last N Months</option>
            <option value="last_n_entries">Last N Entries</option>
            <option value="latest">Latest</option>
          </select>
        </label>
        <label>
          Count (N)
          <input
            type="number"
            min="1"
            value={count}
            onChange={(e) => onCountChange(parseInt(e.target.value, 10) || 1)}
          />
        </label>
        <label>
          Keyword Filter
          <input
            type="text"
            value={keyword}
            placeholder="e.g. trump, economy"
            onChange={(e) => onKeywordChange(e.target.value)}
          />
        </label>
      </div>
      <div className="action-row" style={{ marginTop: "16px" }}>
        <button
          className="button primary"
          onClick={onFetch}
          disabled={isLoading}
        >
          {isLoading ? "Fetching..." : "Fetch Data"}
        </button>
      </div>
    </div>
  );
}
