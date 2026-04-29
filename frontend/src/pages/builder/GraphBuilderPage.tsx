import { useEffect, useState } from "react";
import { LineChartComponent } from "../../components/charts";
import { BarChartComponent } from "../../components/charts";
import { FetchDataForm } from "./components/FetchDataForm";
import { ConfigureGraphForm } from "./components/ConfigureGraphForm";
import { useFetchData } from "./hooks/useFetchData";
import { useAvailableOptions } from "./hooks/useAvailableOptions";
import { useChartConfig } from "./hooks/useChartConfig";

type GraphBuilderPageProps = {
  onNavigate: (
    href: string,
    page: "landing" | "dashboard" | "login" | "signup" | "builder",
  ) => void;
  token: string | null;
};

export default function GraphBuilderPage({
  onNavigate,
  token,
}: GraphBuilderPageProps) {
  // Step 1: Fetch Parameters
  const [source, setSource] = useState("yougov");
  const [scope, setScope] = useState("last_years");
  const [count, setCount] = useState(2);
  const [keyword, setKeyword] = useState("trump");

  const { collection, isLoading, error, fetchData } = useFetchData();

  // Step 2: Configuration Parameters
  const [selectedQuestion, setSelectedQuestion] = useState<string>("");
  const [selectedDemographic, setSelectedDemographic] = useState<string>("");
  const [format, setFormat] = useState("LineGraph");

  const { availableQuestions, availableDemographics } = useAvailableOptions(
    collection,
    selectedQuestion,
  );

  const chartConfig = useChartConfig({
    collection: collection?.data ?? null,
    selectedQuestion,
    selectedDemographic,
    format,
  });

  // Auto-select first available options when they change
  useEffect(() => {
    if (
      availableQuestions.length > 0 &&
      selectedQuestion !== "" &&
      !availableQuestions.includes(selectedQuestion)
    ) {
      setSelectedQuestion(availableQuestions[0]);
    }
  }, [availableQuestions, selectedQuestion]);

  useEffect(() => {
    if (
      availableDemographics.length > 0 &&
      !availableDemographics.includes(selectedDemographic)
    ) {
      const defaultDemo = availableDemographics.find(
        (d) => d.toLowerCase() === "total" || d.toLowerCase() === "overall",
      );
      setSelectedDemographic(defaultDemo || availableDemographics[0]);
    }
  }, [availableDemographics, selectedDemographic]);

  const handleFetch = () => {
    fetchData({ source, scope, count, keyword });
  };

  return (
    <section className="dashboard-page" style={{ paddingTop: "28px" }}>
      <div className="dashboard-toolbar">
        <div className="page-heading">
          <p className="eyebrow">Analysis</p>
          <h1>Graph Builder</h1>
        </div>
      </div>

      <FetchDataForm
        source={source}
        scope={scope}
        count={count}
        keyword={keyword}
        isLoading={isLoading}
        onSourceChange={setSource}
        onScopeChange={setScope}
        onCountChange={setCount}
        onKeywordChange={setKeyword}
        onFetch={handleFetch}
      />

      {error && (
        <div
          className="dashboard-state compact"
          role="alert"
          style={{ marginBottom: "16px" }}
        >
          <p>{error}</p>
        </div>
      )}

      {collection && (
        <ConfigureGraphForm
          selectedQuestion={selectedQuestion}
          selectedDemographic={selectedDemographic}
          format={format}
          availableQuestions={availableQuestions}
          availableDemographics={availableDemographics}
          onQuestionChange={setSelectedQuestion}
          onDemographicChange={setSelectedDemographic}
          onFormatChange={setFormat}
        />
      )}

       {chartConfig && chartConfig.type === "LineGraph" && (
         <LineChartComponent
           data={chartConfig.data as any}
           title={chartConfig.title}
           keys={chartConfig.keys}
           y_unit={chartConfig.y_unit}
         />
       )}

      {chartConfig && chartConfig.type === "BarGraph" && (
        <BarChartComponent
          data={chartConfig.data}
          title={chartConfig.title}
          y_unit={chartConfig.y_unit}
        />
      )}
    </section>
  );
}
