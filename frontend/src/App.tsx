import { useEffect, useState } from "react";
import nexusLogo from "./assets/nexus_logo_professional.png";
import DashboardPage from "./pages/dashboard/DashboardPage";
import LandingPage from "./pages/landing/LandingPage";
import LoginPage from "./pages/login/LoginPage";

type HealthState = "checking" | "online" | "offline";
type PageKey = "landing" | "dashboard" | "login";

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

  return "landing";
}

function App() {
  const [health, setHealth] = useState<HealthState>("checking");
  const [page, setPage] = useState<PageKey>(() =>
    pageFromPath(window.location.pathname)
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

  const navigate = (href: string, nextPage: PageKey) => {
    window.history.pushState(null, "", href);
    setPage(nextPage);
  };

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
          {navItems.map((item) => (
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
        <div className={`health-pill ${health}`} role="status">
          <span className="dot" aria-hidden="true" />
          {health}
        </div>
      </header>

      {page === "landing" && <LandingPage onNavigate={navigate} />}
      {page === "dashboard" && <DashboardPage />}
      {page === "login" && <LoginPage />}
    </main>
  );
}

export default App;
