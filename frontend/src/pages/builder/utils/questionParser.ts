export function stripLeadingNumber(text: string): string {
  return text.replace(/^\d+[a-zA-Z]?\.\s*/, "").trim();
}

export function extractBaseQuestion(title: string, prompt: string): string {
  if (prompt && prompt.trim().length > 0) {
    return stripLeadingNumber(prompt);
  }
  if (title.includes(":")) {
    return stripLeadingNumber(title.split(":").slice(1).join(":"));
  }
  return stripLeadingNumber(title);
}

export function extractTitlePrefix(title: string): string {
  if (title.includes(":")) {
    return stripLeadingNumber(title.split(":")[0]);
  }
  return stripLeadingNumber(title);
}
