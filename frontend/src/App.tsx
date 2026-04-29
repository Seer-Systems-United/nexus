import { useCallback, useEffect, useState } from "react";
import type { AuthResponse } from "./api/types";
import nexusLogo from "./assets/nexus_logo_professional.png";
import GraphBuilderPage from "./pages/builder/GraphBuilderPage";
import DashboardPage from "./pages/dashboard/DashboardPage";
import LandingPage from "./pages/landing/LandingPage";
import LoginPage from "./pages/login/LoginPage";
import SignupPage from "./pages/signup/SignupPage";
import type { AuthSession } from "./utils/authStorage";
import { loadStoredSession, saveSession, clearSession } from "./utils/authStorage";
import { authResponseFromCallbackHash } from "./utils/authCallback";

type HealthState = "checking" | "online" | "offline";
type PageKey = "landing" | "dashboard" | "login" | "signup" | "builder";

function pageFromPath(pathname: string): PageKey {
  if (pathname.startsWith("/dashboard")) {
    return "dashboard";
  }

  if (pathname.startsWith("/builder")) {
    return "builder";
  }

  if (pathname.startsWith("/login")) {
    return "login";
  }

  if (pathname.startsWith("/signup")) {
    return "signup";
  }

  return "landing";
}

function App() {
  const [health, setHealth] = useState<HealthState>("checking");
  const [session, setSession] = useState<AuthSession | null>(loadStoredSession);
  const [page, setPage] = useState<PageKey>(() =>
    pageFromPath(window.location.pathname),
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
      const nextSession = saveSession(auth);
      setSession(nextSession);
      navigate("/dashboard", "dashboard");
    },
    [navigate],
  );

  const handleSignOut = useCallback(() => {
    clearSession();
    setSession(null);
    navigate("/login", "login");
  }, [navigate]);

  const handleSessionExpired = useCallback(() => {
    clearSession();
    setSession(null);
  }, []);

  useEffect(() => {
    if (session && (page === "login" || page === "signup")) {
      navigate("/dashboard", "dashboard");
    }
  }, [navigate, page, session]);

  useEffect(() => {
    if (window.location.pathname !== "/auth/callback") {
      return;
    }

    const auth = authResponseFromCallbackHash(window.location.hash);

    if (auth) {
      handleAuthenticated(auth);
    } else {
      navigate("/login", "login");
    }
  }, [handleAuthenticated, navigate]);

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
        <div className="header-spacer" />
        <div className="header-status">
          <div className={`health-pill ${health}`} role="status">
            <span className="dot" aria-hidden="true" />
            {health}
          </div>
          {session ? (
            <details className="account-menu">
              <summary className="header-action">
                {session.user.name || "Account"}
              </summary>
              <div className="account-menu-panel">
                <a
                  href="/dashboard"
                  onClick={(event) => {
                    event.preventDefault();
                    navigate("/dashboard", "dashboard");
                  }}
                >
                  Dashboard
                </a>
                <a
                  href="/builder"
                  onClick={(event) => {
                    event.preventDefault();
                    navigate("/builder", "builder");
                  }}
                >
                  Graph Builder
                </a>
                <button type="button" onClick={handleSignOut}>
                  Sign Out
                </button>
              </div>
            </details>
          ) : (
            <a
              className="header-action"
              href="/login"
              onClick={(event) => {
                event.preventDefault();
                navigate("/login", "login");
              }}
            >
              Login
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
          user={session?.user ?? null}
        />
      )}
      {page === "builder" && (
        <GraphBuilderPage
          onNavigate={navigate}
          token={session?.token ?? null}
        />
      )}
      {page === "login" && (
        <LoginPage
          onAuthenticated={handleAuthenticated}
          onNavigate={navigate}
        />
      )}
      {page === "signup" && (
        <SignupPage
          onAuthenticated={handleAuthenticated}
          onNavigate={navigate}
        />
      )}
    </main>
  );
}

export default App;
