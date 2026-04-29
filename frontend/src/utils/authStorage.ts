import type { ApiUser, AuthResponse } from "../api/client";

export type AuthSession = {
  token: string;
  expiresAt: number;
  user: ApiUser;
};

const authStorageKey = "nexus.auth";

export function loadStoredSession(): AuthSession | null {
  const storedSession = window.localStorage.getItem(authStorageKey);

  if (!storedSession) {
    return null;
  }

  try {
    const parsed = JSON.parse(storedSession) as AuthSession;

    if (!parsed.token || !parsed.user || parsed.expiresAt <= Date.now()) {
      window.localStorage.removeItem(authStorageKey);
      return null;
    }

    return parsed;
  } catch {
    window.localStorage.removeItem(authStorageKey);
    return null;
  }
}

export function saveSession(auth: AuthResponse): AuthSession {
  const session: AuthSession = {
    token: auth.token,
    expiresAt: Date.now() + auth.expires_in * 1000,
    user: auth.user,
  };

  window.localStorage.setItem(authStorageKey, JSON.stringify(session));
  return session;
}

export function clearSession(): void {
  window.localStorage.removeItem(authStorageKey);
}
