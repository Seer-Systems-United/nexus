import { useEffect, useState } from "react";

export type HealthState = "checking" | "online" | "offline";

export function useHealth(): HealthState {
  const [health, setHealth] = useState<HealthState>("checking");

  useEffect(() => {
    let active = true;

    fetch("/health")
      .then((response) => {
        if (!response.ok) {
          throw new Error(`Health check failed with ${response.status}`);
        }
        return response.text();
      })
      .then((body) => {
        if (active) setHealth(body.trim() === "ok" ? "online" : "offline");
      })
      .catch(() => {
        if (active) setHealth("offline");
      });

    return () => {
      active = false;
    };
  }, []);

  return health;
}
