import type { AuthResponse } from "../../api/client";
import { AuthForm } from "../../components/auth/AuthForm";

type LoginPageProps = {
  onAuthenticated: (auth: AuthResponse) => void;
  onNavigate: (
    href: string,
    page: "landing" | "dashboard" | "login" | "signup",
  ) => void;
};

function LoginPage({ onAuthenticated, onNavigate }: LoginPageProps) {
  return (
    <AuthForm
      mode="login"
      onAuthenticated={onAuthenticated}
      onNavigate={onNavigate}
    />
  );
}

export default LoginPage;
