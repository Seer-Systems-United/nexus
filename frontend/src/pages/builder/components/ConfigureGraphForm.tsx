type ConfigureGraphFormProps = {
  selectedQuestion: string;
  selectedDemographic: string;
  format: string;
  availableQuestions: string[];
  availableDemographics: string[];
  onQuestionChange: (value: string) => void;
  onDemographicChange: (value: string) => void;
  onFormatChange: (value: string) => void;
};

export function ConfigureGraphForm({
  selectedQuestion,
  selectedDemographic,
  format,
  availableQuestions,
  availableDemographics,
  onQuestionChange,
  onDemographicChange,
  onFormatChange,
}: ConfigureGraphFormProps) {
  return (
    <div
      className="dashboard-state"
      style={{ width: "100%", marginBottom: "16px" }}
    >
      <h2>2. Configure Graph</h2>
      <div
        style={{
          display: "grid",
          gridTemplateColumns: "repeat(auto-fit, minmax(200px, 1fr))",
          gap: "16px",
        }}
      >
        <label style={{ minWidth: 0 }}>
          Question / Concept
          <select
            value={selectedQuestion}
            onChange={(e) => onQuestionChange(e.target.value)}
            style={{
              width: "100%",
              textOverflow: "ellipsis",
              overflow: "hidden",
            }}
          >
            <option value="">All Questions</option>
            {availableQuestions.map((q) => (
              <option key={q} value={q}>
                {q}
              </option>
            ))}
          </select>
        </label>
        <label style={{ minWidth: 0 }}>
          Demographic / Category
          <select
            value={selectedDemographic}
            onChange={(e) => onDemographicChange(e.target.value)}
            style={{
              width: "100%",
              textOverflow: "ellipsis",
              overflow: "hidden",
            }}
          >
            {availableDemographics.map((d) => (
              <option key={d} value={d}>
                {d}
              </option>
            ))}
          </select>
        </label>
        <label style={{ minWidth: 0 }}>
          Format
          <select
            value={format}
            onChange={(e) => onFormatChange(e.target.value)}
            style={{
              width: "100%",
              textOverflow: "ellipsis",
              overflow: "hidden",
            }}
          >
            <option value="LineGraph">Line Graph (Trend over time)</option>
            <option value="BarGraph">Bar Graph (Latest snapshot)</option>
          </select>
        </label>
      </div>
    </div>
  );
}
