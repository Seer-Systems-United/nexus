import { ApiRequestError } from "./types";
import type {
  ApiUser,
  AuthResponse,
  LoginRequest,
  SignupRequest,
  DashboardMetric,
  DashboardResponse,
  SourceSummary,
  SourceDataSeries,
  SourceDataSlice,
  SourceDataRow,
  SourceDataGroup,
  SourceDataPanel,
  SourceDataStructure,
  SourceCollection,
} from "./types";

export { ApiRequestError };
export type {
  ApiUser,
  AuthResponse,
  LoginRequest,
  SignupRequest,
  DashboardMetric,
  DashboardResponse,
  SourceSummary,
  SourceDataSeries,
  SourceDataSlice,
  SourceDataRow,
  SourceDataGroup,
  SourceDataPanel,
  SourceDataStructure,
  SourceCollection,
};

type ApiErrorBody = {
  error?: string;
  message?: string;
};

export async function login(request: LoginRequest): Promise<AuthResponse> {
  return postJson<AuthResponse>("/api/v1/auth/login", request);
}

export async function signup(request: SignupRequest): Promise<AuthResponse> {
  return postJson<AuthResponse>("/api/v1/auth/signup", request);
}

export async function getDashboard(token: string): Promise<DashboardResponse> {
  return requestJson<DashboardResponse>("/api/v1/dashboard", {
    headers: {
      authorization: `Bearer ${token}`,
    },
  });
}

export async function listSources(): Promise<SourceSummary[]> {
  return requestJson<SourceSummary[]>("/api/v1/sources");
}

export async function getSource(
  source: string,
  scope?: string,
  count?: number,
  question?: string,
): Promise<SourceCollection> {
  const params = new URLSearchParams();
  if (scope) params.append("scope", scope);
  if (count) params.append("count", count.toString());
  if (question) params.append("question", question);

  const query = params.toString();
  const url = `/api/v1/sources/${source}${query ? `?${query}` : ""}`;

  return requestJson<SourceCollection>(url);
}

async function postJson<TResponse>(
  url: string,
  body: Record<string, string>,
): Promise<TResponse> {
  return requestJson<TResponse>(url, {
    method: "POST",
    headers: {
      "content-type": "application/json",
    },
    body: JSON.stringify(body),
  });
}

async function requestJson<TResponse>(
  url: string,
  init?: RequestInit,
): Promise<TResponse> {
  const response = await fetch(url, init);

  if (!response.ok) {
    const errorBody = await parseErrorBody(response);
    throw new ApiRequestError(
      response.status,
      errorBody.message || "Request failed",
      errorBody.error,
    );
  }

  return response.json() as Promise<TResponse>;
}

async function parseErrorBody(response: Response): Promise<ApiErrorBody> {
  const contentType = response.headers.get("content-type") || "";

  if (!contentType.includes("application/json")) {
    return {};
  }

  try {
    return (await response.json()) as ApiErrorBody;
  } catch {
    return {};
  }
}
