"use client";

import { useAccount, useMsal } from "@azure/msal-react";
import useSWR from "swr";
import { authenticatedFetch } from "@/lib/fetch";
import { apiService } from "@/services/api/client";

export const apiKeys = {
  chat: () => apiService.getUrl("/chats"),
  image: () => apiService.getUrl("/images"),
};

async function imageFetcher<T>(url: string): Promise<T> {
  const response = await authenticatedFetch(url, {
    method: "POST",
    headers: {
      "Content-Type": "application/json",
    },
    body: JSON.stringify({
      prompt: "Show me a playful Toller dog!",
    }),
  });

  if (!response.ok) {
    throw new Error("An error occurred while fetching the data.");
  }

  return response.json();
}

export function useImage() {
  const { accounts } = useMsal();
  const account = useAccount(accounts[0] || undefined);

  const key = account ? apiKeys.image() : null;

  const swr = useSWR(key, account ? (url: string) => imageFetcher(url) : null);

  return swr;
}
