import type { AuthResponse } from "../api/client";

export function authResponseFromCallbackHash(hash: string): AuthResponse | null {
  const params = new URLSearchParams(hash.replace(/^#/, ""));
  const token = params.get("token");
  const expiresIn = Number(params.get("expires_in"));
  const user = {
    id: params.get("user_id") || "",
    name: params.get("user_name") || "",
    email: params.get("user_email"),
    account_number: params.get("user_account_number"),
    created_at: params.get("user_created_at") || "",
  };

  if (!token || !Number.isFinite(expiresIn) || expiresIn <= 0 || !user.id) {
    return null;
  }

  return {
    token,
    token_type: "Bearer",
    expires_in: expiresIn,
    user,
  };
}
