import type { AuthResponse } from "./api/types";
import { AppHeader } from "./app/AppHeader";
import { pageFromPath, type PageKey } from "./app/navigation";
import { useHealth } from "./app/useHealth";
import GraphBuilderPage from "./pages/builder/GraphBuilderPage";
import DashboardPage from "./pages/dashboard/DashboardPage";
import LandingPage from "./pages/landing/LandingPage";
import LoginPage from "./pages/login/LoginPage";
import SignupPage from "./pages/signup/SignupPage";
import { authResponseFromCallbackHash } from "./utils/authCallback";
import type { AuthSession } from "./utils/authStorage";
import { clearSession, loadStoredSession, saveSession } from "./utils/authStorage";
import { useCallback, useEffect, useState } from "react";

function App() {
  const health = useHealth();
  const [session, setSession] = useState<AuthSession | null>(loadStoredSession);
  const [page, setPage] = useState<PageKey>(() =>
    pageFromPath(window.location.pathname),
  );

  const navigate = useCallback((href: string, nextPage: PageKey) => {
    window.history.pushState(null, "", href);
    setPage(nextPage);
  }, []);

  const handleAuthenticated = useCallback(
    (auth: AuthResponse) => {
      setSession(saveSession(auth));
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
    const syncPage = () => setPage(pageFromPath(window.location.pathname));
    window.addEventListener("popstate", syncPage);
    return () => window.removeEventListener("popstate", syncPage);
  }, []);

  useEffect(() => {
    if (session && (page === "login" || page === "signup")) {
      navigate("/dashboard", "dashboard");
    }
  }, [navigate, page, session]);

  useEffect(() => {
    if (window.location.pathname !== "/auth/callback") return;
    const auth = authResponseFromCallbackHash(window.location.hash);
    auth ? handleAuthenticated(auth) : navigate("/login", "login");
  }, [handleAuthenticated, navigate]);

  return (
    <main className="app-shell">
      <AppHeader
        health={health}
        onNavigate={navigate}
        onSignOut={handleSignOut}
        session={session}
      />
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
        <GraphBuilderPage onNavigate={navigate} token={session?.token ?? null} />
      )}
      {page === "login" && (
        <LoginPage onAuthenticated={handleAuthenticated} onNavigate={navigate} />
      )}
      {page === "signup" && (
        <SignupPage onAuthenticated={handleAuthenticated} onNavigate={navigate} />
      )}
    </main>
  );
}

export default App;
