export type ApiUser = {
  id: string;
  name: string;
  email: string | null;
  account_number: string | null;
  created_at: string;
};

export type AuthResponse = {
  token: string;
  token_type: "Bearer";
  expires_in: number;
  user: ApiUser;
};

export type LoginRequest = {
  account_number: string;
  password: string;
};

export type SignupRequest = {
  name: string;
  password: string;
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

export type SourceSummary = {
  id: string;
  name: string;
};

export type SourceDataSeries = {
  label: string;
  values: number[];
};

export type SourceDataSlice = {
  label: string;
  value: number;
};

export type SourceDataRow = {
  label: string;
  values: number[];
};

export type SourceDataGroup = {
  title: string;
  labels: string[];
};

export type SourceDataPanel = {
  columns: string[];
  groups: SourceDataGroup[];
  rows: SourceDataRow[];
};

export type SourceDataStructure =
  | {
      type: "Unstructured";
      data: string;
    }
  | {
      type: "BarGraph";
      title: string;
      x: string[];
      y: number[];
      y_unit: string;
    }
  | {
      type: "LineGraph";
      title: string;
      x: string[];
      series: SourceDataSeries[];
      y_unit: string;
    }
  | {
      type: "PieChart";
      title: string;
      slices: SourceDataSlice[];
      y_unit: string;
    }
  | {
      type: "Crosstab";
      title: string;
      prompt: string;
      panels: SourceDataPanel[];
      y_unit: string;
    };

export type SourceCollection = {
  title: string;
  subtitle: string | null;
  data: SourceDataStructure[];
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
