export type PageKey = "landing" | "dashboard" | "login" | "signup" | "builder";

export function pageFromPath(pathname: string): PageKey {
  if (pathname.startsWith("/dashboard")) return "dashboard";
  if (pathname.startsWith("/builder")) return "builder";
  if (pathname.startsWith("/login")) return "login";
  if (pathname.startsWith("/signup")) return "signup";
  return "landing";
}
