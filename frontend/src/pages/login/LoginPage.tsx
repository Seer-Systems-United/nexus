import { useState, type FormEvent } from "react";
import {
  ApiRequestError,
  type AuthResponse,
  login,
  signup
} from "../../api/client";

type LoginPageProps = {
  mode: "login" | "signup";
  onAuthenticated: (auth: AuthResponse) => void;
  onNavigate: (
    href: string,
    page: "landing" | "dashboard" | "login" | "signup"
  ) => void;
};

function LoginPage({ mode, onAuthenticated, onNavigate }: LoginPageProps) {
  const [name, setName] = useState("");
  const [email, setEmail] = useState("");
  const [password, setPassword] = useState("");
  const [error, setError] = useState<string | null>(null);
  const [isSubmitting, setIsSubmitting] = useState(false);
  const isSignup = mode === "signup";
  const submitLabel = isSignup ? "Create Account" : "Sign In";
  const submittingLabel = isSignup ? "Creating..." : "Signing In...";

  const handleSubmit = async (event: FormEvent<HTMLFormElement>) => {
    event.preventDefault();
    setError(null);
    setIsSubmitting(true);

    try {
      const auth = isSignup
        ? await signup({ name, email, password })
        : await login({ email, password });

      onAuthenticated(auth);
    } catch (requestError) {
      if (requestError instanceof ApiRequestError) {
        setError(requestError.message);
      } else {
        setError("Unable to complete the request");
      }
    } finally {
      setIsSubmitting(false);
    }
  };

  const switchMode = () => {
    const nextMode = isSignup ? "login" : "signup";
    onNavigate(`/${nextMode}`, nextMode);
  };

  return (
    <section className="login-page" aria-labelledby="login-title">
      <form className="login-panel" onSubmit={handleSubmit}>
        <p className="eyebrow">Secure Access</p>
        <div className="form-heading">
          <h1 id="login-title">
            {isSignup ? "Create Account" : "Operator Login"}
          </h1>
          <p>
            {isSignup
              ? "Register a Nexus operator account."
              : "Sign in to continue to the dashboard."}
          </p>
        </div>

        {isSignup && (
          <label>
            Name
            <input
              autoComplete="name"
              name="name"
              onChange={(event) => {
                setName(event.target.value);
              }}
              required
              type="text"
              value={name}
            />
          </label>
        )}

        <label>
          Email
          <input
            autoComplete="email"
            name="email"
            onChange={(event) => {
              setEmail(event.target.value);
            }}
            required
            type="email"
            value={email}
          />
        </label>

        <label>
          Password
          <input
            autoComplete={isSignup ? "new-password" : "current-password"}
            minLength={8}
            name="password"
            onChange={(event) => {
              setPassword(event.target.value);
            }}
            required
            type="password"
            value={password}
          />
        </label>

        {error && (
          <p className="form-error" role="alert">
            {error}
          </p>
        )}

        <button className="button primary" disabled={isSubmitting} type="submit">
          {isSubmitting ? submittingLabel : submitLabel}
        </button>

        <button className="link-button" type="button" onClick={switchMode}>
          {isSignup ? "Use an existing account" : "Create a new account"}
        </button>
      </form>
    </section>
  );
}

export default LoginPage;
