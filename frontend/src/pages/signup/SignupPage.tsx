import type { AuthResponse } from "../../api/client";
import { AuthForm } from "../../components/auth/AuthForm";

type SignupPageProps = {
  onAuthenticated: (auth: AuthResponse) => void;
  onNavigate: (
    href: string,
    page: "landing" | "dashboard" | "login" | "signup",
  ) => void;
};

function SignupPage({ onAuthenticated, onNavigate }: SignupPageProps) {
  return (
    <AuthForm
      mode="signup"
      onAuthenticated={onAuthenticated}
      onNavigate={onNavigate}
    />
  );
}

export default SignupPage;
