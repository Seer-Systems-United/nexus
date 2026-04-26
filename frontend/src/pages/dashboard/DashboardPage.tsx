const federationRows = [
  { name: "Northeast Civic", status: "Synced", polls: 18, tone: "online" },
  { name: "Central County", status: "Review", polls: 7, tone: "review" },
  { name: "Western Municipal", status: "Synced", polls: 11, tone: "online" }
];

function DashboardPage() {
  return (
    <section className="dashboard-page" aria-labelledby="dashboard-title">
      <div className="page-heading">
        <p className="eyebrow">Operations</p>
        <h1 id="dashboard-title">Federation Dashboard</h1>
      </div>

      <div className="dashboard-grid">
        <article className="metric-card strong">
          <span className="metric-value">36</span>
          <span className="metric-label">Open polls</span>
        </article>
        <article className="metric-card">
          <span className="metric-value">142</span>
          <span className="metric-label">Reporting nodes</span>
        </article>
        <article className="metric-card danger">
          <span className="metric-value">3</span>
          <span className="metric-label">Need review</span>
        </article>
      </div>

      <section className="table-panel" aria-label="Federation status">
        <div className="table-header">
          <h2>Federations</h2>
          <span>Live status</span>
        </div>
        <div className="federation-list">
          {federationRows.map((row) => (
            <div className="federation-row" key={row.name}>
              <div>
                <strong>{row.name}</strong>
                <span>{row.polls} active polls</span>
              </div>
              <span className={`status-chip ${row.tone}`}>{row.status}</span>
            </div>
          ))}
        </div>
      </section>
    </section>
  );
}

export default DashboardPage;
