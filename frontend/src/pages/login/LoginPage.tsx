function LoginPage() {
  return (
    <section className="login-page" aria-labelledby="login-title">
      <form className="login-panel">
        <p className="eyebrow">Secure Access</p>
        <h1 id="login-title">Operator Login</h1>

        <label>
          Email
          <input type="email" name="email" autoComplete="email" />
        </label>

        <label>
          Password
          <input
            type="password"
            name="password"
            autoComplete="current-password"
          />
        </label>

        <button className="button primary" type="submit">
          Sign In
        </button>
      </form>
    </section>
  );
}

export default LoginPage;
