import type { PageKey } from "./navigation";
import type { HealthState } from "./useHealth";
import nexusLogo from "../assets/nexus_logo_professional.png";
import type { AuthSession } from "../utils/authStorage";
import type { MouseEvent } from "react";

type AppHeaderProps = {
  health: HealthState;
  onNavigate: (href: string, page: PageKey) => void;
  onSignOut: () => void;
  session: AuthSession | null;
};

export function AppHeader({
  health,
  onNavigate,
  onSignOut,
  session,
}: AppHeaderProps) {
  return (
    <header className="app-header">
      <a className="brand" href="/" onClick={(event) => navigate(event, "/", "landing", onNavigate)}>
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
            <summary className="header-action">{session.user.name || "Account"}</summary>
            <div className="account-menu-panel">
              <a href="/dashboard" onClick={(event) => navigate(event, "/dashboard", "dashboard", onNavigate)}>Dashboard</a>
              <a href="/builder" onClick={(event) => navigate(event, "/builder", "builder", onNavigate)}>Graph Builder</a>
              <button type="button" onClick={onSignOut}>Sign Out</button>
            </div>
          </details>
        ) : (
          <a className="header-action" href="/login" onClick={(event) => navigate(event, "/login", "login", onNavigate)}>Login</a>
        )}
      </div>
    </header>
  );
}

function navigate(
  event: MouseEvent<HTMLElement>,
  href: string,
  page: PageKey,
  onNavigate: (href: string, page: PageKey) => void,
) {
  event.preventDefault();
  onNavigate(href, page);
}
