import { useCallback, useEffect, useMemo, useState } from "react";
import type { ApiUser, AuthResponse } from "./api/client";
import nexusLogo from "./assets/nexus_logo_professional.png";
import DashboardPage from "./pages/dashboard/DashboardPage";
import LandingPage from "./pages/landing/LandingPage";
import LoginPage from "./pages/login/LoginPage";

type HealthState = "checking" | "online" | "offline";
type PageKey = "landing" | "dashboard" | "login" | "signup";
type AuthSession = {
  token: string;
  expiresAt: number;
  user: ApiUser;
};

const authStorageKey = "nexus.auth";

const navItems: Array<{ href: string; label: string; page: PageKey }> = [
  { href: "/", label: "Overview", page: "landing" },
  { href: "/dashboard", label: "Dashboard", page: "dashboard" },
  { href: "/login", label: "Login", page: "login" }
];

function pageFromPath(pathname: string): PageKey {
  if (pathname.startsWith("/dashboard")) {
    return "dashboard";
  }

  if (pathname.startsWith("/login")) {
    return "login";
  }

  if (pathname.startsWith("/signup")) {
    return "signup";
  }

  return "landing";
}

function loadStoredSession(): AuthSession | null {
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

function App() {
  const [health, setHealth] = useState<HealthState>("checking");
  const [session, setSession] = useState<AuthSession | null>(loadStoredSession);
  const [page, setPage] = useState<PageKey>(() =>
    pageFromPath(window.location.pathname)
  );
  const visibleNavItems = useMemo(
    () =>
      session
        ? navItems.filter((item) => item.page !== "login")
        : navItems,
    [session]
  );

  useEffect(() => {
    let active = true;

    fetch("/health")
      .then((response) => {
        if (!response.ok) {
          throw new Error(`Health check failed with ${response.status}`);
        }

        return response.text();
      })
      .then((body) => {
        if (active) {
          setHealth(body.trim() === "ok" ? "online" : "offline");
        }
      })
      .catch(() => {
        if (active) {
          setHealth("offline");
        }
      });

    return () => {
      active = false;
    };
  }, []);

  useEffect(() => {
    const syncPage = () => {
      setPage(pageFromPath(window.location.pathname));
    };

    window.addEventListener("popstate", syncPage);

    return () => {
      window.removeEventListener("popstate", syncPage);
    };
  }, []);

  const navigate = useCallback((href: string, nextPage: PageKey) => {
    window.history.pushState(null, "", href);
    setPage(nextPage);
  }, []);

  const handleAuthenticated = useCallback(
    (auth: AuthResponse) => {
      const nextSession = {
        token: auth.token,
        expiresAt: Date.now() + auth.expires_in * 1000,
        user: auth.user
      };

      window.localStorage.setItem(authStorageKey, JSON.stringify(nextSession));
      setSession(nextSession);
      navigate("/dashboard", "dashboard");
    },
    [navigate]
  );

  const handleSignOut = useCallback(() => {
    window.localStorage.removeItem(authStorageKey);
    setSession(null);
    navigate("/login", "login");
  }, [navigate]);

  const handleSessionExpired = useCallback(() => {
    window.localStorage.removeItem(authStorageKey);
    setSession(null);
  }, []);

  return (
    <main className="app-shell">
      <header className="app-header">
        <a
          className="brand"
          href="/"
          onClick={(event) => {
            event.preventDefault();
            navigate("/", "landing");
          }}
        >
          <img src={nexusLogo} alt="Nexus" />
        </a>
        <nav className="primary-nav" aria-label="Primary navigation">
          {visibleNavItems.map((item) => (
            <a
              aria-current={page === item.page ? "page" : undefined}
              href={item.href}
              key={item.href}
              onClick={(event) => {
                event.preventDefault();
                navigate(item.href, item.page);
              }}
            >
              {item.label}
            </a>
          ))}
        </nav>
        <div className="header-status">
          <div className={`health-pill ${health}`} role="status">
            <span className="dot" aria-hidden="true" />
            {health}
          </div>
          {session ? (
            <button className="header-action" type="button" onClick={handleSignOut}>
              Sign Out
            </button>
          ) : (
            <a
              className="header-action"
              href="/signup"
              onClick={(event) => {
                event.preventDefault();
                navigate("/signup", "signup");
              }}
            >
              Sign Up
            </a>
          )}
        </div>
      </header>

      {page === "landing" && (
        <LandingPage
          isAuthenticated={Boolean(session)}
          onNavigate={navigate}
          userName={session?.user.name}
        />
      )}
      {page === "dashboard" && (
        <DashboardPage
          onNavigate={navigate}
          onUnauthorized={handleSessionExpired}
          token={session?.token ?? null}
        />
      )}
      {(page === "login" || page === "signup") && (
        <LoginPage
          mode={page}
          onAuthenticated={handleAuthenticated}
          onNavigate={navigate}
        />
      )}
    </main>
  );
}

export default App;
