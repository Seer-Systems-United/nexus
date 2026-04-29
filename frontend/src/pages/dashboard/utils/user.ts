import type { ApiUser } from "../../../api/client";

export function accountIdentifier(user: ApiUser): string {
  if (user.account_number) {
    return `Account ${formatAccountNumber(user.account_number)}`;
  }

  return user.email ?? "OpenID account";
}

function formatAccountNumber(accountNumber: string): string {
  return accountNumber.replace(/(\d{4})(?=\d)/g, "$1 ");
}
