"use client";

import type { AccountInfo } from "@azure/msal-browser";
import { useAccount, useMsal } from "@azure/msal-react";
import useSWR from "swr";
import { authenticatedFetch } from "@/lib/fetch";
import { apiService } from "@/services/chat";

export const apiKeys = {
  chat: () => apiService.getUrl("/chat/create"),
};

async function chatFetcher<T>(url: string, account: AccountInfo): Promise<T> {
  const response = await authenticatedFetch(
    url,
    {
      method: "POST",
      headers: {
        "Content-Type": "application/json",
      },
      body: JSON.stringify({
        message: "Tell me a joke about programming!",
      }),
    },
    account,
  );

  if (!response.ok) {
    throw new Error("An error occurred while fetching the data.");
  }

  return response.json();
}

export function useChat() {
  const { accounts } = useMsal();
  const account = useAccount(accounts[0] || undefined);

  // If not authenticated, disable the SWR request
  const key = account ? apiKeys.chat() : null;

  const swr = useSWR(
    key,
    account ? (url: string) => chatFetcher(url, account) : null,
  );

  return swr;
}
