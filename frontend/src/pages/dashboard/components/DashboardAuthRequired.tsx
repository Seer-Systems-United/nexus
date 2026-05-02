type DashboardAuthRequiredProps = {
  onNavigate: (
    href: string,
    page: "landing" | "dashboard" | "login" | "signup",
  ) => void;
};

export function DashboardAuthRequired({
  onNavigate,
}: DashboardAuthRequiredProps) {
  return (
    <section className="dashboard-page" aria-labelledby="dashboard-title">
      <div className="dashboard-state">
        <p className="eyebrow">Authentication Required</p>
        <h1 id="dashboard-title">Sign in to view dashboard access.</h1>
        <div className="action-row">
          <a className="button primary" href="/login" onClick={(event) => {
            event.preventDefault();
            onNavigate("/login", "login");
          }}>
            Sign In
          </a>
          <a className="button secondary" href="/signup" onClick={(event) => {
            event.preventDefault();
            onNavigate("/signup", "signup");
          }}>
            Create Account
          </a>
        </div>
      </div>
    </section>
  );
}
