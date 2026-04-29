import { useMemo } from "react";
import type { SourceCollection, SourceDataStructure } from "../../../api/client";
import { extractBaseQuestion, extractTitlePrefix } from "../utils/questionParser";

type CrosstabStructure = Extract<SourceDataStructure, { type: "Crosstab" }>;

type UseAvailableOptionsReturn = {
  availableQuestions: string[];
  availableDemographics: string[];
};

export function useAvailableOptions(
  collection: SourceCollection | null,
  selectedQuestion: string,
) {
  const availableQuestions = useMemo(() => {
    if (!collection) return [];
    const questions = new Set<string>();
    collection.data.forEach((d) => {
      if (d.type === "Crosstab") {
        questions.add(extractBaseQuestion(d.title, d.prompt));
      }
    });
    return Array.from(questions);
  }, [collection]);

  const availableDemographics = useMemo(() => {
    if (!collection || !selectedQuestion) return [];
    const cols = new Set<string>();
    collection.data.forEach((d) => {
      if (
        d.type === "Crosstab" &&
        extractBaseQuestion(d.title, d.prompt) === selectedQuestion
      ) {
        if (d.panels && d.panels.length > 0) {
          d.panels[0].columns.forEach((c) => cols.add(c));
        }
      }
    });
    return Array.from(cols);
  }, [collection, selectedQuestion]);

  return { availableQuestions, availableDemographics };
}
