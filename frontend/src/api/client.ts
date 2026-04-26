export type ApiUser = {
  id: string;
  name: string;
  email: string;
  created_at: string;
};

export type AuthResponse = {
  token: string;
  token_type: "Bearer";
  expires_in: number;
  user: ApiUser;
};

export type LoginRequest = {
  email: string;
  password: string;
};

export type SignupRequest = LoginRequest & {
  name: string;
};

export type DashboardMetric = {
  label: string;
  value: string;
  status: string;
};

export type DashboardResponse = {
  user: ApiUser;
  metrics: DashboardMetric[];
};

type ApiErrorBody = {
  error?: string;
  message?: string;
};

export class ApiRequestError extends Error {
  readonly status: number;
  readonly code?: string;

  constructor(status: number, message: string, code?: string) {
    super(message);
    this.name = "ApiRequestError";
    this.status = status;
    this.code = code;
  }
}

export async function login(request: LoginRequest): Promise<AuthResponse> {
  return postJson<AuthResponse>("/api/v1/auth/login", request);
}

export async function signup(request: SignupRequest): Promise<AuthResponse> {
  return postJson<AuthResponse>("/api/v1/auth/signup", request);
}

export async function getDashboard(token: string): Promise<DashboardResponse> {
  return requestJson<DashboardResponse>("/api/v1/dashboard/", {
    headers: {
      authorization: `Bearer ${token}`
    }
  });
}

async function postJson<TResponse>(
  url: string,
  body: Record<string, string>
): Promise<TResponse> {
  return requestJson<TResponse>(url, {
    method: "POST",
    headers: {
      "content-type": "application/json"
    },
    body: JSON.stringify(body)
  });
}

async function requestJson<TResponse>(
  url: string,
  init?: RequestInit
): Promise<TResponse> {
  const response = await fetch(url, init);

  if (!response.ok) {
    const errorBody = await parseErrorBody(response);
    throw new ApiRequestError(
      response.status,
      errorBody.message || "Request failed",
      errorBody.error
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
