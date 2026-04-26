import { useState, type FormEvent } from "react";
import { ApiRequestError, type AuthResponse, signup } from "../../api/client";

type SignupPageProps = {
  onAuthenticated: (auth: AuthResponse) => void;
  onNavigate: (
    href: string,
    page: "landing" | "dashboard" | "login" | "signup"
  ) => void;
};

function SignupPage({ onAuthenticated, onNavigate }: SignupPageProps) {
  const [name, setName] = useState("");
  const [email, setEmail] = useState("");
  const [password, setPassword] = useState("");
  const [error, setError] = useState<string | null>(null);
  const [isSubmitting, setIsSubmitting] = useState(false);

  const handleSubmit = async (event: FormEvent<HTMLFormElement>) => {
    event.preventDefault();
    setError(null);
    setIsSubmitting(true);

    try {
      const auth = await signup({ name, email, password });
      onAuthenticated(auth);
    } catch (requestError) {
      if (requestError instanceof ApiRequestError) {
        setError(requestError.message);
      } else {
        setError("Unable to create account");
      }
    } finally {
      setIsSubmitting(false);
    }
  };

  return (
    <section className="login-page" aria-labelledby="signup-title">
      <form className="login-panel" onSubmit={handleSubmit}>
        <p className="eyebrow">Secure Access</p>
        <div className="form-heading">
          <h1 id="signup-title">Create Account</h1>
          <p>Register a Nexus operator account.</p>
        </div>

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
            autoComplete="new-password"
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
          {isSubmitting ? "Creating..." : "Create Account"}
        </button>

        <div className="form-separator" aria-hidden="true">
          <span />
          or
          <span />
        </div>

        <a className="button google" href="/api/v1/auth/google/login">
          Continue with Google
        </a>

        <button
          className="link-button"
          type="button"
          onClick={() => {
            onNavigate("/login", "login");
          }}
        >
          Use an existing account
        </button>
      </form>
    </section>
  );
}

export default SignupPage;
