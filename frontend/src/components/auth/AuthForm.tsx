import { type FormEvent, useState } from "react";
import { ApiRequestError, type AuthResponse, login, signup } from "../../api/client";

type AuthFormProps = {
  mode: "login" | "signup";
  onAuthenticated: (auth: AuthResponse) => void;
  onNavigate: (
    href: string,
    page: "landing" | "dashboard" | "login" | "signup",
  ) => void;
};

type AuthCredentials = {
  account_number?: string;
  name?: string;
  password: string;
};

export function AuthForm({ mode, onAuthenticated, onNavigate }: AuthFormProps) {
  const [accountNumber, setAccountNumber] = useState("");
  const [name, setName] = useState("");
  const [password, setPassword] = useState("");
  const [error, setError] = useState<string | null>(null);
  const [isSubmitting, setIsSubmitting] = useState(false);

  const handleSubmit = async (event: FormEvent<HTMLFormElement>) => {
    event.preventDefault();
    setError(null);
    setIsSubmitting(true);

    try {
      const credentials: AuthCredentials =
        mode === "login"
          ? { account_number: accountNumber, password }
          : { name, password };

      const auth =
        mode === "login"
          ? await login(credentials as any)
          : await signup(credentials as any);

      onAuthenticated(auth);
    } catch (requestError) {
      if (requestError instanceof ApiRequestError) {
        setError(requestError.message);
      } else {
        setError(
          mode === "login" ? "Unable to complete the request" : "Unable to create account",
        );
      }
    } finally {
      setIsSubmitting(false);
    }
  };

  const isLogin = mode === "login";

  return (
    <section className="login-page" aria-labelledby={`${mode}-title`}>
      <form className="login-panel" onSubmit={handleSubmit}>
        <p className="eyebrow">Secure Access</p>
        <div className="form-heading">
          <h1 id={`${mode}-title`}>
            {isLogin ? "Operator Login" : "Create Account"}
          </h1>
          <p>
            {isLogin
              ? "Use your account number to continue to the dashboard."
              : "Register a Nexus webhook account."}
          </p>
        </div>

        {isLogin && (
          <label>
            Account Number
            <input
              autoComplete="username"
              inputMode="numeric"
              name="account_number"
              onChange={(event) => setAccountNumber(event.target.value)}
              pattern="[0-9 -]{16,19}"
              required
              type="text"
              value={accountNumber}
            />
          </label>
        )}

        {!isLogin && (
          <label>
            Name
            <input
              autoComplete="name"
              name="name"
              onChange={(event) => setName(event.target.value)}
              required
              type="text"
              value={name}
            />
          </label>
        )}

        <label>
          Password
          <input
            autoComplete={isLogin ? "current-password" : "new-password"}
            minLength={8}
            name="password"
            onChange={(event) => setPassword(event.target.value)}
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
          {isSubmitting
            ? isLogin
              ? "Signing In..."
              : "Creating..."
            : isLogin
              ? "Sign In"
              : "Create Account"}
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
            onNavigate(isLogin ? "/signup" : "/login", isLogin ? "signup" : "login");
          }}
        >
          {isLogin ? "Create a new account" : "Use an existing account"}
        </button>
      </form>
    </section>
  );
}
